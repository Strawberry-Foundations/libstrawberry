use serde_yaml::Value;
use std::fs;
use crate::colors::{BLUE, BOLD, C_RESET, CYAN, GREEN, MAGENTA, RED, RESET, UNDERLINE, WHITE, YELLOW};

/// Strings (String Loader) is a Stblib function to simplify the localization of Rust programs.
pub struct Strings {
    pub language: String,
    pub lang_str_object: Value,
    pub replace_placeholders: bool,
}

/// Load data from a file
/// # Panics
///
/// - Will panic when file cannot be read as string

#[must_use]
pub fn load_language_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read language strings")
}

impl Strings {
    /// Create a new Strings object
    /// # Panics
    ///
    /// - Will panic when serde couldn't convert to yaml from str

    #[must_use]
    pub fn new(language: &str, lang_strings: &str) -> Self {
        let lang_object = serde_yaml::from_str(lang_strings).unwrap();

        Self {
            language: language.to_string(),
            lang_str_object: lang_object,
            replace_placeholders: false
        }
    }

    /// Create a new Strings object with the use of color placeholders
    /// # Panics
    ///
    /// - Will panic when serde couldn't convert to yaml from str

    #[must_use]
    pub fn new_with_placeholders(language: &str, lang_strings: &str) -> Self {
        let lang_object = serde_yaml::from_str(lang_strings).unwrap();

        Self {
            language: language.to_string(),
            lang_str_object: lang_object,
            replace_placeholders: true
        }
    }

    /// Load a string
    /// # Panics
    ///
    /// - Will panic when string cannot be loaded from language file

    #[must_use]
    pub fn load(&self, string: &str) -> String {
        if self.replace_placeholders {
            let string = &self.lang_str_object[&self.language][string].as_str();

            string.unwrap().to_string()
                .replace("{red}", RED)
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
        }
        else {
            self.lang_str_object[&self.language][string].as_str().unwrap().to_string()
        }
    }

    /// Load a string and replace the given parameters with the placeholders inside the string
    /// # Panics
    ///
    /// - Will panic when string cannot be loaded from language file

    #[must_use]
    pub fn load_with_params(&self, string: &str, params: &[&dyn std::fmt::Display]) -> String {
        let string = self.load(string);

        let has_placeholders = string.contains('%');

        if has_placeholders {
            let mut formatted_message = string;

            for param in params {
                if let Some(index) = formatted_message.find("%s") {
                    formatted_message.replace_range(index..(index + 2), &param.to_string());
                }
                else if let Some(index) = formatted_message.find("%d") {
                    if let Ok(value) = param.to_string().parse::<i64>() {
                        formatted_message.replace_range(index..(index + 2), &value.to_string());
                    }
                }
            }
            formatted_message.to_string()
        } else {
            string
        }

    }
}

/// Macro for loading strings. Requires the stringloader object. `str!(my_loader, "hello")`
#[macro_export]
macro_rules! str {
    ($loader:expr, $string:expr) => {
        println!("{}", $loader.load($string))
    };
}