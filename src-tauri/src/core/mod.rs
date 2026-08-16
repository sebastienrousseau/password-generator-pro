// Copyright © 2022-2023 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/// Action module
pub mod action;
/// Menu identifiers
pub mod ids;
/// Menu module
pub mod menu;
/// Tray module
pub mod tray;
/// Website module
pub mod website;
/// Words module
pub mod words;

/// Re-exported modules
pub use {action::*, ids::*, menu::*, tray::*, website::*, words::*};
