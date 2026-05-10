// SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
// SPDX-License-Identifier: MIT

/// Reactive access to URL query parameters.
pub mod query_params;
/// Programmatic navigation primitives built on top of yew-router's `Navigator`.
pub mod use_navigation;

pub use query_params::use_query_params;
pub use use_navigation::{Navigation, use_navigation};
