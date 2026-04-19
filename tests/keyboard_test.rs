//! Keyboard navigation tests.

use yew_nav_link::utils::{KeyboardDirection, KeyboardNavConfig};

#[test]
fn keyboard_direction_clone() {
    let forward = KeyboardDirection::Forward;
    // KeyboardDirection implements Copy, so it's automatically copied
    let copied = forward;
    assert_eq!(forward, copied);
}

#[test]
fn keyboard_direction_backward() {
    let backward = KeyboardDirection::Backward;
    assert_ne!(KeyboardDirection::Forward, backward);
}

#[test]
fn keyboard_direction_debug() {
    let forward = KeyboardDirection::Forward;
    let debug_str = format!("{forward:?}");
    assert!(debug_str.contains("Forward"));
}

#[test]
fn keyboard_direction_partial_eq() {
    assert_eq!(KeyboardDirection::Forward, KeyboardDirection::Forward);
    assert_eq!(KeyboardDirection::Backward, KeyboardDirection::Backward);
    assert_ne!(KeyboardDirection::Forward, KeyboardDirection::Backward);
}

#[test]
fn keyboard_nav_config_new() {
    let config = KeyboardNavConfig::new();
    assert!(config.wrap);
    assert!(!config.vertical);
}

#[test]
fn keyboard_nav_config_default() {
    let config = KeyboardNavConfig::default();
    // Default derived values for bool are false
    assert!(!config.wrap);
    assert!(!config.vertical);
}

#[test]
fn keyboard_nav_config_with_wrap() {
    let config = KeyboardNavConfig::new().with_wrap(false);
    assert!(!config.wrap);
}

#[test]
fn keyboard_nav_config_with_vertical() {
    let config = KeyboardNavConfig::new().with_vertical(true);
    assert!(config.vertical);
}

#[test]
fn keyboard_nav_config_chained() {
    let config = KeyboardNavConfig::new()
        .with_wrap(false)
        .with_vertical(true);
    assert!(!config.wrap);
    assert!(config.vertical);
}

#[test]
fn keyboard_nav_config_clone() {
    let config1 = KeyboardNavConfig::new().with_wrap(false);
    let config2 = config1.clone();
    assert_eq!(config1, config2);
}

#[test]
fn keyboard_nav_config_partial_eq() {
    let config1 = KeyboardNavConfig::new();
    let config2 = KeyboardNavConfig::new();
    assert_eq!(config1, config2);

    let config3 = KeyboardNavConfig::new().with_wrap(false);
    assert_ne!(config1, config3);
}

#[test]
fn keyboard_nav_config_debug() {
    let config = KeyboardNavConfig::new().with_wrap(false);
    let debug_str = format!("{config:?}");
    assert!(debug_str.contains("KeyboardNavConfig"));
}

// Tests for handle_arrow_key

#[test]
fn handle_arrow_key_horizontal_forward() {
    let config = KeyboardNavConfig::new();
    let next = yew_nav_link::utils::handle_arrow_key("ArrowRight", 0, 5, &config);
    assert_eq!(next, Some(1));
}

#[test]
fn handle_arrow_key_horizontal_backward() {
    let config = KeyboardNavConfig::new();
    let prev = yew_nav_link::utils::handle_arrow_key("ArrowLeft", 2, 5, &config);
    assert_eq!(prev, Some(1));
}

#[test]
fn handle_arrow_key_horizontal_wrap_forward() {
    let config = KeyboardNavConfig::new().with_wrap(true);
    let next = yew_nav_link::utils::handle_arrow_key("ArrowRight", 4, 5, &config);
    assert_eq!(next, Some(0));
}

#[test]
fn handle_arrow_key_horizontal_no_wrap() {
    let config = KeyboardNavConfig::new().with_wrap(false);
    let next = yew_nav_link::utils::handle_arrow_key("ArrowRight", 4, 5, &config);
    assert_eq!(next, None);
}

#[test]
fn handle_arrow_key_vertical_forward() {
    let config = KeyboardNavConfig::new().with_vertical(true);
    let next = yew_nav_link::utils::handle_arrow_key("ArrowDown", 0, 5, &config);
    assert_eq!(next, Some(1));
}

#[test]
fn handle_arrow_key_vertical_backward() {
    let config = KeyboardNavConfig::new().with_vertical(true);
    let prev = yew_nav_link::utils::handle_arrow_key("ArrowUp", 2, 5, &config);
    assert_eq!(prev, Some(1));
}

#[test]
fn handle_arrow_key_invalid_key() {
    let config = KeyboardNavConfig::new();
    let next = yew_nav_link::utils::handle_arrow_key("Escape", 0, 5, &config);
    assert_eq!(next, None);
}

#[test]
fn handle_arrow_key_empty_list() {
    // Empty list should return None
    let config = KeyboardNavConfig::new();
    let next = yew_nav_link::utils::handle_arrow_key("ArrowRight", 0, 0, &config);
    assert_eq!(next, None);
}

#[test]
fn handle_arrow_key_single_item() {
    let config = KeyboardNavConfig::new().with_wrap(true);
    let next = yew_nav_link::utils::handle_arrow_key("ArrowRight", 0, 1, &config);
    assert_eq!(next, Some(0));
}

// Tests for handle_home_end

#[test]
fn handle_home_end_home() {
    let result = yew_nav_link::utils::handle_home_end("Home", 3, 10);
    assert_eq!(result, Some(0));
}

#[test]
fn handle_home_end_end() {
    let result = yew_nav_link::utils::handle_home_end("End", 0, 10);
    assert_eq!(result, Some(9));
}

#[test]
fn handle_home_end_empty_list() {
    let result = yew_nav_link::utils::handle_home_end("End", 0, 0);
    assert_eq!(result, Some(0));
}

#[test]
fn handle_home_end_invalid_key() {
    let result = yew_nav_link::utils::handle_home_end("Escape", 0, 10);
    assert_eq!(result, None);
}

// Tests for is_activation_key

#[test]
fn is_activation_key_enter() {
    assert!(yew_nav_link::utils::is_activation_key("Enter"));
}

#[test]
fn is_activation_key_space() {
    assert!(yew_nav_link::utils::is_activation_key(" "));
}

#[test]
fn is_activation_key_space_name() {
    assert!(yew_nav_link::utils::is_activation_key("Space"));
}

#[test]
fn is_activation_key_not_activation() {
    assert!(!yew_nav_link::utils::is_activation_key("ArrowRight"));
}

// Tests for is_navigation_key

#[test]
fn is_navigation_key_arrow() {
    assert!(yew_nav_link::utils::is_navigation_key("ArrowUp"));
    assert!(yew_nav_link::utils::is_navigation_key("ArrowDown"));
    assert!(yew_nav_link::utils::is_navigation_key("ArrowLeft"));
    assert!(yew_nav_link::utils::is_navigation_key("ArrowRight"));
}

#[test]
fn is_navigation_key_home_end() {
    assert!(yew_nav_link::utils::is_navigation_key("Home"));
    assert!(yew_nav_link::utils::is_navigation_key("End"));
}

#[test]
fn is_navigation_key_tab() {
    assert!(yew_nav_link::utils::is_navigation_key("Tab"));
}

#[test]
fn is_navigation_key_not_navigation() {
    assert!(!yew_nav_link::utils::is_navigation_key("Enter"));
    assert!(!yew_nav_link::utils::is_navigation_key("Escape"));
}
