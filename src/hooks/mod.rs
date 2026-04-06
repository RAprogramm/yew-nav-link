//! Reactive hooks for navigation state.
//!
//! Provides Yew hooks for reading route information, checking active state,
//! and building breadcrumbs.

mod route;

pub use route::{
    use_breadcrumbs, use_is_active, use_is_exact_active, use_is_partial_active, use_route_info,
    BreadcrumbItem,
};
