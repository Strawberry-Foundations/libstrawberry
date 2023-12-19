use serde_json::Value;

pub struct JSON;

impl JSON {
    pub fn from_str(string: &str) -> Value {
        let value: Value = serde_json::from_str(&string).unwrap();

        value
    }
}
