use chrono::prelude::*;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
