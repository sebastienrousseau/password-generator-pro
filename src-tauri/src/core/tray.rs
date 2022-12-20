// Copyright © 2022-2023 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::NAME;
use convert_case::{Case, Casing};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

#[tauri::command]
pub fn system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit Password Generator   ⌘Q");
    let website: CustomMenuItem = CustomMenuItem::new("website".to_string(), "Get Started");
    let documentation: CustomMenuItem =
        CustomMenuItem::new("documentation".to_string(), "Documentation");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide Password Generator   ⌘H");
    let name = format!("About {}", NAME.to_case(Case::Title));
    let quick_password: CustomMenuItem = CustomMenuItem::new("quick_password".to_string(), "Copy Password to Clipboard");
    let quick_uuid: CustomMenuItem = CustomMenuItem::new("quick_uuid".to_string(), "Copy UUID to Clipboard");
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("about", name))
        .add_item(website)
        .add_item(documentation)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quick_password)
        .add_item(quick_uuid)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}
