// Copyright © 2022-2026 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Menu identifiers, defined once.
//!
//! The tray menu and the application menu share four entries — About,
//! Get Started, Documentation and Quit — and both feed the same
//! dispatcher. Declaring the identifiers in each module meant two
//! definitions of the same string constant, which clippy flags as an
//! ambiguous glob re-export and which would silently diverge the first
//! time one was edited.

/// Show the about dialog.
pub const ABOUT: &str = "about";
/// Open the product website.
pub const WEBSITE: &str = "website";
/// Open the documentation.
pub const DOCUMENTATION: &str = "documentation";
/// Open the release notes.
pub const RELEASE_NOTES: &str = "release-notes";
/// Open the issue tracker.
pub const REPORT_ISSUE: &str = "report-issue";
/// Open the licence.
pub const LICENSE: &str = "license";
/// Open the acknowledgements.
pub const ACKNOWLEDGEMENTS: &str = "acknowledgements";
/// Toggle main-window visibility.
pub const HIDE: &str = "hide";
/// Copy a freshly generated password.
pub const QUICK_PASSWORD: &str = "quick_password";
/// Copy a freshly generated UUID.
pub const QUICK_UUID: &str = "quick_uuid";
/// Save a QR code to a file.
pub const QUICK_QRCODE: &str = "quick_qrcode";
/// Terminate the application.
pub const QUIT: &str = "quit";

/// Every identifier the tray menu emits, in display order.
pub const TRAY_ITEM_IDS: [&str; 8] = [
    ABOUT,
    WEBSITE,
    DOCUMENTATION,
    QUICK_PASSWORD,
    QUICK_UUID,
    QUICK_QRCODE,
    HIDE,
    QUIT,
];

/// Every identifier the application menu emits, in display order.
pub const MENU_ITEM_IDS: [&str; 8] = [
    ABOUT,
    WEBSITE,
    DOCUMENTATION,
    RELEASE_NOTES,
    REPORT_ISSUE,
    LICENSE,
    ACKNOWLEDGEMENTS,
    QUIT,
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn tray_ids_are_unique() {
        let seen: HashSet<_> = TRAY_ITEM_IDS.iter().collect();
        assert_eq!(seen.len(), TRAY_ITEM_IDS.len(), "duplicate tray id");
    }

    #[test]
    fn menu_ids_are_unique() {
        let seen: HashSet<_> = MENU_ITEM_IDS.iter().collect();
        assert_eq!(seen.len(), MENU_ITEM_IDS.len(), "duplicate menu id");
    }

    #[test]
    fn ids_are_non_empty() {
        for id in TRAY_ITEM_IDS.iter().chain(MENU_ITEM_IDS.iter()) {
            assert!(!id.is_empty(), "empty identifier");
        }
    }

    #[test]
    fn ids_use_a_single_naming_convention() {
        for id in TRAY_ITEM_IDS.iter().chain(MENU_ITEM_IDS.iter()) {
            assert!(
                id.chars()
                    .all(|c| c.is_ascii_lowercase() || c == '_' || c == '-'),
                "identifier is not lowercase kebab/snake: {id}"
            );
        }
    }

    #[test]
    fn shared_entries_appear_in_both_menus() {
        for id in [ABOUT, WEBSITE, DOCUMENTATION, QUIT] {
            assert!(TRAY_ITEM_IDS.contains(&id), "{id} missing from the tray");
            assert!(MENU_ITEM_IDS.contains(&id), "{id} missing from the menu");
        }
    }

    #[test]
    fn tray_only_entries_are_not_in_the_app_menu() {
        for id in [QUICK_PASSWORD, QUICK_UUID, QUICK_QRCODE, HIDE] {
            assert!(TRAY_ITEM_IDS.contains(&id));
            assert!(
                !MENU_ITEM_IDS.contains(&id),
                "{id} unexpectedly appears in the application menu"
            );
        }
    }
}
