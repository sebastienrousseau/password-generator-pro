#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

/// Import the convert_case crate to convert the string to the desired case.
pub use convert_case::{Case, Casing};

/// Import the random generator and the integrator random extension traits.
pub use rand::{seq::SliceRandom, thread_rng, Rng};

/// Import the tauri manager.
pub use tauri::Manager;

/// Import the chrono crate to get the current date and time.
pub use chrono::{DateTime, Utc};

/// Import the webbrowser crate to open the website in the default browser.
pub use webbrowser;

pub use consts::*;
mod consts;
mod tray;
mod words;

pub use sys_locale::get_locale;

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

  let ascii: Vec<char> = SPECIAL.iter().map(|&c| c as char).collect();
  // Generate `len` random words from the word list.
  for _ in 0..len {
    // Choose a random word from the list.
    let word = if let Some(w) = words::WORD_LIST.choose(&mut rng) {
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
    .or_else(|_| Err("Failed to hash password".to_string()))?;

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
fn main() {
  let locale = get_locale().unwrap_or_else(|| String::from("en-GB"));
  println!("The current locale is {}", locale);
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![generate_password,])
    .system_tray(tray::system_tray())
    .on_system_tray_event(|app, event| match event {
      tauri::SystemTrayEvent::MenuItemClick { id, .. } => {
        // Get the item handle from the id of the item clicked on the system tray menu item list.
        let item_handle = app.tray_handle().get_item(&id);

        // Get the current time and date.
        let now: DateTime<Utc> = Utc::now();

        // Match the id of the item clicked.
        match id.as_str() {
          "hide" => {
            // If the id is "hide", hide the window.
            let window = app.get_window("main").unwrap();
            if window.is_visible().unwrap() {
              window.hide().unwrap();
              item_handle.set_title("Show").unwrap();
              println!("{} [Info] SystemTrayEvent: hiding main window", now);
            } else {
              // If the window is already hidden, show it.
              window.show().unwrap();
              item_handle.set_title("Hide").unwrap();
              println!("{} [Info] SystemTrayEvent: showing main window", now);
            }
          }
          "website" => {
            // If the id is "website", open the website in the default browser.
            println!(
              "{} [Info] SystemTrayEvent: opening password-generator.pro website",
              now
            );
            webbrowser::open("https://password-generator.pro").unwrap();
          }
          // If the id is "quit", quit the application.
          "quit" => {
            println!("{} [Info] SystemTrayEvent: quitting application", now);
            std::process::exit(0);
          }
          _ => {}
        }
      }
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
