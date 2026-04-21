use yew::prelude::*;
use yew_nav_link::{Match, NavLink, nav_link};

use crate::{
    doc_page::{DemoCard, DocPage, Tip},
    routes::Route
};

const NAV_LINK_SRC: &str = include_str!("../../../../src/active_link/nav_link.rs");

const CODE_EXACT: &str = "\
use yew_nav_link::NavLink;
use yew_router::prelude::*;

# #[derive(Clone, PartialEq, Routable)]
# enum Route {
#     #[at(\"/\")]
#     Home,
#     #[at(\"/nav-link\")]
#     NavLinkDoc,
#     #[at(\"/nav-list\")]
#     NavListDoc,
# }
#[component]
fn Nav() -> Html {
    html! {
        <nav>
            <NavLink<Route> to={Route::Home}>{ \"Home\" }</NavLink<Route>>
            <NavLink<Route> to={Route::NavLinkDoc}>{ \"NavLink\" }</NavLink<Route>>
            <NavLink<Route> to={Route::NavListDoc}>{ \"NavList\" }</NavLink<Route>>
        </nav>
    }
}";

const CODE_PARTIAL: &str = "\
use yew_nav_link::NavLink;
use yew_router::prelude::*;

# #[derive(Clone, PartialEq, Routable)]
# enum Route {
#     #[at(\"/\")]
#     Home,
# }
#[component]
fn Nav() -> Html {
    html! {
        <nav>
            // Stays active on /, /anything
            <NavLink<Route> to={Route::Home} partial=true>
                { \"Home (partial)\" }
            </NavLink<Route>>
        </nav>
    }
}";

const CODE_FUNCTION: &str = "\
use yew_nav_link::{nav_link, Match};

# #[derive(Clone, PartialEq, Debug, Routable)]
# enum Route {
#     #[at(\"/\")]
#     Home,
#     #[at(\"/nav-link\")]
#     NavLinkDoc,
# }
#[component]
fn Menu() -> Html {
    html! {
        <ul class=\"nav\">
            <li>{ nav_link(Route::Home, \"Home\", Match::Exact) }</li>
            <li>{ nav_link(Route::NavLinkDoc, \"Docs\", Match::Partial) }</li>
        </ul>
    }
}";

#[function_component]
pub fn NavLinkDoc() -> Html {
    html! {
        <DocPage source={NAV_LINK_SRC.to_string()}>

            <DemoCard
                title="Exact Match"
                description={html!{
                    <>
                        { "Default behavior. Active only when the URL matches exactly:" }
                    </>
                }}
                code={CODE_EXACT.to_string()}
                tip={html!{
                    <Tip>
                        { "Only the link matching the current URL has " }
                        <code>{ "class=\"nav-link active\"" }</code>
                        { ". All others have " }<code>{ "class=\"nav-link\"" }</code>{ "." }
                    </Tip>
                }}
            >
                <nav>
                    <ul class="nav-list">
                        <li class="nav-item">
                            <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                        </li>
                        <li class="nav-item">
                            <NavLink<Route> to={Route::NavLinkDoc}>{ "NavLink" }</NavLink<Route>>
                        </li>
                        <li class="nav-item">
                            <NavLink<Route> to={Route::NavListDoc}>{ "NavList" }</NavLink<Route>>
                        </li>
                    </ul>
                </nav>
            </DemoCard>

            <DemoCard
                title="Partial Match"
                description={html!{
                    <>
                        { "Use " }<code>{ "partial=true" }</code>
                        { " to keep the link active on nested routes:" }
                    </>
                }}
                code={CODE_PARTIAL.to_string()}
                tip={html!{
                    <Tip>
                        { "Compares segments: " }<code>{ "/docs" }</code>
                        { " matches " }<code>{ "/docs/api" }</code>
                        { " but NOT " }<code>{ "/documentation" }</code>{ "." }
                    </Tip>
                }}
            >
                <nav>
                    <ul class="nav-list">
                        <li class="nav-item">
                            <NavLink<Route> to={Route::Home} partial=true>
                                { "Home (partial)" }
                            </NavLink<Route>>
                        </li>
                    </ul>
                </nav>
            </DemoCard>

            <DemoCard
                title="Function Syntax"
                description={html!{
                    <>
                        { "Use " }<code>{ "nav_link()" }</code>
                        { " for text-only links without JSX:" }
                    </>
                }}
                code={CODE_FUNCTION.to_string()}
                tip={html!{
                    <Tip>
                        { "Third argument: " }<code>{ "Match::Exact" }</code>
                        { " or " }<code>{ "Match::Partial" }</code>{ "." }
                    </Tip>
                }}
            >
                <nav>
                    <ul class="nav-list">
                        <li class="nav-item">
                            { nav_link(Route::Home, "Home (Exact)", Match::Exact) }
                        </li>
                        <li class="nav-item">
                            { nav_link(Route::NavLinkDoc, "NavLink (Exact)", Match::Exact) }
                        </li>
                    </ul>
                </nav>
            </DemoCard>

        </DocPage>
    }
}
