//! Keyboard navigation configuration.

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct KeyboardNavConfig {
    pub wrap:     bool,
    pub vertical: bool
}

impl KeyboardNavConfig {
    pub fn new() -> Self {
        Self {
            wrap:     true,
            vertical: false
        }
    }

    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn with_vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }
}
