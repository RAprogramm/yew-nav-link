//! Keyboard navigation utilities.
//!
//! Provides helpers for keyboard-accessible navigation.

mod config;
mod direction;
mod handlers;

pub use config::KeyboardNavConfig;
pub use direction::KeyboardDirection;
pub use handlers::{handle_arrow_key, handle_home_end, is_activation_key, is_navigation_key};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyboard_nav_config_default() {
        let config = KeyboardNavConfig::new();
        assert!(config.wrap);
        assert!(!config.vertical);
    }

    #[test]
    fn keyboard_nav_config_builder() {
        let config = KeyboardNavConfig::new()
            .with_wrap(false)
            .with_vertical(true);
        assert!(!config.wrap);
        assert!(config.vertical);
    }

    #[test]
    fn handle_arrow_key_forward() {
        let result = handle_arrow_key("ArrowRight", 0, 5, &KeyboardNavConfig::new());
        assert_eq!(result, Some(1));
    }

    #[test]
    fn handle_arrow_key_backward() {
        let result = handle_arrow_key("ArrowLeft", 3, 5, &KeyboardNavConfig::new());
        assert_eq!(result, Some(2));
    }

    #[test]
    fn handle_arrow_key_invalid() {
        let result = handle_arrow_key("Enter", 0, 5, &KeyboardNavConfig::new());
        assert_eq!(result, None);
    }

    #[test]
    fn handle_home_end_home() {
        let result = handle_home_end("Home", 5, 10);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn handle_home_end_end() {
        let result = handle_home_end("End", 0, 10);
        assert_eq!(result, Some(9));
    }

    #[test]
    fn is_activation_key_tests() {
        assert!(is_activation_key("Enter"));
        assert!(is_activation_key(" "));
        assert!(!is_activation_key("ArrowRight"));
    }

    #[test]
    fn is_navigation_key_tests() {
        assert!(is_navigation_key("ArrowRight"));
        assert!(is_navigation_key("ArrowLeft"));
        assert!(is_navigation_key("Home"));
        assert!(!is_navigation_key("Enter"));
    }
}
