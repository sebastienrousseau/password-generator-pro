//! # Utilities for logging messages to the console.
//! An easy-to-configure and flexible logger function.
//! Log a message to the console with a timestamp, info, message, and details.
//!
//! # Usage
//!
//! ```rust
//! use crate::core::logger;
//!
//! // Log a message to the console with a timestamp, info, message, and details
//! let logger = logger::logger(
//!     "2022-12-18 2:02:08.706196 +00:00:00",
//!     "INFO",
//!     "SystemTrayEvent",
//!     "Showing main window"
//! );
//! assert_eq!(logger, ());
//! ```
//!

// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#[tauri::command]
pub fn set_log(time: &str, info: &str, message: &str, details: &str) {
    println!(
        "🔒 {} - [{}] {}: {}",
        time,
        info,
        message,
        details
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        let logger = set_log(
            "2022-22-22 22:22:22.222222 +00:00:00",
            "INFO",
            "SystemTrayEvent",
            "Showing main window");
        assert_eq!(logger, ());
    }
}
