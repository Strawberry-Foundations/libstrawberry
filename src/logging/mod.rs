pub mod featureset;
pub mod formats;
pub mod level;

use crate::logging::featureset::FeatureSet;
use crate::logging::formats::LogFormat;
use crate::logging::level::LogLevel;
use crate::time::current_time;
use std::fmt::Display;

/// A simple console logger struct with custom formatting and more
pub struct Logger {
    pub feat_set: FeatureSet,
    pub formatting: LogFormat,
}

impl Logger {
    /// Create a new Logger object
    #[must_use]
    pub const fn new(feature_set: FeatureSet, formatter: LogFormat) -> Self {
        Self {
            feat_set: feature_set,
            formatting: formatter,
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

        if self.formatting.log_options.levelname_lowercase {
            level_str.to_lowercase()
        } else {
            String::from(level_str)
        }
    }

    /// Will parse various placeholders that can be used by custom logging formats
    fn parse(&self, level: &LogLevel, content: &impl ToString) -> String {
        let (template, level_str) = match level {
            LogLevel::OK => (&self.formatting.ok, self.map_loglevel(level)),
            LogLevel::INFO => (&self.formatting.info, self.map_loglevel(level)),
            LogLevel::ERROR => (&self.formatting.error, self.map_loglevel(level)),
            LogLevel::WARNING => (&self.formatting.warning, self.map_loglevel(level)),
            LogLevel::CRITICAL => (&self.formatting.critical, self.map_loglevel(level)),
            LogLevel::PANIC => (&self.formatting.panic, self.map_loglevel(level)),
        };

        template
            .replace("[%<levelname>%]", &level_str)
            .replace("[%<message>%]", &content.to_string())
            .replace(
                "[%<time>%]",
                current_time(&self.formatting.log_options.timestamp_format).as_str(),
            )
    }

    /// Default log function
    pub fn ok(&self, log_message: impl Display) {
        println!("{}", self.parse(&LogLevel::OK, &log_message));
    }

    /// Info log function
    pub fn info(&self, log_message: impl Display) {
        println!("{}", self.parse(&LogLevel::INFO, &log_message));
    }

    /// Error log function
    pub fn error(&self, log_message: impl Display) {
        println!("{}", self.parse(&LogLevel::ERROR, &log_message));
    }

    /// Warning log function
    pub fn warning(&self, log_message: impl Display) {
        println!("{}", self.parse(&LogLevel::WARNING, &log_message));
    }

    /// Critical log function
    pub fn critical(&self, log_message: impl Display) {
        println!("{}", self.parse(&LogLevel::CRITICAL, &log_message));
    }

    /// Panic log function
    pub fn panic(&self, log_message: impl Display) {
        println!("{}", self.parse(&LogLevel::PANIC, &log_message));
    }

    /// Panic log function which will exit with exit code 1
    pub fn error_panic(&self, log_message: impl Display) -> ! {
        println!("{}", self.parse(&LogLevel::ERROR, &log_message));
        std::process::exit(1);
    }

    /// Critical log function which will exit with exit code 1
    pub fn critical_panic(&self, log_message: impl Display) -> ! {
        println!("{}", self.parse(&LogLevel::CRITICAL, &log_message));
        std::process::exit(1);
    }

    /// Panic log function which will exit with exit code 1
    pub fn panic_crash(&self, log_message: impl Display) -> ! {
        println!("{}", self.parse(&LogLevel::PANIC, &log_message));
        std::process::exit(1);
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger {
            feat_set: FeatureSet {
                enable_file_handler: false,
            },
            formatting: formats::default_fmt(),
        }
    }
}
