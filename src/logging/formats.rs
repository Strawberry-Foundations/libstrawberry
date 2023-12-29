use crate::colors::{BLUE, C_RESET, RED, RESET, YELLOW};
pub struct LogFormatExt {
    pub time_fmt: String,
    pub levelname_lowercase: bool,
}

pub struct LogFormat {
    pub info: String,
    pub error: String,
    pub default: String,
    pub warning: String,
    pub critical: String,
    pub extensions: LogFormatExt
}

pub fn strawberry_chat_fmt() -> LogFormat {
    LogFormat {
        info: format!("{C_RESET}[%<time>%] {BLUE}[%<levelname>%]{RESET}    [%<message>%]"),
        error: format!("{C_RESET}[%<time>%] {RED}[%<levelname>%]{RESET}   [%<message>%]"),
        default: format!("{C_RESET}[%<time>%] {BLUE}[%<levelname>%]{RESET}   [%<message>%]"),
        warning: format!("{C_RESET}[%<time>%] {YELLOW}[%<levelname>%]{RESET} [%<message>%]"),
        critical: format!("{C_RESET}[%<time>%] {RED}[%<levelname>%]{RESET} [%<message>%]"),
        extensions: LogFormatExt {
            time_fmt: "%H:%M".to_string(),
            levelname_lowercase: false
        },
    }
}