// SPDX-License-Identifier: MIT OR Apache-2.0

mod consts;     // src-tauri/src/consts.rs
mod logger;     // src-tauri/src/core/logger.rs
mod menu;       // src-tauri/src/menu.rs
mod tray;       // src-tauri/src/tray.rs
mod website;    // src-tauri/src/core/website.rs
mod words;      // src-tauri/src/words.rs

pub use {
    consts::*,
    logger::*,
    menu::*,
    tray::*,
    website::*,
    words::*,
};
