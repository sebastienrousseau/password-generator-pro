use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

pub fn system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit Password Generator   ⌘Q");
    let website: CustomMenuItem = CustomMenuItem::new("website".to_string(), "What's New");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide Password Generator   ⌘H");

    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("about", "About Password Generator"))
        .add_item(website)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}
