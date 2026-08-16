// Copyright © 2022-2026 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! System-tray menu construction.
//!
//! Tauri 2 replaced the `SystemTray` / `SystemTrayMenu` types with the
//! generic [`tauri::menu`] API shared by the tray and the application
//! menu. Menu items are now built against a [`Manager`], because each
//! item is registered with the running app rather than being a free
//! value, so the builder takes an app handle where the v1 version took
//! nothing.

use crate::core::ids::*;
use crate::NAME;
use convert_case::{Case, Casing};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::{Manager, Runtime};

/// The label shown for the "About" entry, e.g. `About Password Generator Pro`.
///
/// Split out from menu construction so it can be asserted without a
/// running application.
#[must_use]
pub fn about_label() -> String {
    format!("About {}", NAME.to_case(Case::Title))
}

/// Build the system-tray menu.
///
/// # Errors
///
/// Returns [`tauri::Error`] if any menu item cannot be registered with
/// the application — for example when the platform menu backend is
/// unavailable.
pub fn build_tray_menu<R: Runtime, M: Manager<R>>(app: &M) -> tauri::Result<Menu<R>> {
    let about = MenuItem::with_id(app, ABOUT, about_label(), true, None::<&str>)?;
    let website = MenuItem::with_id(app, WEBSITE, "Get Started", true, None::<&str>)?;
    let documentation = MenuItem::with_id(app, DOCUMENTATION, "Documentation", true, None::<&str>)?;
    let quick_password = MenuItem::with_id(
        app,
        QUICK_PASSWORD,
        "Copy Password to Clipboard",
        true,
        None::<&str>,
    )?;
    let quick_uuid = MenuItem::with_id(
        app,
        QUICK_UUID,
        "Copy UUID to Clipboard",
        true,
        None::<&str>,
    )?;
    let quick_qrcode = MenuItem::with_id(
        app,
        QUICK_QRCODE,
        "Save QR Code to File",
        true,
        None::<&str>,
    )?;
    // The accelerator is declared rather than baked into the label, so
    // the platform renders it in its own convention.
    let hide = MenuItem::with_id(
        app,
        HIDE,
        "Hide Password Generator",
        true,
        Some("CmdOrCtrl+H"),
    )?;
    let quit = MenuItem::with_id(
        app,
        QUIT,
        "Quit Password Generator",
        true,
        Some("CmdOrCtrl+Q"),
    )?;

    Menu::with_items(
        app,
        &[
            &about,
            &website,
            &documentation,
            &PredefinedMenuItem::separator(app)?,
            &quick_password,
            &quick_uuid,
            &quick_qrcode,
            &PredefinedMenuItem::separator(app)?,
            &hide,
            &PredefinedMenuItem::separator(app)?,
            &quit,
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn about_label_is_title_cased() {
        let label = about_label();
        assert!(label.starts_with("About "), "got {label:?}");
        // NAME is kebab/snake in the manifest; the label must not leak that.
        assert!(!label.contains('-'), "label kept a hyphen: {label:?}");
        assert!(!label.contains('_'), "label kept an underscore: {label:?}");
    }
}
