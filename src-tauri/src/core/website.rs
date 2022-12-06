// SPDX-License-Identifier: MIT OR Apache-2.0

#[tauri::command]

/// Open the website in the default browser.
pub fn website(url: &str) {
    webbrowser::open(url).unwrap();
}
