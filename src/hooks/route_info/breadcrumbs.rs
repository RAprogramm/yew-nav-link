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
#[derive(Clone, Debug)]
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

            // Segments
            let is_last = segments.len();
            for (i, segment) in segments.iter().enumerate() {
                built.push('/');
                built.push_str(segment);
                let is_last = i + 1 == is_last;
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
