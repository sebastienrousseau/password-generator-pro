//! # Utilities for generating UUIDs.
//!
//! # Usage
//!
//! ```rust
//! use crate::core::uuids;
//!
//! // create a new UUID
//! let uuid = uuids::get_uuids();
//! assert_eq!(uuid.len(), 36);
//! ```
//!

// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use uuid::Uuid;

/// Generates a new UUID and returns it as a string.
/// Example: 3b6c0b0a-0b0a-4b0a-0b0a-0b0a0b0a0b0a
pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_uuid() {
        let uuid = generate_uuid();
        assert_eq!(uuid.len(), 36);
    }
}
