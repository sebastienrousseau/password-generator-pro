// Allow dead code in this module, since it's all packet constants anyways.
#![allow(dead_code)]

// 32 characters unique characters
pub const SPECIAL: &[u8] = b"!@#$%^&*()_+-=[]{};':,./<>?";

pub const VERSION: &[u8] = b"0.0.1";

/// The default hash cost to use for generating
/// bcrypt password hashes.
pub const HASH_COST: u32 = 8;

// Low-level error codes. Return as negatives.
pub const ERR_NONE: u8 = 0x00; // No error
pub const ERR_INVALID_CMD: u8 = 0x01; // Invalid command
pub const ERR_INVALID_PAR: u8 = 0x02; // Invalid parameter
pub const ERR_INVALID_LEN: u8 = 0x03; // Invalid message length
pub const ERR_INVALID_SEQ: u8 = 0x04; // Invalid message sequencing
pub const ERR_TIMEOUT: u8 = 0x05; // Timed out
pub const ERR_INVALID_MAC: u8 = 0x06; // Invalid MAC
pub const ERR_BUSY: u8 = 0x07; // Device or resource busy
pub const ERR_LOCKED: u8 = 0x08; // Device or resource locked
pub const ERR_NOT_FOUND: u8 = 0x09; // Device or resource not found
pub const ERR_EXISTS: u8 = 0x0a; // Device or resource already exists
pub const ERR_INVALID: u8 = 0x0b; // Invalid argument
