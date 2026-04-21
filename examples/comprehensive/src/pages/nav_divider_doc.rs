use yew::prelude::*;
use yew_nav_link::{NavDivider, NavItem, NavLink, NavList};

use crate::{
    doc_page::{DemoCard, DocPage, Tip},
    routes::Route
};

const SRC: &str = include_str!("../../../../src/nav/divider.rs");

const CODE_BASIC: &str = "\
use yew_nav_link::{NavDivider, NavItem, NavLink, NavList};

# #[derive(Clone, PartialEq, Routable)]
# enum Route { #[at(\"/\")] Home, #[at(\"/nav-link\")] NavLinkDoc }
#[component]
fn Nav() -> Html {
    html! {
        <NavList>
            <NavItem>
                <NavLink<Route> to={Route::Home}>{ \"Home\" }</NavLink<Route>>
            </NavItem>
            <NavDivider />
            <NavItem>
                <NavLink<Route> to={Route::NavLinkDoc}>{ \"NavLink\" }</NavLink<Route>>
            </NavItem>
        </NavList>
    }
}";

const CODE_TEXT: &str = "\
use yew_nav_link::{NavDivider, NavItem, NavLink, NavList};

# #[derive(Clone, PartialEq, Routable)]
# enum Route { #[at(\"/\")] Home, #[at(\"/badge\")] BadgeDoc }
#[component]
fn Nav() -> Html {
    html! {
        <NavList>
            <NavItem>
                <NavLink<Route> to={Route::Home}>{ \"Home\" }</NavLink<Route>>
            </NavItem>
            <NavDivider text=\"Section\" />
            <NavItem>
                <NavLink<Route> to={Route::BadgeDoc}>{ \"Badge\" }</NavLink<Route>>
            </NavItem>
        </NavList>
    }
}";

#[function_component]
pub fn NavDividerDoc() -> Html {
    html! {
        <DocPage source={SRC.to_string()}>

            <DemoCard
                title="Basic Divider"
                description={html!{
                    <>
                        { "Place " }<code>{ "<NavDivider />" }</code>
                        { " between items to separate sections:" }
                    </>
                }}
                code={CODE_BASIC.to_string()}
                tip={html!{
                    <Tip>
                        { "Renders " }<code>{ "<li role=\"separator\">" }</code>
                        { " with a top border." }
                    </Tip>
                }}
            >
                <NavList>
                    <NavItem>
                        <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                    </NavItem>
                    <NavDivider />
                    <NavItem>
                        <NavLink<Route> to={Route::NavLinkDoc}>{ "NavLink" }</NavLink<Route>>
                    </NavItem>
                </NavList>
            </DemoCard>

            <DemoCard
                title="Divider with Text"
                description={html!{
                    <>
                        { "Add a label with " }<code>{ "text=\"...\"" }</code>{ ":" }
                    </>
                }}
                code={CODE_TEXT.to_string()}
                tip={html!{
                    <Tip>
                        { "Text is centered on the divider line." }
                    </Tip>
                }}
            >
                <NavList>
                    <NavItem>
                        <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                    </NavItem>
                    <NavDivider text="Section" />
                    <NavItem>
                        <NavLink<Route> to={Route::BadgeDoc}>{ "Badge" }</NavLink<Route>>
                    </NavItem>
                </NavList>
            </DemoCard>

        </DocPage>
    }
}
