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

fn provider_label_or_default(
    provider: Option<&BreadcrumbLabelProviderContext>,
    path: &str,
) -> String {
    provider
        .map(|p| p.0.label_for_path(path))
        .unwrap_or_else(|| path.to_string())
}

fn build_breadcrumb_items<R>(
    route: &R,
    path: &str,
    provider: Option<&BreadcrumbLabelProviderContext>,
) -> Vec<BreadcrumbItem<R>>
where
    R: Routable + Clone + PartialEq + 'static,
{
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    let mut items = Vec::new();
    let mut built = String::new();

    items.push(BreadcrumbItem {
        route: route.clone(),
        label: provider_label_or_default(provider, "/"),
        is_active: segments.is_empty(),
    });

    let segments_len = segments.len();
    for (index, segment) in segments.iter().enumerate() {
        built.push('/');
        built.push_str(segment);
        items.push(BreadcrumbItem {
            route: route.clone(),
            label: provider_label_or_default(provider, &built),
            is_active: index + 1 == segments_len,
        });
    }

    items
}

/// A single item in a breadcrumb trail.
#[derive(Clone, Debug)]
pub struct BreadcrumbItem<R> {
    /// The route this breadcrumb points to.
    pub route: R,
    /// Human-readable label for the breadcrumb.
    pub label: String,
    /// Whether this breadcrumb represents the currently active route.
    pub is_active: bool,
}

/// Returns a list of [`BreadcrumbItem`]s representing the current navigation
/// path.
#[hook]
pub fn use_breadcrumbs<R>() -> Vec<BreadcrumbItem<R>>
where
    R: Routable + Clone + PartialEq + 'static,
{
    let current = use_route::<R>();
    let provider = use_context::<BreadcrumbLabelProviderContext>();

    match current {
        Some(route) => {
            let path = route.to_path();
            build_breadcrumb_items(&route, &path, provider.as_ref())
        }
        None => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Routable)]
    enum TestRoute {
        #[at("/")]
        Home,
        #[at("/docs/api")]
        DocsApi,
    }

    struct StaticProvider;

    impl BreadcrumbLabelProvider for StaticProvider {
        fn label_for_path(&self, path: &str) -> String {
            match path {
                "/" => "Root".to_string(),
                "/docs" => "Docs".to_string(),
                "/docs/api" => "API".to_string(),
                other => other.to_string(),
            }
        }
    }

    #[test]
    fn provider_label_falls_back_to_path() {
        assert_eq!(super::provider_label_or_default(None, "/docs"), "/docs");
    }

    #[test]
    fn provider_label_uses_custom_provider() {
        let provider = BreadcrumbLabelProviderContext(Rc::new(StaticProvider));
        assert_eq!(
            super::provider_label_or_default(Some(&provider), "/docs"),
            "Docs"
        );
    }

    #[test]
    fn build_breadcrumb_items_for_root_path() {
        let items = super::build_breadcrumb_items(&TestRoute::Home, "/", None);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].label, "/");
        assert!(items[0].is_active);
    }

    #[test]
    fn build_breadcrumb_items_for_nested_path() {
        let provider = BreadcrumbLabelProviderContext(Rc::new(StaticProvider));
        let items =
            super::build_breadcrumb_items(&TestRoute::DocsApi, "/docs/api", Some(&provider));

        assert_eq!(items.len(), 3);
        assert_eq!(items[0].label, "Root");
        assert!(!items[0].is_active);
        assert_eq!(items[1].label, "Docs");
        assert!(!items[1].is_active);
        assert_eq!(items[2].label, "API");
        assert!(items[2].is_active);
    }
}
