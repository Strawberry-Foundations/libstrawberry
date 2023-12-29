use std::any::type_name;
use std::thread;
use std::time::Duration;

use chrono::prelude::*;
use regex::Regex;


pub fn sleep(time: u64) {
    thread::sleep(Duration::from_secs(time));
}

pub fn ms_sleep(time: u64) {
    thread::sleep(Duration::from_millis(time));
}

pub fn current_time(format: &str) -> String {
    let local: DateTime<Local> = Local::now();
    local.format(format).to_string()
}

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn escape_ansi(string: &str) -> String {
    let ansi_escape = Regex::new(r"(?:\x1B[@-_]|[\x80-\x9F])[0-?]*[ -/]*[@-~]").unwrap();
    ansi_escape.replace_all(string, "").to_string()
}

pub fn contains_whitespace(string: &str) -> bool {
    for c in string.chars() {
        if c == ' ' {
            return true;
        }
    }
    false
}

pub fn is_empty_or_whitespace(string: &str) -> bool {
    string.chars().all(|c| c.is_whitespace())
}
