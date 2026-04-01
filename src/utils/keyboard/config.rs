//! Keyboard navigation configuration.

/// Configuration for keyboard-driven navigation behavior.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct KeyboardNavConfig {
    /// Whether navigation wraps around when reaching the first or last item.
    pub wrap: bool,
    /// Whether arrow keys map to vertical (`ArrowUp`/`ArrowDown`) movement.
    pub vertical: bool,
}

impl KeyboardNavConfig {
    /// Creates a new config with wrapping enabled and horizontal orientation.
    pub fn new() -> Self {
        Self {
            wrap: true,
            vertical: false,
        }
    }

    /// Sets whether navigation wraps around at boundaries.
    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// Sets whether navigation is vertical instead of horizontal.
    pub fn with_vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }
}
