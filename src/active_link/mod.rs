// SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
// SPDX-License-Identifier: MIT

/// Active-state matching strategy used by [`NavLink`](nav_link::NavLink).
pub mod mode;
/// The [`NavLink`](nav_link::NavLink) component and its function-syntax helper.
pub mod nav_link;
/// Type-checked properties for [`NavLink`](nav_link::NavLink).
pub mod props;
/// Helpers shared between the component and its callers (path-prefix matching,
/// class-name building).
pub mod utils;

pub use mode::Match;
pub use nav_link::{NavLink, nav_link};
pub use props::NavLinkProps;
pub use utils::is_path_prefix;
