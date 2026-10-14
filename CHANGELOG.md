# Changelog

Notable changes to `password-generator-pro`. Earlier releases are described on
the [GitHub releases page](https://github.com/sebastienrousseau/password-generator-pro/releases).

## [0.0.9] - Unreleased

### Security

- **Dev tooling advisories cleared**: vitest 4.1.11 and patched pins
  for brace-expansion, devalue, source-map-js and
  postcss-selector-parser close the 14 open npm Dependabot alerts.
  `pnpm audit` goes from 22 findings to 1 (braces, which has no
  patched release yet).
- **Still open**: glib 0.18.5 (GHSA-wrw7-89jp-8q8g, fixed in 0.20). It
  comes from gtk 0.18, which every current Tauri 2 release requires on
  Linux, so it moves when Tauri does.

### Changed

- **Breaking: the MSRV is now Rust 1.89** (was 1.77), one floor shared
  with the maintainer's other Rust crates. The code itself needs 1.77.
