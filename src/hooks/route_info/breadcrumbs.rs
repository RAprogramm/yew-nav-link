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
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
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
    enum SimpleRoute {
        #[at("/")]
        Home,
        #[at("/about")]
        About,
        #[at("/docs")]
        Docs,
        #[at("/docs/api")]
        Api,
        #[at("/docs/api/v1")]
        ApiV1
    }

    #[derive(Clone, PartialEq, Debug, Routable)]
    enum ParamRoute {
        #[at("/")]
        Home,
        #[at("/users/:id")]
        User { id: String }
    }

    #[derive(Clone, PartialEq, Debug, Routable)]
    enum RootOnlyRoute {
        #[at("/")]
        Root
    }

    struct TestLabelProvider;

    impl BreadcrumbLabelProvider for TestLabelProvider {
        fn label_for_path(&self, path: &str) -> String {
            match path {
                "/" => "Home".to_string(),
                "/about" => "About".to_string(),
                "/docs" => "Docs".to_string(),
                "/docs/api" => "API".to_string(),
                "/docs/api/v1" => "V1".to_string(),
                "/users/42" => "User #42".to_string(),
                _ => path.to_string()
            }
        }
    }

    // ===== BreadcrumbItem tests =====

    #[test]
    fn breadcrumb_item_new() {
        let item = BreadcrumbItem {
            route:     SimpleRoute::Home,
            label:     "Home".to_string(),
            is_active: true
        };
        assert_eq!(item.label, "Home");
        assert!(item.is_active);
        assert_eq!(item.route.to_path(), "/");
    }

    #[test]
    fn breadcrumb_item_inactive() {
        let item = BreadcrumbItem {
            route:     SimpleRoute::About,
            label:     "About".to_string(),
            is_active: false
        };
        assert!(!item.is_active);
        assert_eq!(item.label, "About");
    }

    #[test]
    fn breadcrumb_item_clone_preserves_all_fields() {
        let item1 = BreadcrumbItem {
            route:     SimpleRoute::Api,
            label:     "Root".to_string(),
            is_active: true
        };
        let item2 = item1.clone();
        assert_eq!(item1, item2);
        assert_eq!(item2.label, "Root");
        assert!(item2.is_active);
    }

    #[test]
    fn breadcrumb_item_eq_with_same_values() {
        let item1 = BreadcrumbItem {
            route:     SimpleRoute::Home,
            label:     "Home".to_string(),
            is_active: true
        };
        let item2 = BreadcrumbItem {
            route:     SimpleRoute::Home,
            label:     "Home".to_string(),
            is_active: true
        };
        assert_eq!(item1, item2);
    }

    #[test]
    fn breadcrumb_item_neq_different_label() {
        let item1 = BreadcrumbItem {
            route:     SimpleRoute::Home,
            label:     "Home".to_string(),
            is_active: true
        };
        let item2 = BreadcrumbItem {
            route:     SimpleRoute::Home,
            label:     "Index".to_string(),
            is_active: true
        };
        assert_ne!(item1, item2);
    }

    #[test]
    fn breadcrumb_item_neq_different_state() {
        let item1 = BreadcrumbItem {
            route:     SimpleRoute::Home,
            label:     "Home".to_string(),
            is_active: true
        };
        let item2 = BreadcrumbItem {
            route:     SimpleRoute::Home,
            label:     "Home".to_string(),
            is_active: false
        };
        assert_ne!(item1, item2);
    }

    #[test]
    fn breadcrumb_item_neq_different_route() {
        let item1 = BreadcrumbItem {
            route:     SimpleRoute::Docs,
            label:     "Docs".to_string(),
            is_active: false
        };
        let item2 = BreadcrumbItem {
            route:     SimpleRoute::Api,
            label:     "Docs".to_string(),
            is_active: false
        };
        assert_ne!(item1, item2);
    }

    #[test]
    fn breadcrumb_item_debug_contains_all_fields() {
        let item = BreadcrumbItem {
            route:     SimpleRoute::Home,
            label:     "Home".to_string(),
            is_active: true
        };
        let debug_str = format!("{item:?}");
        assert!(debug_str.contains("BreadcrumbItem"));
        assert!(debug_str.contains("Home"));
        assert!(debug_str.contains("is_active"));
    }

    #[test]
    fn breadcrumb_item_long_label() {
        let label = "Extremely long breadcrumb label to test string handling in various scenarios"
            .to_string();
        let item = BreadcrumbItem {
            route:     SimpleRoute::Home,
            label:     label.clone(),
            is_active: false
        };
        assert_eq!(item.label, label);
        assert!(!item.is_active);
    }

    #[test]
    fn breadcrumb_item_short_label() {
        let item = BreadcrumbItem {
            route:     SimpleRoute::Home,
            label:     "a".to_string(),
            is_active: true
        };
        assert_eq!(item.label, "a");
    }

    #[test]
    fn breadcrumb_item_clone_deep_copy() {
        let item1 = BreadcrumbItem {
            route:     SimpleRoute::ApiV1,
            label:     "Deep".to_string(),
            is_active: true
        };
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[test]
    fn breadcrumb_item_root_path() {
        let item = BreadcrumbItem {
            route:     SimpleRoute::Home,
            label:     "/".to_string(),
            is_active: true
        };
        assert_eq!(item.route.to_path(), "/");
    }

    #[test]
    fn breadcrumb_item_nested_path() {
        let item = BreadcrumbItem {
            route:     SimpleRoute::ApiV1,
            label:     "/docs/api/v1".to_string(),
            is_active: true
        };
        assert_eq!(item.route.to_path(), "/docs/api/v1");
    }

    // ===== BreadcrumbLabelProvider tests =====

    #[test]
    fn breadcrumb_label_provider_returns_custom_labels() {
        let provider = TestLabelProvider;
        assert_eq!(provider.label_for_path("/"), "Home");
        assert_eq!(provider.label_for_path("/about"), "About");
        assert_eq!(provider.label_for_path("/docs/api/v1"), "V1");
    }

    #[test]
    fn breadcrumb_label_provider_returns_path_for_unknown() {
        let provider = TestLabelProvider;
        assert_eq!(provider.label_for_path("/unknown/path"), "/unknown/path");
        assert_eq!(provider.label_for_path("/missing"), "/missing");
    }

    #[test]
    fn breadcrumb_label_provider_empty_path_not_root() {
        let provider = TestLabelProvider;
        assert_ne!(provider.label_for_path(""), "Home");
    }

    #[test]
    fn breadcrumb_label_provider_whitespace() {
        let provider = TestLabelProvider;
        assert_eq!(provider.label_for_path("   "), "   ");
    }

    #[test]
    fn breadcrumb_label_provider_special_chars() {
        let provider = TestLabelProvider;
        assert_eq!(provider.label_for_path("@#$%"), "@#$%");
    }

    // ===== BreadcrumbLabelProviderContext tests =====

    #[test]
    fn context_eq_same_rc() {
        let rc = Rc::new(TestLabelProvider);
        let ctx1 = BreadcrumbLabelProviderContext(rc.clone());
        let ctx2 = BreadcrumbLabelProviderContext(rc);
        assert!(ctx1 == ctx2);
    }

    #[test]
    fn context_neq_different_rc() {
        let ctx1 = BreadcrumbLabelProviderContext(Rc::new(TestLabelProvider));
        let ctx2 = BreadcrumbLabelProviderContext(Rc::new(TestLabelProvider));
        assert!(ctx1 != ctx2);
    }

    #[test]
    fn context_clone_preserves_identity() {
        let rc = Rc::new(TestLabelProvider);
        let ctx1 = BreadcrumbLabelProviderContext(rc);
        let ctx2 = ctx1.clone();
        assert!(ctx1 == ctx2);
    }

    // ===== use_breadcrumbs tests =====

    #[test]
    fn use_breadcrumbs_simple_route() {
        let _result = use_breadcrumbs::<SimpleRoute>();
    }

    #[test]
    fn use_breadcrumbs_param_route() {
        let _result = use_breadcrumbs::<ParamRoute>();
    }

    #[test]
    fn use_breadcrumbs_multiple_calls() {
        let _r1 = use_breadcrumbs::<SimpleRoute>();
        let _r2 = use_breadcrumbs::<SimpleRoute>();
        let _r3 = use_breadcrumbs::<SimpleRoute>();
    }

    #[test]
    fn use_breadcrumbs_root_only_route() {
        let _result = use_breadcrumbs::<RootOnlyRoute>();
    }

    #[test]
    fn use_breadcrumbs_all_route_types() {
        let _simple = use_breadcrumbs::<SimpleRoute>();
        let _param = use_breadcrumbs::<ParamRoute>();
        let _root = use_breadcrumbs::<RootOnlyRoute>();
    }

    // ===== Negative tests =====

    #[test]
    fn breadcrumb_item_neq_negatives() {
        let item1 = BreadcrumbItem {
            route:     SimpleRoute::Home,
            label:     "Home".to_string(),
            is_active: true
        };
        let mut item2 = item1.clone();
        item2.label = "Other".to_string();
        assert_ne!(item1, item2);
        item2.is_active = false;
        assert_ne!(item1, item2);
    }
}
