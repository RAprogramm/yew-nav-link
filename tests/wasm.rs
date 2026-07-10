// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Integration tests that exercise the public API in a real browser.
//!
//! Native test runs treat this target as empty — the `cfg(target_arch =
//! "wasm32")` gate strips the body. The full suite runs under
//! `wasm-pack test --headless --chrome --firefox --test wasm`.

#![cfg(target_arch = "wasm32")]

// Cargo's default module resolution would look for `tests/common.rs` and
// `tests/nav_link.rs` next to this file. We want the wasm-specific
// submodules under `tests/wasm/` so the directory stays self-contained;
// `#[path]` redirects the lookup accordingly.

#[path = "wasm/common.rs"]
mod common;

#[path = "wasm/nav_link.rs"]
mod nav_link;

#[path = "wasm/components.rs"]
mod components;

#[path = "wasm/navigation.rs"]
mod navigation;
