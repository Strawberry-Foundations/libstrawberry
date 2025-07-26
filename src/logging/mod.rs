pub mod features;
pub mod formats;
pub mod level;

use crate::logging::features::LoggingFeatures;
use crate::logging::formats::LogFormat;
use crate::logging::level::LogLevel;
use crate::time::current_time;
use std::fmt::Display;

/// A simple console logger struct with custom formatting and more
pub struct Logger {
    pub features: LoggingFeatures,
    pub format: LogFormat,
}

impl Logger {
    /// Create a new Logger object
    #[must_use]
    pub const fn new(features: LoggingFeatures, format: LogFormat) -> Self {
        Self {
            features,
            format,
        }
    }

    /// Will parse some logging things >.>
    fn map_loglevel(&self, level: &LogLevel) -> String {
        let level_str = match level {
            LogLevel::OK => "OK",
            LogLevel::INFO => "INFO",
            LogLevel::ERROR => "ERROR",
            LogLevel::WARNING => "WARNING",
            LogLevel::CRITICAL => "CRITICAL",
            LogLevel::PANIC => "PANIC",
        };

        if self.format.log_options.levelname_lowercase {
            level_str.to_lowercase()
        } else {
            String::from(level_str)
        }
    }

    /// Will parse various placeholders that can be used by custom logging formats
    fn parse(&self, level: &LogLevel, string: &impl ToString) -> String {
        let (template, level_str) = match level {
            LogLevel::OK => (&self.format.ok, self.map_loglevel(level)),
            LogLevel::INFO => (&self.format.info, self.map_loglevel(level)),
            LogLevel::ERROR => (&self.format.error, self.map_loglevel(level)),
            LogLevel::WARNING => (&self.format.warning, self.map_loglevel(level)),
            LogLevel::CRITICAL => (&self.format.critical, self.map_loglevel(level)),
            LogLevel::PANIC => (&self.format.panic, self.map_loglevel(level)),
        };

        template
            .replace("[%<levelname>%]", &level_str)
            .replace("[%<message>%]", &string.to_string())
            .replace(
                "[%<time>%]",
                current_time(&self.format.log_options.timestamp_format).as_str(),
            )
    }

    /// Default log function
    pub fn ok(&self, message: impl Display) {
        println!("{}", self.parse(&LogLevel::OK, &message));
    }

    /// Info log function
    pub fn info(&self, message: impl Display) {
        println!("{}", self.parse(&LogLevel::INFO, &message));
    }

    /// Error log function
    pub fn error(&self, message: impl Display) {
        println!("{}", self.parse(&LogLevel::ERROR, &message));
    }

    /// Warning log function
    pub fn warning(&self, message: impl Display) {
        println!("{}", self.parse(&LogLevel::WARNING, &message));
    }

    /// Critical log function
    pub fn critical(&self, message: impl Display) {
        println!("{}", self.parse(&LogLevel::CRITICAL, &message));
    }

    /// Panic log function
    pub fn panic(&self, message: impl Display) {
        println!("{}", self.parse(&LogLevel::PANIC, &message));
    }

    /// Panic log function which will exit with exit code 1
    pub fn error_panic(&self, message: impl Display) -> ! {
        println!("{}", self.parse(&LogLevel::ERROR, &message));
        std::process::exit(1);
    }

    /// Critical log function which will exit with exit code 1
    pub fn critical_panic(&self, message: impl Display) -> ! {
        println!("{}", self.parse(&LogLevel::CRITICAL, &message));
        std::process::exit(1);
    }

    /// Panic log function which will exit with exit code 1
    pub fn panic_crash(&self, message: impl Display) -> ! {
        println!("{}", self.parse(&LogLevel::PANIC, &message));
        std::process::exit(1);
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger {
            features: LoggingFeatures {
                enable_file_handler: false,
            },
            format: formats::default_fmt(),
        }
    }
}
