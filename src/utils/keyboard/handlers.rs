//! Keyboard navigation handlers.

use super::{config::KeyboardNavConfig, direction::KeyboardDirection};

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

pub fn handle_home_end(key: &str, _current_index: usize, total_items: usize) -> Option<usize> {
    match key {
        "Home" => Some(0),
        "End" => Some(total_items.saturating_sub(1)),
        _ => None,
    }
}

pub fn is_activation_key(key: &str) -> bool {
    matches!(key, "Enter" | " " | "Space")
}

pub fn is_navigation_key(key: &str) -> bool {
    matches!(
        key,
        "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" | "Home" | "End" | "Tab"
    )
}
