use serde_json::Value;

use crate::id::credentials::StrawberryIdCredentials;
use crate::id::StrawberryId;
use crate::constants::STRAWBERRY_ID_API;


#[derive(Debug, Default, Clone)]
pub struct StrawberryIdVerifier {
    pub status: bool,
    pub credentials: StrawberryIdCredentials,
    pub strawberry_id: StrawberryId,
}

impl StrawberryIdVerifier {
    pub async fn verify(&mut self, username: &String, token: &String) -> eyre::Result<Option<StrawberryIdVerifier>> {
            
        let auth = reqwest::get(format!("{STRAWBERRY_ID_API}api/auth?username={}&token={}", username, token)).await?;
        let body = auth.text().await?;

        let mut client_auth = StrawberryIdVerifier::default();


        if let Ok(data) = serde_json::from_str::<Value>(&body) {
            if data["data"]["status"] == "Ok" {
                client_auth.strawberry_id.full_name = data["data"]["user"]["full_name"].as_str().unwrap().to_string();
                client_auth.strawberry_id.email = data["data"]["user"]["email"].as_str().unwrap().to_string();
                client_auth.strawberry_id.profile_picture = data["data"]["user"]["profile_picture_url"].as_str().unwrap().to_string();
                client_auth.strawberry_id.username = data["data"]["user"]["username"].as_str().unwrap().to_string();

                return Ok(Some(client_auth))
            }
        }

        Ok(None)
    }
}