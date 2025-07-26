use regex::Regex;

/// Escape ansi characters from a string
/// # Panics
/// - Will panic when regex object cannot be created
#[must_use]
pub fn escape_ansi(string: &str) -> String {
    let ansi_escape = Regex::new(r"(?:\x1B[@-_]|[\x80-\x9F])[0-?]*[ -/]*[@-~]").unwrap();
    ansi_escape.replace_all(string, "").to_string()
}

/// Check if a variable contains whitespaces and return a bool
#[must_use]
pub fn contains_whitespace(string: &str) -> bool {
    for c in string.chars() {
        if c == ' ' {
            return true;
        }
    }
    false
}

/// Check if a variable is empty or contains whitespaces
#[must_use]
pub fn is_empty_or_whitespace(string: &str) -> bool {
    string.chars().all(char::is_whitespace)
}
