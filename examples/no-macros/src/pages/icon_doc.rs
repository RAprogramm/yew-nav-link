use crate::doc_page::{DemoCard, DocPage, Tip};
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{
    components::{NavIcon, NavIconSize, NavLinkWithIcon},
    NavItem, NavLink, NavList,
};

const SRC: &str = include_str!("../../../../src/components/icon.rs");

const CODE_ICONS: &str = "\
use yew_nav_link::components::{NavIcon, NavIconSize};
use yew_nav_link::{NavItem, NavLink, NavList};

# #[derive(Clone, PartialEq, Routable)]
# enum Route { #[at(\"/\")] Home }
#[component]
fn Nav() -> Html {
    html! {
        <NavList>
            <NavItem>
                <NavLink<Route> to={Route::Home}>
                    <NavIcon name=\"home\" size={NavIconSize::Small} />
                    { \" Small\" }
                </NavLink<Route>>
            </NavItem>
            <NavItem>
                <NavLink<Route> to={Route::Home}>
                    <NavIcon name=\"gear\" size={NavIconSize::Medium} />
                    { \" Medium\" }
                </NavLink<Route>>
            </NavItem>
        </NavList>
    }
}";

#[function_component]
pub fn IconDoc() -> Html {
    html! {
        <DocPage source={SRC.to_string()}>

            <DemoCard
                title="Icon Sizes"
                description={html!{
                    <>
                        { "Three sizes: " }<code>{ "Small" }</code>{ ", " }
                        <code>{ "Medium" }</code>{ ", " }<code>{ "Large" }</code>{ ":" }
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
                            <NavIcon name="home" size={NavIconSize::Small} />
                            { " Small icon" }
                        </NavLink<Route>>
                    </NavItem>
                    <NavItem>
                        <NavLink<Route> to={Route::Home}>
                            <NavIcon name="gear" size={NavIconSize::Medium} />
                            { " Medium icon (default)" }
                        </NavLink<Route>>
                    </NavItem>
                    <NavItem>
                        <NavLink<Route> to={Route::Home}>
                            <NavIcon name="user" size={NavIconSize::Large} />
                            { " Large icon" }
                        </NavLink<Route>>
                    </NavItem>
                </NavList>
            </DemoCard>

            <DemoCard
                title="NavLinkWithIcon"
                description={html!{ "Wrap icon + text together:" }}
                code={"use yew_nav_link::components::{NavLinkWithIcon, NavIcon, NavIconSize};\n\nhtml! { <NavLinkWithIcon icon={NavIconSize::Medium}><NavIcon name=\"star\" />{ \" Featured\" }</NavLinkWithIcon> }".to_string()}
                tip={html! { <></> }}
            >
                <NavLinkWithIcon icon={NavIconSize::Medium}>
                    <NavIcon name="star" />
                    { " Featured" }
                </NavLinkWithIcon>
            </DemoCard>

        </DocPage>
    }
}
