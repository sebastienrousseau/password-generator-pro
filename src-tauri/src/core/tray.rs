// Copyright © 2022-2023 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// Import necessary modules and libraries
use crate::NAME;
use convert_case::{Case, Casing};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

// Constants for menu item identifiers
const QUIT: &str = "quit";
const WEBSITE: &str = "website";
const DOCUMENTATION: &str = "documentation";
const HIDE: &str = "hide";
const QUICK_PASSWORD: &str = "quick_password";
const QUICK_UUID: &str = "quick_uuid";
const QUICK_QRCODE: &str = "quick_qrcode";
const ABOUT: &str = "about";

/// Create and configure the system tray.
///
/// This function sets up the system tray with custom and native menu items.
/// Custom items include quick actions for generating passwords, UUIDs, and QR codes,
/// as well as options for opening the website and documentation.
///
/// Returns a `SystemTray` object with the configured menu.
#[tauri::command]
pub fn system_tray() -> SystemTray {
    // Create menu items for basic operations
    let quit = CustomMenuItem::new(QUIT.to_string(), "Quit Password Generator   ⌘Q");
    let website = CustomMenuItem::new(WEBSITE.to_string(), "Get Started");
    let documentation = CustomMenuItem::new(DOCUMENTATION.to_string(), "Documentation");
    let hide = CustomMenuItem::new(HIDE.to_string(), "Hide Password Generator   ⌘H");

    // Create an "About" menu item with a dynamically generated name
    let name = format!("About {}", NAME.to_case(Case::Title));

    // Create menu items for quick actions
    let quick_password = CustomMenuItem::new(QUICK_PASSWORD.to_string(), "Copy Password to Clipboard");
    let quick_uuid = CustomMenuItem::new(QUICK_UUID.to_string(), "Copy UUID to Clipboard");
    let quick_qrcode = CustomMenuItem::new(QUICK_QRCODE.to_string(), "Save QR Code to File");

    // Assemble the menu items into a system tray menu
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(ABOUT, name))
        .add_item(website)
        .add_item(documentation)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quick_password)
        .add_item(quick_uuid)
        .add_item(quick_qrcode)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    // Initialize and return the system tray with the constructed menu
    SystemTray::new().with_menu(tray_menu)
}
