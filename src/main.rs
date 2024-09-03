#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use geek_mail::backend::sample_emails::get_sample_emails;
use geek_mail::backend::types::context::SelectedEmailNumber;
use geek_mail::backend::types::email::Email;
use geek_mail::components::preview_col::PreviewCol;
use geek_mail::components::viewer::Viewer;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    // Build cool things ✌️

    let emails = get_sample_emails(7);
    let selected_email_number = use_context_provider(|| Signal::new(SelectedEmailNumber(None)));
    let selected_email: Option<Email> = selected_email_number()
        .0
        .map(|email_number| emails[email_number as usize]);
    rsx! {
        div { class: "flex flex-row min-h-screen text-center
                    dark:bg-neutral-800 dark:text-neutral-200",
            div { class: "col w-64 min-w-fill
                    bg-gradient-to-r from-neutral-900 to-neutral-800",
                PreviewCol { emails }
            }

            div { class: "",
                Viewer { email: selected_email }
            }
        }
    }
}
