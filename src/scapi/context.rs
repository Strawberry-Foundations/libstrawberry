use crate::stbchat::net::OutgoingPacketStream;
use crate::stbchat::packet::ServerPacket;
use tokio::io::WriteHalf;
use tokio::net::TcpStream;

pub struct Context {
    /// The user who executed the command
    pub executor: String,

    /// Arguments that the executor passed
    pub args: Vec<String>,

    /// Target channel of user
    pub channel: Channel,
}

pub struct Channel {
    pub w_server: OutgoingPacketStream<WriteHalf<TcpStream>>,
}

impl Channel {
    /// # Panics
    ///
    pub async fn send(&mut self, message: impl ToString) {
        self.w_server
            .write(ServerPacket::Message {
                message: message.to_string(),
            })
            .await
            .expect("Err");
    }
}
