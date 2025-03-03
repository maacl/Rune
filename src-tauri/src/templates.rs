use maud::{html, Markup};

pub fn login_form(ticket: String) -> Markup {
    html! {
        div."grid grid-cols-6 gap-4" {
            div."col-span-3 ..." {
               form."w-50" tauri-invoke="join" hx-swap="outerHTML" {
                    fieldset."fieldset bg-base-200 border border-base-300 p-2 rounded-box" {
                        input."input input-sm w-50" type="text" name="username" placeholder="Username" { "" }
                        button."btn btn-neutral btn-outline btn-primary btn-sm w-50" type="submit" { "Create Tpoic" }
                        (connected())
                    }
                }
            }
        }
    }
}

pub fn message(sender: String, message: String) -> Markup {
    html! {
        div.@if sender == "me" {"chat chat-start"}  @else {"chat chat-end"} {
            div."chat-image avatar" {
                div."w-10 rounded-full" {
                    img alt="Tailwind CSS chat bubble component" src="https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp" {}
                }
            }
            div."chat-header" {
                (sender)
                time."text-xs opacity-50" { "12:45" }
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
        li {a."truncate" { span."truncate" {(topic)}}}
    }
}