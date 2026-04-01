//! # Route Hooks
//!
//! Reactive Yew hooks for working with the current route and navigation state.
//! All hooks depend on [`yew_router`] and require a `<BrowserRouter>` ancestor.
//!
//! # Example
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::hooks::{use_route_info, use_is_active, use_is_partial_active};
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Debug, Routable)]
//! # enum Route { #[at("/")] Home, #[at("/docs")] Docs, #[at("/docs/api")] DocsApi }
//! #[component]
//! fn Status() -> Html {
//!     let current = use_route_info::<Route>();
//!     let is_home = use_is_active(Route::Home);
//!     let docs_partial = use_is_partial_active(Route::Docs);
//!
//!     html! {
//!         <div>
//!             <p>{ format!("Route: {:?}", current) }</p>
//!             <p>{ format!("Home active: {}", is_home) }</p>
//!             <p>{ format!("Docs partial: {}", docs_partial) }</p>
//!         </div>
//!     }
//! }
//! ```
//!
//! # Hooks
//!
//! | Hook | Return | Description |
//! |------|--------|-------------|
//! | `use_route_info::<R>()` | `Option<R>` | Current route or `None` |
//! | `use_is_active(route)` | `bool` | Exact match check |
//! | `use_is_exact_active(route)` | `bool` | Alias for `use_is_active` |
//! | `use_is_partial_active(route)` | `bool` | Prefix match (segment-wise) |
//! | `use_breadcrumbs::<R>()` | `Vec<BreadcrumbItem<R>>` | Breadcrumb trail |

use yew::prelude::*;
use yew_router::prelude::*;

/// Returns the currently active route, or `None` if no route is matched.
#[hook]
pub fn use_route_info<R: Routable + 'static>() -> Option<R> {
    use_route::<R>()
}

/// Returns `true` when the given route exactly matches the current URL.
#[hook]
pub fn use_is_active<R>(route: R) -> bool
where
    R: Routable + Clone + PartialEq + 'static,
{
    let current = use_route::<R>();
    current.as_ref() == Some(&route)
}

/// Returns `true` when the given route exactly matches the current URL.
///
/// This is an alias for [`use_is_active`]; both perform exact matching.
#[hook]
pub fn use_is_exact_active<R>(route: R) -> bool
where
    R: Routable + Clone + PartialEq + 'static,
{
    let current = use_route::<R>();
    current.as_ref() == Some(&route)
}

/// Returns `true` when the current URL starts with the given route's path.
///
/// Matches segment-wise, so `/docs` matches `/docs/api` but not `/documentation`.
#[hook]
pub fn use_is_partial_active<R>(route: R) -> bool
where
    R: Routable + Clone + PartialEq + 'static,
{
    let current = use_route::<R>();
    match current.as_ref() {
        Some(current_route) => {
            let target_path = route.to_path();
            let current_path = current_route.to_path();
            is_path_prefix(&target_path, &current_path)
        }
        None => false,
    }
}

/// Returns a list of [`BreadcrumbItem`]s representing the current navigation path.
#[hook]
pub fn use_breadcrumbs<R>() -> Vec<BreadcrumbItem<R>>
where
    R: Routable + Clone + PartialEq + 'static,
{
    let current = use_route::<R>();
    match current {
        Some(route) => {
            let label = route.to_path();
            vec![BreadcrumbItem {
                route: route.clone(),
                label: label.to_string(),
                is_active: true,
            }]
        }
        None => vec![],
    }
}

/// A single item in a breadcrumb trail.
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `route` | `R` | The route this breadcrumb points to |
/// | `label` | `String` | Human-readable label |
/// | `is_active` | `bool` | Whether this is the current route |
#[derive(Clone, Debug)]
pub struct BreadcrumbItem<R> {
    /// The route this breadcrumb points to.
    pub route: R,
    /// Human-readable label for the breadcrumb.
    pub label: String,
    /// Whether this breadcrumb represents the currently active route.
    pub is_active: bool,
}

fn is_path_prefix(target: &str, current: &str) -> bool {
    let target_parts: Vec<&str> = target.split('/').filter(|s| !s.is_empty()).collect();
    let current_parts: Vec<&str> = current.split('/').filter(|s| !s.is_empty()).collect();

    for (t, c) in target_parts.iter().zip(current_parts.iter()) {
        if t != c {
            return false;
        }
    }

    target_parts.len() <= current_parts.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_path_prefix_exact() {
        assert!(is_path_prefix("/docs", "/docs"));
        assert!(is_path_prefix("/", "/"));
    }

    #[test]
    fn is_path_prefix_nested() {
        assert!(is_path_prefix("/docs", "/docs/api"));
        assert!(is_path_prefix("/docs", "/docs/api/v1"));
    }

    #[test]
    fn is_path_prefix_not_match() {
        assert!(!is_path_prefix("/docs", "/about"));
        assert!(!is_path_prefix("/doc", "/docs"));
    }

    #[test]
    fn is_path_prefix_root() {
        assert!(is_path_prefix("/", "/any/path"));
    }

    #[test]
    fn breadcrumb_item_creation() {
        #[derive(Clone, PartialEq)]
        struct TestRoute;
        let item = BreadcrumbItem {
            route: TestRoute,
            label: "/test".to_string(),
            is_active: true,
        };
        assert_eq!(item.label, "/test");
        assert!(item.is_active);
    }
}
