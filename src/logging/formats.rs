use crate::colors::{BLUE, BOLD, C_RESET, RED, YELLOW, GRAY};

/// Struct for creating a log format
pub struct LogFormatOptions {
    pub timestamp_format: String,
    pub levelname_lowercase: bool,
}

/// Struct for creating a log format
pub struct LogFormat {
    pub info: String,
    pub error: String,
    pub ok: String,
    pub warning: String,
    pub critical: String,
    pub panic: String,
    pub log_options: LogFormatOptions,
}

/// Default log format, used by Strawberry Chat
#[must_use]
pub fn default_fmt() -> LogFormat {
    LogFormat {
        info: format!("{C_RESET}{BOLD}{GRAY}[%<time>%] {BLUE}[%<levelname>%]{C_RESET}    [%<message>%]"),
        error: format!("{C_RESET}{BOLD}{GRAY}[%<time>%] {RED}[%<levelname>%]{C_RESET}   [%<message>%]"),
        ok: format!("{C_RESET}{BOLD}{GRAY}[%<time>%] {BLUE}[%<levelname>%]{C_RESET}   [%<message>%]"),
        warning: format!(
            "{C_RESET}{BOLD}{GRAY}[%<time>%] {YELLOW}[%<levelname>%]{C_RESET} [%<message>%]"
        ),
        critical: format!("{C_RESET}{BOLD}{GRAY}[%<time>%] {RED}[%<levelname>%]{C_RESET} [%<message>%]"),
        panic: format!("{C_RESET}{BOLD}{GRAY}[%<time>%] {RED}[%<levelname>%]{C_RESET}   [%<message>%]"),
        log_options: LogFormatOptions {
            timestamp_format: "%Y-%m-%d %H:%M".to_string(),
            levelname_lowercase: false,
        },
    }
}

/// Primitive log format
#[must_use]
pub fn basic_fmt() -> LogFormat {
    LogFormat {
        info: "[%<levelname>%]: [%<message>%]".to_string(),
        error: "[%<levelname>%]: [%<message>%]".to_string(),
        ok: "[%<levelname>%]: [%<message>%]".to_string(),
        warning: "[%<levelname>%]: [%<message>%]".to_string(),
        critical: "[%<levelname>%]: [%<message>%]".to_string(),
        panic: "[%<levelname>%]: [%<message>%]".to_string(),
        log_options: LogFormatOptions {
            timestamp_format: String::new(),
            levelname_lowercase: false,
        },
    }
}
