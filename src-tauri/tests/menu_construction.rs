// Copyright © 2022-2026 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Menu and tray construction, against Tauri's mock runtime.
//!
//! Under Tauri 1 these builders returned free values that could only be
//! observed by launching a real window, so nothing asserted that the
//! menu contained the items the dispatcher handles. A menu entry wired
//! to nothing is silent in production: the user clicks and the
//! application does nothing.
//!
//! ## Why this test has no harness
//!
//! `muda`, the menu backend Tauri 2 uses, refuses to build a menu off
//! the main thread on macOS:
//!
//! ```text
//! `muda::Menu` can only be created on the main thread
//! ```
//!
//! Rust's default test harness runs every `#[test]` on a spawned
//! thread, so these assertions panic there — `--test-threads=1` does not
//! help, because that still is not the *main* thread. Declaring
//! `harness = false` makes this file's `main` the test binary, which the
//! runner executes on the main thread.

use tauri::menu::MenuItemKind;
use tauri::test::{mock_builder, mock_context, noop_assets};
use tauri::App;

/// The mock application these checks run against.
type MockApp = App<tauri::test::MockRuntime>;

/// A named assertion over a built application.
type Check = (&'static str, fn(&MockApp));

/// Build a headless application to construct menus against.
fn app() -> MockApp {
    mock_builder()
        .build(mock_context(noop_assets()))
        .expect("mock app should build")
}

/// Collect the ids of every item in a menu, recursing into submenus.
fn collect_ids<R: tauri::Runtime>(items: &[MenuItemKind<R>], out: &mut Vec<String>) {
    for item in items {
        match item {
            MenuItemKind::MenuItem(i) => out.push(i.id().0.clone()),
            MenuItemKind::Submenu(s) => {
                if let Ok(children) = s.items() {
                    collect_ids(&children, out);
                }
            },
            _ => {},
        }
    }
}

fn tray_menu_exposes_every_expected_id(app: &MockApp) {
    let menu = password_generator_pro::core::tray::build_tray_menu(app.handle())
        .expect("tray menu should build");
    let mut ids = Vec::new();
    collect_ids(&menu.items().expect("menu items readable"), &mut ids);

    for expected in password_generator_pro::core::ids::TRAY_ITEM_IDS {
        assert!(
            ids.iter().any(|id| id == expected),
            "tray menu is missing {expected}; built ids: {ids:?}"
        );
    }
}

fn application_menu_exposes_every_expected_id(app: &MockApp) {
    let menu = password_generator_pro::core::menu::create_menu(app.handle())
        .expect("application menu should build");
    let mut ids = Vec::new();
    collect_ids(&menu.items().expect("menu items readable"), &mut ids);

    for expected in password_generator_pro::core::ids::MENU_ITEM_IDS {
        assert!(
            ids.iter().any(|id| id == expected),
            "application menu is missing {expected}; built ids: {ids:?}"
        );
    }
}

fn every_emitted_id_has_an_action(app: &MockApp) {
    let tray = password_generator_pro::core::tray::build_tray_menu(app.handle()).unwrap();
    let menu = password_generator_pro::core::menu::create_menu(app.handle()).unwrap();

    let mut ids = Vec::new();
    collect_ids(&tray.items().unwrap(), &mut ids);
    collect_ids(&menu.items().unwrap(), &mut ids);

    assert!(!ids.is_empty(), "no ids collected; the menu walk is broken");
    for id in &ids {
        assert!(
            password_generator_pro::core::action::action_for(id).is_some(),
            "menu emits {id} but the dispatcher has no action for it — \
             clicking it would silently do nothing"
        );
    }
}

fn menus_can_be_rebuilt(app: &MockApp) {
    let _first = password_generator_pro::core::tray::build_tray_menu(app.handle()).unwrap();
    let second = password_generator_pro::core::tray::build_tray_menu(app.handle());
    assert!(
        second.is_ok(),
        "rebuilding the tray menu failed: {:?}",
        second.err()
    );
}

fn submenus_build_independently(app: &MockApp) {
    let help = password_generator_pro::core::menu::create_help_menu(app.handle(), "Test App")
        .expect("help submenu should build");
    let mut ids = Vec::new();
    collect_ids(&help.items().unwrap(), &mut ids);
    for expected in ["website", "documentation", "release-notes", "quit"] {
        assert!(
            ids.iter().any(|id| id == expected),
            "help menu missing {expected}"
        );
    }

    password_generator_pro::core::menu::create_file_menu(app.handle())
        .expect("file submenu should build");
    password_generator_pro::core::menu::create_window_menu(app.handle())
        .expect("window submenu should build");
}

/// Dispatching an id nothing handles must be a silent no-op.
///
/// The old code matched on strings inline with no fallback arm; a
/// renamed id fell through to whatever the last arm happened to be.
fn unknown_ids_are_ignored(app: &MockApp) {
    for id in ["", "not-a-menu-id", "QUIT", "quit ", "\u{1f600}"] {
        password_generator_pro::perform(app.handle(), id);
    }
}

/// Toggling with no window present must not panic.
///
/// The mock app has no "main" webview window, which is exactly the
/// state the real app is in if the window was destroyed while the tray
/// icon lives on — the case that used to `unwrap`.
fn toggle_without_a_window_does_not_panic(app: &MockApp) {
    password_generator_pro::perform(app.handle(), password_generator_pro::core::ids::HIDE);
}

fn main() {
    let app = app();

    let checks: [Check; 7] = [
        (
            "tray_menu_exposes_every_expected_id",
            tray_menu_exposes_every_expected_id,
        ),
        (
            "application_menu_exposes_every_expected_id",
            application_menu_exposes_every_expected_id,
        ),
        (
            "every_emitted_id_has_an_action",
            every_emitted_id_has_an_action,
        ),
        ("menus_can_be_rebuilt", menus_can_be_rebuilt),
        ("submenus_build_independently", submenus_build_independently),
        ("unknown_ids_are_ignored", unknown_ids_are_ignored),
        (
            "toggle_without_a_window_does_not_panic",
            toggle_without_a_window_does_not_panic,
        ),
    ];

    for (name, check) in checks {
        check(&app);
        println!("test {name} ... ok");
    }

    println!(
        "\ntest result: ok. {} passed; 0 failed (menu construction, main thread)",
        checks.len()
    );
}
