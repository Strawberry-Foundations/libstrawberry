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

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "api_response")]
pub enum StbchatApiResponse {
    #[serde(rename = "new_user")]
    NewUser {
        username: String,
        nickname: String,
        role_color: String,
        badge: String,
    }
}