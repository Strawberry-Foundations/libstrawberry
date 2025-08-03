use chrono::prelude::*;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq)]
/// Days of the week as an enum, used for the get_weekday_from_epoch() function
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

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

/// Returns the weekday as an enum based on the provided Unix time
pub fn get_weekday_from_epoch(time: u64) -> Option<Weekday> {
    let weekday = (f64::floor(time as f64 / 86400.) + 4.) % 7.;

    match weekday {
        0. => Some(Weekday::Sunday),
        1. => Some(Weekday::Monday),
        2. => Some(Weekday::Tuesday),
        3. => Some(Weekday::Wednesday),
        4. => Some(Weekday::Thursday),
        5. => Some(Weekday::Friday),
        6. => Some(Weekday::Saturday),
        _ => None,
    }
}
