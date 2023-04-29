// Copyright © 2022-2023 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/// Constant values used in the application.

/// Acknowledgements URL.
pub const ACKNOWLEDGEMENTS: &str =
    "https://github.com/sebastienrousseau/password-generator-pro/graphs/contributors";

/// Description of the application.
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Documentation URL.
pub const DOCUMENTATION: &str = "https://password-generator.pro/docs";

/// Hash cost.
pub const HASH_COST: u32 = 8;

/// Homepage URL.
pub const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

/// Issue URL.
pub const ISSUE: &str = "https://github.com/sebastienrousseau/password-generator-pro/issues";

/// License URL.
pub const LICENSE_URL: &str =
    "https://github.com/sebastienrousseau/password-generator-pro/blob/main/COPYRIGHT";

/// Name of the application.
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Release URL.
pub const RELEASE: &str = "https://github.com/sebastienrousseau/password-generator-pro/releases";

/// pub const SHA: &str = env!("VERGEN_GIT_SHA");

/// Special characters.
pub const SPECIAL: &[u8] = b"!@#$%^&*()_+-=[]{};':,./<>?";

/// Version of the application.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
