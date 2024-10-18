use std::fs;
use eyre::Error;
use serde::{Deserialize, Serialize};

use crate::id::error::CredentialsError;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StrawberryIdCredentials {
    pub username: String,
    pub token: String,
}

impl StrawberryIdCredentials {
    /// # Errors
    /// Will return `Err` if user home was not found or the credentials file not exists
    pub fn fetch() -> Result<Self, Error> {
        if let Some(home_dir) = dirs::home_dir() {
            let config_dir = home_dir.join(".config").join("strawberry-id");
            let credentials_path = config_dir.join("credentials.yml");

            if credentials_path.exists() {
                let credentials_str = fs::read_to_string(&credentials_path)?;

                let credentials: Self = serde_yaml::from_str(&credentials_str)?;

                Ok(credentials)
            } else {
                Err(CredentialsError::MissingCredentials.into())
            }
        } else {
            Err(CredentialsError::HomeNotFound.into())
        }
    }

    /// # Errors
    /// Will return `Err` if ...
    /// - User home was not found
    /// - User already logged in
    /// - The strawberry-id config directory could not be created
    /// - Serialize error
    /// - Write error
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
                let credentials = Self {
                    username,
                    token,
                };

                return match serde_yaml::to_string(&credentials) {
                    Ok(credentials_str) => {
                        if let Err(err) = fs::write(&credentials_path, credentials_str) {
                            return Err(CredentialsError::WriteError(err.to_string()).into())
                        }
                        Ok(())
                    }
                    Err(err) => Err(CredentialsError::SerializeError(err.to_string()).into()),
                }
            }
            return Err(CredentialsError::AlreadyExists.into())

        }
        Err(CredentialsError::HomeNotFound.into())
    }
}