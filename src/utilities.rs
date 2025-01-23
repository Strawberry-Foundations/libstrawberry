use std::any::type_name;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::prelude::*;
use regex::Regex;

/// Simplified version of `thread:sleep()` for waiting x seconds
pub fn sleep(time: u64) {
    thread::sleep(Duration::from_secs(time));
}

/// Simplified version of `thread:sleep()` for waiting x milliseconds
pub fn ms_sleep(time: u64) {
    thread::sleep(Duration::from_millis(time));
}

/// Fetch the current time in a given time format
#[must_use]
pub fn current_time(format: &str) -> String {
    let local: DateTime<Local> = Local::now();
    local.format(format).to_string()
}

/// Get current unix epoch time
/// # Panics
/// - Will panic when clock may have gone backwards

#[must_use]
pub fn unix_time() -> u64 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).expect("err!").as_secs()
}

/// Get the type of a variable
#[must_use]
pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

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
