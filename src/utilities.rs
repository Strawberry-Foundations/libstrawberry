use std::any::type_name;
use std::thread;
use std::time::Duration;
use chrono::prelude::*;

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