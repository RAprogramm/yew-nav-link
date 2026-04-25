//! Documentation metadata for auto-generation.
//!
//! Centralized metadata about all library components and hooks.
//! Used for generating consistent documentation pages and API references.

use crate::routes::Route;

// ── Component Metadata ─────────────────────────────────────────

#[derive(Clone)]
pub struct ComponentInfo {
    pub name:        &'static str,
    pub description: &'static str,
    pub route:       Route,
    pub since:       &'static str,
    pub features:    &'static [&'static str]
}

impl ComponentInfo {
    pub fn new(
        name: &'static str,
        description: &'static str,
        route: Route,
        since: &'static str
    ) -> Self {
        Self {
            name,
            description,
            route,
            since,
            features: &[]
        }
    }

    pub fn with_features(mut self, features: &'static [&'static str]) -> Self {
        self.features = features;
        self
    }
}

pub fn all_components() -> &'static [ComponentInfo] {
    Box::leak(
        vec![
            ComponentInfo::new(
                "NavLink",
                "Auto-active navigation link with exact and partial matching",
                Route::NavLinkDoc,
                "0.1.0"
            ),
            ComponentInfo::new(
                "NavList",
                "Semantic unordered list wrapper for navigation with ARIA support",
                Route::NavListDoc,
                "0.1.0"
            ),
            ComponentInfo::new(
                "NavDivider",
                "Visual separator for navigation items with optional label",
                Route::NavDividerDoc,
                "0.1.0"
            ),
            ComponentInfo::new(
                "Badge",
                "Small badge indicator for counts or status",
                Route::BadgeDoc,
                "0.1.0"
            ),
            ComponentInfo::new(
                "NavDropdown",
                "Dropdown menu component for navigation",
                Route::DropdownDoc,
                "0.1.0"
            ),
            ComponentInfo::new(
                "NavIcon",
                "Icon wrapper with size variants for navigation items",
                Route::IconDoc,
                "0.1.0"
            ),
            ComponentInfo::new(
                "NavTabs",
                "Tab navigation component with ARIA support",
                Route::TabsDoc,
                "0.1.0"
            ),
            ComponentInfo::new(
                "Pagination",
                "Page navigation component with ellipsis support",
                Route::PaginationDoc,
                "0.1.0"
            ),
        ]
        .into_boxed_slice()
    )
}

// ── Hook Metadata ──────────────────────────────────────────────

#[derive(Clone)]
pub struct HookInfo {
    pub name:        &'static str,
    pub description: &'static str,
    pub route:       Route,
    pub since:       &'static str,
    pub return_type: &'static str
}

impl HookInfo {
    pub fn new(
        name: &'static str,
        description: &'static str,
        route: Route,
        since: &'static str,
        return_type: &'static str
    ) -> Self {
        Self {
            name,
            description,
            route,
            since,
            return_type
        }
    }
}

pub fn all_hooks() -> &'static [HookInfo] {
    Box::leak(
        vec![
            HookInfo::new(
                "use_route_info",
                "Get current route information including path and params",
                Route::HooksDoc,
                "0.1.0",
                "RouteInfo<R>"
            ),
            HookInfo::new(
                "use_is_active",
                "Check if current route matches a target route exactly",
                Route::HooksDoc,
                "0.1.0",
                "bool"
            ),
            HookInfo::new(
                "use_is_partial_active",
                "Check if current route starts with a target route",
                Route::HooksDoc,
                "0.1.0",
                "bool"
            ),
            HookInfo::new(
                "use_navigation",
                "Get navigation controller for programmatic routing",
                Route::NavigationHooks,
                "0.8.0",
                "Navigation<R>"
            ),
            HookInfo::new(
                "use_route_params",
                "Extract route parameters from current route",
                Route::NavigationHooks,
                "0.8.0",
                "RouteParams"
            ),
            HookInfo::new(
                "use_query_params",
                "Parse query string parameters from current URL",
                Route::NavigationHooks,
                "0.8.0",
                "QueryParams"
            ),
        ]
        .into_boxed_slice()
    )
}

// ── Example Metadata ───────────────────────────────────────────

#[derive(Clone)]
pub struct ExampleInfo {
    pub name:        &'static str,
    pub description: &'static str,
    pub route:       Route,
    pub category:    ExampleCategory
}

#[derive(Clone, Copy, PartialEq)]
pub enum ExampleCategory {
    Basic,
    CssFramework,
    Advanced
}

impl ExampleCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExampleCategory::Basic => "Basic",
            ExampleCategory::CssFramework => "CSS Framework",
            ExampleCategory::Advanced => "Advanced"
        }
    }
}

impl ExampleInfo {
    pub fn new(
        name: &'static str,
        description: &'static str,
        route: Route,
        category: ExampleCategory
    ) -> Self {
        Self {
            name,
            description,
            route,
            category
        }
    }
}

pub fn all_examples() -> &'static [ExampleInfo] {
    Box::leak(
        vec![
            ExampleInfo::new(
                "Basic",
                "Simple 3-page app demonstrating component and function syntax",
                Route::BasicExample,
                ExampleCategory::Basic
            ),
            ExampleInfo::new(
                "Bootstrap 5",
                "Zero-config compatibility with Bootstrap 5 classes",
                Route::BootstrapExample,
                ExampleCategory::CssFramework
            ),
            ExampleInfo::new(
                "Tailwind CSS",
                "Dashboard sidebar with @apply directives",
                Route::TailwindExample,
                ExampleCategory::CssFramework
            ),
            ExampleInfo::new(
                "Nested Routes",
                "Multi-level navigation with partial matching",
                Route::NestedRoutesExample,
                ExampleCategory::Advanced
            ),
        ]
        .into_boxed_slice()
    )
}

// ── Feature Metadata ───────────────────────────────────────────

#[derive(Clone)]
pub struct FeatureInfo {
    pub name:        &'static str,
    pub description: &'static str,
    pub default:     bool,
    pub since:       &'static str
}

pub fn all_features() -> &'static [FeatureInfo] {
    Box::leak(
        vec![FeatureInfo {
            name:        "default",
            description: "Default feature set including core functionality",
            default:     true,
            since:       "0.1.0"
        }]
        .into_boxed_slice()
    )
}

// ── Version Metadata ───────────────────────────────────────────

pub fn current_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn yew_version() -> &'static str {
    "0.23"
}

pub fn yew_router_version() -> &'static str {
    "0.20"
}

pub fn msrv() -> &'static str {
    "1.75"
}
