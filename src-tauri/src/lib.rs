#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod iroh_local;
mod message;
mod templates;
mod ticket;
mod topic;

use std::str::FromStr;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{thread, time::Duration};

use anyhow::{Context, Result};
use futures_lite::StreamExt;
use petname::petname;
use tokio::sync::Mutex;

use iroh_gossip::{
    net::{Event, GossipEvent, GossipReceiver, GossipSender},
    proto::TopicId,
};

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;

use iroh_local::Iroh;
use message::Message;
use templates::{connected, login_form, message, new_topic, send_form};
use ticket::Ticket;

struct AppState {
    iroh: Iroh,
    topics: HashMap<String, GossipSender>,
    active_topic: String,
}
impl AppState {
    fn new(iroh: Iroh, topics: HashMap<String, GossipSender>, active_topic: String) -> Self {
        AppState {
            iroh,
            topics,
            active_topic,
        }
    }

    fn iroh(&self) -> &Iroh {
        &self.iroh
    }
}

#[tauri::command]
async fn select_topic(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, Mutex<AppState>>,
    topic: String,
) -> Result<String, String> {
    println!("> topic selected: {topic}");

    Ok(new_topic(topic).into_string())
}
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

    let _ = unlocked_state
        .topics
        .get(&unlocked_state.active_topic)
        .expect("Topic not found.")
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
    ticket: String
) -> Result<String, String> {
    println!("Username: {username}");

    let mut unlocked_state = state.lock().await;
    if ticket.is_empty() {
    let topic_id = TopicId::from_bytes(rand::random());
    let node_ids = vec![];
    let topic = unlocked_state.iroh.gossip.subscribe(topic_id, node_ids).unwrap();
    let new_ticket = {
        let me = unlocked_state
            .iroh
            .endpoint
            .node_addr()
            .await
            .map_err(|_| "Failed to get node addr.".to_string());
        let nodes = vec![me.unwrap()];
        Ticket { topic: topic_id, nodes }
    };
    println!("> ticket to join us: {new_ticket}");
    app_handle
        .clipboard()
        .write_text(new_ticket.to_string())
        .unwrap();

    println!("> ticket copied to clip board!");

    let (sender, receiver) = topic.split();

    tokio::spawn(subscribe_loop(receiver, app_handle.clone()));

    let message = Message::AboutMe {
        from: unlocked_state.iroh.endpoint.node_id(),
        name: username,
    };

    let _ = sender
        .broadcast(message.to_vec().into())
        .await
        .map_err(|_| "Failed to send message".to_string());

    let topic_name = petname(7, ":").expect("Uanble to create topic name.");

    unlocked_state.topics.insert(topic_name.clone(), sender);
    unlocked_state.active_topic = topic_name.clone();

    //let t: &GossipSender = unlocked_state.topics.last().unwrap();

    let _ = app_handle.emit("new_topic", new_topic(topic_name).into_string());

    let _ = app_handle.emit("connected", connected().into_string());

    Ok(login_form().into_string())
    } 
    
    else 
    
    
    {
    let Ticket { topic, nodes } = Ticket::from_str(&ticket).expect("Ticket could not be created from string.");
    let node_ids: Vec<_> = nodes.iter().map(|p| p.node_id).collect();

    for node in nodes.into_iter() {
        unlocked_state.iroh.endpoint.add_node_addr(node).expect("Cannot add node adr.");
    }

    let (sender, receiver) = unlocked_state.iroh.gossip.subscribe_and_join(topic, node_ids).await.expect("Could not subscribe and join.").split();
    println!("> connected!");

    // broadcast our name, if set
        let message = Message::AboutMe {from:unlocked_state.iroh.endpoint.node_id(),  name: username };
        sender.broadcast(message.to_vec().into()).await.expect("Could not send message.");

    // subscribe and print loop
    tokio::spawn(subscribe_loop(receiver, app_handle.clone()));

    unlocked_state.topics.insert(topic.to_string(), sender);
    unlocked_state.active_topic = topic.to_string().clone();

    let _ = app_handle.clone().emit("new_topic", new_topic(topic.to_string()).into_string());

    let _ = app_handle.emit("connected", connected().into_string());

    Ok(login_form().into_string())
    }
}

async fn setup<R: tauri::Runtime>(
    app_handle: tauri::AppHandle<R>,
    app_data_dir: PathBuf,
) -> Result<()> {
    // get the applicaiton data root, join with "iroh_data" to get the data root for the iroh node
    let data_root = app_data_dir.join("iroh_data");

    let iroh = Iroh::new(data_root).await?;
    app_handle.manage(Mutex::new(AppState::new(iroh, HashMap::new(), "".into())));

    thread::sleep(Duration::from_millis(2000));

    let _ = app_handle.emit("connected", connected().into_string());

    println!("Ready!!");

    Ok(())
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
        .invoke_handler(tauri::generate_handler![join, send, select_topic])
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
