// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

/// A trait for providing custom breadcrumb labels.
pub trait BreadcrumbLabelProvider: Send + Sync {
    /// Returns a human-readable label for the given path.
    ///
    /// The path arrives percent-decoded (`/users/hello world`, not
    /// `/users/hello%20world`), so implementations match against the
    /// human-readable form.
    fn label_for_path(&self, path: &str) -> String;
}

/// Percent-decodes a path for display, keeping the raw text when the decoded
/// bytes are not valid UTF-8.
fn display_path(path: &str) -> String {
    crate::utils::percent_decode(path).unwrap_or_else(|| path.to_string())
}

/// Yew context wrapper around a [`BreadcrumbLabelProvider`].
///
/// Place an instance into the tree with `<ContextProvider<…>>` to override
/// the default path-as-label behaviour of [`use_breadcrumbs`]. Equality is
/// pointer-equality on the inner [`Rc`], so re-renders happen only when the
/// concrete provider value changes.
///
/// The inner `Rc` is **not** publicly accessible — construct via
/// [`BreadcrumbLabelProviderContext::new`] and read with
/// [`BreadcrumbLabelProviderContext::provider`]. Keeping the field private
/// lets future versions evolve the representation (e.g. a provider chain or
/// internal cache) without breaking consumers.
#[derive(Clone)]
pub struct BreadcrumbLabelProviderContext(Rc<dyn BreadcrumbLabelProvider>);

impl BreadcrumbLabelProviderContext {
    /// Wraps the given provider so it can be passed to `ContextProvider`.
    #[must_use]
    pub fn new(provider: Rc<dyn BreadcrumbLabelProvider>) -> Self {
        Self(provider)
    }

    /// Returns a clone of the inner [`Rc`] for callers that need to invoke
    /// the provider directly.
    #[must_use]
    pub fn provider(&self) -> Rc<dyn BreadcrumbLabelProvider> {
        Rc::clone(&self.0)
    }
}

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

/// Resolves `path` to its actual route, falling back to `fallback` when no
/// real route matches.
///
/// [`Routable::recognize`] substitutes the `#[not_found]` route itself
/// instead of returning `None`, so the result is accepted only when it
/// round-trips back to the requested path. Without this check an intermediate
/// breadcrumb for an unrouted prefix would link to the 404 page.
fn recognized_or<R>(path: &str, fallback: &R) -> R
where
    R: Routable + Clone
{
    R::recognize(path)
        .filter(|route| route.to_path() == path)
        .unwrap_or_else(|| fallback.clone())
}

/// Returns a list of [`BreadcrumbItem`]s representing the current navigation
/// path.
///
/// Each item's `route` is resolved from its own path prefix via
/// [`Routable::recognize`], so parent breadcrumbs navigate to their actual
/// routes. When a prefix does not correspond to any route in `R` (e.g.
/// `/users` when only `/users/:id` exists), the item falls back to the
/// current route, even when `R` declares a `#[not_found]` route.
///
/// Labels are percent-decoded: a route serialized as `/users/hello%20world`
/// yields the default label `/users/hello world`, and a
/// [`BreadcrumbLabelProvider`] receives the decoded path as well.
#[hook]
pub fn use_breadcrumbs<R>() -> Vec<BreadcrumbItem<R>>
where
    R: Routable + Clone + PartialEq + 'static
{
    let current = use_route::<R>();
    let provider = use_context::<BreadcrumbLabelProviderContext>();

    current.map_or_else(Vec::new, |route| {
        let path = route.to_path();
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut items = Vec::new();
        let mut built = String::new();
        let root_label = provider
            .as_ref()
            .map_or_else(|| "/".to_string(), |p| p.0.label_for_path("/"));
        items.push(BreadcrumbItem {
            route:     recognized_or("/", &route),
            label:     root_label,
            is_active: segments.is_empty()
        });
        let total = segments.len();
        for (i, segment) in segments.iter().enumerate() {
            built.push('/');
            built.push_str(segment);
            let is_last = i + 1 == total;
            let readable = display_path(&built);
            let label = provider
                .as_ref()
                .map_or_else(|| readable.clone(), |p| p.0.label_for_path(&readable));
            items.push(BreadcrumbItem {
                route: recognized_or(&built, &route),
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

    #[derive(Clone, PartialEq, Debug, Routable)]
    enum NotFoundRoute {
        #[at("/")]
        Home,
        #[at("/users/:id")]
        User { id: String },
        #[not_found]
        #[at("/404")]
        NotFound
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

    // ===== BreadcrumbLabelProviderContext API tests =====

    #[test]
    fn context_new_and_provider_round_trip() {
        let ctx = BreadcrumbLabelProviderContext::new(Rc::new(TestLabelProvider));
        let provider = ctx.provider();
        assert_eq!(provider.label_for_path("/docs"), "Docs");
    }

    #[test]
    fn context_provider_clones_share_the_same_rc() {
        let ctx = BreadcrumbLabelProviderContext::new(Rc::new(TestLabelProvider));
        assert!(Rc::ptr_eq(&ctx.provider(), &ctx.provider()));
    }

    // ===== display_path tests =====

    #[test]
    fn display_path_decodes_percent_sequences() {
        assert_eq!(display_path("/users/hello%20world"), "/users/hello world");
    }

    #[test]
    fn display_path_keeps_literal_plus() {
        assert_eq!(display_path("/lang/c++"), "/lang/c++");
    }

    #[test]
    fn display_path_keeps_raw_text_on_invalid_utf8() {
        assert_eq!(display_path("/bad/%FF"), "/bad/%FF");
    }

    // ===== recognized_or tests =====

    #[test]
    fn recognized_or_returns_matching_route() {
        let fallback = NotFoundRoute::Home;
        let resolved = recognized_or::<NotFoundRoute>("/users/42", &fallback);
        assert_eq!(
            resolved,
            NotFoundRoute::User {
                id: "42".to_string()
            }
        );
    }

    #[test]
    fn recognized_or_falls_back_for_unrouted_prefix_with_not_found_route() {
        let fallback = NotFoundRoute::User {
            id: "42".to_string()
        };
        let resolved = recognized_or::<NotFoundRoute>("/users", &fallback);
        assert_eq!(resolved, fallback);
    }

    #[test]
    fn recognized_or_resolves_explicit_not_found_path() {
        let fallback = NotFoundRoute::Home;
        let resolved = recognized_or::<NotFoundRoute>("/404", &fallback);
        assert_eq!(resolved, NotFoundRoute::NotFound);
    }

    #[test]
    fn recognized_or_resolves_root() {
        let fallback = NotFoundRoute::NotFound;
        let resolved = recognized_or::<NotFoundRoute>("/", &fallback);
        assert_eq!(resolved, NotFoundRoute::Home);
    }

    #[test]
    fn recognized_or_falls_back_without_not_found_route() {
        let fallback = ParamRoute::User {
            id: "7".to_string()
        };
        let resolved = recognized_or::<ParamRoute>("/users", &fallback);
        assert_eq!(resolved, fallback);
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
