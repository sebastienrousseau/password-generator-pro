// Copyright © 2022-2026 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Menu-event dispatch, as data.
//!
//! In Tauri 1 the tray and menu handlers were two ~60-line closures that
//! matched on a string id and performed effects inline. Nothing in them
//! could be tested without a running application and a real window.
//!
//! The mapping from identifier to intent is pure, so it lives here as a
//! total function returning an [`Action`]. The Tauri handlers become a
//! thin `match` that performs the effect. That keeps the interesting
//! part — which id means what, and what happens to an unknown id —
//! under unit test.

use crate::util::constant::{
    ACKNOWLEDGEMENTS, DOCUMENTATION, HOMEPAGE, ISSUE, LICENSE_URL, RELEASE,
};

/// What a menu or tray activation should do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Show the about dialog.
    ShowAbout,
    /// Open a URL in the user's browser.
    OpenUrl(&'static str),
    /// Copy a freshly generated password to the clipboard.
    CopyPassword,
    /// Copy a freshly generated UUID to the clipboard.
    CopyUuid,
    /// Prompt for a path and save a QR code there.
    SaveQrCode,
    /// Toggle main-window visibility.
    ToggleWindow,
    /// Terminate the application.
    Quit,
}

/// Map a menu identifier to the action it performs.
///
/// Returns `None` for an unrecognised identifier, which callers treat as
/// a no-op — the same behaviour as the `_ => {}` arm this replaces.
#[must_use]
pub fn action_for(id: &str) -> Option<Action> {
    Some(match id {
        crate::core::ids::ABOUT => Action::ShowAbout,
        crate::core::ids::WEBSITE => Action::OpenUrl(HOMEPAGE),
        crate::core::ids::DOCUMENTATION => Action::OpenUrl(DOCUMENTATION),
        crate::core::ids::RELEASE_NOTES => Action::OpenUrl(RELEASE),
        crate::core::ids::REPORT_ISSUE => Action::OpenUrl(ISSUE),
        crate::core::ids::LICENSE => Action::OpenUrl(LICENSE_URL),
        crate::core::ids::ACKNOWLEDGEMENTS => Action::OpenUrl(ACKNOWLEDGEMENTS),
        crate::core::ids::QUICK_PASSWORD => Action::CopyPassword,
        crate::core::ids::QUICK_UUID => Action::CopyUuid,
        crate::core::ids::QUICK_QRCODE => Action::SaveQrCode,
        crate::core::ids::HIDE => Action::ToggleWindow,
        crate::core::ids::QUIT => Action::Quit,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ids::{MENU_ITEM_IDS, TRAY_ITEM_IDS};

    #[test]
    fn every_tray_id_maps_to_an_action() {
        for id in TRAY_ITEM_IDS {
            assert!(
                action_for(id).is_some(),
                "tray id {id} has no action — the menu can emit an event nothing handles"
            );
        }
    }

    #[test]
    fn every_menu_id_maps_to_an_action() {
        for id in MENU_ITEM_IDS {
            assert!(
                action_for(id).is_some(),
                "menu id {id} has no action — the menu can emit an event nothing handles"
            );
        }
    }

    #[test]
    fn unknown_ids_are_a_no_op() {
        for id in ["", "nope", "ABOUT", "quit ", " quit", "Quit"] {
            assert_eq!(action_for(id), None, "unexpected action for {id:?}");
        }
    }

    #[test]
    fn about_maps_to_show_about() {
        assert_eq!(action_for("about"), Some(Action::ShowAbout));
    }

    #[test]
    fn quit_maps_to_quit() {
        assert_eq!(action_for("quit"), Some(Action::Quit));
    }

    #[test]
    fn hide_toggles_rather_than_hiding() {
        // The id is historical; the behaviour is a toggle. Asserted so a
        // future reader does not "fix" the handler to only hide.
        assert_eq!(action_for("hide"), Some(Action::ToggleWindow));
    }

    #[test]
    fn url_actions_carry_distinct_destinations() {
        let urls: Vec<&str> = [
            "website",
            "documentation",
            "release-notes",
            "report-issue",
            "license",
            "acknowledgements",
        ]
        .iter()
        .map(|id| match action_for(id) {
            Some(Action::OpenUrl(u)) => u,
            other => panic!("{id} did not map to OpenUrl, got {other:?}"),
        })
        .collect();

        let unique: std::collections::HashSet<_> = urls.iter().collect();
        assert_eq!(
            unique.len(),
            urls.len(),
            "two menu entries open the same URL: {urls:?}"
        );
    }

    #[test]
    fn url_actions_are_https() {
        for id in ["website", "documentation", "release-notes", "report-issue"] {
            if let Some(Action::OpenUrl(u)) = action_for(id) {
                assert!(u.starts_with("https://"), "{id} opens a non-https URL: {u}");
            }
        }
    }

    #[test]
    fn quick_actions_are_distinct() {
        assert_eq!(action_for("quick_password"), Some(Action::CopyPassword));
        assert_eq!(action_for("quick_uuid"), Some(Action::CopyUuid));
        assert_eq!(action_for("quick_qrcode"), Some(Action::SaveQrCode));
    }
}
