use serde_json::Value;

#[deprecated(since = "0.5.16", note = "This feature has been deprecated as it is no longer used and has no purpose.")]
pub struct JSON;

impl JSON {
    /// JSON Reader to read data from a string and parse it as json data    
    /// This feature has been deprecated as it is no longer used and has no purpose.
    /// 
    /// # Errors
    ///
    /// - Will return `Err` if text object cannot be converted to json
    #[deprecated(since = "0.5.16", note = "This feature has been deprecated as it is no longer used and has no purpose.")]
    pub fn from_str(string: &str) -> Result<Value, serde_json::Error> {
        serde_json::from_str(string)
    }
}
