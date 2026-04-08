//! Reactive hooks for navigation state.
//!
//! Provides Yew hooks for reading route information, checking active state,
//! building breadcrumbs, and programmatic navigation.

mod navigation;
mod route_info;

pub use navigation::{
    use_navigation, use_query_params, use_route_params, Navigation, QueryParams, RouteParams,
};
pub use route_info::{
    use_breadcrumbs, use_is_active, use_is_exact_active, use_is_partial_active, use_route_info,
    BreadcrumbItem, BreadcrumbLabelProvider,
};
