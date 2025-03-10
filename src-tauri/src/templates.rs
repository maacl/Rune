use maud::{html, Markup};
use chrono::prelude::*;


pub fn login_form() -> Markup {
    html! {
        div."grid grid-cols-6 gap-4" {
            div."col-span-3 ..." {
               form."w-50" tauri-invoke="join" hx-swap="outerHTML" {
                    fieldset."fieldset bg-base-200 border border-base-300 p-2 rounded-box" {
                        input."input input-sm w-50" type="text" name="username" placeholder="Username" { "" }
                        input."input input-sm w-50" type="text" name="ticket" placeholder="Ticket" { "" }
                        button."btn btn-neutral btn-outline btn-primary btn-sm w-50" type="submit" { "Create/Join Topic" }
                        (connected())
                    }
                }
            }
        }
    }
}

pub fn message(sender: String, message: String) -> Markup {
    let avatar = if sender == "me" {"https://i.pravatar.cc/150?img=3"} else {"https://i.pravatar.cc/150?img=43"};
    let ht = Local::now().format("%Y-%m-%d %H:%M:%S");
    html! {
        div.@if sender == "me" {"chat chat-start"}  @else {"chat chat-end"} {
            div."chat-image avatar" {
                div."w-10 rounded-full" {
                    img alt="Tailwind CSS chat bubble component" src=(avatar) {}
                }
            }
            div."chat-header" {
                (sender)
                time."text-xs opacity-50" { (ht) }
            }
            div."chat-bubble" {(message)}
            div."chat-footer opacity-50" {"delivered"}
        }
    }
}

pub fn send_form() -> Markup {
    html! {
        div {
            form tauri-invoke="send" hx-swap="outerHTML" {
                label."w-1/10 drawer-button lg:hidden float-left" for="my-drawer-2" {
                svg."size-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" {
                path stroke-linecap="round" stroke-linejoin="round" d="m8.25 4.5 7.5 7.5-7.5 7.5" {}
                }
                }
                textarea."textarea textarea w-9/10 p-4" name="msg" placeholder="Sent..." {}
                button."btn btn-neutral p-4 mt-4 float-right" type="submit" {"Send"}
            }

        }
    }
}

pub fn connected() -> Markup {
    html! {
        div {
            span."inline-flex items-center bg-green-100 text-green-800 text-xs font-medium px-2.5 py-0.5 rounded-full dark:bg-green-900 dark:text-green-300" {
                span."w-2 h-2 me-1 bg-green-500 rounded-full" {}
                "Connected"
            }
        }
    }
}

pub fn new_topic(topic: String) -> Markup {
    html! {
        li { button."btn btn-ghost w-50 btn-sm" tauri-invoke="select_topic" hx-swap="innerHTML" name="topic" value=(topic) {span."truncate" {(topic)} }}
    }
}
