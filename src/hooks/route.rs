//! Route-related hooks for navigation.
//!
//! Provides reactive hooks for working with routes and navigation state.

use yew::prelude::*;
use yew_router::prelude::*;

#[hook]
pub fn use_route_info<R: Routable + 'static>() -> Option<R> {
    use_route::<R>()
}

#[hook]
pub fn use_is_active<R>(route: R) -> bool
where
    R: Routable + Clone + PartialEq + 'static,
{
    let current = use_route::<R>();
    current.as_ref() == Some(&route)
}

#[hook]
pub fn use_is_exact_active<R>(route: R) -> bool
where
    R: Routable + Clone + PartialEq + 'static,
{
    let current = use_route::<R>();
    current.as_ref() == Some(&route)
}

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

#[derive(Clone, Debug)]
pub struct BreadcrumbItem<R> {
    pub route: R,
    pub label: String,
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
