use yew::prelude::*;
use yew_nav_link::{NavBadge, NavHeader, NavItem, NavLink, NavList, NavText};

use crate::{
    doc_page::{DemoCard, DocPage, Tip},
    routes::Route
};

const SRC: &str = include_str!("../../../src/components/badge.rs");

const CODE_BADGE: &str = "\
use yew_nav_link::{NavBadge, NavItem, NavLink, NavList};

# #[derive(Clone, PartialEq, Routable)]
# enum Route {
#     #[at(\"/\")]
#     Home,
# }
#[component]
fn Nav() -> Html {
    html! {
        <NavList>
            <NavItem>
                <NavLink<Route> to={Route::Home}>
                    { \"Messages \" }
                    <NavBadge variant=\"danger\">{ \"3\" }</NavBadge>
                </NavLink<Route>>
            </NavItem>
            <NavItem>
                <NavLink<Route> to={Route::Home}>
                    { \"Tasks \" }
                    <NavBadge variant=\"success\" pill=true>{ \"12\" }</NavBadge>
                </NavLink<Route>>
            </NavItem>
        </NavList>
    }
}";

const CODE_HEADER: &str = "\
use yew_nav_link::{NavHeader, NavItem, NavLink, NavList};

# #[derive(Clone, PartialEq, Routable)]
# enum Route { #[at(\"/\")] Home, #[at(\"/nav-link\")] NavLinkDoc }
#[component]
fn Nav() -> Html {
    html! {
        <NavList>
            <NavHeader text=\"Main\" />
            <NavItem><NavLink<Route> to={Route::Home}>{ \"Home\" }</NavLink<Route>></NavItem>
            <NavHeader text=\"Docs\" />
            <NavItem><NavLink<Route> to={Route::NavLinkDoc}>{ \"NavLink\" }</NavLink<Route>></NavItem>
        </NavList>
    }
}";

#[function_component]
pub fn BadgeDoc() -> Html {
    html! {
        <DocPage source={SRC.to_string()}>

            <DemoCard
                title="NavBadge"
                description={html!{
                    <>
                        { "Place " }<code>{ "<NavBadge>" }</code>
                        { " inside a link to show counts or status:" }
                    </>
                }}
                code={CODE_BADGE.to_string()}
                tip={html!{
                    <Tip>
                        { "Variants: " }<code>{ "primary" }</code>{ ", " }<code>{ "danger" }</code>
                        { ", " }<code>{ "success" }</code>{ ", " }<code>{ "warning" }</code>
                        { ". Use " }<code>{ "pill=true" }</code>{ " for rounded shape." }
                    </Tip>
                }}
            >
                <NavList>
                    <NavItem>
                        <NavLink<Route> to={Route::Home}>
                            { "Messages " }
                            <NavBadge variant="danger">{ "3" }</NavBadge>
                        </NavLink<Route>>
                    </NavItem>
                    <NavItem>
                        <NavLink<Route> to={Route::Home}>
                            { "Tasks " }
                            <NavBadge variant="success" pill=true>{ "12" }</NavBadge>
                        </NavLink<Route>>
                    </NavItem>
                    <NavItem>
                        <NavLink<Route> to={Route::Home}>
                            { "Alerts " }
                            <NavBadge variant="warning">{ "!" }</NavBadge>
                        </NavLink<Route>>
                    </NavItem>
                </NavList>
            </DemoCard>

            <DemoCard
                title="NavHeader"
                description={html!{ "Label sections inside a navigation list:" }}
                code={CODE_HEADER.to_string()}
                tip={html!{
                    <Tip>
                        { "Renders " }<code>{ "<li role=\"presentation\">" }</code>
                        { " with " }<code>{ "class=\"nav-header\"" }</code>{ "." }
                    </Tip>
                }}
            >
                <NavList>
                    <NavHeader text="Main" />
                    <NavItem><NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>></NavItem>
                    <NavHeader text="Documentation" />
                    <NavItem><NavLink<Route> to={Route::NavLinkDoc}>{ "NavLink" }</NavLink<Route>></NavItem>
                </NavList>
            </DemoCard>

            <DemoCard
                title="NavText"
                description={html!{
                    <>
                        { "Static text label, useful for version info:" }
                    </>
                }}
                code={"use yew_nav_link::NavText;\n\nhtml! { <NavText text=\"v0.6.0\" /> }".to_string()}
                tip={html! { <></> }}
            >
                <NavList>
                    <NavItem><NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>></NavItem>
                    <NavText text="v0.6.0" />
                </NavList>
            </DemoCard>

        </DocPage>
    }
}
