use std::io::Write;
use std::net::TcpStream;
use crate::constants::KEEPALIVE_MESSAGE;

/// Simple keep alive function for std tcp streams
/// # Panics
///
/// - Will panic when stream is no longer open/writable
pub fn keep_alive(mut stream: TcpStream, duration: u16) {
    loop {
        crate::utilities::sleep(u64::from(duration));
        stream.write_all(KEEPALIVE_MESSAGE.as_bytes()).unwrap();
    }
}
