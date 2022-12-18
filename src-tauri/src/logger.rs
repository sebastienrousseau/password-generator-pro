//! # Logger
//!
//! Logger provides an opinionated logger with a simple, readable output format.
//!
//! # Usage
//!
//! ```rust
//! use logger::Logger;
//!
//! // Log a message to the console with a timestamp, info, message, and details to stderr.
//! let logger = Logger::new(
//!     "2022-22-22 22:22:22.222222 +00:00:00",
//!     "INFO",
//!     "SystemTrayEvent",
//!     "Showing main window"
//! );
//! assert_eq!(logger, ());
//! ```
//!

// Copyright 2022-2023 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/// Implements [`Logger`] to log a message to the console with a simple, readable output format.
///
/// # Arguments
///
/// * `time` - A string slice that holds the timestamp.
/// * `info` - A string slice that holds the info.
/// * `message` - A string slice that holds the message.
/// * `details` - A string slice that holds the details.
///

#[non_exhaustive]
pub struct Logger {
    time: String,
    info: String,
    message: String,
    details: String,
}

impl Logger {
    /// Initializes a new [`Logger`].
    ///
    /// ```no_run
    /// use logger::Logger;
    /// Logger::new(
    ///     "2022-22-22 22:22:22.222222 +00:00:00",
    ///     "INFO",
    ///     "SystemTrayEvent",
    ///     "Showing main window"
    ///);
    /// ```
    ///
    #[must_use]
    pub fn new(time: &str, info: &str, message: &str, details: &str) -> Self {
        Self {
            time: time.to_string(),
            info: info.to_string(),
            message: message.to_string(),
            details: details.to_string(),
        }
    }

    /// Logs a message to the console with a simple, readable output format.
    ///
    /// ```no_run
    /// use logger::Logger;
    /// Logger::new(
    ///     "2022-22-22 22:22:22.222222 +00:00:00",
    ///     "INFO",
    ///     "SystemTrayEvent",
    ///     "Showing main window"
    ///).log();
    /// ```
    ///
    pub fn log(&self) {
        println!(
            "🔒 {} - [{}] {}: {}",
            self.time, self.info, self.message, self.details
        );
    }
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        Self {
            time: self.time.clone(),
            info: self.info.clone(),
            message: self.message.clone(),
            details: self.details.clone(),
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            time: String::new(),
            info: String::new(),
            message: String::new(),
            details: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        let logger = Logger::new(
            "2022-22-22 22:22:22.222222 +00:00:00",
            "INFO",
            "SystemTrayEvent",
            "Showing main window",
        );
        assert_eq!(logger.time, "2022-22-22 22:22:22.222222 +00:00:00");
        assert_eq!(logger.info, "INFO");
        assert_eq!(logger.message, "SystemTrayEvent");
        assert_eq!(logger.details, "Showing main window");
    }

    #[test]
    fn test_logger_log() {
        let logger = Logger::new(
            "2022-22-22 22:22:22.222222 +00:00:00",
            "INFO",
            "SystemTrayEvent",
            "Showing main window",
        );
        logger.log();
    }

    #[test]
    fn test_logger_clone() {
        let logger = Logger::new(
            "2022-22-22 22:22:22.222222 +00:00:00",
            "INFO",
            "SystemTrayEvent",
            "Showing main window",
        );
        let logger_clone = logger.clone();
        assert_eq!(logger.time, logger_clone.time);
        assert_eq!(logger.info, logger_clone.info);
        assert_eq!(logger.message, logger_clone.message);
        assert_eq!(logger.details, logger_clone.details);
    }

    #[test]
    fn test_logger_default() {
        let logger = Logger::default();
        assert_eq!(logger.time, "");
        assert_eq!(logger.info, "");
        assert_eq!(logger.message, "");
        assert_eq!(logger.details, "");
    }
}
