pub struct UrlRequest;

#[deprecated(since = "0.5.16", note = "This feature has been deprecated as it is no longer used and has no purpose.")]
pub struct UrlHandler {
    pub url: String,
}

impl UrlRequest {
    /// Simple function to request an url
    /// # Errors
    ///
    /// - Will return `Err` if `url` is not reachable

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
    /// Url Handler struct for requesting an url from a created struct
    /// This feature has been deprecated as it is no longer used and has no purpose.
    /// # Errors
    ///
    /// - Will return `Err` if `url` is not reachable
    
    #[deprecated(since = "0.5.16", note = "This feature has been deprecated as it is no longer used and has no purpose.")]
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
