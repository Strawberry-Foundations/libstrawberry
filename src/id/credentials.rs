use std::fs;
use serde::{Deserialize, Serialize};

use crate::id::error::CredentialsError;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StrawberryIdCredentials {
    pub username: String,
    pub token: String,
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

    pub fn save(username: String, token: String) -> eyre::Result<()> {
        if let Some(home_dir) = dirs::home_dir() {
            let config_dir = home_dir.join(".config").join("strawberry-id");
            let credentials_path = config_dir.join("credentials.yml");

            if !config_dir.exists() {
                if let Err(err) = fs::create_dir_all(&config_dir) {
                    return Err(CredentialsError::DirectoryCreationError(err.to_string()).into())
                }
            }

            if !credentials_path.exists() {
                let credentials = StrawberryIdCredentials {
                    username,
                    token,
                };

                match serde_yaml::to_string(&credentials) {
                    Ok(credentials_str) => {
                        if let Err(err) = fs::write(&credentials_path, credentials_str) {
                            return Err(CredentialsError::WriteError(err.to_string()).into())
                        }
                        return Ok(())
                    }
                    Err(err) => return Err(CredentialsError::SerializeError(err.to_string()).into()),
                }
            }
            return Err(CredentialsError::AlreadyExists.into())

        }
        return Err(CredentialsError::HomeNotFound.into())
    }
}