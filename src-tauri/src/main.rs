#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate cmn;
extern crate psph;
pub use crate::core::*;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use cmn::Constants;
use convert_case::{Case, Casing};
use psph::Password;
use serde;
use std::fs;
use tauri::api::dialog;
use tauri::Manager;
use time::OffsetDateTime;
use util::{constant::*, date::Date, logger::Logger, qrcode::QRCode, uuid::UUID};

/// The core module
pub mod core;
/// The util module
pub mod util {
    /// The constant module
    pub mod constant;
    /// The date module
    pub mod date;
    /// The logger module
    pub mod logger;
    /// The qrcode module
    pub mod qrcode;
    /// The uuid module
    pub mod uuid;
}

/// PasswordGenerator stores a randomly generated password
/// and the bcrypt hash of the password.
#[derive(serde::Serialize)]
struct PasswordGenerator {
    password: String,
    hash: String,
    uuid: String,
}

/// Generates a random password using a given length and separator.
#[tauri::command]
fn generate_password(len: u8, separator: &str) -> Result<PasswordGenerator, String> {
    // Initialize the constants, specifically the special characters.
    let new_constant = Constants::new();
    let constants = new_constant.constants();
    let special_chars: Vec<char> = constants
        .iter()
        .find(|&c| c.name == "SPECIAL_CHARS")
        .unwrap()
        .value
        .chars()
        .collect();

    // Generate a random password with the given length, separator, and
    // special characters.
    let password = Password::new(len, separator, special_chars);

    // Convert the password to a string
    let pass = password.to_string();

    // Hash the password
    let hash =
        bcrypt::hash(pass.as_bytes(), 8).map_err(|_| "Failed to hash password".to_string())?;

    let uuid = UUID::uuid();

    // let qrcode = QRCode::qrcode(&uuid);

    // Return the password and hash
    Ok(PasswordGenerator {
        password: pass,
        hash,
        uuid,
    })
}

/// The main function
/// This is the entry point for the application
/// It also sets up the commands that can be called from the webview
/// and the system tray
#[tauri::command]
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_password, website])
        .system_tray(crate::system_tray())
        .on_system_tray_event(|app, event| {
            if let tauri::SystemTrayEvent::MenuItemClick { id, .. } = event {
                // Get the item handle from the id of the item clicked on the system tray menu item list.
                let item_handle = app.tray_handle().get_item(&id);
                let utc = Date::now();
                let logger = Logger::new(&utc, "Info", "SystemTrayEvent", id.as_str());
                let name = NAME.to_case(Case::Title);
                let year = format!("{}", OffsetDateTime::now_utc().year());
                let copyright = format!("© {} {}\nAll rights reserved.", year, name);
                let description = DESCRIPTION.to_string();
                let sha_short = "";
                // SHA.split_at(7).0.to_string();
                let version = format!("Version {} ({})", VERSION, sha_short);
                let window = app.get_window("main").unwrap();
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

                // Match the id of the item clicked.
                match id.as_str() {
                    "about" => {
                        logger.log();
                        dialog::message(
                            Some(&window),
                            name,
                            format!("{}\n\n {}\n\n {}\n", description, version, copyright),
                        );
                    }
                    "documentation" => {
                        logger.log();
                        crate::website(DOCUMENTATION);
                    }
                    "quick_password" => {
                        logger.log();
                        ctx.set_contents(generate_password(4, "-").unwrap().password)
                            .unwrap();
                    }
                    "quick_uuid" => {
                        logger.log();
                        ctx.set_contents(UUID::uuid()).unwrap();
                    }
                    "quick_qrcode" => {
                        println!("Save as...");
                        dialog::FileDialogBuilder::default()
                            .add_filter("SVG", &["svg"])
                            .save_file(|path_buf| match path_buf {
                                Some(path_buf) => {
                                    let path = path_buf.to_str().unwrap();
                                    let qrcode = QRCode::qrcode(
                                        &generate_password(4, "-").unwrap().password,
                                    );
                                    fs::write(path, qrcode).unwrap();
                                    println!("Saved as {}", path);
                                }
                                None => {
                                    println!("No file selected");
                                }
                            });
                    }
                    // => { logger.log(); ctx.set_contents(QRCode::qrcode(&generate_password(4, "-").unwrap().password).to_string()).unwrap();}
                    "hide" => {
                        // If the id is "hide", hide the window.
                        let window = app.get_window("main").unwrap();
                        if window.is_visible().unwrap() {
                            window.hide().unwrap();
                            item_handle
                                .set_title("Show Password Generator Pro")
                                .unwrap();
                            logger.log();
                        } else {
                            // If the window is already hidden, show it.
                            window.show().unwrap();
                            item_handle
                                .set_title("Hide Password Generator Pro")
                                .unwrap();
                            logger.log();
                        }
                    }
                    "quit" => {
                        logger.log();
                        std::process::exit(0);
                    }
                    "website" => {
                        logger.log();
                        crate::website(HOMEPAGE);
                    }
                    _ => {}
                }
            }
        })
        .menu(crate::create_menu())
        .on_menu_event(|event| {
            let utc = Date::now();
            let logger = Logger::new(&utc, "Info", "MenuEvent", event.menu_item_id());
            let name = NAME.to_case(Case::Title);
            let year = format!("{}", OffsetDateTime::now_utc().year());
            let copyright = format!("© {} {}\nAll rights reserved.", year, name);
            let description = DESCRIPTION.to_string();
            let sha_short = "";
            // SHA.split_at(7).0.to_string();
            let version = format!("Version {} ({})", VERSION, sha_short);
            let window = event.window();

            match event.menu_item_id() {
                "about" => {
                    logger.log();
                    dialog::message(
                        Some(window),
                        name,
                        format!("{}\n\n {}\n\n {}\n", description, version, copyright),
                    );
                }
                "acknowledgements" => {
                    logger.log();
                    crate::website(ACKNOWLEDGEMENTS);
                }
                "documentation" => {
                    logger.log();
                    crate::website(DOCUMENTATION);
                }
                "license" => {
                    logger.log();
                    crate::website(LICENSE_URL);
                }
                "quit" => {
                    logger.log();
                    std::process::exit(0);
                }
                "release-notes" => {
                    logger.log();
                    crate::website(RELEASE);
                }
                "report-issue" => {
                    logger.log();
                    crate::website(ISSUE);
                }
                "website" => {
                    logger.log();
                    crate::website(HOMEPAGE);
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
