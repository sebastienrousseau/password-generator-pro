// Copyright © 2022-2026 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

//! Binary entry point.
//!
//! Everything lives in the library so it can be linked by integration
//! tests; this file only starts it.

fn main() {
    password_generator_pro::run();
}
