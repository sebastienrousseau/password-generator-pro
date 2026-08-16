# Tauri 1 → 2 migration

This document records what changed, why, and what is deliberately not
covered by tests. It is written for someone who has to modify this code
later, not as a release note.

## Why the migration touched more than the Tauri API

Tauri 1 kept the whole application in `main.rs`, a binary crate.
A binary crate cannot be imported by an integration test and does not
run doc tests, so none of the menu, tray or dispatch code was reachable
from any test — and, separately, **the repository had no workspace root
manifest**, so `cargo` invoked from the repository root failed with:

```text
error: could not find `Cargo.toml` in `/…/password-generator-pro` or any parent directory
```

The shared `rust-ci.yml` pipeline runs cargo from the root and has no
`working-directory` input. Rust CI had therefore never compiled or
tested this project at all. Adding `Cargo.toml` at the root is what made
the rest of this work measurable.

## API changes

| Tauri 1 | Tauri 2 |
| --- | --- |
| `SystemTray` / `SystemTrayMenu` | `TrayIconBuilder` / `Menu` |
| `CustomMenuItem::new(id, label)` | `MenuItem::with_id(app, id, label, enabled, accel)` |
| `MenuItem::Quit`, `MenuItem::Hide` … | `PredefinedMenuItem::quit(app, None)` etc. |
| `app.get_window("main")` | `app.get_webview_window("main")` |
| `tauri::api::dialog` | `tauri-plugin-dialog` (`DialogExt`) |
| `tauri::api::clipboard` | `tauri-plugin-clipboard-manager` |
| `@tauri-apps/api/tauri` (`invoke`) | `@tauri-apps/api/core` |
| `@tauri-apps/api/clipboard` | `@tauri-apps/plugin-clipboard-manager` |
| `tauri.conf.json` → `devPath` / `distDir` | `devUrl` / `frontendDist` |
| `tauri.conf.json` → `withGlobalTauri` | moved under `app` |
| `tauri.conf.json` → `systemTray` | `app.trayIcon` |
| `tauri.conf.json` → `macOS.license` | `bundle.licenseFile` |
| `allowlist` | `capabilities/default.json` |

### Permissions

The v1 `allowlist` granted `dialog: all`, `clipboard: all` and
`window.setIcon`. `capabilities/default.json` replaces that with the
narrowest v2 permissions that preserve what the application actually
does — message dialogs, a save dialog, clipboard writes, and show/hide
of the main window. Anything the code does not use is not granted.

## Structural changes

### `main.rs` → `lib.rs` + a three-line binary

`fn main()` became `pub fn run()` in a new library target, with
`main.rs` reduced to a call into it. This is what allows
`tests/menu_construction.rs` to link against the menu and tray builders,
and what makes the crate's doc examples run — 22 doc tests that had
never executed now do.

### `core/ids.rs`

The menu id strings were previously duplicated as literals in both
`menu.rs` and `tray.rs`. They are now defined once, alongside
`TRAY_ITEM_IDS` and `MENU_ITEM_IDS`, which lets a test assert that every
id the menus emit is one the dispatcher handles.

### `core/action.rs`

`perform` used to match on id strings inline and execute the effect in
the same arm. The *decision* — which id means which action — is now a
pure function, `action_for(&str) -> Option<Action>`. This is the part
worth testing, and it is the reason coverage of the dispatch logic is
97% while `perform` itself is not.

The old match had no fallback arm, so a renamed id fell through to
whatever the last arm happened to be. `action_for` returns `None`, and
`perform` returns early.

## Bugs found and fixed during the migration

These were pre-existing; the migration surfaced them.

1. **`website()` panicked.** The v1 body was
   `webbrowser::open(url).unwrap()`, which aborts the process on any
   machine where no browser can be launched — headless CI, a
   locked-down desktop. It also accepted any string, so a `file:` or
   `javascript:` URL went straight to the platform opener. It now
   validates the scheme and returns `bool`.
2. **The frontend crashed on unshipped locales.** `Translate` did
   `locales[locale][key]` with no fallback, and the locale comes from
   `Intl.DateTimeFormat().resolvedOptions().locale` — the operating
   system's setting. On `nb-NO`, `he-IL`, `vi-VN` and every other tag
   with no table, that is `undefined[key]`: a TypeError thrown from
   `App.svelte`'s script block, and a blank window. `resolveLocale` now
   falls back by language subtag, then to `en-GB`.
3. **`@tauri-apps/plugin-clipboard-manager` was imported but never
   declared** in `package.json`, so a clean checkout could not build.
4. **`svelte-check` could not run.** v3.6.2 loads
   `svelte/compiler.cjs`, which Svelte 5 removed, so the `check` script
   failed before checking anything. On v4 it reports **0 errors across
   771 files** — after fixing the 5 real type errors it found, which
   had been invisible while the tool was dead.
5. **The mount point was assumed to exist.** `new App({ target:
   document.getElementById('app') })` passes `null` when `#app` is
   missing, failing deep inside Svelte. It now throws with the cause.
6. **`webbrowser` carried an unpatched advisory.** RUSTSEC-2026-0257:
   on Unix, the URL was substituted into the `BROWSER` environment
   template *before* tokenising, so a URL retaining spaces became extra
   browser arguments — reproduced upstream by injecting
   `--remote-debugging-port` and `--proxy-server` into Chromium. The
   scheme guard added to `website()` blocks the non-HTTP(S) URLs the
   advisory needs, but the crate is bumped 0.8 → 1.2.4 regardless.
   `cargo audit` goes from 1 vulnerability to 0. The 17 remaining
   notices are `unmaintained`/`unsound` flags on the gtk-rs stack that
   Tauri pulls in transitively on Linux; they are not fixable here.
7. **The release profile would have been silently dropped.** Cargo
   ignores `[profile.*]` in a non-root workspace member and only warns.
   Adding the workspace root would have discarded `lto`, `opt-level =
   "s"`, `panic = "abort"` and `strip` — shipping a larger, slower
   binary. The profiles moved to the root manifest verbatim.

## Tests

`cargo test --workspace` runs **92 tests, 0 failures** (29 test
functions existed before; none of the doc tests ran).

### `tests/menu_construction.rs` has `harness = false`

`muda`, the menu backend Tauri 2 uses, refuses to build a menu off the
main thread on macOS:

```text
`muda::Menu` can only be created on the main thread
```

Rust's default test harness runs every `#[test]` on a spawned thread,
and `--test-threads=1` does not help because that is still not the
*main* thread. `harness = false` makes the file's own `main` the test
binary, which the runner executes on the main thread.

### What is not covered, and why

Coverage is **86.15% of regions, 87.52% of lines** (`cargo llvm-cov
--workspace`). It is not 100%, and the gap is not incidental:

| File | Regions | Why the remainder is unreachable from a test |
| --- | --- | --- |
| `core/action.rs` | 97.32% | — |
| `core/website.rs` | 95.77% | the `webbrowser::open` call itself |
| `core/menu.rs` | 83.33% | error paths of `muda` builders |
| `core/tray.rs` | 85.90% | error paths of `muda` builders |
| `lib.rs` | 58.10% | see below |
| `main.rs` | 0.00% | a three-line call into `run()` |

`perform`'s arms are OS effects: launching a browser, writing the system
clipboard, raising a native dialog, and `app.exit(0)`. A test that
covered them would open a browser on the machine running it, clobber the
developer's clipboard, block on a modal, or terminate the test process.
Only two arms have no side effect and both are tested — an unrecognised
id, and `ToggleWindow` with no window present.

`run()` is uncovered because it calls `tauri::Builder::run`, which blocks
on the platform event loop.

This is why `action_for` was extracted: the decision is pure and tested;
the effect is thin enough to read.

## Benchmarks

`cargo bench` (criterion). Representative figures from an Apple Silicon
machine:

| Benchmark | Time (median, 95% CI) |
| --- | --- |
| bcrypt at `HASH_COST` | 19.285 ms [19.023, 19.597] |
| `uuid v4` | 90.6 ns [88.2, 93.2] |
| `qrcode`, 5-byte input | 512 µs [476, 552] |
| `qrcode`, 40-byte input | 798 µs [736, 857] |
| `action_for` over all 16 menu ids | 57.7 ns [56.8, 58.5] |
| `action_for` miss | 2.94 ns [2.92, 2.96] |

bcrypt dominates `generate_password` by four orders of magnitude and is
the reason the tray's "Copy Password" action is worth watching: it runs
synchronously while the user waits. The benchmark exists so that a
change to `HASH_COST` is a visible decision rather than a silent one.

## Toolchain

`rust-version` is 1.77 (`OnceLock`, C-string literals in the v2 macro
expansion). CI pins pnpm 7 via `pnpm/action-setup@v2`; the lockfile is
`lockfileVersion: '6.0'` and was regenerated with pnpm 8 to keep that
format — a newer pnpm rewrites it to `9.0`, which CI cannot read.
