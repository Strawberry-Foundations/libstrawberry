use std::thread;
use std::time::Duration;

pub fn sleep(time: u64) {
    thread::sleep(Duration::from_secs(time));
}

pub fn ms_sleep(time: u64) {
    thread::sleep(Duration::from_millis(time));
}
