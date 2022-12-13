// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

#[cfg(target_os = "macos")]
use tauri::AboutMetadata;

const APP_NAME: &str = "Password Generator Pro";

pub fn create_menu() -> Menu {
    let mut menu = Menu::new();
    #[cfg(target_os = "macos")]
    {
        menu = menu.add_submenu(Submenu::new(
        APP_NAME,
        Menu::new()
            .add_native_item(MenuItem::About(
            APP_NAME.to_string(),
            AboutMetadata::default(),
            ))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Services)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::HideOthers)
            .add_native_item(MenuItem::ShowAll)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
        ));
    }

    let mut file_menu = Menu::new();
    file_menu = file_menu

        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::CloseWindow);
    #[cfg(not(target_os = "macos"))]
    {
        file_menu = file_menu.add_native_item(MenuItem::Quit);
    }
    menu = menu.add_submenu(Submenu::new("File", file_menu));

    let mut window_menu = Menu::new();
    window_menu = window_menu.add_native_item(MenuItem::Minimize);
    #[cfg(target_os = "macos")]
    {
        window_menu = window_menu.add_native_item(MenuItem::Zoom);
        window_menu = window_menu.add_native_item(MenuItem::Separator);
    }
    window_menu = window_menu.add_native_item(MenuItem::CloseWindow);
    menu = menu.add_submenu(Submenu::new("Window", window_menu));


    let mut help_menu = Menu::new();
    help_menu = help_menu.add_item(
    CustomMenuItem::new("get-started".to_string(), "Get Started")
    );

    help_menu = help_menu.add_item(
        CustomMenuItem::new("documentation".to_string(), "Documentation")
    );

    help_menu = help_menu.add_item(
        CustomMenuItem::new("release".to_string(), "Show Release Notes")
    );

    help_menu = help_menu.add_native_item(MenuItem::Separator);

    help_menu = help_menu.add_item(
        CustomMenuItem::new("bug".to_string(), "Report Issue")
    );

    help_menu = help_menu.add_native_item(MenuItem::Separator);

    help_menu = help_menu.add_item(
        CustomMenuItem::new("license".to_string(), "View License")
    );

    help_menu = help_menu.add_item(
        CustomMenuItem::new("privacy".to_string(), "Privacy Statement")
    );

    help_menu = help_menu.add_native_item(MenuItem::Separator);

    help_menu = help_menu.add_item(
        CustomMenuItem::new("about".to_string(), "About")
        .accelerator("CmdOrCtrl+I"),
    );

    help_menu = help_menu.add_item(
        CustomMenuItem::new("quit".to_string(), "Quit")
        .accelerator("CmdOrCtrl+Q"),
    );
    menu = menu.add_submenu(Submenu::new("Help", help_menu));

    menu
}

// pub fn init() -> Menu {
//     let app_name: &str = "Password Generator";
//     let mut menu = Menu::new();
//     #[cfg(target_os = "macos")]
//     {
//         menu = menu.add_submenu(Submenu::new(
//             app_name,
//             Menu::new()
//                 .add_native_item(MenuItem::About(app_name.to_string(), AboutMetadata::default()))
//                 .add_native_item(MenuItem::Separator)
//                 .add_native_item(MenuItem::Services)
//                 .add_native_item(MenuItem::Separator)
//                 .add_native_item(MenuItem::Hide)
//                 .add_native_item(MenuItem::HideOthers)
//                 .add_native_item(MenuItem::ShowAll)
//                 .add_native_item(MenuItem::Separator)
//                 .add_native_item(MenuItem::Quit),
//         ));
//     }

//     let mut file_menu = Menu::new();
//     file_menu = file_menu.add_native_item(MenuItem::About(app_name.to_string(), AboutMetadata::default()));
//     file_menu = file_menu.add_native_item(MenuItem::Separator);
//     file_menu = file_menu.add_native_item(MenuItem::Services);
//     file_menu = file_menu.add_native_item(MenuItem::Separator);

//     file_menu = file_menu.add_native_item(MenuItem::CloseWindow);
//     #[cfg(not(target_os = "macos"))]
//     {
//         file_menu = file_menu.add_native_item(MenuItem::Quit);
//     }
//     menu = menu.add_submenu(Submenu::new("File", file_menu));

//     #[cfg(not(target_os = "linux"))]
//     let mut edit_menu = Menu::new();
//     #[cfg(target_os = "macos")]
//     {
//         edit_menu = edit_menu.add_native_item(MenuItem::Undo);
//         edit_menu = edit_menu.add_native_item(MenuItem::Redo);
//         edit_menu = edit_menu.add_native_item(MenuItem::Separator);
//     }
//     #[cfg(not(target_os = "linux"))]
//     {
//         edit_menu = edit_menu.add_native_item(MenuItem::Cut);
//         edit_menu = edit_menu.add_native_item(MenuItem::Copy);
//         edit_menu = edit_menu.add_native_item(MenuItem::Paste);
//     }
//     #[cfg(target_os = "macos")]
//     {
//         edit_menu = edit_menu.add_native_item(MenuItem::SelectAll);
//     }
//     #[cfg(not(target_os = "linux"))]
//     {
//         menu = menu.add_submenu(Submenu::new("Edit", edit_menu));
//     }
//     #[cfg(target_os = "macos")]
//     {
//         menu = menu.add_submenu(Submenu::new(
//             "View",
//             Menu::new().add_native_item(MenuItem::EnterFullScreen),
//         ));
//     }

//     let mut window_menu = Menu::new();
//     window_menu = window_menu.add_native_item(MenuItem::Minimize);
//     #[cfg(target_os = "macos")]
//     {
//         window_menu = window_menu.add_native_item(MenuItem::Zoom);
//         window_menu = window_menu.add_native_item(MenuItem::Separator);
//     }
//     window_menu = window_menu.add_native_item(MenuItem::CloseWindow);
//     menu = menu.add_submenu(Submenu::new("Window", window_menu));

//     menu
// }
