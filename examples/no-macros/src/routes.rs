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
    #[not_found]
    #[at("/404")]
    NotFound,
}
