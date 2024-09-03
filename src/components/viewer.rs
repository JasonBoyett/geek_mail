#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::backend::{geek_phrase::get_geek_phrase, types::email::Email};

#[component]
pub fn Viewer(email: Option<Email>) -> Element {
    match email {
        Some(email) => rsx! {
            ValidViewer { email }
        },
        None => rsx! {
            EmptyViewer {}
        },
    }
}

#[component]
fn ValidViewer(email: Email) -> Element {
    rsx! {
        div { class: "h-full ml-2 items-center justify-center border-l
                border-slate-600 text-left",
            div { class: "ml-2",
                div { dangerous_inner_html: email.html_body }
            }
        }
    }
}

fn EmptyViewer() -> Element {
    rsx! {
        div { class: "h-full items-center justify-center border-l
                        border-slate-600 text-left",
            div { class: "ml-6",
                div { class: "flex w-full min-h-screen text-center
                        text-4xl items-center justify-center",
                    { get_geek_phrase() }
                }
            }
        }
    }
}
