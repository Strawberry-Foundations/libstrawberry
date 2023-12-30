/// TODO: Use built-in logging from stblib (stblib::logging)

use serde_json::Value;
use std::fmt::{Display, Formatter};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::exit;
use std::string::ToString;

mod addons;

use crate::colors::{BLUE, BOLD, CYAN, C_RESET, GREEN, RED, YELLOW};
use crate::utilities;
use crate::utilities::current_time;

const VERSION: &str = "1.0.0";
const FULL_VERSION: &str = "_dev-vacakes-stblib::rs_stmbv2";

pub struct Bot {
    pub username: String,
    pub token: String,
    pub address: String,
    pub port: u16,
    pub stream: TcpStream,
    pub send_stream: TcpStream,
    pub enable_user_input: bool,
    pub log_recv_msg: bool,
    pub log_msg: String,
    pub json_fmt: bool,
}

pub struct Command<'a> {
    pub name: &'a str,
    pub handler: fn(Vec<String>) -> Vec<String>,
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
    pub fn new(username: &str, token: &str, address: &str, port: u16, json_fmt: bool) -> Self {
        pub fn connect(address: &str, port: u16) -> TcpStream {
            let host = format!("{address}:{port}");

            TcpStream::connect(host).expect("Error opening stream")
        }

        Self {
            username: username.to_string(),
            token: token.to_string(),
            address: address.to_string(),
            port,
            stream: connect(address, port),
            send_stream: connect(address, port),
            enable_user_input: false,
            log_recv_msg: false,
            log_msg: format!(
                "{}{}{}  {}scapi  -->  {}{}",
                CYAN, BOLD, "", "", C_RESET, ""
            ),
            json_fmt,
        }
    }

    pub fn flag_handler(&mut self, enable_user_input: bool, log_recv_msg: bool) {
        self.enable_user_input = enable_user_input;
        self.log_recv_msg = log_recv_msg;
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
    pub fn login(&mut self) {
        self.stream
            .write_all(self.username.as_bytes())
            .expect("Error writing stream");
        utilities::ms_sleep(250);
        self.stream
            .write_all(self.token.as_bytes())
            .expect("Error writing stream");
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

    fn recv(&mut self) {
        if self.json_fmt {
            let mut count: i8 = 0;

            loop {
                let mut buffer = [0u8; 1];
                let mut str_buffer = String::new();
                let mut wraps = 0;

                loop {
                    let stream_reader = match self.stream.read(&mut buffer) {
                        Ok(r) => r,
                        Err(e) => panic!(
                            "{}",
                            self.log_fmt(
                                format!("Error while reading from stream: {e}"),
                                &LogLevel::ERROR
                            )
                            .as_str()
                        ),
                    };

                    if stream_reader == 0 {
                        self.logger("Server connection closed", &LogLevel::ERROR);
                        exit(1)
                    }

                    match buffer[0] as char {
                        '{' => {
                            wraps += 1;
                            str_buffer.push('{');
                        }
                        '}' => {
                            wraps -= 1;
                            str_buffer.push('}');
                        }
                        c => str_buffer.push(c),
                    }

                    if wraps == 0 {
                        break;
                    }
                }

                count += 1;

                let msg: Value = match serde_json::from_str(&str_buffer) {
                    Ok(ok) => ok,
                    Err(e) => {
                        self.logger(
                            format!("Error desering packet ({str_buffer}): {e}").as_str(),
                            &LogLevel::ERROR,
                        );
                        continue;
                    }
                };

                if count > 8 {
                    match msg["message_type"].as_str() {
                        Some("system_message") => self.logger(
                            msg["message"]["content"].as_str().unwrap(),
                            &LogLevel::MESSAGE,
                        ),
                        Some("user_message") => self.logger(
                            format!(
                                "{}{} (@{}){}{C_RESET} {}",
                                msg["role_color"].as_str().unwrap(),
                                msg["nickname"].as_str().unwrap(),
                                msg["username"].as_str().unwrap().to_lowercase(),
                                addons::badge_handler(msg["badge"].as_str().unwrap())
                                    .unwrap_or_default(),
                                msg["message"]["content"].as_str().unwrap()
                            )
                            .as_str(),
                            &LogLevel::MESSAGE,
                        ),

                        None => unreachable!(),
                        m => self.logger(
                            format!(
                                "{YELLOW}Unimplemented packet {} - full packet: {}",
                                m.unwrap(),
                                str_buffer
                            )
                            .as_str(),
                            &LogLevel::WARNING,
                        ),
                    }
                }
            }
        }
    }

    pub fn run(&mut self) {
        self.logger(
            format!("{GREEN}Starting scapi {VERSION} (v{VERSION}{FULL_VERSION})").as_str(),
            &LogLevel::INFO,
        );
        if self.enable_user_input {
            self.logger(
                format!(
                    "{YELLOW}Flag {GREEN}{BOLD}'enabled_user_input'{C_RESET}{YELLOW} is enabled"
                )
                .as_str(),
                &LogLevel::INFO,
            );
        }
        if self.log_recv_msg {
            self.logger(
                format!("{YELLOW}Flag {GREEN}{BOLD}'log_recv_msg'{C_RESET}{YELLOW} is enabled")
                    .as_str(),
                &LogLevel::INFO,
            );
        }

        self.recv();
    }
}
