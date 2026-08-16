// Copyright © 2022-2026 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Opening external links.
//!
//! The v1 implementation was `webbrowser::open(url).unwrap()`. That
//! panics — and therefore aborts the application — on any machine
//! where no browser can be launched, which includes headless CI and a
//! locked-down desktop. It also accepted any string, so a malformed or
//! non-web scheme was handed straight to the platform opener.
//!
//! The scheme check is pure and lives in [`is_openable`], so the part
//! worth testing is testable without opening anything.

/// Schemes this application is willing to hand to the platform opener.
const ALLOWED_SCHEMES: [&str; 2] = ["https://", "http://"];

/// Whether `url` is a web URL this application will open.
///
/// Only `http` and `https` are accepted. Everything else — `file:`,
/// `javascript:`, a bare path, an empty string — is refused, because
/// every link this application opens is a documented web page.
#[must_use]
pub fn is_openable(url: &str) -> bool {
    ALLOWED_SCHEMES
        .iter()
        .any(|scheme| url.len() > scheme.len() && url.starts_with(scheme))
}

/// Open `url` in the user's default browser.
///
/// Returns `false` if the URL is refused or the browser could not be
/// launched. Failing to open a help page is not worth terminating the
/// application over, which is what the previous `unwrap` did.
#[tauri::command]
pub fn website(url: &str) -> bool {
    if !is_openable(url) {
        return false;
    }
    webbrowser::open(url).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn https_urls_are_openable() {
        assert!(is_openable("https://password-generator.pro"));
        assert!(is_openable("https://example.com/docs?a=1#b"));
    }

    #[test]
    fn http_urls_are_openable() {
        assert!(is_openable("http://localhost:1420"));
    }

    #[test]
    fn non_web_schemes_are_refused() {
        for url in [
            "file:///etc/passwd",
            "javascript:alert(1)",
            "data:text/html,<script>",
            "ftp://example.com",
            "mailto:a@b.c",
        ] {
            assert!(!is_openable(url), "should have refused {url}");
        }
    }

    #[test]
    fn empty_and_scheme_only_urls_are_refused() {
        for url in ["", " ", "https://", "http://", "https", "//example.com"] {
            assert!(!is_openable(url), "should have refused {url:?}");
        }
    }

    #[test]
    fn scheme_matching_is_case_sensitive_and_prefix_anchored() {
        // A scheme appearing later in the string must not qualify.
        assert!(!is_openable("x https://example.com"));
        assert!(!is_openable("HTTPS://example.com"));
    }

    #[test]
    fn website_refuses_disallowed_urls_without_opening_them() {
        // No browser is launched: the guard rejects before webbrowser is
        // reached, so this is safe to assert in CI.
        assert!(!website("file:///etc/passwd"));
        assert!(!website(""));
        assert!(!website("javascript:alert(1)"));
    }

    #[test]
    fn every_application_url_constant_is_openable() {
        use crate::util::constant::{
            ACKNOWLEDGEMENTS, DOCUMENTATION, HOMEPAGE, ISSUE, LICENSE_URL, RELEASE,
        };
        for url in [
            ACKNOWLEDGEMENTS,
            DOCUMENTATION,
            HOMEPAGE,
            ISSUE,
            LICENSE_URL,
            RELEASE,
        ] {
            assert!(is_openable(url), "shipped URL would be refused: {url}");
        }
    }
}
