use crate::constants::KEEPALIVE_MESSAGE;
use std::io::Write;
use std::net::TcpStream;

/// Simple keep alive function for std tcp streams
/// # Panics
///
/// - Will panic when stream is no longer open/writable
#[deprecated(
    note = "This function is deprecated and will be removed in a future version.",
    since = "1.0.0"
)]
pub fn keep_alive(mut stream: TcpStream, duration: u16) {
    loop {
        crate::time::sleep(u64::from(duration));
        stream.write_all(KEEPALIVE_MESSAGE.as_bytes()).unwrap();
    }
}
