#![allow(clippy::needless_pass_by_value)]

use lettre::{Message, Transport};
use lettre::transport::smtp::Error;
use lettre::transport::smtp::response::Response;
use lettre::message::header::ContentType as ContentTypeLettre;

use crate::email::message::{ContentType, Content};
use crate::email::server::Server;

pub mod server;
pub mod message;
pub mod credentials;

#[derive(Default)]
pub struct Email {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body_type: ContentType,
    pub content: Content,
}

impl Email {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn subject(mut self, subject: impl ToString) -> Self {
        self.subject = subject.to_string();

        self
    }

    #[must_use]
    pub const fn body(mut self, body_type: ContentType, content: Content) -> Self {
        self.body_type = body_type;
        self.content = content;

        self
    }

    #[must_use]
    pub fn from(mut self, from: impl ToString) -> Self {
        self.from = from.to_string();

        self
    }

    #[must_use]
    pub fn to(mut self, to: impl ToString) -> Self {
        self.to = to.to_string();

        self
    }

    /// # Errors
    /// Will cause error when something gone wrong while building lettre email
    /// # Panics
    /// Will panic when something gone wrong while building lettre email
    pub fn send(self, server: Server) -> Result<Response, Error> {
        let content_type = match self.body_type {
            ContentType::PLAIN => ContentTypeLettre::TEXT_PLAIN,
            ContentType::HTML => ContentTypeLettre::TEXT_HTML
        };

        let body= match self.content {
            Content::Default => "",
            Content::Plain(str) | Content::Html(str) => str,
        };

        let email_lettre = Message::builder()
            .from(self.from.parse().unwrap())
            .to(self.to.parse().unwrap())
            .subject(self.subject)
            .header(content_type)
            .body(body.to_string())
            .unwrap();

        server.mailer.send(&email_lettre)
    }
}