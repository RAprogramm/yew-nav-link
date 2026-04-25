use yew::prelude::*;
use yew_nav_link::{
    components::{NavIcon, NavIconSize, NavLinkWithIcon},
    NavItem, NavLink, NavList
};

use crate::{
    doc_page::{DemoCard, DocPage, Tip},
    routes::Route
};

const SRC: &str = include_str!("../../../src/components/icon.rs");

const CODE_ICONS: &str = "\
use yew_nav_link::components::{NavIcon, NavIconSize};
use yew_nav_link::{NavItem, NavLink, NavList};

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
                    <NavIcon size={NavIconSize::Small}>{ \"▲\" }</NavIcon>
                    { \" Small\" }
                </NavLink<Route>>
            </NavItem>
            <NavItem>
                <NavLink<Route> to={Route::Home}>
                    <NavIcon size={NavIconSize::Medium}>{ \"▲\" }</NavIcon>
                    { \" Medium\" }
                </NavLink<Route>>
            </NavItem>
            <NavItem>
                <NavLink<Route> to={Route::Home}>
                    <NavIcon size={NavIconSize::Large}>{ \"▲\" }</NavIcon>
                    { \" Large\" }
                </NavLink<Route>>
            </NavItem>
        </NavList>
    }
}";

const CODE_WITH_ICON: &str = "\
use yew_nav_link::components::{NavLinkWithIcon, NavIcon, NavIconSize};

html! {
    <NavLinkWithIcon icon={NavIconSize::Medium}>
        <NavIcon>{ \"★\" }</NavIcon>
        { \" Featured\" }
    </NavLinkWithIcon>
}";

#[function_component]
pub fn IconDoc() -> Html {
    html! {
        <DocPage source={SRC.to_string()}>

            <DemoCard
                title="Icon Sizes"
                description={html!{
                    <>
                        { "Three sizes: " }<code>{ "Small" }</code>{ " (0.75rem), " }
                        <code>{ "Medium" }</code>{ " (1.5rem), " }<code>{ "Large" }</code>
                        { " (2.25rem). Pass any content as children:" }
                    </>
                }}
                code={CODE_ICONS.to_string()}
                tip={html!{
                    <Tip>
                        { "Renders " }<code>{ "<i aria-hidden=\"true\">" }</code>
                        { " with " }<code>{ "nav-icon-sm" }</code>{ "/" }
                        <code>{ "nav-icon-md" }</code>{ "/" }<code>{ "nav-icon-lg" }</code>{ "." }
                    </Tip>
                }}
            >
                <NavList>
                    <NavItem>
                        <NavLink<Route> to={Route::Home}>
                            <NavIcon size={NavIconSize::Small}>{ "▲" }</NavIcon>
                            { " Small icon" }
                        </NavLink<Route>>
                    </NavItem>
                    <NavItem>
                        <NavLink<Route> to={Route::Home}>
                            <NavIcon size={NavIconSize::Medium}>{ "▲" }</NavIcon>
                            { " Medium icon (default)" }
                        </NavLink<Route>>
                    </NavItem>
                    <NavItem>
                        <NavLink<Route> to={Route::Home}>
                            <NavIcon size={NavIconSize::Large}>{ "▲" }</NavIcon>
                            { " Large icon" }
                        </NavLink<Route>>
                    </NavItem>
                </NavList>
            </DemoCard>

            <DemoCard
                title="NavLinkWithIcon"
                description={html!{ "Wrap icon + text together:" }}
                code={CODE_WITH_ICON.to_string()}
                tip={html! { <></> }}
            >
                <NavLinkWithIcon icon={NavIconSize::Medium}>
                    <NavIcon>{ "★" }</NavIcon>
                    { " Featured" }
                </NavLinkWithIcon>
            </DemoCard>

        </DocPage>
    }
}
