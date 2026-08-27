//! The pieces of Password Generator Pro that work without a GUI.
//!
//! ```sh
//! cargo run --manifest-path src-tauri/Cargo.toml --example core_api
//! ```
//!
//! Everything here is side-effect free. The parts that are not -- opening
//! a browser, writing the clipboard, raising a native dialog -- are
//! deliberately left out, because an example that hijacks your browser
//! when you run it is a bad example.

use password_generator_pro::core::action::{action_for, Action};
use password_generator_pro::core::ids::{MENU_ITEM_IDS, TRAY_ITEM_IDS};
use password_generator_pro::core::website::is_openable;
use password_generator_pro::util::date::Date;
use password_generator_pro::util::logger::Logger;
use password_generator_pro::util::qrcode::QRCode;
use password_generator_pro::util::uuid::UUID;

fn main() {
    heading("Menu ids resolve to actions");
    // Every id the menus expose must map to something. An id that maps
    // to None is a menu entry wired to nothing: the user clicks and the
    // application silently does nothing.
    for id in MENU_ITEM_IDS.iter().chain(TRAY_ITEM_IDS.iter()).take(8) {
        match action_for(id) {
            Some(action) => println!("  {id:<22} -> {}", describe(&action)),
            None => println!("  {id:<22} -> (unmapped)"),
        }
    }

    heading("URL vetting");
    // `website()` refuses anything is_openable rejects, so this is the
    // check that keeps a menu item from launching an arbitrary scheme.
    for url in [
        "https://example.com",
        "http://example.com",
        "javascript:alert(1)",
        "file:///etc/passwd",
        "not a url",
    ] {
        println!(
            "  {:<24} {}",
            url,
            if is_openable(url) {
                "openable"
            } else {
                "rejected"
            }
        );
    }

    heading("Identifiers");
    println!("  uuid            {}", UUID::uuid());
    println!("  uuid (again)    {}", UUID::uuid());

    heading("QR code");
    let svg = QRCode::qrcode("https://example.com");
    println!("  {} bytes of SVG for https://example.com", svg.len());

    heading("Clock");
    println!("  now             {}", Date::now());
    println!("  year / month    {} / {}", Date::year(), Date::month());
    println!("  timezone        {}", Date::timezone());

    heading("Logging");
    // Logger writes a structured line; the value is the format, so the
    // example shows it rather than describing it.
    Logger::new(&Date::now(), "Info", "Example", "core_api example ran").log();
}

/// A short label for an action, for display only.
fn describe(action: &Action) -> &'static str {
    match action {
        Action::ShowAbout => "show the about dialog",
        Action::OpenUrl(_) => "open a URL",
        Action::CopyPassword => "copy a generated password",
        Action::CopyUuid => "copy a UUID",
        Action::SaveQrCode => "save a QR code",
        Action::ToggleWindow => "show or hide the window",
        Action::Quit => "quit",
    }
}

fn heading(title: &str) {
    println!("\n== {title}");
}
