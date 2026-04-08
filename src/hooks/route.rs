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
//!         <span style={if active { "color: var(--green)" } else { "color: var(--red)" }}>
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

use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

/// A trait for providing custom breadcrumb labels.
pub trait BreadcrumbLabelProvider: Send + Sync {
    /// Returns a human-readable label for the given path.
    fn label_for_path(&self, path: &str) -> String;
}

/// Context type for storing the breadcrumb label provider.
#[derive(Clone)]
struct BreadcrumbLabelProviderContext(Rc<dyn BreadcrumbLabelProvider>);

impl PartialEq for BreadcrumbLabelProviderContext {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

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
///
/// If a custom [`BreadcrumbLabelProvider`] is set via
/// [`set_breadcrumb_label_provider`](fn.set_breadcrumb_label_provider.html),
/// it will be used to generate human-readable labels.
#[hook]
pub fn use_breadcrumbs<R>() -> Vec<BreadcrumbItem<R>>
where
    R: Routable + Clone + PartialEq + 'static
{
    let current = use_route::<R>();
    let provider = use_context::<BreadcrumbLabelProviderContext>();

    match current {
        Some(route) => {
            let path = route.to_path();
            let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
            let mut items = Vec::new();
            let mut built = String::new();

            // Root
            let root_label = provider
                .as_ref()
                .map(|p| p.0.label_for_path("/"))
                .unwrap_or_else(|| "/".to_string());
            items.push(BreadcrumbItem {
                route:     route.clone(),
                label:     root_label,
                is_active: segments.is_empty()
            });

            for (i, seg) in segments.iter().enumerate() {
                built.push('/');
                built.push_str(seg);
                let is_last = i == segments.len() - 1;
                let label = provider
                    .as_ref()
                    .map(|p| p.0.label_for_path(&built))
                    .unwrap_or_else(|| built.clone());
                items.push(BreadcrumbItem {
                    route: route.clone(),
                    label,
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
    crate::nav_link::is_path_prefix(target, current)
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

    #[test]
    fn breadcrumb_item_not_active() {
        #[derive(Clone, PartialEq)]
        struct TestRoute;
        let item = BreadcrumbItem {
            route:     TestRoute,
            label:     "/test".to_string(),
            is_active: false
        };
        assert_eq!(item.label, "/test");
        assert!(!item.is_active);
    }

    #[test]
    fn breadcrumb_item_clone() {
        #[derive(Clone, PartialEq, Debug)]
        struct TestRoute;
        let item1 = BreadcrumbItem {
            route:     TestRoute,
            label:     "/test".to_string(),
            is_active: true
        };
        let item2 = item1.clone();
        assert_eq!(item1.label, item2.label);
        assert_eq!(item1.is_active, item2.is_active);
    }

    #[test]
    fn breadcrumb_item_debug() {
        #[derive(Clone, PartialEq, Debug)]
        struct TestRoute;
        let item = BreadcrumbItem {
            route:     TestRoute,
            label:     "/test".to_string(),
            is_active: true
        };
        let debug_str = format!("{:?}", item);
        assert!(debug_str.contains("BreadcrumbItem"));
    }

    #[test]
    fn breadcrumb_label_provider_context_creation() {
        struct TestProvider;
        impl BreadcrumbLabelProvider for TestProvider {
            fn label_for_path(&self, path: &str) -> String {
                path.to_string()
            }
        }
        let ctx = BreadcrumbLabelProviderContext(Rc::new(TestProvider));
        let _ = ctx;
    }

    #[test]
    fn breadcrumb_label_provider_context_clone() {
        struct TestProvider;
        impl BreadcrumbLabelProvider for TestProvider {
            fn label_for_path(&self, path: &str) -> String {
                path.to_string()
            }
        }
        let ctx1 = BreadcrumbLabelProviderContext(Rc::new(TestProvider));
        let ctx2 = ctx1.clone();
        let _ = ctx2;
    }

    #[test]
    fn breadcrumb_label_provider_context_partial_eq() {
        struct TestProvider;
        impl BreadcrumbLabelProvider for TestProvider {
            fn label_for_path(&self, path: &str) -> String {
                path.to_string()
            }
        }
        let ctx1 = BreadcrumbLabelProviderContext(Rc::new(TestProvider));
        let ctx2 = BreadcrumbLabelProviderContext(Rc::new(TestProvider));
        assert!(ctx1 == ctx2);
    }

    #[test]
    fn breadcrumb_label_provider_trait() {
        struct TestProvider;
        impl BreadcrumbLabelProvider for TestProvider {
            fn label_for_path(&self, path: &str) -> String {
                format!("Custom: {}", path)
            }
        }
        let provider = TestProvider;
        assert_eq!(provider.label_for_path("/test"), "Custom: /test");
        assert_eq!(provider.label_for_path("/"), "Custom: /");
    }

    #[test]
    fn is_path_prefix_edge_cases() {
        assert!(is_path_prefix("/", "/"));
        assert!(is_path_prefix("/test", "/test"));
        assert!(!is_path_prefix("/test", "/testing"));
        assert!(!is_path_prefix("/test", "/test2"));
        assert!(!is_path_prefix("/testing", "/test"));
    }

    #[test]
    fn is_path_prefix_with_trailing_slash() {
        assert!(is_path_prefix("/docs", "/docs/"));
        assert!(is_path_prefix("/docs/", "/docs/api"));
    }

    #[test]
    fn is_path_prefix_empty_strings() {
        assert!(is_path_prefix("", "/test"));
        assert!(!is_path_prefix("/test", ""));
    }
}
