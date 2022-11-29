use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

pub(crate) fn system_tray() -> SystemTray {
    let about = CustomMenuItem::new("about".to_string(), "About");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit Password Generator   ⌘Q");
    let website: CustomMenuItem = CustomMenuItem::new("website".to_string(), "Website");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");

    let tray_menu = SystemTrayMenu::new()
        .add_item(about)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_item(website)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}
