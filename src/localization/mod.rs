use crate::colors::{
    BLUE, BOLD, C_RESET, CYAN, GREEN, MAGENTA, RED, RESET, UNDERLINE, WHITE, YELLOW,
};
use serde_yaml::Value;
use std::fs;

/// `Localization` provides convenient access to localized strings and supports color placeholders.
pub struct Localization {
    pub language: String,
    pub strings: Value,
    pub color_placeholders: bool,
}

/// Reads the contents of a file into a string.
/// # Panics
/// Panics if the file cannot be read.
#[must_use]
pub fn read_language_file(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read language file")
}

impl Localization {
    /// Creates a new `Localization` instance without color placeholders.
    /// # Panics
    /// Panics if YAML parsing fails.
    #[must_use]
    pub fn new(language: &str, yaml_content: &str, enable_placeholders: bool) -> Self {
        let strings = serde_yaml::from_str(yaml_content).expect("Failed to parse YAML");
        Self {
            language: language.to_string(),
            strings,
            color_placeholders: enable_placeholders,
        }
    }

    /// Retrieves a localized string by key, optionally replacing color placeholders.
    /// # Panics
    /// Panics if the string key is missing.
    #[must_use]
    pub fn get(&self, key: &str) -> String {
        let raw = self.strings[&self.language][key]
            .as_str()
            .expect("Missing string key")
            .to_string();

        if self.color_placeholders {
            raw.replace("{red}", RED)
                .replace("{green}", GREEN)
                .replace("{yellow}", YELLOW)
                .replace("{blue}", BLUE)
                .replace("{magenta}", MAGENTA)
                .replace("{cyan}", CYAN)
                .replace("{white}", WHITE)
                .replace("{reset}", RESET)
                .replace("{creset}", C_RESET)
                .replace("{bold}", BOLD)
                .replace("{underline}", UNDERLINE)
        } else {
            raw
        }
    }

    /// Retrieves a localized string and replaces `%s`/`%d` placeholders with provided parameters.
    /// # Panics
    /// Panics if the string key is missing.
    #[must_use]
    pub fn get_with_params(&self, key: &str, params: &[&dyn std::fmt::Display]) -> String {
        let mut message = self.get(key);

        for param in params {
            if let Some(idx) = message.find("%s") {
                message.replace_range(idx..idx + 2, &param.to_string());
            } else if let Some(idx) = message.find("%d") {
                if let Ok(value) = param.to_string().parse::<i64>() {
                    message.replace_range(idx..idx + 2, &value.to_string());
                }
            }
        }
        message
    }
}

/// Macro for printing a localized string using a `Localization` instance.
/// Usage: `localize!(loc, "greeting")`
#[macro_export]
macro_rules! localize {
    ($loc:expr, $key:expr) => {
        println!("{}", $loc.get($key))
    };
}
