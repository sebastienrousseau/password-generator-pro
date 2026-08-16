// Copyright © 2022-2026 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Application menu construction.
//!
//! Tauri 2 unified the menu types: `CustomMenuItem` became
//! [`MenuItem`], the native `MenuItem::Quit`-style variants became
//! [`PredefinedMenuItem`] constructors, and every item is built against
//! a [`Manager`] rather than as a free value. The shape of the menu is
//! unchanged from v1 — the same submenus, in the same order, emitting
//! the same identifiers.

use crate::core::ids::*;
use crate::NAME;
use convert_case::{Case, Casing};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::{Manager, Runtime};

/// The application name, title-cased for display.
#[must_use]
pub fn display_name() -> String {
    NAME.to_string().to_case(Case::Title)
}

/// Build the main application menu.
///
/// The macOS application submenu is compiled in only on macOS, matching
/// the platform convention; other platforms get `Quit` on the File menu
/// instead.
///
/// # Errors
///
/// Returns [`tauri::Error`] if a menu item cannot be registered with the
/// application.
pub fn create_menu<R: Runtime, M: Manager<R>>(app: &M) -> tauri::Result<Menu<R>> {
    let name = display_name();
    let menu = Menu::new(app)?;

    #[cfg(target_os = "macos")]
    menu.append(&create_macos_menu(app, &name)?)?;

    menu.append(&create_file_menu(app)?)?;
    menu.append(&create_window_menu(app)?)?;
    menu.append(&create_help_menu(app, &name)?)?;

    Ok(menu)
}

/// Build the macOS application submenu.
///
/// # Errors
///
/// Returns [`tauri::Error`] if a menu item cannot be registered.
#[cfg(target_os = "macos")]
pub fn create_macos_menu<R: Runtime, M: Manager<R>>(
    app: &M,
    name: &str,
) -> tauri::Result<Submenu<R>> {
    let about = MenuItem::with_id(app, ABOUT, format!("About {name}"), true, None::<&str>)?;
    Submenu::with_items(
        app,
        name,
        true,
        &[
            &about,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::hide(app, None)?,
            &PredefinedMenuItem::hide_others(app, None)?,
            &PredefinedMenuItem::show_all(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::quit(app, None)?,
        ],
    )
}

/// Build the File submenu.
///
/// # Errors
///
/// Returns [`tauri::Error`] if a menu item cannot be registered.
pub fn create_file_menu<R: Runtime, M: Manager<R>>(app: &M) -> tauri::Result<Submenu<R>> {
    let close = PredefinedMenuItem::close_window(app, None)?;

    #[cfg(target_os = "macos")]
    let items: Vec<&dyn tauri::menu::IsMenuItem<R>> = vec![&close];

    #[cfg(not(target_os = "macos"))]
    let quit = PredefinedMenuItem::quit(app, None)?;
    #[cfg(not(target_os = "macos"))]
    let items: Vec<&dyn tauri::menu::IsMenuItem<R>> = vec![&close, &quit];

    Submenu::with_items(app, "File", true, &items)
}

/// Build the Window submenu.
///
/// # Errors
///
/// Returns [`tauri::Error`] if a menu item cannot be registered.
pub fn create_window_menu<R: Runtime, M: Manager<R>>(app: &M) -> tauri::Result<Submenu<R>> {
    let minimize = PredefinedMenuItem::minimize(app, None)?;

    #[cfg(target_os = "macos")]
    let maximize = PredefinedMenuItem::maximize(app, None)?;
    #[cfg(target_os = "macos")]
    let items: Vec<&dyn tauri::menu::IsMenuItem<R>> = vec![&minimize, &maximize];

    #[cfg(not(target_os = "macos"))]
    let items: Vec<&dyn tauri::menu::IsMenuItem<R>> = vec![&minimize];

    Submenu::with_items(app, "Window", true, &items)
}

/// Build the Help submenu.
///
/// # Errors
///
/// Returns [`tauri::Error`] if a menu item cannot be registered.
pub fn create_help_menu<R: Runtime, M: Manager<R>>(
    app: &M,
    name: &str,
) -> tauri::Result<Submenu<R>> {
    let website = MenuItem::with_id(app, WEBSITE, "Get Started", true, None::<&str>)?;
    let documentation = MenuItem::with_id(app, DOCUMENTATION, "Documentation", true, None::<&str>)?;
    let release_notes = MenuItem::with_id(app, RELEASE_NOTES, "Release Notes", true, None::<&str>)?;
    let report_issue = MenuItem::with_id(app, REPORT_ISSUE, "Report Issue", true, None::<&str>)?;
    let license = MenuItem::with_id(app, LICENSE, "License Agreement", true, None::<&str>)?;
    let acknowledgements = MenuItem::with_id(
        app,
        ACKNOWLEDGEMENTS,
        "Acknowledgements",
        true,
        None::<&str>,
    )?;
    let quit = MenuItem::with_id(app, QUIT, format!("Quit {name}"), true, Some("CmdOrCtrl+Q"))?;

    // "About" lives in the application submenu on macOS, which does not
    // exist on Windows or Linux — so on those platforms the item was
    // absent from the whole menu bar and there was no way to reach the
    // version, copyright or licence. Help is where both platforms put
    // it by convention.
    #[cfg(not(target_os = "macos"))]
    let about = MenuItem::with_id(app, ABOUT, format!("About {name}"), true, None::<&str>)?;

    let mut items: Vec<&dyn tauri::menu::IsMenuItem<R>> = Vec::new();
    let separator = PredefinedMenuItem::separator(app)?;

    items.push(&website);
    items.push(&documentation);
    items.push(&release_notes);
    items.push(&separator);
    items.push(&report_issue);
    items.push(&separator);
    items.push(&license);
    items.push(&acknowledgements);

    #[cfg(not(target_os = "macos"))]
    {
        items.push(&separator);
        items.push(&about);
    }

    items.push(&separator);
    items.push(&quit);

    Submenu::with_items(app, "Help", true, &items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_name_is_title_cased() {
        let name = display_name();
        assert!(!name.is_empty());
        assert!(!name.contains('-'), "name kept a hyphen: {name:?}");
        assert!(!name.contains('_'), "name kept an underscore: {name:?}");
    }
}
