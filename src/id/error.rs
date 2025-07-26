use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CredentialsError {
    #[error("credentials.yml does not exist")]
    MissingCredentials,
    #[error("user home directory not found")]
    HomeNotFound,
    #[error("couldn't create directory")]
    DirectoryCreationError(String),
    #[error("couldn't write to file")]
    WriteError(String),
    #[error("couldn't serialize data")]
    SerializeError(String),
    #[error("credentials already exists")]
    AlreadyExists(PathBuf),
}

#[derive(Error, Debug)]
pub enum VerifierError {
    #[error("invalid user credentials")]
    InvalidCredentials,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("couldn't request login code")]
    RequestError,
    #[error("couldn't request strawberry id api server")]
    ServerError,
    #[error("requested code is in a invalid format")]
    CodeNotBeingCode,
    #[error("invalid data format")]
    InvalidDataFormat,
}
