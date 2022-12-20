// Copyright © 2022-2023 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::NAME;
use convert_case::{Case, Casing};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

// Create the menu for the application.
#[tauri::command]
pub fn create_menu() -> Menu {
    let mut menu = Menu::new();

    #[cfg(target_os = "macos")]
    {
        menu = menu.add_submenu(Submenu::new(
            NAME.to_string().to_case(Case::Title),
            Menu::new()
                .add_item(CustomMenuItem::new(
                    "about".to_string(),
                    format!("About {}", NAME.to_string().to_case(Case::Title)),
                ))
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Hide)
                .add_native_item(MenuItem::HideOthers)
                .add_native_item(MenuItem::ShowAll)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Quit),
        ));
    }

    // Create the file menu.
    let mut file_menu = Menu::new();

    file_menu = file_menu.add_native_item(MenuItem::CloseWindow);

    #[cfg(not(target_os = "macos"))]
    {
        file_menu = file_menu.add_native_item(MenuItem::Quit);
    }
    menu = menu.add_submenu(Submenu::new("File", file_menu));

    // Create the window menu.
    let mut window_menu = Menu::new();

    window_menu = window_menu.add_native_item(MenuItem::Minimize);

    #[cfg(target_os = "macos")]
    {
        window_menu = window_menu.add_native_item(MenuItem::Zoom);
    }
    menu = menu.add_submenu(Submenu::new("Window", window_menu));

    // Create the help menu.
    let mut help_menu = Menu::new();

    help_menu = help_menu.add_item(CustomMenuItem::new("website".to_string(), "Get Started"));
    help_menu = help_menu.add_item(CustomMenuItem::new(
        "documentation".to_string(),
        "Documentation",
    ));
    help_menu = help_menu.add_item(CustomMenuItem::new(
        "release-notes".to_string(),
        "Release Notes",
    ));
    help_menu = help_menu.add_native_item(MenuItem::Separator);
    help_menu = help_menu.add_item(CustomMenuItem::new(
        "report-issue".to_string(),
        "Report Issue",
    ));
    help_menu = help_menu.add_native_item(MenuItem::Separator);
    help_menu = help_menu.add_item(CustomMenuItem::new(
        "license".to_string(),
        "License Agreement",
    ));
    help_menu = help_menu.add_item(CustomMenuItem::new(
        "acknowledgements".to_string(),
        "Acknowledgements",
    ));
    help_menu = help_menu.add_native_item(MenuItem::Separator);
    help_menu = help_menu.add_item(
        CustomMenuItem::new(
            "quit".to_string(),
            format!("Quit {}", NAME.to_string().to_case(Case::Title)),
        )
        .accelerator("CmdOrCtrl+Q"),
    );

    menu = menu.add_submenu(Submenu::new("Help", help_menu));

    menu
}
