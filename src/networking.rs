pub struct UrlRequest;

pub struct UrlHandler {
    pub url: String,
}

impl UrlRequest {
    /// # Errors
    ///
    /// Will return `Err` if `url` is not reachable

    pub fn request(url: &str) -> Result<String, String> {
        reqwest::blocking::get(url).map_or_else(
            |e| Err(format!("Error sending request: {e}")),
            |res| match res.status() {
                ok if (200..300).contains(&ok.as_u16()) => Ok(res.text().unwrap_or_default()),
                err_code => Err(format!("Non-OK Status Code: {err_code}")),
            },
        )
    }
}

impl UrlHandler {
    /// # Errors
    ///
    /// Will return `Err` if `url` is not reachable

    pub fn request(&self) -> Result<String, String> {
        reqwest::blocking::get(&self.url).map_or_else(
            |e| Err(format!("Error while requesting url: {e}")),
            |res| match res.status() {
                ok if (200..300).contains(&ok.as_u16()) => Ok(res.text().unwrap_or_default()),
                err_code => Err(format!("Non-OK Status Code: {err_code}")),
            },
        )
    }
}
