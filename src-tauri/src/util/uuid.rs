//! # UUID Utility
//!
//! UUID provides a simple way to generate a new UUID.
//!
//! # Usage
//!
//! ```rust
//! use uuid::UUID;
//!
//! // create a new UUID
//! let uuid = UUID::uuid();
//! assert_eq!(uuid.len(), 36);
//! ```
//!

// Copyright © 2022-2023 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/// Implements [`UUID`] to generate a new UUID.
///
/// # Example
///
/// ```rust
/// use uuid::UUID;
///
/// let uuid = UUID::uuid();
/// assert_eq!(uuid.len(), 36);
/// ```


use uuid::Uuid;

/// UUID Utility
///
/// By default, a new UUID is generated.
#[non_exhaustive]
pub struct UUID;

impl UUID {
    /// Initializes a new [`UUID`].
    ///
    /// ```no_run
    /// use uuid::UUID;
    ///
    /// let uuid = UUID::new();
    /// assert_eq!(uuid, "3b6c0b0a-0b0a-4b0a-0b0a-0b0a0b0a0b0a");
    /// ```
    pub fn uuid() -> String {
        Uuid::new_v4().to_string()
    }
}

impl Clone for UUID {
    fn clone(&self) -> Self {
        UUID
    }
}

impl Default for UUID {
    fn default() -> Self {
        UUID
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_uuid() {
        let uuid = UUID::uuid();
        assert_eq!(uuid.len(), 36);
    }

    #[test]
    fn test_clone() {
        let uuid = UUID::uuid();
        let uuid_clone = uuid.clone();
        assert_eq!(uuid, uuid_clone);
    }


}
