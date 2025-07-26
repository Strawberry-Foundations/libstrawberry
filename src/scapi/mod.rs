#![cfg(feature = "stbchat")]
#![allow(clippy::future_not_send)]
use tokio::io::{ReadHalf, WriteHalf, split};
/// TODO: Use built-in logging from stblib
use tokio::net::TcpStream;

use num_traits::ToPrimitive;
use std::fmt::{Display, Formatter};
use std::string::ToString;
use std::time::Duration;

pub mod addons;
pub mod command;
pub mod context;
pub mod flags;
pub mod permissions;

use crate::colors::{BLUE, BOLD, C_RESET, CYAN, GREEN, RED, YELLOW};
use crate::scapi::command::Command;
use crate::scapi::context::{Channel, Context};
use crate::scapi::flags::BotFlags;
use crate::scapi::permissions::PermissionList;
use crate::stbchat::net::{IncomingPacketStream, OutgoingPacketStream};
use crate::stbchat::packet::{ClientPacket, ServerPacket};
use crate::time::current_time;

const VERSION: &str = "1.0.0";
const FULL_VERSION: &str = "_dev-vacakes-stblib::rs_stbmv3";
const AUTHORS: [&str; 1] = ["Juliandev02"];
const CODENAME: &str = "Vanilla Cake";
const API: &str = "https://api.strawberryfoundations.xyz/v1/";

pub struct Bot {
    pub username: String,
    pub token: String,
    pub address: String,
    pub port: u16,
    pub prefix: String,

    pub r_server: Option<IncomingPacketStream<ReadHalf<TcpStream>>>,
    pub w_server: Option<OutgoingPacketStream<WriteHalf<TcpStream>>>,
}

impl Bot {
    pub fn new(
        username: impl ToString,
        token: impl ToString,
        address: impl ToString,
        port: usize,
        prefix: impl ToString,
    ) -> Self {
        let bot = Self {
            username: username.to_string(),
            token: token.to_string(),
            address: address.to_string(),
            port: port.to_u16().unwrap(),
            prefix: prefix.to_string(),
            r_server: None,
            w_server: None,
        };

        bot
    }

    pub async fn run(mut self) {
        let host = format!("{}:{}", self.address, self.port);
        let stream = TcpStream::connect(host).await.unwrap();
        let sock_ref = socket2::SockRef::from(&stream);

        let mut ka = socket2::TcpKeepalive::new();
        ka = ka.with_time(Duration::from_secs(20));
        ka = ka.with_interval(Duration::from_secs(20));

        sock_ref.set_tcp_keepalive(&ka).unwrap();

        let (r_server, w_server) = split(stream);

        let r_server = IncomingPacketStream::wrap(r_server);
        let w_server = OutgoingPacketStream::wrap(w_server);

        self.r_server = self.r_server;
        self.w_server = self.w_server;
    }
}
