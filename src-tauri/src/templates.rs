use maud::{html, Markup};

pub fn login_form(ticket: String) -> Markup {
    html! {
        div."grid grid-cols-6 gap-4" {
            div."col-span-3 ..." {
               form tauri-invoke="join" hx-swap="outerHTML" {
                    fieldset."fieldset w-xs bg-base-200 border border-base-300 p-4 rounded-box" {

                        legend."fieldset-legend" { "Login" }
                        label."fieldset-label" { "Username" }
                        input.input type="text" name="username" placeholder="Username" { "" }

                        button."btn btn-neutral mt-4" type="submit" { "Login" }
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
        div."col-span-3   ..." {
            form tauri-invoke="send" hx-swap="outerHTML" {
                fieldset."fieldset w-xs bg-base-200 border border-base-300 p-4 rounded-box" {
                    legend."fieldset-legend" { "Chat"}
                    label."fieldset-label" {"Message"}
                    textarea."textarea" name="msg" placeholder="Sent..." {}
                    button."btn btn-neutral mt-4" type="submit" {"Send"}
                }
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
        option {(topic)}
    }
}
