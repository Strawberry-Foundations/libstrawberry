#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::should_implement_trait)]
#![allow(dead_code)]

pub mod colors;
pub mod email;
pub mod ext;
pub mod file;
#[cfg(feature = "strawberryid")]
pub mod id;
pub mod localization;
pub mod logging;
#[cfg(feature = "notifications")]
pub mod notifications;
pub mod reflection;
#[cfg(feature = "stbchat-scapi")]
pub mod scapi;
pub mod stbchat;
pub mod string;
pub mod time;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const STRAWBERRY_ID_API: &str = "https://id.strawberryfoundations.org/v2/";
