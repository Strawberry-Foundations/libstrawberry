#![cfg(feature = "stbchat")]

pub mod net;
pub mod object;
pub mod packet;
pub mod error;
pub mod keep_alive;

pub const PROTOCOL_VERSION: &str = "3";