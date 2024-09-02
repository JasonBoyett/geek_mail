#![allow(non_snake_case)]

use super::preview::Preview;
use crate::backend::types::email::Email;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Debug, Props)]
pub struct PreviewColProps {
    emails: Vec<Email>,
}
pub fn PreviewCol(props: PreviewColProps) -> Element {
    rsx! {
        div { class: "col min-w-fill",
            {(0..props.emails.len()).map(|i| {
                rsx! {
                    Preview {
                        id: i as i32,
                        email: props.emails[i]
                    }
                }
            })}
        }
    }
}
