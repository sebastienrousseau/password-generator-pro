#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use convert_case::{Case, Casing};
use crate::core::*;
use date::Time;
use logger::Logger;
use rand::{seq::SliceRandom, thread_rng, Rng};
use tauri::api::dialog;
use tauri::Manager;


pub mod core;
pub mod logger;
pub mod date;



/// PasswordGenerator stores a randomly generated password
/// and the bcrypt hash of the password.
#[derive(serde::Serialize)]
struct PasswordGenerator {
    password: String,
    hash: String,
    uuid: String,
}

/// Generates a random password from the word list, made up of `len`
/// words, joined together by `separator`.
#[tauri::command]
fn generate_password(len: u8, separator: &str) -> Result<PasswordGenerator, String> {
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

    let uuid = crate::core::generate_uuid();

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
                let utc = Time::now();
                let logger = Logger::new(&utc, "Info", "SystemTrayEvent", id.as_str());
                let name = NAME.to_case(Case::Title);
                let year = format!("{}", OffsetDateTime::now_utc().year());
                let copyright = format!("© {} {}\nAll rights reserved.", year, name);
                let description = DESCRIPTION.to_string();
                let sha_short = SHA.split_at(7).0.to_string();
                let version = format!("Version {} ({})", VERSION, sha_short);
                let window = app.get_window("main").unwrap();

                // Match the id of the item clicked.
                match id.as_str() {
                    "about" => { logger.log(); dialog::message(
                            Some(&window),
                            name,
                            format!("{}\n\n {}\n\n {}\n", description, version, copyright),
                        );
                    }
                    "documentation" => {logger.log(); crate::website(DOCUMENTATION);}
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
                    "quit" => { logger.log(); std::process::exit(0);}
                    "website" => {logger.log(); crate::website(HOMEPAGE);}
                    _ => {}
                }
            }
        })
        .menu(crate::create_menu())
        .on_menu_event(|event| {
            let utc = Time::year();
            let logger = Logger::new(&utc, "Info", "MenuEvent",event.menu_item_id());
            let name = NAME.to_case(Case::Title);
            let year = format!("{}", OffsetDateTime::now_utc().year());
            let copyright = format!("© {} {}\nAll rights reserved.", year, name);
            let description = DESCRIPTION.to_string();
            let sha_short = SHA.split_at(7).0.to_string();
            let version = format!("Version {} ({})", VERSION, sha_short);
            let window = event.window();

            match event.menu_item_id() {
                "about" => { logger.log(); dialog::message(Some(window), name, format!("{}\n\n {}\n\n {}\n", description, version, copyright));}
                "acknowledgements" => { logger.log(); crate::website(ACKNOWLEDGEMENTS);}
                "documentation" => { logger.log(); crate::website(DOCUMENTATION);}
                "license" => { logger.log(); crate::website(LICENSE_URL);}
                "quit" => { logger.log(); std::process::exit(0);}
                "release-notes" => { logger.log(); crate::website(RELEASE);}
                "report-issue" => { logger.log(); crate::website(ISSUE);}
                "website" => { logger.log(); crate::website(HOMEPAGE);}
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
