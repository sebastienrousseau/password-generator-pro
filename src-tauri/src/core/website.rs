// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#[tauri::command]

/// Open the website in the default browser.
pub fn website(url: &str) {
    webbrowser::open(url).unwrap();
}
