#![cfg(feature = "stbchat")]
#![allow(clippy::future_not_send)]
/// TODO: Use built-in logging from stblib


use tokio::net::TcpStream;
use tokio::io::{ReadHalf, split, WriteHalf};

use std::fmt::{Display, Formatter};
use std::string::ToString;
use std::time::Duration;


pub mod addons;
pub mod command;
pub mod flags;
pub mod permissions;
pub mod context;

use crate::colors::{BLUE, BOLD, CYAN, C_RESET, GREEN, RED, YELLOW};
use crate::scapi::command::Command;
use crate::scapi::context::{Channel, Context};
use crate::scapi::flags::BotFlags;
use crate::scapi::permissions::PermissionList;
use crate::stbchat::net::{IncomingPacketStream, OutgoingPacketStream};
use crate::stbchat::packet::{ClientPacket, ServerPacket};
use crate::utilities::current_time;

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
    pub flags: BotFlags,
    pub permissions: PermissionList,
    pub log_msg: String,
    pub cmds: Vec<Command>,
    pub w_server: OutgoingPacketStream<WriteHalf<TcpStream>>,
    pub r_server: IncomingPacketStream<ReadHalf<TcpStream>>,
}


pub enum LogLevel {
    MESSAGE,
    INFO,
    WARNING,
    ERROR,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MESSAGE => write!(f, "{GREEN}MESSAGE"),
            Self::INFO => write!(f, "{BLUE}INFO"),
            Self::WARNING => write!(f, "{YELLOW}WARNING"),
            Self::ERROR => write!(f, "{RED}ERROR"),
        }
    }
}

impl Bot {
    #[must_use]
    /// # Panics
    /// 
    pub async fn new(username: &str, token: &str, address: &str, port: u16) -> Self {
        let stream = TcpStream::connect((address, port)).await.unwrap();

        let sock_ref = socket2::SockRef::from(&stream);

        let mut ka = socket2::TcpKeepalive::new();
        ka = ka.with_time(Duration::from_secs(20));
        ka = ka.with_interval(Duration::from_secs(20));

        sock_ref.set_tcp_keepalive(&ka).unwrap();

        let (r_server, w_server) = split(stream);

        let r_server = IncomingPacketStream::wrap(r_server);
        let w_server = OutgoingPacketStream::wrap(w_server);

        Self {
            username: username.to_string(),
            token: token.to_string(),
            address: address.to_string(),
            port,
            prefix: "!".to_string(),
            flags: BotFlags { enable_user_input: false, log_recv_msg: false },
            permissions: PermissionList {
                trusted: vec![],
                admin: vec![],
                custom: vec![],
                owner: String::new(),
            },
            log_msg: format!(
                "{}{}{}  {}scapi  -->  {}{}",
                CYAN, BOLD, "", "", C_RESET, ""
            ),
            cmds: vec![],
            w_server,
            r_server
        }
    }

    pub fn flag_handler(&mut self, enable_user_input: bool, log_recv_msg: bool) {
        self.flags.enable_user_input = enable_user_input;
        self.flags.log_recv_msg = log_recv_msg;
    }

    pub fn logger(&mut self, message: impl Display, log_type: &LogLevel) {
        self.log_msg = format!(
            "{CYAN}{BOLD}{time}  {log_type}\tscapi --> {C_RESET}{message}",
            time = current_time("%Y-%m-%d %H:%M:%S")
        );
        println!("{}", self.log_msg);
    }

    pub fn log_fmt(&mut self, message: impl Display, log_type: &LogLevel) -> String {
        format!(
            "{CYAN}{BOLD}{time}  {log_type}\tscapi --> {C_RESET}{message}",
            time = current_time("%Y-%m-%d %H:%M:%S")
        )
    }

    /// # Panics
    ///
    /// - Will panic if stream is closed/not writeable
    /// 
    /// # Errors
    pub async fn login(&mut self) -> eyre::Result<()> {
        self.w_server.write(ServerPacket::Login {
            username: self.username.clone(),
            password: self.token.clone()
        }).await
    }

    /* fn send(&mut self) {
        if self.enable_user_input {
            let mut line_reader = rustyline::DefaultEditor::new().unwrap();

            loop {
                let message = line_reader.readline("").unwrap();
                line_reader.add_history_entry(&message).unwrap();
                self.stream.write(message.as_bytes()).expect("Error writing stream");
            }
        }
    } */

    pub fn register_command(&mut self, command: Command) {
        self.cmds.push(command);
    }

    pub async fn run_command(self, name: String, args: Vec<String>) {
        let res = self.exec_command(name, args).await;
        match res {
            Ok(Some(_text)) => {

            }
            Ok(None) => {},
            Err(_e) => {}
        };
    }

    async fn exec_command(self, name: String, args: Vec<String>) -> Result<Option<String>, String> {
        let Some(cmd) = self.cmds.into_iter().find(|cmd| cmd.name == name || cmd.aliases.contains(&name.as_str())) else {
            return Err(format!("Command '{name}' not found"))
        };

        (cmd.handler)(
            Context {
                executor: String::new(),
                args,
                channel: Channel {
                    w_server: self.w_server
                },
            }
        ).await
    }

    async fn recv(&mut self) {
        loop {
            match self.r_server.read::<ClientPacket>().await {
                Ok(ClientPacket::SystemMessage { message}) => {
                    self.logger(message, &LogLevel::INFO);
                },

                Ok(ClientPacket::UserMessage { author, message }) => {
                    let fmt = format!("{}{} (@{}){}{C_RESET} {}", author.username,
                        author.nickname,
                        author.role_color,
                        addons::badge_handler(author.badge.as_str()).unwrap(),
                        message);

                    self.logger(fmt, &LogLevel::MESSAGE);

                    if message.starts_with('/') && message.len() > 1 {
                        let _parts: Vec<String> = message[1..]
                            .split_ascii_whitespace()
                            .map(String::from)
                            .collect();

                        // &self.run_command(parts[0].to_string(), parts[1..].to_vec()).await
                    }
                },

                Ok(ClientPacket::Event { event_type}) => {
                    if event_type == "event.login" {
                        continue
                    }
                }
                Err(_) => break,
                _ => println!(
                    "{RED}{BOLD}[UImp] {YELLOW}{BOLD}Unimplemented package received"
                )
            }
        }
    }

    pub async fn run(&mut self) {
        self.logger(
            format!("{GREEN}Starting scapi {VERSION} (v{VERSION}{FULL_VERSION})").as_str(),
            &LogLevel::INFO,
        );
        if self.flags.enable_user_input {
            self.logger(
                format!(
                    "{YELLOW}Flag {GREEN}{BOLD}'enabled_user_input'{C_RESET}{YELLOW} is enabled"
                )
                .as_str(),
                &LogLevel::INFO,
            );
        }
        if self.flags.log_recv_msg {
            self.logger(
                format!("{YELLOW}Flag {GREEN}{BOLD}'log_recv_msg'{C_RESET}{YELLOW} is enabled")
                    .as_str(),
                &LogLevel::INFO,
            );
        }

        self.recv().await;
    }
}
