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
            let replacements = [
                ("{red}", RED),
                ("{green}", GREEN),
                ("{yellow}", YELLOW),
                ("{blue}", BLUE),
                ("{magenta}", MAGENTA),
                ("{cyan}", CYAN),
                ("{white}", WHITE),
                ("{reset}", RESET),
                ("{creset}", C_RESET),
                ("{bold}", BOLD),
                ("{underline}", UNDERLINE),
            ];
            let mut result = raw;
            for (ph, val) in replacements.iter() {
                result = result.replace(ph, val);
            }
            result
        } else {
            raw
        }
    }

    /// Retrieves a localized string and replaces `%s`/`%d` placeholders with provided parameters.
    /// # Panics
    /// Panics if the string key is missing.
    #[must_use]
    pub fn get_with_params(&self, key: &str, params: &[&dyn std::fmt::Display]) -> String {
        let message = self.get(key);
        let mut param_idx = 0;
        let mut result = String::new();
        let mut chars = message.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '%' {
                if let Some(&next) = chars.peek() {
                    match next {
                        's' => {
                            chars.next();
                            if param_idx < params.len() {
                                result.push_str(&params[param_idx].to_string());
                                param_idx += 1;
                            } else {
                                result.push_str("%s");
                            }
                        }
                        'd' => {
                            chars.next();
                            if param_idx < params.len() {
                                if let Ok(value) = params[param_idx].to_string().parse::<i64>() {
                                    result.push_str(&value.to_string());
                                } else {
                                    result.push_str(&params[param_idx].to_string());
                                }
                                param_idx += 1;
                            } else {
                                result.push_str("%d");
                            }
                        }
                        _ => {
                            result.push('%');
                            result.push(next);
                            chars.next();
                        }
                    }
                } else {
                    result.push('%');
                }
            } else {
                result.push(c);
            }
        }
        result
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
