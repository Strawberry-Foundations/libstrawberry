use serde_json::Value;

use crate::id::credentials::StrawberryIdCredentials;
use crate::id::StrawberryId;
use crate::constants::STRAWBERRY_ID_API;
use crate::id::error::VerifierError;


#[derive(Debug, Default, Clone)]
pub struct StrawberryIdVerifier {
    pub strawberry_id: StrawberryId,
}

impl StrawberryIdVerifier {
    pub fn verify(credentials: StrawberryIdCredentials) -> eyre::Result<Self, eyre::Error> {
        let api_response = reqwest::blocking::get(format!(
            "{STRAWBERRY_ID_API}api/auth?username={}&token={}", 
            credentials.username, credentials.token
        ))?.text()?;
        
        let mut verifier = Self::default();
        
        if let Ok(data) = serde_json::from_str::<Value>(&api_response) {
            if data["data"]["status"] == "Ok" {
                verifier.strawberry_id.full_name = data["data"]["user"]["full_name"].as_str().unwrap().to_string();
                verifier.strawberry_id.email = data["data"]["user"]["email"].as_str().unwrap().to_string();
                verifier.strawberry_id.profile_picture = data["data"]["user"]["profile_picture_url"].as_str().unwrap().to_string();
                verifier.strawberry_id.username = data["data"]["user"]["username"].as_str().unwrap().to_string();

                return Ok(verifier)
            }
        }

        Err(VerifierError::InvalidCredentials.into())
    }
}