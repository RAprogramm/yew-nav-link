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
#[must_use]
pub fn handle_arrow_key(
    key: &str,
    current_index: usize,
    total_items: usize,
    config: &KeyboardNavConfig
) -> Option<usize> {
    let direction = if config.vertical {
        match key {
            "ArrowDown" => Some(KeyboardDirection::Forward),
            "ArrowUp" => Some(KeyboardDirection::Backward),
            _ => None
        }
    } else {
        match key {
            "ArrowRight" => Some(KeyboardDirection::Forward),
            "ArrowLeft" => Some(KeyboardDirection::Backward),
            _ => None
        }
    };

    let direction = direction?;

    // Handle empty list
    if total_items == 0 {
        return None;
    }

    let next = match direction {
        KeyboardDirection::Forward => current_index + 1,
        KeyboardDirection::Backward => current_index.saturating_sub(1)
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
#[must_use]
pub fn handle_home_end(key: &str, _current_index: usize, total_items: usize) -> Option<usize> {
    match key {
        "Home" => Some(0),
        "End" => Some(total_items.saturating_sub(1)),
        _ => None
    }
}

/// Returns `true` if the key activates a focused element (`Enter`, space).
#[must_use]
pub fn is_activation_key(key: &str) -> bool {
    matches!(key, "Enter" | " " | "Space")
}

/// Returns `true` if the key is a navigation key (arrows, Home, End, Tab).
#[must_use]
pub fn is_navigation_key(key: &str) -> bool {
    matches!(
        key,
        "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" | "Home" | "End" | "Tab"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_arrow_key_valid_keys() {
        let config = KeyboardNavConfig {
            wrap:     false,
            vertical: true
        };
        let result = handle_arrow_key("ArrowDown", 0, 5, &config);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn handle_arrow_key_invalid_key() {
        let config = KeyboardNavConfig::default();
        let result = handle_arrow_key("Enter", 0, 5, &config);
        assert_eq!(result, None);
    }

    #[test]
    fn handle_arrow_key_invalid_key_horizontal() {
        let config = KeyboardNavConfig {
            vertical: false,
            ..KeyboardNavConfig::default()
        };
        let result = handle_arrow_key("Enter", 0, 5, &config);
        assert_eq!(result, None);
    }

    #[test]
    fn handle_arrow_key_invalid_key_both_directions() {
        let config = KeyboardNavConfig {
            vertical: true,
            ..KeyboardNavConfig::default()
        };
        let result = handle_arrow_key("Tab", 0, 5, &config);
        assert_eq!(result, None);
    }

    #[test]
    fn handle_arrow_key_empty_list() {
        let config = KeyboardNavConfig::default();
        let result = handle_arrow_key("ArrowDown", 0, 0, &config);
        assert_eq!(result, None);
    }

    #[test]
    fn handle_arrow_key_vertical_forward() {
        let config = KeyboardNavConfig {
            vertical: true,
            ..KeyboardNavConfig::default()
        };
        let result = handle_arrow_key("ArrowDown", 2, 5, &config);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn handle_arrow_key_vertical_backward() {
        let config = KeyboardNavConfig {
            vertical: true,
            ..KeyboardNavConfig::default()
        };
        let result = handle_arrow_key("ArrowUp", 2, 5, &config);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn handle_arrow_key_horizontal_forward() {
        let config = KeyboardNavConfig {
            vertical: false,
            ..KeyboardNavConfig::default()
        };
        let result = handle_arrow_key("ArrowRight", 2, 5, &config);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn handle_arrow_key_horizontal_backward() {
        let config = KeyboardNavConfig {
            vertical: false,
            ..KeyboardNavConfig::default()
        };
        let result = handle_arrow_key("ArrowLeft", 2, 5, &config);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn handle_arrow_key_wrap_enabled() {
        let config = KeyboardNavConfig {
            wrap: true,
            ..KeyboardNavConfig::default()
        };
        let result = handle_arrow_key("ArrowRight", 4, 5, &config);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn handle_arrow_key_wrap_disabled() {
        let config = KeyboardNavConfig {
            wrap: false,
            ..KeyboardNavConfig::default()
        };
        let result = handle_arrow_key("ArrowRight", 4, 5, &config);
        assert_eq!(result, None);
    }

    #[test]
    fn handle_arrow_key_boundary_forward() {
        let config = KeyboardNavConfig {
            wrap: false,
            ..KeyboardNavConfig::default()
        };
        let result = handle_arrow_key("ArrowDown", 10, 10, &config);
        assert_eq!(result, None);
    }

    #[test]
    fn handle_arrow_key_boundary_backward() {
        let config = KeyboardNavConfig {
            wrap: false,
            ..KeyboardNavConfig::default()
        };
        let result = handle_arrow_key("ArrowUp", 0, 5, &config);
        assert_eq!(result, None);
    }

    #[test]
    fn handle_home_end_home() {
        let result = handle_home_end("Home", 5, 10);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn handle_home_end_end() {
        let result = handle_home_end("End", 5, 10);
        assert_eq!(result, Some(9));
    }

    #[test]
    fn handle_home_end_end_empty_list() {
        let result = handle_home_end("End", 5, 0);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn handle_home_end_invalid_key() {
        let result = handle_home_end("Enter", 5, 10);
        assert_eq!(result, None);
    }

    #[test]
    fn handle_home_end_with_current_index() {
        let result = handle_home_end("End", 2, 5);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn is_activation_key_enter() {
        assert!(is_activation_key("Enter"));
    }

    #[test]
    fn is_activation_key_space() {
        assert!(is_activation_key(" "));
    }

    #[test]
    fn is_activation_key_space_keyword() {
        assert!(is_activation_key("Space"));
    }

    #[test]
    fn is_activation_key_invalid() {
        assert!(!is_activation_key("EnterSpace"));
        assert!(!is_activation_key("Tab"));
    }

    #[test]
    fn is_navigation_key_arrows() {
        assert!(is_navigation_key("ArrowUp"));
        assert!(is_navigation_key("ArrowDown"));
        assert!(is_navigation_key("ArrowLeft"));
        assert!(is_navigation_key("ArrowRight"));
    }

    #[test]
    fn is_navigation_key_special() {
        assert!(is_navigation_key("Home"));
        assert!(is_navigation_key("End"));
        assert!(is_navigation_key("Tab"));
    }

    #[test]
    fn is_navigation_key_invalid() {
        assert!(!is_navigation_key("Enter"));
        assert!(!is_navigation_key("Space"));
    }
}
