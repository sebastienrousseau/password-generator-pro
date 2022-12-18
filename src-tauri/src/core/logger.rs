//! # Logger
//!
//! Logger provides an opinionated logger with a simple, readable output format.
//!
//! # Usage
//!
//! ```rust
//! use crate::core::logger;
//!
//! // Log a message to the console with a timestamp, info, message, and details to stderr.
//! let logger = logger::logger(
//!     "2022-22-22 22:22:22.222222 +00:00:00",
//!     "INFO",
//!     "SystemTrayEvent",
//!     "Showing main window"
//! );
//! assert_eq!(logger, ());
//! ```
//!

// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

pub struct Logger {
    time: String,
    info: String,
    message: String,
    details: String,
}

impl Logger {
    pub fn new(time: &str, info: &str, message: &str, details: &str) -> Self {
        Self {
            time: time.to_string(),
            info: info.to_string(),
            message: message.to_string(),
            details: details.to_string(),
        }
    }

    pub fn log(&self) {
        println!(
            "🔒 {} - [{}] {}: {}",
            self.time, self.info, self.message, self.details
        );
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
}


// pub fn set_log(time: &str, info: &str, message: &str, details: &str) {
//     println!(
//         "🔒 {} - [{}] {}: {}",
//         time,
//         info,
//         message,
//         details
//     );
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_logger() {
//         let logger = set_log(
//             "2022-22-22 22:22:22.222222 +00:00:00",
//             "INFO",
//             "SystemTrayEvent",
//             "Showing main window");
//         assert_eq!(logger, ());
//     }
// }
