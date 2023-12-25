use serde_yaml::Value;
use std::fs;

pub struct Strings {
    pub language: String,
    pub lang_str_object: Value,
}

pub fn load_language_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read language strings")
}

impl Strings {
    pub fn new(language: &str, lang_strings: &str) -> Self {
        let lang_object = serde_yaml::from_str(lang_strings).unwrap();

        Self {
            language: language.to_string(),
            lang_str_object: lang_object,
        }
    }

    pub fn str(&self, string: &str) -> String {
        let string = &self.lang_str_object[&self.language][string].as_str();
        string.unwrap().to_string()
    }
}
