//! # Route Hooks
//!
//! Reactive hooks that read the current route from Yew Router. All of these
//! re-render automatically when the URL changes — no manual subscriptions
//! needed. They all require a `<BrowserRouter>` ancestor in your component
//! tree.
//!
//! # use_route_info
//!
//! Returns the current route as `Option<R>`, or `None` if nothing matches.
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::hooks::use_route_info;
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Debug, Routable)]
//! # enum Route {
//! #     #[at("/")]
//! #     Home,
//! #     #[at("/docs")]
//! #     Docs,
//! # }
//! #[component]
//! fn CurrentPage() -> Html {
//!     let route = use_route_info::<Route>();
//!     html! { <p>{ format!("You are on: {:?}", route) }</p> }
//! }
//! ```
//!
//! # use_is_active
//!
//! Returns `true` when the given route exactly matches the current URL.
//! Use this to show/hide content based on the active page.
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::hooks::use_is_active;
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Debug, Routable)]
//! # enum Route {
//! #     #[at("/")]
//! #     Home,
//! # }
//! #[component]
//! fn Nav() -> Html {
//!     let active = use_is_active(Route::Home);
//!     html! {
//!         <span class={if active { "highlighted" } else { "" }}>
//!             { "Home" }
//!         </span>
//!     }
//! }
//! ```
//!
//! # use_is_partial_active
//!
//! Returns `true` when the current path **starts with** the given route's path.
//! Compares path segments, so `/docs` matches `/docs/api` but NOT
//! `/documentation`. Perfect for sidebar navigation where a parent link should
//! stay highlighted on nested pages.
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::hooks::use_is_partial_active;
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Debug, Routable)]
//! # enum Route {
//! #     #[at("/docs")]
//! #     Docs,
//! # }
//! #[component]
//! fn Sidebar() -> Html {
//!     let docs_active = use_is_partial_active(Route::Docs);
//!     html! {
//!         <a class={if docs_active { "active" } else { "" }}>
//!             { "Documentation" }
//!         </a>
//!     }
//! }
//! ```
//!
//! # use_breadcrumbs
//!
//! Returns a `Vec<BreadcrumbItem<R>>` representing the route hierarchy.
//! Each item has a `route`, a `label`, and an `is_active` flag (true for
//! the last/current item).

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
    R: Routable + Clone + PartialEq + 'static
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
    R: Routable + Clone + PartialEq + 'static
{
    let current = use_route::<R>();
    current.as_ref() == Some(&route)
}

/// Returns `true` when the current URL starts with the given route's path.
///
/// Matches segment-wise, so `/docs` matches `/docs/api` but not
/// `/documentation`.
#[hook]
pub fn use_is_partial_active<R>(route: R) -> bool
where
    R: Routable + Clone + PartialEq + 'static
{
    let current = use_route::<R>();
    match current.as_ref() {
        Some(current_route) => {
            let target_path = route.to_path();
            let current_path = current_route.to_path();
            is_path_prefix(&target_path, &current_path)
        }
        None => false
    }
}

/// Returns a list of [`BreadcrumbItem`]s representing the current navigation
/// path.
///
/// Splits the current URL into segments and returns one item per level.
/// For a path like `/docs/api/reference`, returns items for `/`, `/docs`,
/// `/docs/api`, and `/docs/api/reference` (last one marked as active).
#[hook]
pub fn use_breadcrumbs<R>() -> Vec<BreadcrumbItem<R>>
where
    R: Routable + Clone + PartialEq + 'static
{
    let current = use_route::<R>();
    match current {
        Some(route) => {
            let path = route.to_path();
            let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
            let mut items = Vec::new();
            let mut built = String::new();

            // Root
            items.push(BreadcrumbItem {
                route:     route.clone(),
                label:     "/".to_string(),
                is_active: segments.is_empty()
            });

            for (i, seg) in segments.iter().enumerate() {
                built.push('/');
                built.push_str(seg);
                let is_last = i == segments.len() - 1;
                items.push(BreadcrumbItem {
                    route:     route.clone(),
                    label:     built.clone(),
                    is_active: is_last
                });
            }

            items
        }
        None => vec![]
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
    pub route:     R,
    /// Human-readable label for the breadcrumb.
    pub label:     String,
    /// Whether this breadcrumb represents the currently active route.
    pub is_active: bool
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
            route:     TestRoute,
            label:     "/test".to_string(),
            is_active: true
        };
        assert_eq!(item.label, "/test");
        assert!(item.is_active);
    }
}
