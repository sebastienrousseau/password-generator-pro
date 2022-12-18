#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use convert_case::{Case, Casing};
use crate::core::*;
use rand::{seq::SliceRandom, thread_rng, Rng};
use tauri::api::dialog;
use tauri::Manager;

mod core;

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
        let word = format!(
            "{}{}{}",
            word.to_case(Case::Title),
            ascii.choose(&mut rng).unwrap(),
            nb
        );
        // Generate a new random number between 0 and 99.
        nb = rng.gen_range(0..99);
        // Add the word to the vector of words.
        words.push(word);
    }

    // Join the words together with the separator
    let pass = words.join(separator);

    // Hash the password
    let hash = bcrypt::hash(pass.as_bytes(), HASH_COST)
        .map_err(|_| "Failed to hash password".to_string())?;

    // Return the password and hash
    Ok(GeneratedPassword {
        password: pass,
        hash,
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

                let name = NAME.to_case(Case::Title);
                let year = format!("{}", OffsetDateTime::now_utc().year());
                let copyright = format!("© {} {}\nAll rights reserved.", year, name);
                let description = DESCRIPTION.to_string();
                let sha_short = SHA.split_at(7).0.to_string();
                let version = format!("Version {} ({})", VERSION, sha_short);
                let window = app.get_window("main").unwrap();

                // Match the id of the item clicked.
                match id.as_str() {
                    "about" => {
                        let logger = Logger::new(&get_time(), "Info", "SystemTrayEvent", "Opening about dialog");
                        logger.log();

                        dialog::message(
                            Some(&window),
                            name,
                            format!("{}\n\n {}\n\n {}\n", description, version, copyright),
                        );
                    }
                    "hide" => {
                        // If the id is "hide", hide the window.
                        let window = app.get_window("main").unwrap();
                        if window.is_visible().unwrap() {
                            window.hide().unwrap();
                            item_handle
                                .set_title("Show Password Generator Pro")
                                .unwrap();
                            let logger = Logger::new(&get_time(), "Info", "SystemTrayEvent", "Hiding main window");
                            logger.log();
                        } else {
                            // If the window is already hidden, show it.
                            window.show().unwrap();
                            item_handle
                                .set_title("Hide Password Generator Pro")
                                .unwrap();
                                let logger = Logger::new(&get_time(), "Info", "SystemTrayEvent", "Showing main window");
                                logger.log();
                        }
                    }
                    "documentation" => {
                        // If the id is "website", open the website in the default browser.
                        let logger = Logger::new(&get_time(), "Info",
                            "SystemTrayEvent",
                            "Opening website in default browser",
                        );
                        logger.log();
                        crate::website(DOCUMENTATION);
                    }
                    "website" => {
                        // If the id is "website", open the website in the default browser.
                        let logger = Logger::new(&get_time(), "Info",
                            "SystemTrayEvent",
                            "Opening website in default browser",
                        );
                        logger.log();
                        crate::website(HOMEPAGE);
                    }
                    // If the id is "quit", quit the application.
                    "quit" => {
                        let logger = Logger::new(&get_time(), "Info", "SystemTrayEvent", "Quitting application");
                        logger.log();
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        })
        .menu(crate::create_menu())
        .on_menu_event(|event| match event.menu_item_id() {
            "about" => {
                let logger = Logger::new(&get_time(), "Info", "MenuEvent", "Opening about dialog");
                logger.log();
                let name = NAME.to_case(Case::Title);
                let year = format!("{}", OffsetDateTime::now_utc().year());
                let copyright = format!("© {} {}\nAll rights reserved.", year, name);
                let description = DESCRIPTION.to_string();
                let sha_short = SHA.split_at(7).0.to_string();
                let uuids = crate::core::generate_uuid();
                let version = format!("Version {} ({})", VERSION, sha_short);
                let window = event.window();

                dialog::message(
                    Some(window),
                    name,
                    format!("{}\n\n {}\n\n {}\n{}", description, version, copyright, uuids),
                );
            }
            "acknowledgements" => {
                let logger = Logger::new(&get_time(), "Info", "MenuEvent", "Opening Acknowledgements dialog");
                logger.log();
                crate::website(ACKNOWLEDGEMENTS);
            }
            "documentation" => {
                let logger = Logger::new(&get_time(), "Info", "MenuEvent", "Opening Documentation in default browser");
                logger.log();
                crate::website(DOCUMENTATION);
            }
            "license" => {
                let logger = Logger::new(&get_time(), "Info", "MenuEvent", "Opening License in default browser");
                logger.log();
                crate::website(LICENSE_URL);
            }
            "quit" => {
                let logger = Logger::new(&get_time(), "Info", "SystemTrayEvent", "Quitting application");
                logger.log();
                std::process::exit(0);
            }
            "release-notes" => {
                let logger = Logger::new(&get_time(), "Info", "MenuEvent", "Opening Release notes in default browser");
                logger.log();
                crate::website(RELEASE);
            }
            "report-issue" => {
                let logger = Logger::new(&get_time(), "Info", "MenuEvent", "Opening Issue tracker in default browser");
                logger.log();
                crate::website(ISSUE);
            }
            "website" => {
                let logger = Logger::new(&get_time(), "Info", "MenuEvent", "Opening Website in default browser");
                logger.log();
                crate::website(HOMEPAGE);
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
