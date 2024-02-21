use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub nickname: String,
    pub badge: String,
    pub role_color: String,
    pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMeta {
    pub username: String,
}