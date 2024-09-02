use dioxus::prelude::*;

#[derive(PartialEq, Clone, Debug, Props, Copy)]
pub struct Email {
    pub email_type: EmailType,
    pub subject: &'static str,
    pub plain_text: &'static str,
    pub html_body: &'static str,
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum EmailType {
    Sent(SentMail),
    Received(ReceivedMail),
}

#[derive(PartialEq, Clone, Copy, Debug, Props)]
pub struct SentMail {
    pub recipient: &'static str,
}

#[derive(PartialEq, Clone, Copy, Debug, Props)]
pub struct ReceivedMail {
    pub sender: &'static str,
    pub read: bool,
}
