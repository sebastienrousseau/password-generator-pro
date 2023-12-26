// Copyright © 2022-2023 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/// Menu module
pub mod menu;
/// Tray module
pub mod tray;
/// Website module
pub mod website;
/// Words module
pub mod words;

/// Re-exported modules
pub use {
    menu::*,
    tray::*,
    website::*,
    words::*,
};
