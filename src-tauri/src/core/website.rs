// Copyright © 2022-2023 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/// Open the website in the default browser.
#[tauri::command]
pub fn website(url: &str) {
    webbrowser::open(url).unwrap();
}
