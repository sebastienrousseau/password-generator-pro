//! # UUID Utility
//!
//! UUID provides a simple way to generate a new UUID.
//!
//! # Usage
//!
//! ```rust
//! use password_generator_pro::util::uuid::UUID;
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
/// use password_generator_pro::util::uuid::UUID;
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
    /// ```rust
    /// use password_generator_pro::util::uuid::UUID;
    ///
    /// let uuid = UUID::uuid();
    /// // A hyphenated v4 UUID is 36 characters; the value is random,
    /// // so the shape is what can be asserted.
    /// assert_eq!(uuid.len(), 36);
    /// assert_eq!(uuid.matches('-').count(), 4);
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

    #[test]
    fn unit_struct_clone_and_default_are_usable() {
        // Clone and Default are part of the public surface; without
        // these they were never constructed, so a panic added to either
        // would have gone unnoticed.
        let a = UUID;
        let _b = a.clone();
        let _c = UUID::default();
    }

    #[test]
    fn uuid_is_hyphenated_v4_shape() {
        let uuid = UUID::uuid();
        assert_eq!(uuid.matches('-').count(), 4, "not hyphenated: {uuid}");
        let groups: Vec<&str> = uuid.split('-').collect();
        assert_eq!(
            groups.iter().map(|g| g.len()).collect::<Vec<_>>(),
            vec![8, 4, 4, 4, 12],
            "unexpected group widths in {uuid}"
        );
        assert_eq!(
            groups[2].as_bytes()[0],
            b'4',
            "version nibble is not 4: {uuid}"
        );
    }

    #[test]
    fn uuids_are_lowercase_hex() {
        let uuid = UUID::uuid();
        assert!(
            uuid.chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase() || c == '-'),
            "unexpected characters in {uuid}"
        );
    }

    #[test]
    fn successive_uuids_differ() {
        assert_ne!(UUID::uuid(), UUID::uuid());
    }
}
