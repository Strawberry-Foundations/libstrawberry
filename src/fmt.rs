use serde_json::Value;

pub struct JSON;

impl JSON {
    /// # Errors
    ///
    /// - Will return `Err` if text object cannot be converted to json

    pub fn from_str(string: &str) -> Result<Value, serde_json::Error> {
        serde_json::from_str(string)
    }
}
