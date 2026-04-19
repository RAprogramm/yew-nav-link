use yew_router::prelude::*;

#[derive(Clone, PartialEq, Debug, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/nav-link")]
    NavLinkDoc,
    #[at("/nav-list")]
    NavListDoc,
    #[at("/nav-divider")]
    NavDividerDoc,
    #[at("/badge")]
    BadgeDoc,
    #[at("/dropdown")]
    DropdownDoc,
    #[at("/icon")]
    IconDoc,
    #[at("/tabs")]
    TabsDoc,
    #[at("/pagination")]
    PaginationDoc,
    #[at("/hooks")]
    HooksDoc,
    #[at("/breadcrumbs")]
    BreadcrumbsDoc,
    #[at("/utils")]
    UtilsDoc,
    #[at("/examples")]
    ExamplesHome,
    #[at("/examples/basic")]
    BasicExample,
    #[at("/examples/bootstrap")]
    BootstrapExample,
    #[at("/examples/tailwind")]
    TailwindExample,
    #[at("/examples/nested-routes")]
    NestedRoutesExample,
    #[at("/features/custom-css")]
    CustomCssFeatures,
    #[at("/features/navigation-hooks")]
    NavigationHooks,
    #[at("/features/custom-breadcrumbs")]
    CustomBreadcrumbs,
    #[not_found]
    #[at("/404")]
    NotFound,
}
