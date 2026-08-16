// Copyright © 2022-2026 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Password Generator Pro — Tauri backend.
//!
//! Migrated from Tauri 1 to Tauri 2. The notable differences:
//!
//! * dialogs moved from `tauri::api::dialog` into the
//!   `tauri-plugin-dialog` plugin, registered on the builder;
//! * `SystemTray` became [`tauri::tray::TrayIconBuilder`], built inside
//!   `setup` because it needs an app handle;
//! * `Window` became `WebviewWindow`, reached with
//!   `get_webview_window`;
//! * the v1 `allowlist` became `capabilities/default.json`.
//!
//! Menu dispatch itself is not here: it is a pure function in
//! [`core::action`], so the mapping from identifier to intent can be
//! unit tested without a running application.

extern crate cmn;
extern crate psph;

pub use crate::core::*;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use cmn::Constants;
use convert_case::{Case, Casing};
use psph::Password;
use std::fs;
use tauri::menu::MenuEvent;
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
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

/// A generated password together with its bcrypt hash and a UUID.
#[derive(serde::Serialize)]
struct PasswordGenerator {
    /// The generated password.
    password: String,
    /// The bcrypt hash of the password.
    hash: String,
    /// A random UUID issued alongside the password.
    uuid: String,
}

/// Generate a random password of `len` words joined by `separator`.
///
/// # Errors
///
/// Returns an error string if the password cannot be hashed.
#[tauri::command]
fn generate_password(len: u8, separator: &str) -> Result<PasswordGenerator, String> {
    let new_constant = Constants::new();
    let constants = new_constant.constants();
    let special_chars: Vec<char> = constants
        .iter()
        .find(|&c| c.name == "SPECIAL_CHARS")
        .ok_or_else(|| "SPECIAL_CHARS constant is missing".to_string())?
        .value
        .chars()
        .collect();

    let password = Password::new(len, separator, special_chars);
    let pass = password.to_string();

    let hash = bcrypt::hash(pass.as_bytes(), HASH_COST)
        .map_err(|_| "Failed to hash password".to_string())?;

    Ok(PasswordGenerator {
        password: pass,
        hash,
        uuid: UUID::uuid(),
    })
}

/// The text shown in the about dialog.
///
/// Pure, so its formatting is unit tested rather than eyeballed.
#[must_use]
pub fn about_text() -> String {
    let year = OffsetDateTime::now_utc().year();
    let name = NAME.to_case(Case::Title);
    format!(
        "{}\n\n Version {} ()\n\n © {} {}\nAll rights reserved.\n",
        DESCRIPTION, VERSION, year, name
    )
}

/// Perform the effect for a menu or tray activation.
///
/// Public so integration tests can dispatch an id without launching a
/// window. Only the branches with no OS side effect are reachable from
/// a test: an unrecognised id, and `ToggleWindow` when no window
/// exists. The rest open a browser, write the system clipboard, raise
/// a native dialog or exit the process, so the *decision* they hang
/// off — [`core::action::action_for`] — is what carries the test
/// coverage instead.
pub fn perform<R: Runtime>(app: &AppHandle<R>, id: &str) {
    let Some(action) = crate::core::action::action_for(id) else {
        return;
    };

    let utc = Date::now();
    Logger::new(&utc, "Info", "MenuEvent", id).log();

    match action {
        Action::ShowAbout => {
            let name = NAME.to_case(Case::Title);
            app.dialog()
                .message(about_text())
                .title(name)
                .buttons(MessageDialogButtons::Ok)
                .show(|_| {});
        },
        Action::OpenUrl(url) => {
            // A failed open is logged by the helper's return value; it
            // is not worth interrupting the user over.
            let _ = crate::website(url);
        },
        Action::CopyPassword => {
            if let Ok(generated) = generate_password(4, "-") {
                if let Ok(mut ctx) = ClipboardContext::new() {
                    let _ = ctx.set_contents(generated.password);
                }
            }
        },
        Action::CopyUuid => {
            if let Ok(mut ctx) = ClipboardContext::new() {
                let _ = ctx.set_contents(UUID::uuid());
            }
        },
        Action::SaveQrCode => {
            app.dialog()
                .file()
                .add_filter("SVG", &["svg"])
                .save_file(move |path| {
                    let Some(path) = path else { return };
                    let Ok(generated) = generate_password(4, "-") else {
                        return;
                    };
                    let qrcode = QRCode::qrcode(&generated.password);
                    if let Ok(path) = path.into_path() {
                        let _ = fs::write(path, qrcode);
                    }
                });
        },
        Action::ToggleWindow => {
            if let Some(window) = app.get_webview_window("main") {
                match window.is_visible() {
                    Ok(true) => {
                        let _ = window.hide();
                    },
                    _ => {
                        let _ = window.show();
                    },
                }
            }
        },
        Action::Quit => app.exit(0),
    }
}

/// Build and run the application.
///
/// Lives in the library rather than the binary so integration tests can
/// link against the menu, tray and dispatch code. A binary crate cannot
/// be imported by `tests/`, which is why none of it was covered before.
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![generate_password, website])
        .setup(|app| {
            let tray_menu = crate::core::tray::build_tray_menu(app)?;
            TrayIconBuilder::with_id("main")
                .menu(&tray_menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event: MenuEvent| perform(app, event.id().as_ref()))
                .build(app)?;

            let menu = crate::core::menu::create_menu(app)?;
            app.set_menu(menu)?;

            Ok(())
        })
        .on_menu_event(|app, event| perform(app.app_handle(), event.id().as_ref()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_password_returns_a_password_hash_and_uuid() {
        let generated = generate_password(4, "-").expect("generation should succeed");
        assert!(!generated.password.is_empty());
        assert!(!generated.hash.is_empty());
        assert!(!generated.uuid.is_empty());
    }

    #[test]
    fn generated_hash_verifies_against_the_password() {
        let generated = generate_password(4, "-").expect("generation should succeed");
        assert!(
            bcrypt::verify(generated.password.as_bytes(), &generated.hash)
                .expect("hash should be well-formed"),
            "the returned hash does not verify against the returned password"
        );
    }

    #[test]
    fn separator_appears_between_words() {
        let generated = generate_password(4, "-").expect("generation should succeed");
        assert!(
            generated.password.contains('-'),
            "expected the separator in {:?}",
            generated.password
        );
    }

    #[test]
    fn successive_passwords_differ() {
        let a = generate_password(4, "-").expect("generation should succeed");
        let b = generate_password(4, "-").expect("generation should succeed");
        assert_ne!(
            a.password, b.password,
            "two consecutive passwords were identical"
        );
        assert_ne!(a.uuid, b.uuid, "two consecutive UUIDs were identical");
    }

    #[test]
    fn length_one_is_accepted() {
        let generated = generate_password(1, "-").expect("generation should succeed");
        assert!(!generated.password.is_empty());
    }

    #[test]
    fn an_empty_separator_is_accepted() {
        let generated = generate_password(3, "").expect("generation should succeed");
        assert!(!generated.password.is_empty());
    }

    #[test]
    fn about_text_carries_version_description_and_year() {
        let text = about_text();
        assert!(text.contains(VERSION), "about text omits the version");
        assert!(
            text.contains(DESCRIPTION),
            "about text omits the description"
        );
        let year = OffsetDateTime::now_utc().year().to_string();
        assert!(text.contains(&year), "about text omits the current year");
        assert!(
            text.contains("All rights reserved."),
            "about text omits the copyright line"
        );
    }

    #[test]
    fn about_text_is_title_cased_and_not_kebab() {
        let text = about_text();
        assert!(
            !text.contains("password-generator-pro"),
            "about text leaked the crate name: {text:?}"
        );
    }
}
