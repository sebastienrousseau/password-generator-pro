// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod consts; // src-tauri/src/consts.rs
pub mod logger; // src-tauri/src/core/logger.rs
pub mod menu;   // src-tauri/src/menu.rs
pub mod tray;   // src-tauri/src/tray.rs
pub mod website;// src-tauri/src/core/website.rs
pub mod words;  // src-tauri/src/words.rs

pub use {
    consts::*,
    logger::*,
    menu::*,
    tray::*,
    website::*,
    words::*,
};
