// Copyright © 2022-2023 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

pub mod menu; // src-tauri/src/core/menu.rs
pub mod tray; // src-tauri/src/core/tray.rs
pub mod website; // src-tauri/src/core/website.rs
pub mod words; // src-tauri/src/words.rs

pub use {
    menu::*,
    tray::*,
    website::*,
    words::*,
};
