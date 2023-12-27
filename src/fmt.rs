use serde_json::Value;

pub struct JSON;

impl JSON {
    #![allow(dead_code)]
    pub fn from_str(string: &str) -> Result<Value, serde_json::Error> {
        serde_json::from_str(string)
    }
}
