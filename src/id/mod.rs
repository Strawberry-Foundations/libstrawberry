use eyre::Error;
use serde_json::Value;
use crate::constants::STRAWBERRY_ID_API;
use crate::id::error::ApiError;

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

impl StrawberryId {
    pub async fn request_code() -> Result<String, Error> {
        let request = reqwest::get(format!("{STRAWBERRY_ID_API}api/request")).await?;
        let code = if request.status().is_success() {
            match request.text().await {
                Ok(code) => code,
                Err(..) => return Err(ApiError::RequestError.into())
            }
        } else {
            return Err(ApiError::ServerError.into())
        };

        Ok(code)
    }

    pub async fn callback(code: String) -> Result<Option<Self>, Error> {
        let mut strawberry_id = Self::default();

        let request = reqwest::get(format!("{STRAWBERRY_ID_API}api/oauth/callback?code={code}")).await?;
        let body = request.text().await?;

        if let Ok(data) = Self::serializer(body.as_str()) {
            let status = match data.get("data").and_then(|v| v.get("status")) {
                Some(status) => status.as_str().unwrap(),
                None => return Err(ApiError::InvalidDataFormat.into())
            };

            if status != "Invalid code" && status != "Not authenticated" {
                if let Some(user_data) = data["data"]["user"].as_object() {
                    strawberry_id.email = user_data.get("email").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    strawberry_id.full_name = user_data.get("full_name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    strawberry_id.profile_picture = user_data.get("profile_picture_url").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    strawberry_id.username = user_data.get("username").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    strawberry_id.token = user_data.get("token").and_then(|v| v.as_str()).unwrap_or("").to_string();
                }
                else {
                    return Err(ApiError::InvalidDataFormat.into())
                }
            }
            else {
                return Ok(None)
            }
        };

        Ok(Some(strawberry_id))
    }

    fn serializer(text: &str) -> Result<Value, serde_json::Error> {
        let serializer = serde_json::from_str(text)?;
        Ok(serializer)
    }
}