use std::thread;
use std::time::Duration;
use chrono::prelude::*;

pub fn sleep(time: u64) {
    thread::sleep(Duration::from_secs(time));
}

pub fn ms_sleep(time: u64) {
    thread::sleep(Duration::from_millis(time));
}

fn current_time(format: String) -> String {
    let local: DateTime<Local> = Local::now();
    local.format(format.as_str()).to_string()
}