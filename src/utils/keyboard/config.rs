// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Keyboard navigation configuration.

/// Configuration for keyboard-driven navigation behavior.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyboardNavConfig {
    /// Whether navigation wraps around when reaching the first or last item.
    pub wrap:     bool,
    /// Whether arrow keys map to vertical (`ArrowUp`/`ArrowDown`) movement.
    pub vertical: bool
}

impl Default for KeyboardNavConfig {
    /// Matches [`KeyboardNavConfig::new`], so `..Default::default()` and
    /// `new()` produce identical behavior.
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardNavConfig {
    /// Creates a new config with wrapping enabled and horizontal orientation.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            wrap:     true,
            vertical: false
        }
    }

    /// Sets whether navigation wraps around at boundaries.
    #[must_use]
    pub const fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// Sets whether navigation is vertical instead of horizontal.
    #[must_use]
    pub const fn with_vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }
}
