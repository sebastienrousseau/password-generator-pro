// Copyright © 2022-2023 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::NAME;
use convert_case::{Case, Casing};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

/// Create the main application menu.
///
/// This function constructs the menu by adding various submenus.
/// Platform-specific menus and options are conditionally compiled.
#[tauri::command]
pub fn create_menu() -> Menu {
    let mut menu = Menu::new();
    // Format the application name for consistent display.
    let formatted_name = NAME.to_string().to_case(Case::Title);

    // Add macOS-specific menu items.
    #[cfg(target_os = "macos")]
    {
        menu = menu.add_submenu(create_macos_menu(&formatted_name));
    }

    // Add the File, Window, and Help menus.
    menu = menu.add_submenu(create_file_menu());
    menu = menu.add_submenu(create_window_menu());
    menu = menu.add_submenu(create_help_menu(&formatted_name));

    menu
}

/// Create the macOS-specific menu.
///
/// This function returns a `Submenu` tailored for macOS, including
/// 'About' and 'Quit' options.
fn create_macos_menu(formatted_name: &str) -> Submenu {
    Submenu::new(
        formatted_name.to_string(),
        Menu::new()
            .add_item(CustomMenuItem::new("about".to_string(), format!("About {}", formatted_name)))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::HideOthers)
            .add_native_item(MenuItem::ShowAll)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
    )
}

/// Create the File menu.
///
/// This function returns a `Submenu` that contains common file operations.
/// Platform-specific options are conditionally compiled.
fn create_file_menu() -> Submenu {
    let mut file_menu = Menu::new();
    file_menu = file_menu.add_native_item(MenuItem::CloseWindow);
    
    // Add a Quit option for non-macOS platforms.
    #[cfg(not(target_os = "macos"))]
    {
        file_menu = file_menu.add_native_item(MenuItem::Quit);
    }

    Submenu::new("File", file_menu)
}

/// Create the Window menu.
///
/// This function returns a `Submenu` that contains window management options.
/// macOS-specific options are conditionally compiled.
fn create_window_menu() -> Submenu {
    let mut window_menu = Menu::new();
    window_menu = window_menu.add_native_item(MenuItem::Minimize);

    // Add a Zoom option for macOS platforms.
    #[cfg(target_os = "macos")]
    {
        window_menu = window_menu.add_native_item(MenuItem::Zoom);
    }

    Submenu::new("Window", window_menu)
}

/// Create the Help menu.
///
/// This function returns a `Submenu` that contains links to documentation,
/// release notes, and other useful resources.
fn create_help_menu(formatted_name: &str) -> Submenu {
    let mut help_menu = Menu::new();
    help_menu = help_menu
        .add_item(CustomMenuItem::new("website".to_string(), "Get Started"))
        .add_item(CustomMenuItem::new("documentation".to_string(), "Documentation"))
        .add_item(CustomMenuItem::new("release-notes".to_string(), "Release Notes"))
        .add_native_item(MenuItem::Separator)
        .add_item(CustomMenuItem::new("report-issue".to_string(), "Report Issue"))
        .add_native_item(MenuItem::Separator)
        .add_item(CustomMenuItem::new("license".to_string(), "License Agreement"))
        .add_item(CustomMenuItem::new("acknowledgements".to_string(), "Acknowledgements"))
        .add_native_item(MenuItem::Separator)
        .add_item(
            CustomMenuItem::new(
                "quit".to_string(),
                format!("Quit {}", formatted_name),
            )
            .accelerator("CmdOrCtrl+Q"),
        );

    Submenu::new("Help", help_menu)
}

