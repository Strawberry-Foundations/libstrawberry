use std::fs;
use serde_yaml::Value;

pub struct Strings {
    pub language: String,
    pub lang_str_object: Value,
}

fn load_language_file(path: &str) -> String {
    let lang_yml =
        fs::read_to_string(path)
            .expect("Could not read language strings");
    lang_yml
}

impl Strings {
    pub fn new(language: &str, path: &str) -> Strings {
        let language_strings = load_language_file(path);
        let lang_object = serde_yaml::from_str(&language_strings).unwrap();

        let str_loader = Strings {
            language: language.to_string(),
            lang_str_object: lang_object,
        };

        str_loader
    }

    pub fn str(&self, string: &str) -> String {
        let string = &self.lang_str_object[&self.language][string].as_str();
        string.unwrap().to_string()
    }
}