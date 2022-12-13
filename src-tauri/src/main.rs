#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

pub use convert_case::{Case, Casing};
pub use rand::{seq::SliceRandom, thread_rng, Rng};
pub use std::{thread, time::Duration};
pub use tauri::api::dialog;
pub use tauri::Manager;
pub use time::OffsetDateTime;

/// Import the core module.
mod core;

/// Use the core modules.
use crate::core::*;

/// GeneratedPassword stores a randomly generated password
/// and the bcrypt hash of the password.
#[derive(serde::Serialize)]
struct GeneratedPassword {
    password: String,
    hash: String,
}

/// Generates a random password from the word list, made up of `len`
/// words, joined together by `separator`.
#[tauri::command]
fn generate_password(len: u8, separator: &str) -> Result<GeneratedPassword, String> {
    // Setup a random number generator
    let mut rng = thread_rng();

    // Generate a random number between 0 and 99.
    let mut nb: i32 = rng.gen_range(0..=999);

    // Create a new vector to store the words in.
    let mut words: Vec<String> = Vec::new();

    // Convert the special characters to a vector of chars.
    let ascii: Vec<char> = SPECIAL.iter().map(|&c| c as char).collect();

    // Generate `len` random words from the word list.
    for _ in 0..len {
        // Choose a random word from the list.
        let word = if let Some(w) = crate::WORD_LIST.choose(&mut rng) {
            // If a word was found, return it.
            w
        } else {
            return Err("No words found".to_string());
        };

        // Convert the word to title case and add a number to the end
        let word = format!("{}{}{}", word.to_case(Case::Title), ascii.choose(&mut rng).unwrap(), nb);
        // Generate a new random number between 0 and 99.
        nb = rng.gen_range(0..99);
        // Add the word to the vector of words.
        words.push(word);
    }

    // Join the words together with the separator
    let pass = words.join(separator);

    // Hash the password
    let hash = bcrypt::hash(pass.as_bytes(), HASH_COST).map_err(|_| "Failed to hash password".to_string())?;

    // Return the password and hash
    Ok(GeneratedPassword { password: pass, hash })
}

/// The main function
/// This is the entry point for the application
/// It also sets up the commands that can be called from the webview
/// and the system tray
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_password, logger, website])
        .system_tray(crate::system_tray())
        .on_system_tray_event(|app, event| {
            if let tauri::SystemTrayEvent::MenuItemClick { id, .. } = event {
                // Get the item handle from the id of the item clicked on the system tray menu item list.
                let item_handle = app.tray_handle().get_item(&id);

                let title = format!("{}", env!("CARGO_PKG_NAME").to_case(Case::Title));
                let year = format!("{}", OffsetDateTime::now_utc().year());
                let copyright = format!("© {} {}. All rights reserved.", year, title);
                let description = format!(env!("CARGO_PKG_DESCRIPTION"));
                let version = format!("Version {}", env!("CARGO_PKG_VERSION"));
                let window = app.get_window("main").unwrap();

                // Match the id of the item clicked.
                match id.as_str() {
                    "about" => {
                        crate::logger(
                            "Info",
                            "SystemTrayEvent",
                            "Opening about dialog",
                        );

                        dialog::message(
                            Some(&window),
                            title,
                            description + "\n\n" + version.as_str() + "\n\n" + copyright.as_str()
                        );
                    }
                    "hide" => {
                        // If the id is "hide", hide the window.
                        let window = app.get_window("main").unwrap();
                        if window.is_visible().unwrap() {
                            window.hide().unwrap();
                            item_handle.set_title("Show Password Generator Pro").unwrap();
                            crate::logger(
                                "Info",
                                "SystemTrayEvent",
                                "Hiding main window"
                            );
                        } else {
                            // If the window is already hidden, show it.
                            window.show().unwrap();
                            item_handle.set_title("Hide Password Generator Pro").unwrap();
                            crate::logger(
                                "Info",
                                "SystemTrayEvent",
                                "Showing main window"
                            );
                        }
                    }
                    "website" => {
                        // If the id is "website", open the website in the default browser.
                        crate::logger(
                            "Info",
                            "SystemTrayEvent",
                            "Opening website in default browser"
                        );
                        crate::website("https://password-generator.pro");
                    }
                    // If the id is "quit", quit the application.
                    "quit" => {
                        crate::logger(
                            "Info",
                            "SystemTrayEvent",
                            "Quitting application"
                        );
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        })
        .menu(crate::create_menu())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
