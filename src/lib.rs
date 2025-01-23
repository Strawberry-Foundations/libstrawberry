#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::should_implement_trait)]
#![allow(dead_code)]

pub mod colors;
pub mod constants;
pub mod email;
pub mod external;
pub mod file;
pub mod id;
pub mod logging;
pub mod networking;
pub mod stbchat;
pub mod strings;
pub mod utilities;

#[cfg(feature = "notifications")]
pub mod notifications;
#[cfg(feature = "stbchat-scapi")]
pub mod scapi;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
