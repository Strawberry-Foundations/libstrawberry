use std::fs;
use serde::{Deserialize, Serialize};

use crate::id::error::CredentialsError;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StrawberryIdAuthenticator {
    pub username: Option<String>,
    pub token: Option<String>,
}

impl StrawberryIdAuthenticator {
    pub fn fetch() -> Result<StrawberryIdAuthenticator, Box<dyn std::error::Error>> {
        if let Some(home_dir) = dirs::home_dir() {
            let config_dir = home_dir.join(".config").join("strawberry-id");
            let credentials_path = config_dir.join("credentials.yml");

            if credentials_path.exists() {
                let credentials_str = fs::read_to_string(&credentials_path)?;

                let credentials: StrawberryIdAuthenticator = serde_yaml::from_str(&credentials_str)?;

                Ok(credentials)
            } else {
                Err(Box::from(CredentialsError::MissingCredentials))
            }
        } else {
            Err(Box::from(CredentialsError::HomeNotFound))
        }
    }

    pub fn unwrap(self) -> (String, String) {
        (self.username.clone().unwrap(), self.token.unwrap())
    }
}