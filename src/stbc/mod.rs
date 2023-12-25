use std::io::Write;
use std::net::TcpStream;
use crate::constants::KEEPALIVE_MESSAGE;

pub fn keep_alive(mut stream: TcpStream, duration: u16) {
    loop {
        crate::utilities::sleep(duration as u64);
        stream.write_all(KEEPALIVE_MESSAGE.as_bytes()).unwrap();
    }
}
