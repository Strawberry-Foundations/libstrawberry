use std::fs;
use serde::{Deserialize, Serialize};

use crate::id::error::CredentialsError;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StrawberryIdCredentials {
    pub username: Option<String>,
    pub token: Option<String>,
}

impl StrawberryIdCredentials {
    pub fn fetch() -> Result<StrawberryIdCredentials, eyre::Error> {
        if let Some(home_dir) = dirs::home_dir() {
            let config_dir = home_dir.join(".config").join("strawberry-id");
            let credentials_path = config_dir.join("credentials.yml");

            if credentials_path.exists() {
                let credentials_str = fs::read_to_string(&credentials_path)?;

                let credentials: StrawberryIdCredentials = serde_yaml::from_str(&credentials_str)?;

                Ok(credentials)
            } else {
                Err(CredentialsError::MissingCredentials.into())
            }
        } else {
            Err(CredentialsError::HomeNotFound.into())
        }
    }

    pub fn unwrap(self) -> (String, String) {
        (self.username.clone().unwrap(), self.token.unwrap())
    }
}