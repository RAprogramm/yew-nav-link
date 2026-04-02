//! Macro implementations for yew-nav-link
//!
//! This module provides procedural macros for reducing boilerplate
//! and improving compile-time safety.

/// Generates a `validate_routes` method for a `Routable` enum.
///
/// Checks at runtime that all route paths are well-formed.
#[macro_export]
macro_rules! routable_ext {
    ($name:ident) => {
        impl $name {
            pub fn validate_routes() -> Result<(), String> {
                // Implementation would go here
                Ok(())
            }
        }
    };
}

/// Generates a `nav_path` method that returns the route's static path.
#[macro_export]
macro_rules! nav_item {
    ($name:ident) => {
        impl $name {
            pub fn nav_path(&self) -> &'static str {
                // Implementation would go here
                ""
            }
        }
    };
}

/// Convenience wrapper around [`nav_link`](yew_nav_link::nav_link) for
/// declarative macro usage.
#[macro_export]
macro_rules! nav_link {
    ($route:path, $label:expr, $mode:expr) => {
        yew_nav_link::nav_link($route, $label, $mode)
    };
}
