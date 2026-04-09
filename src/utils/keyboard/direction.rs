//! Keyboard direction enum.

/// Direction of keyboard navigation within a list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[must_use]
pub enum KeyboardDirection {
    /// Move to the next item (right or down).
    Forward,
    /// Move to the previous item (left or up).
    Backward
}
