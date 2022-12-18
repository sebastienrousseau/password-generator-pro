//! # Utilities for getting the current date and time in UTC.
//! Get the current date and time in UTC.
//!
//! # Usage
//!
//! ```rust
//! use crate::core::time;
//!
//! // Get the current date and time in UTC
//! let utc = time::get_time();
//! assert_eq!(utc.len(), 36);
//! ```
//!

// Copyright 2022-2023 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use time::OffsetDateTime;

/// Returns the current date and time in UTC.
pub fn get_time() -> String {
    OffsetDateTime::now_utc().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_time() {
        let utc = get_time();
        assert_eq!(utc != "", true);
    }
}