pub mod credentials;
pub mod error;
pub mod verifier;

#[derive(Debug, Default, Clone)]
pub struct StrawberryId {
    pub email: String,
    pub full_name: String,
    pub profile_picture: String,
    pub username: String,
    pub token: String,
}