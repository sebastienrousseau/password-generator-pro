// SPDX-License-Identifier: MIT OR Apache-2.0

mod commands;   // src-tauri/src/core/commands.rs
mod consts;     // src-tauri/src/consts.rs
mod logger;     // src-tauri/src/core/logger.rs
mod menu;       // src-tauri/src/menu.rs
mod tray;       // src-tauri/src/tray.rs
mod words;      // src-tauri/src/words.rs

pub use commands::*;
pub use consts::*;
pub use logger::*;
pub use menu::*;
pub use tray::*;
pub use words::*;