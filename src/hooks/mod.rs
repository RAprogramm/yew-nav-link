//! Reactive hooks for navigation state.
//!
//! Provides Yew hooks for reading route information, checking active state,
//! building breadcrumbs, and programmatic navigation.

mod navigation;
mod route_info;

pub use navigation::{Navigation, use_navigation, use_query_params};
pub use route_info::{
    BreadcrumbItem, BreadcrumbLabelProvider, use_breadcrumbs, use_is_active, use_is_exact_active,
    use_is_partial_active, use_route_info
};
