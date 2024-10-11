#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::should_implement_trait)]
#![allow(dead_code)]

pub mod file;
pub mod utilities;
pub mod networking;
pub mod fmt;
pub mod colors;
pub mod strings;
pub mod logging;
pub mod constants;
pub mod stbchat;
pub mod email;
pub mod id;
pub mod external;

#[cfg(feature = "notifications")] pub mod notifications;
#[cfg(feature = "stbchat-scapi")] pub mod scapi;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");