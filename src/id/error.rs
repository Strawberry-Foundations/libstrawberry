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
    #[error("couldn't serialize data")]
    AlreadyExists,

}

#[derive(Error, Debug)]
pub enum VerifierError {
    #[error("invalid user credentials")]
    InvalidCredentials,
}