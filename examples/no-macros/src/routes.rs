//! Application routes.

use yew_router::prelude::*;

#[derive(Clone, PartialEq, Debug, Routable)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/links")]
    Links,
    #[at("/lists")]
    Lists,
    #[at("/container")]
    Container,
    #[at("/hooks")]
    Hooks,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// Local routes for examples - used in components
#[derive(Clone, PartialEq, Debug, Routable)]
pub enum LinksRoute {
    #[at("/links")]
    Home,
    #[at("/links/about")]
    About,
    #[at("/links/contact")]
    Contact,
}

#[derive(Clone, PartialEq, Debug, Routable)]
pub enum ListsRoute {
    #[at("/lists")]
    Home,
    #[at("/lists/products")]
    Products,
    #[at("/lists/pricing")]
    Pricing,
}

#[derive(Clone, PartialEq, Debug, Routable)]
pub enum ContainerRoute {
    #[at("/container")]
    Home,
    #[at("/container/products")]
    Products,
    #[at("/container/pricing")]
    Pricing,
}

#[derive(Clone, PartialEq, Debug, Routable)]
pub enum HookRoute {
    #[at("/hooks")]
    Home,
    #[at("/hooks/about")]
    About,
    #[at("/hooks/services")]
    Services,
}
