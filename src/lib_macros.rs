//! Macro implementations for yew-nav-link
//!
//! This module provides procedural macros for reducing boilerplate
//! and improving compile-time safety.

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

#[macro_export]
macro_rules! nav_link {
    ($route:path, $label:expr, $mode:expr) => {
        yew_nav_link::nav_link($route, $label, $mode)
    };
}
