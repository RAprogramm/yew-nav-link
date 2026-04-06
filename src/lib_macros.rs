//! Macro implementations for yew-nav-link
//!
//! This module provides declarative macros for reducing boilerplate.
//!
//! **Note**: The `routable_ext!` and `nav_item!` macros are stubs pending
//! full implementation. The `nav_link!` macro is functional.

/// Generates a `validate_routes` method for a `Routable` enum.
///
/// **Status**: Stub - not yet implemented.
#[macro_export]
macro_rules! routable_ext {
    ($name:ident) => {
        compile_error!("routable_ext! macro is not yet implemented");
    };
}

/// Generates a `nav_path` method that returns the route's static path.
///
/// **Status**: Stub - not yet implemented.
#[macro_export]
macro_rules! nav_item {
    ($name:ident) => {
        compile_error!("nav_item! macro is not yet implemented");
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
