#![cfg(feature = "stbchat")]

pub mod error;
pub mod keep_alive;
pub mod net;
pub mod object;
pub mod packet;

pub const PROTOCOL_VERSION: &str = "3";
