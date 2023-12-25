use std::io::Write;
use std::net::TcpStream;

pub fn keep_alive(mut stream: TcpStream, duration: u16) {
    loop {
        crate::utilities::sleep(duration as u64);
        stream.write_all(b"[#<keepalive.event.sent>]").expect("Failed to send Keep Alive");
    }
}
