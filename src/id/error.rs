use thiserror::Error;

#[derive(Error, Debug)]
pub enum CredentialsError {
    #[error("credentials.yml does not exist")]
    MissingCredentials,
    #[error("user home directory not found")]
    HomeNotFound
}