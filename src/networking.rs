use reqwest;

pub struct UrlRequest;

pub struct UrlHandler {
    pub url: String,
}

impl UrlRequest {
    pub fn request(url: &str) -> String {
        match reqwest::blocking::get(url) {
            Ok(response) => {
                if response.status().is_success() {
                    response.text().unwrap()
                } else {
                    format!("Error: {}", response.status())
                }
            }
            Err(_) => String::from("Error while requesting url")
        }
    }
}

impl UrlHandler {
    pub fn request(&self) -> String {
        match reqwest::blocking::get(&self.url) {
            Ok(response) => {
                if response.status().is_success() {
                    response.text().unwrap()
                }
                else {
                    format!("Error: {}", response.status())
                }
            }
            Err(_) => String::from("Error while requesting url")
        }
    }
}