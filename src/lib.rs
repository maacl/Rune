#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod iroh_local;
mod message;
mod templates;
mod ticket;
mod topic;

use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{Context, Result};
use futures_lite::StreamExt;
use tokio::sync::Mutex;

use iroh_gossip::{
    net::{Event, GossipEvent, GossipReceiver, GossipSender},
    proto::TopicId,
};

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;

use iroh_local::Iroh;
use message::Message;
use templates::{connected, login_form, message, send_form};
use ticket::Ticket;

#[tauri::command]
async fn send(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, Mutex<AppState>>,
    msg: String,
) -> Result<String, String> {
    let unlocked_state = state.lock().await;

    let m = Message::Message {
        from: unlocked_state.iroh.endpoint.node_id(),
        text: msg.clone(),
    };

    let _ = app_handle.emit("message", message("me".into(), msg).into_string());

    let _ = unlocked_state.topics[0]
        .broadcast(m.to_vec().into())
        .await
        .map_err(|_| "Failed to send message".to_string());

    Ok(send_form().into())
}

#[tauri::command]
async fn join(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, Mutex<AppState>>,
    username: String,
) -> Result<String, String> {
    println!("Username: {username}");

    let id = TopicId::from_bytes(rand::random());
    let node_ids = vec![];
    let mut unlocked_state = state.lock().await;
    let topic = unlocked_state.iroh.gossip.subscribe(id, node_ids).unwrap();

    let new_ticket = {
        let me = unlocked_state
            .iroh
            .endpoint
            .node_addr()
            .await
            .map_err(|_| "Failed to get node addr.".to_string());
        let nodes = vec![me.unwrap()];
        Ticket { topic: id, nodes }
    };
    println!("> ticket to join us: {new_ticket}");
    
    app_handle.clipboard().write_text(new_ticket.to_string()).unwrap();

    println!("> ticket copied to clip board!");

    let (sender, receiver) = topic.split();

    tokio::spawn(subscribe_loop(receiver, app_handle));

    let message = Message::AboutMe {
        from: unlocked_state.iroh.endpoint.node_id(),
        name: username,
    };

    let _ = sender
        .broadcast(message.to_vec().into())
        .await
        .map_err(|_| "Failed to send message".to_string());

    unlocked_state.topics.push(sender);

    Ok(login_form(new_ticket.to_string()).into())
}

async fn setup<R: tauri::Runtime>(
    app_handle: tauri::AppHandle<R>,
    app_data_dir: PathBuf,
) -> Result<()> {
    // get the applicaiton data root, join with "iroh_data" to get the data root for the iroh node
    let data_root = app_data_dir.join("iroh_data");

    let iroh = Iroh::new(data_root).await?;
    app_handle.manage(Mutex::new(AppState::new(iroh, Vec::new())));

    let _ = app_handle.emit("connected", connected().into_string());

    println!("Ready!!");

    Ok(())
}

struct AppState {
    iroh: Iroh,
    topics: Vec<GossipSender>,
}
impl AppState {
    fn new(iroh: Iroh, topics: Vec<GossipSender>) -> Self {
        AppState { iroh, topics }
    }

    fn iroh(&self) -> &Iroh {
        &self.iroh
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        // .plugin( /* Add your Tauri plugin here */ )
        // Add your commands here that you will call from your JS code
        .setup(|app| {
            let handle = app.handle().clone();
            let app_data_dir = app
                .path()
                .app_data_dir()
                .context("can't get application data directory")?;
            tauri::async_runtime::spawn(async move {
                println!("starting backend...");
                if let Err(err) = setup(handle, app_data_dir).await {
                    eprintln!("failed: {:?}", err);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![join, send])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn subscribe_loop(mut receiver: GossipReceiver, app: AppHandle) -> Result<()> {
    // keep track of the mapping between `NodeId`s and names
    let mut names = HashMap::new();
    // iterate over all events
    while let Some(event) = receiver.try_next().await? {
        // if the Event is a `GossipEvent::Received`, let's deserialize the message:
        if let Event::Gossip(GossipEvent::Received(msg)) = event {
            // deserialize the message and match on the
            // message type:
            match Message::from_bytes(&msg.content)? {
                Message::AboutMe { from, name } => {
                    // if it's an `AboutMe` message
                    // add and entry into the map
                    // and print the name
                    names.insert(from, name.clone());
                    println!("> {} is now known as {}", from.fmt_short(), name);
                }
                Message::Message { from, text } => {
                    // if it's a `Message` message,
                    // get the name from the map
                    // and print the message
                    let name = names
                        .get(&from)
                        .map_or_else(|| from.fmt_short(), String::to_string);
                    let sender = from.to_string();

                    let payload = message(sender, text.clone()).into_string();

                    let _ = app.emit("message", payload);
                    println!("{}: {}", name, text);
                }
            }
        }
    }
    Ok(())
}
