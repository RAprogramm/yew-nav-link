//! Container components with macros.

use yew::prelude::*;
use yew_nav_link::{NavLink, NavList, NavItem, NavDivider};
use yew_router::prelude::*;

use crate::routes::{AppRoute, ContainerRoute};

#[component]
pub fn Container() -> Html {
    html! {
        <div class="container-page">
            <h2>{ "Container Components - Macros Version" }</h2>
            <div class="card">
                <h3>{ "1. Full NavList with NavItem" }</h3>
                <p>{ "Container components + links:" }</p>
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
                    <strong>{ "Code (11 lines):" }</strong>
                    <pre>{ r#"<NavList aria_label="Primary Navigation">
    <NavItem>
        <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
    </NavItem>
    ...
</NavList>"# }</pre>
                    <p>{ "With macros, this would be just 3 lines using nav_list!" }</p>
                </div>
            </div>
            <div class="card">
                <h3>{ "Back to Home" }</h3>
                <NavLink<AppRoute> to={AppRoute::Home}>{ "← Back to Home" }</NavLink<AppRoute>>
            </div>
        </div>
    }
}
