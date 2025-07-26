use serde::{Deserialize, Serialize};

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
    #[serde(rename = "user_joined")]
    UserJoined {
        username: String,
        nickname: String,
        role_color: String,
        badge: String,
    },
    #[serde(rename = "user_left")]
    UserLeft { username: String },
    #[serde(rename = "user_data")]
    UserData { data: User },
}
