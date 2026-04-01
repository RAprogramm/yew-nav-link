//! Container components example - NavList, NavItem, NavDivider.

use yew::prelude::*;
use yew_nav_link::{NavLink, NavList, NavItem, NavDivider};
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Debug, Routable)]
enum ContainerRoute {
    #[at("/")]
    Home,
    #[at("/products")]
    Products,
    #[at("/pricing")]
    Pricing,
}

#[component]
pub fn Container() -> Html {
    html! {
        <div class="container-page">
            <h2>{ "Container Components" }</h2>
            <div class="card">
                <h3>{ "NavList Container" }</h3>
                <p>{ "NavList provides semantic <ul> wrapper with proper ARIA attributes:" }</p>
                <NavList aria_label="Primary Navigation">
                    <NavItem>
                        <NavLink<ContainerRoute> to={ContainerRoute::Home}>{ "Home" }</NavLink<ContainerRoute>>
                    </NavItem>
                    <NavItem>
                        <NavLink<ContainerRoute> to={ContainerRoute::Products}>{ "Products" }</NavLink<ContainerRoute>>
                    </NavItem>
                    <NavItem>
                        <NavLink<ContainerRoute> to={ContainerRoute::Pricing}>{ "Pricing" }</NavLink<ContainerRoute>>
                    </NavItem>
                </NavList>
                <div class="info-box">
                    <strong>{ "Props:" }</strong>
                    <ul>
                        <li>{ "<code>classes</code> - Custom CSS classes" }</li>
                        <li>{ "<code>id</code> - HTML id attribute" }</li>
                        <li>{ "<code>aria_label</code> - ARIA label" }</li>
                    </ul>
                </div>
            </div>
            <div class="card">
                <h3>{ "NavItem Wrapper" }</h3>
                <p>{ "NavItem wraps links in semantic <li> with disabled state:" }</p>
                <NavList>
                    <NavItem>
                        <NavLink<ContainerRoute> to={ContainerRoute::Home}>{ "Normal" }</NavLink<ContainerRoute>>
                    </NavItem>
                    <NavItem disabled=true>
                        <NavLink<ContainerRoute> to={ContainerRoute::Products}>{ "Disabled" }</NavLink<ContainerRoute>>
                    </NavItem>
                </NavList>
            </div>
            <div class="card">
                <h3>{ "NavDivider" }</h3>
                <p>{ "NavDivider provides semantic separators:" }</p>
                <NavList>
                    <NavItem>
                        <NavLink<ContainerRoute> to={ContainerRoute::Home}>{ "Home" }</NavLink<ContainerRoute>>
                    </NavItem>
                    <NavDivider text="divider" />
                    <NavItem>
                        <NavLink<ContainerRoute> to={ContainerRoute::Products}>{ "Products" }</NavLink<ContainerRoute>>
                    </NavItem>
                </NavList>
                <div class="info-box">
                    <strong>{ "Props:" }</strong>
                    <ul>
                        <li>{ "<code>vertical</code> - For vertical dividers" }</li>
                        <li>{ "<code>text</code> - Optional text" }</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
