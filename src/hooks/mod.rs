//! Reactive hooks for navigation state.
//!
//! Provides Yew hooks for reading route information, checking active state,
//! building breadcrumbs, and programmatic navigation.

mod navigation;
mod route;

pub use navigation::{
    Navigation, QueryParams, RouteParams, use_navigation, use_query_params, use_route_params
};
pub use route::{
    BreadcrumbItem, BreadcrumbLabelProvider, use_breadcrumbs, use_is_active, use_is_exact_active,
    use_is_partial_active, use_route_info
};
