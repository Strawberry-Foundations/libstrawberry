#![allow(clippy::future_not_send, clippy::needless_pass_by_value)]

use serde::{Deserialize, Serialize};
use crate::stbm::stbchat::object::{StbchatApiResponse, User, UserMeta};

/// # A packet sent from the server to the client (Server -> Client)
/// - `SystemMessage`: A message sent from the system
/// - `UserMessage`: A message sent from a user
/// - `Notification`: Tells the client to show a notification
/// - `Backend`: Sends the username to the client
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "packet_type")]
pub enum ClientPacket {
    #[serde(rename = "system_message")]
    SystemMessage {
        message: String
    },
    #[serde(rename = "user_message")]
    UserMessage {
        author: User,
        message: String,
    },
    #[serde(rename = "notification_backend")]
    Notification {
        title: String,
        username: String,
        avatar_url: String,
        content: String,
        bell: bool,
    },
    #[serde(rename = "stbchat_event")]
    Event {
        event_type: String,
    },
    #[serde(rename = "stbchat_backend")]
    Backend {
        user_meta: UserMeta
    },
    #[serde(rename = "stbchat_api")]
    ApiResponse {
        response_type: String,
        response: StbchatApiResponse
    }
}


/// # A packet sent from the client to the server (Client -> Server)
/// - `Login`: A event packet for receiving the users credentials
/// - `Message`: A message sent from client
#[derive(Serialize, Deserialize)]
#[serde(tag = "packet_type")]
pub enum ServerPacket {
    Login {
        username: String,
        password: String
    },
    Register {
        username: String,
        password: String,
        role_color: String,
    },
    Message {
        message: String
    },
    ApiRequest {
        request_type: String
    }
}
