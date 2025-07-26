use crate::colors::{BLUE, BOLD, C_RESET, RED, YELLOW};

/// Struct for creating a log format
pub struct LogFormatExt {
    pub time_fmt: String,
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
    pub extensions: LogFormatExt,
}

/// Default log format, used by Strawberry Chat
#[must_use]
pub fn default_fmt() -> LogFormat {
    LogFormat {
        info: format!("{C_RESET}{BOLD}[%<time>%] {BLUE}[%<levelname>%]{C_RESET}    [%<message>%]"),
        error: format!("{C_RESET}{BOLD}[%<time>%] {RED}[%<levelname>%]{C_RESET}   [%<message>%]"),
        ok: format!("{C_RESET}{BOLD}[%<time>%] {BLUE}[%<levelname>%]{C_RESET}   [%<message>%]"),
        warning: format!(
            "{C_RESET}{BOLD}[%<time>%] {YELLOW}[%<levelname>%]{C_RESET} [%<message>%]"
        ),
        critical: format!("{C_RESET}{BOLD}[%<time>%] {RED}[%<levelname>%]{C_RESET} [%<message>%]"),
        panic: format!("{C_RESET}{BOLD}[%<time>%] {RED}[%<levelname>%]{C_RESET}   [%<message>%]"),
        extensions: LogFormatExt {
            time_fmt: "%Y-%m-%d %H:%M".to_string(),
            levelname_lowercase: false,
        },
    }
}

/// Primitive log format
#[must_use]
pub fn basic_fmt() -> LogFormat {
    LogFormat {
        info: format!("[%<levelname>%]: [%<message>%]"),
        error: format!("[%<levelname>%]: [%<message>%]"),
        ok: format!("[%<levelname>%]: [%<message>%]"),
        warning: format!("[%<levelname>%]: [%<message>%]"),
        critical: format!("[%<levelname>%]: [%<message>%]"),
        panic: format!("[%<levelname>%]: [%<message>%]"),
        extensions: LogFormatExt {
            time_fmt: "".to_string(),
            levelname_lowercase: false,
        },
    }
}
