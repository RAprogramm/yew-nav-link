use crate::doc_page::{DemoCard, DocPage, Tip};
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{NavItem, NavLink, NavList};

const SRC: &str = include_str!("../../../../src/nav/list.rs");

const CODE_BASIC: &str = "\
use yew_nav_link::{NavLink, NavList};

# #[derive(Clone, PartialEq, Routable)]
# enum Route { #[at(\"/\")] Home, #[at(\"/nav-link\")] NavLinkDoc, #[at(\"/nav-list\")] NavListDoc }
#[component]
fn Nav() -> Html {
    html! {
        <NavList aria_label=\"Main Navigation\">
            <NavLink<Route> to={Route::Home}>{ \"Home\" }</NavLink<Route>>
            <NavLink<Route> to={Route::NavLinkDoc}>{ \"NavLink\" }</NavLink<Route>>
            <NavLink<Route> to={Route::NavListDoc}>{ \"NavList\" }</NavLink<Route>>
        </NavList>
    }
}";

const CODE_DISABLED: &str = "\
use yew_nav_link::{NavItem, NavLink, NavList};

# #[derive(Clone, PartialEq, Routable)]
# enum Route { #[at(\"/\")] Home }
#[component]
fn Nav() -> Html {
    html! {
        <NavList>
            <NavItem>
                <NavLink<Route> to={Route::Home}>{ \"Enabled\" }</NavLink<Route>>
            </NavItem>
            <NavItem disabled=true>
                <NavLink<Route> to={Route::Home}>{ \"Disabled\" }</NavLink<Route>>
            </NavItem>
        </NavList>
    }
}";

#[function_component]
pub fn NavListDoc() -> Html {
    html! {
        <DocPage source={SRC.to_string()}>

            <DemoCard
                title="Basic Usage"
                description={html!{
                    <>
                        { "Wrap " }<code>{ "<NavLink>" }</code>
                        { " items inside " }<code>{ "<NavList>" }</code>{ ":" }
                    </>
                }}
                code={CODE_BASIC.to_string()}
                tip={html!{
                    <Tip>
                        { "Renders " }<code>{ "<ul role=\"list\" aria-label=\"...\">" }</code>
                        { ". Each child gets " }<code>{ "<li role=\"listitem\">" }</code>{ "." }
                    </Tip>
                }}
            >
                <NavList aria_label="Main Navigation">
                    <NavItem>
                        <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                    </NavItem>
                    <NavItem>
                        <NavLink<Route> to={Route::NavLinkDoc}>{ "NavLink" }</NavLink<Route>>
                    </NavItem>
                    <NavItem>
                        <NavLink<Route> to={Route::NavListDoc}>{ "NavList" }</NavLink<Route>>
                    </NavItem>
                </NavList>
            </DemoCard>

            <DemoCard
                title="Disabled Items"
                description={html!{
                    <>
                        { "Use " }<code>{ "disabled=true" }</code>
                        { " on " }<code>{ "<NavItem>" }</code>{ ":" }
                    </>
                }}
                code={CODE_DISABLED.to_string()}
                tip={html!{
                    <Tip>
                        { "Gets " }<code>{ "class=\"nav-item disabled\"" }</code>{ "." }
                    </Tip>
                }}
            >
                <NavList>
                    <NavItem>
                        <NavLink<Route> to={Route::Home}>{ "Enabled" }</NavLink<Route>>
                    </NavItem>
                    <NavItem disabled=true>
                        <NavLink<Route> to={Route::NotFound}>{ "Disabled" }</NavLink<Route>>
                    </NavItem>
                </NavList>
            </DemoCard>

        </DocPage>
    }
}
