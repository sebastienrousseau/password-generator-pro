// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

pub mod constants; // src-tauri/src/constants.rs
pub mod logger; // src-tauri/src/core/logger.rs
pub mod menu; // src-tauri/src/menu.rs
pub mod tray; // src-tauri/src/tray.rs
pub mod website; // src-tauri/src/core/website.rs
pub mod words; // src-tauri/src/words.rs

pub use {constants::*, logger::*, menu::*, tray::*, website::*, words::*};
