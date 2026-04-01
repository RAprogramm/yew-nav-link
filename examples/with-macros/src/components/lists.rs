//! NavList examples with macros - but using manual syntax for now.

use yew::prelude::*;
use yew_nav_link::{NavLink, NavList, NavItem, nav_link, Match};
use yew_router::prelude::*;

use crate::routes::{AppRoute, ListsRoute};

#[component]
pub fn Lists() -> Html {
    html! {
        <div class="lists-page">
            <h2>{ "NavList Examples - Macros Version" }</h2>
            <div class="card">
                <h3>{ "1. Manual NavList (With NavList container)" }</h3>
                <p>{ "Using NavList container for proper ARIA attributes:" }</p>
                <NavList aria_label="Primary Navigation">
                    <NavItem>
                        <NavLink<ListsRoute> to={ListsRoute::Home}>{ "Home" }</NavLink<ListsRoute>>
                    </NavItem>
                    <NavItem>
                        <NavLink<ListsRoute> to={ListsRoute::Products}>{ "Products" }</NavLink<ListsRoute>>
                    </NavItem>
                    <NavItem>
                        <NavLink<ListsRoute> to={ListsRoute::Pricing}>{ "Pricing" }</NavLink<ListsRoute>>
                    </NavItem>
                </NavList>
                <div class="info-box">
                    <strong>{ "Code (10 lines):" }</strong>
                    <pre>{ r#"<NavList aria_label="Primary Navigation">
    <NavItem>
        <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
    </NavItem>
    ...
</NavList>"# }</pre>
                    <p>{ "With macros, this would be just 3 lines using <code>nav_list!</code>" }</p>
                </div>
            </div>
            <div class="card">
                <h3>{ "2. Using nav_link Function" }</h3>
                <p>{ "For text-only links:" }</p>
                <NavList>
                    <NavItem>
                        { nav_link(ListsRoute::Home, "Home", Match::Exact) }
                    </NavItem>
                    <NavItem>
                        { nav_link(ListsRoute::Products, "Products", Match::Exact) }
                    </NavItem>
                    <NavItem>
                        { nav_link(ListsRoute::Pricing, "Pricing", Match::Exact) }
                    </NavItem>
                </NavList>
                <div class="info-box">
                    <strong>{ "Code (10 lines):" }</strong>
                    <pre>{ r#"<NavList>
    <NavItem>
        { nav_link(Route::Home, "Home", Match::Exact) }
    </NavItem>
    ...
</NavList>"# }</pre>
                    <p>{ "With macros, this would be just 3 lines using <code>nav_list!</code>" }</p>
                </div>
            </div>
            <div class="card">
                <h3>{ "Back to Home" }</h3>
                <NavLink<AppRoute> to={AppRoute::Home}>{ "← Back to Home" }</NavLink<AppRoute>>
            </div>
        </div>
    }
}
