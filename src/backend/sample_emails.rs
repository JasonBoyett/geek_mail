use super::types::email::Email;

pub fn get_sample_email() -> Email {
    Email {
        email_type: super::types::email::EmailType::Received(super::types::email::ReceivedMail {
            sender: "John Doe",
            read: true,
        }),
        subject: "Hello, from subject",
        plain_text: "This is a simple example of a Dioxus app.",
        html_body: "<p>This is a simple example of a Dioxus app.</p>",
    }
}

pub fn get_sample_emails(count: i32) -> Vec<Email> {
    (0..count)
        .map(|_| Email {
            email_type: super::types::email::EmailType::Received(
                super::types::email::ReceivedMail {
                    sender: "John Doe",
                    read: false,
                },
            ),
            subject: "Hello, from subject",
            plain_text: "This is a simple example of a Dioxus app.",
            html_body: "<div style=\"background-color: red; font-size: x-large\">
                    <h1>Hello, World!</h1>
                    <div style=\"background-color: blue; font-size: large;\">
                    <p>This is some sample text.</p>
                    <p>I want it to be kind of long so that I can see how it looks when it wraps.</p>
                    <p style=\"color: pink;\">I hope it looks good!</p>
                </div>
            </div>",
        })
        .collect()
}
