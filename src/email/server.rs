#![allow(clippy::needless_pass_by_value)]

use crate::email::credentials::Credentials;
use lettre::SmtpTransport;

pub struct Server {
    pub address: String,
    pub port: u16,
    pub mailer: SmtpTransport,
}

impl Server {
    /// # Panics
    /// Will panic if smtp server is not reachable
    pub fn new(address: impl ToString, port: u16, credentials: Credentials) -> Self {
        let cred_lettre = lettre::transport::smtp::authentication::Credentials::new(
            credentials.email,
            credentials.password,
        );

        let mailer = SmtpTransport::relay(address.to_string().as_str())
            .unwrap()
            .credentials(cred_lettre)
            .build();

        Self {
            address: address.to_string(),
            port,
            mailer,
        }
    }
}
