// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

pub mod constants; // src-tauri/src/core/constants.rs
pub mod logger; // src-tauri/src/core/logger.rs
pub mod menu; // src-tauri/src/core/menu.rs
pub mod time; // src-tauri/src/core/time.rs
pub mod tray; // src-tauri/src/core/tray.rs
pub mod uuid; // src-tauri/src/core/uuid.rs
pub mod website; // src-tauri/src/core/website.rs
pub mod words; // src-tauri/src/words.rs

pub use {
    constants::*,
    logger::*,
    menu::*,
    self::uuid::*,
    self::time::*,
    tray::*,
    website::*,
    words::*,
};
