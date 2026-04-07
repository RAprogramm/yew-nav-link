//! Keyboard navigation handlers.

use super::{config::KeyboardNavConfig, direction::KeyboardDirection};

/// Computes the next index for an arrow key press.
///
/// Returns `None` if the key is not an arrow key, or if the boundary
/// is reached and wrapping is disabled.
///
/// # Arguments
///
/// * `key` - The key name (`"ArrowUp"`, `"ArrowDown"`, `"ArrowLeft"`,
///   `"ArrowRight"`).
/// * `current_index` - The currently focused item index.
/// * `total_items` - Total number of navigable items.
/// * `config` - Navigation configuration (wrap, vertical).
pub fn handle_arrow_key(
    key: &str,
    current_index: usize,
    total_items: usize,
    config: &KeyboardNavConfig,
) -> Option<usize> {
    let direction = if config.vertical {
        match key {
            "ArrowDown" => Some(KeyboardDirection::Forward),
            "ArrowUp" => Some(KeyboardDirection::Backward),
            _ => None,
        }
    } else {
        match key {
            "ArrowRight" => Some(KeyboardDirection::Forward),
            "ArrowLeft" => Some(KeyboardDirection::Backward),
            _ => None,
        }
    };

    let direction = direction?;

    // Handle empty list
    if total_items == 0 {
        return None;
    }

    let next = match direction {
        KeyboardDirection::Forward => current_index + 1,
        KeyboardDirection::Backward => current_index.saturating_sub(1),
    };

    if config.wrap {
        Some(next % total_items)
    } else if (direction == KeyboardDirection::Backward && current_index == 0)
        || (direction == KeyboardDirection::Forward && current_index >= total_items - 1)
    {
        None
    } else {
        Some(next)
    }
}

/// Returns the target index for `Home` (first item) or `End` (last item) key
/// presses.
///
/// Returns `None` for any other key.
pub fn handle_home_end(key: &str, _current_index: usize, total_items: usize) -> Option<usize> {
    match key {
        "Home" => Some(0),
        "End" => Some(total_items.saturating_sub(1)),
        _ => None,
    }
}

/// Returns `true` if the key activates a focused element (`Enter`, space).
pub fn is_activation_key(key: &str) -> bool {
    matches!(key, "Enter" | " " | "Space")
}

/// Returns `true` if the key is a navigation key (arrows, Home, End, Tab).
pub fn is_navigation_key(key: &str) -> bool {
    matches!(
        key,
        "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" | "Home" | "End" | "Tab"
    )
}
