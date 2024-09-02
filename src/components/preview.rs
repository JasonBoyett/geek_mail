#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::backend::types::context::SelectedEmailNumber;
use crate::backend::types::email::{Email, EmailType};

#[derive(PartialEq, Clone, Debug, Props)]
pub struct PreviewArgs {
    pub id: i32,
    pub email: Email,
}
pub fn Preview(props: PreviewArgs) -> Element {
    let display_name = match props.email.email_type {
        EmailType::Sent(email) => String::from(email.recipient),
        EmailType::Received(email) => String::from(email.sender),
    };
    let mut currently_selected_email_number = use_context::<Signal<SelectedEmailNumber>>();
    let id = SelectedEmailNumber(Some(props.id));
    let mut show_read = use_signal(|| match props.email.email_type {
        EmailType::Sent(_) => true,
        EmailType::Received(email) => !email.read,
    });
    let preview_text = match props.email.plain_text.len() {
        0..=50 => props.email.plain_text,
        _ => &format!("{}...", &props.email.plain_text[0..50]),
    };

    rsx! {
        div {
            class: "ml-2 flex h-24 w-60 justify-center border-b border-t border-slate-600 text-left",
            //below is some tuple type magic shinanigans. Basically we are just setting
            //the currently selected email number to the id of the email that was clicked
            onclick: move |_| {
                if currently_selected_email_number() == id {
                    currently_selected_email_number.write().0 = None;
                    return;
                }
                currently_selected_email_number.write().0 = id.0;
                if show_read() {
                    match props.email.email_type {
                        EmailType::Sent(_) => {}
                        EmailType::Received(_) => {
                            show_read.set(!show_read());
                        }
                    }
                }
            },
            div { class: "mr-4 flex h-full items-center justify-center",
                ReadIndicator { visible: show_read() }
            }
            PreviewContainer { is_selected: currently_selected_email_number() == id,
                p { class: "ml-2 text-left text-lg font-semibold text-neutral-50",
                    {props.email.subject}
                }
                p { class: "ml-2 text-left font-bold text-neutral-50", {display_name} }
                p { class: "ml-2 text-left text-xs text-neutral-300", {preview_text} }
            }
        }
    }
}

#[component]
fn ReadIndicator(visible: bool) -> Element {
    println!("ReadIndicator: {}", visible);
    rsx! {

        div {
            class: format!(
                "col-auto h-3 w-3 rounded-xl {}",
                match visible {
                    true => "bg-blue-500 drop-shadow-md",
                    false => "",
                },
            )
        }
    }
}

#[derive(PartialEq, Clone, Debug, Props)]
struct PreviewContainerProps {
    is_selected: bool,
    children: Element,
}
fn PreviewContainer(props: PreviewContainerProps) -> Element {
    rsx! {
        div {
            class: format!(
                "col-auto w-52 rounded-md {}",
                match props.is_selected {
                    true => "bg-blue-500",
                    false => "",
                },
            ),
            { props.children }
        }
    }
}
