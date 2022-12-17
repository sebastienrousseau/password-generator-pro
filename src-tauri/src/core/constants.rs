// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

pub use time::OffsetDateTime;

// Allow dead code in this module, since it's all packet constants anyways.
// #![allow(dead_code)]

// 32 characters unique characters
pub const SPECIAL: &[u8] = b"!@#$%^&*()_+-=[]{};':,./<>?";

// Cargo package metadata constants
// pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
// pub const LICENSE: &str = env!("CARGO_PKG_LICENSE");
pub const NAME: &str = env!("CARGO_PKG_NAME");
// pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SHA: &str = env!("VERGEN_GIT_SHA");

pub const ACKNOWLEDGEMENTS: &str =
    "https://github.com/sebastienrousseau/password-generator-pro/graphs/contributors";
pub const DOCUMENTATION: &str = "https://password-generator.pro/docs";
pub const LICENSE_URL: &str =
    "https://github.com/sebastienrousseau/password-generator-pro/blob/main/COPYRIGHT";
pub const RELEASE: &str = "https://github.com/sebastienrousseau/password-generator-pro/releases";
pub const ISSUE: &str = "https://github.com/sebastienrousseau/password-generator-pro/issues";

/// The default hash cost to use for generating
/// bcrypt password hashes.
pub const HASH_COST: u32 = 8;

// Low-level error codes. Return as negatives.
// pub const ERR_NONE: u8 = 0x00; // No error
// pub const ERR_INVALID_CMD: u8 = 0x01; // Invalid command
// pub const ERR_INVALID_PAR: u8 = 0x02; // Invalid parameter
// pub const ERR_INVALID_LEN: u8 = 0x03; // Invalid message length
// pub const ERR_INVALID_SEQ: u8 = 0x04; // Invalid message sequencing
// pub const ERR_TIMEOUT: u8 = 0x05; // Timed out
// pub const ERR_INVALID_MAC: u8 = 0x06; // Invalid MAC
// pub const ERR_BUSY: u8 = 0x07; // Device or resource busy
// pub const ERR_LOCKED: u8 = 0x08; // Device or resource locked
// pub const ERR_NOT_FOUND: u8 = 0x09; // Device or resource not found
// pub const ERR_EXISTS: u8 = 0x0a; // Device or resource already exists
// pub const ERR_INVALID: u8 = 0x0b; // Invalid argument
