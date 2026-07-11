// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Integration tests that exercise the public API in a real browser.
//!
//! Native test runs treat this target as empty — the `cfg(target_arch =
//! "wasm32")` gate strips the body. The full suite runs under
//! `wasm-pack test --headless --chrome --firefox --test wasm`.
//!
//! Cargo's default module resolution would look for `tests/common.rs` and
//! `tests/nav_link.rs` next to this file; the `#[path]` attributes redirect
//! the lookup so the wasm-specific submodules stay under `tests/wasm/`.

#![cfg(target_arch = "wasm32")]

#[path = "wasm/common.rs"]
mod common;

#[path = "wasm/nav_link.rs"]
mod nav_link;

#[path = "wasm/components.rs"]
mod components;

#[path = "wasm/navigation.rs"]
mod navigation;

#[path = "wasm/dropdown.rs"]
mod dropdown;

#[path = "wasm/pagination.rs"]
mod pagination;

#[path = "wasm/tabs.rs"]
mod tabs;

#[path = "wasm/hooks.rs"]
mod hooks;
