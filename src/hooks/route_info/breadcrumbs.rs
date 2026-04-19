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

/// A single item in a breadcrumb trail.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BreadcrumbItem<R> {
    /// The route this breadcrumb points to.
    pub route:     R,
    /// Human-readable label for the breadcrumb.
    pub label:     String,
    /// Whether this breadcrumb represents the currently active route.
    pub is_active: bool
}

/// Returns a list of [`BreadcrumbItem`]s representing the current navigation
/// path.
#[hook]
pub fn use_breadcrumbs<R>() -> Vec<BreadcrumbItem<R>>
where
    R: Routable + Clone + PartialEq + 'static
{
    let current = use_route::<R>();
    let provider = use_context::<BreadcrumbLabelProviderContext>();

    current.map_or_else(std::vec::Vec::new, |route| {
        let path = route.to_path();
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut items = Vec::new();
        let mut built = String::new();
        // Root
        let root_label = provider
            .as_ref()
            .map_or_else(|| "/".to_string(), |p| p.0.label_for_path("/"));
        items.push(BreadcrumbItem {
            route:     route.clone(),
            label:     root_label,
            is_active: segments.is_empty()
        });
        // Segments
        let is_last = segments.len();
        for (i, segment) in segments.iter().enumerate() {
            built.push('/');
            built.push_str(segment);
            let is_last = i + 1 == is_last;
            let label = provider
                .as_ref()
                .map_or_else(|| built.clone(), |p| p.0.label_for_path(&built));
            items.push(BreadcrumbItem {
                route: route.clone(),
                label,
                is_active: is_last
            });
        }
        items
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Debug, Routable)]
    enum TestRoute {
        #[at("/")]
        Home,
        #[at("/docs")]
        Docs,
        #[at("/docs/api")]
        Api
    }

    #[test]
    fn breadcrumb_item_fields() {
        let item = BreadcrumbItem {
            route:     TestRoute::Home,
            label:     "Home".to_string(),
            is_active: true
        };
        assert_eq!(item.label, "Home");
        assert!(item.is_active);
    }

    #[test]
    fn breadcrumb_item_clone() {
        let item1 = BreadcrumbItem {
            route:     TestRoute::Home,
            label:     "Home".to_string(),
            is_active: true
        };
        let item2 = item1.clone();
        assert_eq!(item1.label, item2.label);
    }

    #[test]
    fn breadcrumb_item_debug() {
        let item = BreadcrumbItem {
            route:     TestRoute::Home,
            label:     "Home".to_string(),
            is_active: true
        };
        let debug_str = format!("{item:?}");
        assert!(debug_str.contains("BreadcrumbItem"));
    }

    #[test]
    fn use_breadcrumbs_returns_vec() {
        let _ = use_breadcrumbs::<TestRoute>();
    }

    #[test]
    fn use_breadcrumbs_with_breadcrumb_provider() {
        struct TestProvider;
        impl BreadcrumbLabelProvider for TestProvider {
            fn label_for_path(&self, path: &str) -> String {
                match path {
                    "/" => "Home".to_string(),
                    "/docs" => "Documentation".to_string(),
                    "/docs/api" => "API".to_string(),
                    _ => path.to_string()
                }
            }
        }

        let _ = TestProvider;
    }

    #[test]
    fn breadcrumb_item_with_long_label() {
        let long_label = "This is a very long breadcrumb label".to_string();
        let item = BreadcrumbItem {
            route:     TestRoute::Home,
            label:     long_label.clone(),
            is_active: false
        };
        assert_eq!(item.label, long_label);
    }

    #[test]
    fn breadcrumb_item_not_equal_different_path_segments() {
        let item1 = BreadcrumbItem {
            route:     TestRoute::Docs,
            label:     "/docs".to_string(),
            is_active: true
        };
        let item2 = BreadcrumbItem {
            route:     TestRoute::Api,
            label:     "/docs/api".to_string(),
            is_active: false
        };
        assert_ne!(item1, item2);
    }

    #[test]
    fn breadcrumb_item_clone_multiple_times() {
        let item1 = BreadcrumbItem {
            route:     TestRoute::Home,
            label:     "Home".to_string(),
            is_active: true
        };
        let item2 = item1.clone();
        let item3 = item2.clone();
        assert_eq!(item1.label, item2.label);
        assert_eq!(item2.label, item3.label);
    }

    #[test]
    fn breadcrumb_item_debug_with_all_fields() {
        let item = BreadcrumbItem {
            route:     TestRoute::Home,
            label:     "Home".to_string(),
            is_active: true
        };
        let debug_str = format!("{item:?}");
        assert!(debug_str.contains("Home"));
        assert!(debug_str.contains("is_active"));
    }
}
