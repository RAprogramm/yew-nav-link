//! NavList examples - showing full syntax.

use yew::prelude::*;
use yew_nav_link::{NavLink, NavList, NavItem, nav_link, Match};
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Debug, Routable)]
enum ListsRoute {
    #[at("/")]
    Home,
    #[at("/products")]
    Products,
    #[at("/pricing")]
    Pricing,
}

#[component]
pub fn Lists() -> Html {
    html! {
        <div class="lists-page">
            <h2>{ "NavList Examples" }</h2>
            <div class="card">
                <h3>{ "1. Manual List Structure" }</h3>
                <p>{ "Writing the full HTML structure:" }</p>
                <nav>
                    <ul class="nav-list">
                        <li class="nav-item">
                            <NavLink<ListsRoute> to={ListsRoute::Home}>{ "Home" }</NavLink<ListsRoute>>
                        </li>
                        <li class="nav-item">
                            <NavLink<ListsRoute> to={ListsRoute::Products}>{ "Products" }</NavLink<ListsRoute>>
                        </li>
                        <li class="nav-item">
                            <NavLink<ListsRoute> to={ListsRoute::Pricing}>{ "Pricing" }</NavLink<ListsRoute>>
                        </li>
                    </ul>
                </nav>
                <div class="info-box">
                    <strong>{ "Code:" }</strong>
                    <pre>{ r#"<ul class="nav-list">
    <li class="nav-item">
        <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
    </li>
    ...
</ul>"# }</pre>
                    <p>{ "Lines: 12 for 3 links" }</p>
                </div>
            </div>
            <div class="card">
                <h3>{ "2. Using NavList Container" }</h3>
                <p>{ "NavList provides semantic <ul> with proper ARIA attributes:" }</p>
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
                    <strong>{ "Code:" }</strong>
                    <pre>{ r#"<NavList aria_label="Primary Navigation">
    <NavItem>
        <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
    </NavItem>
    ...
</NavList>"# }</pre>
                    <p>{ "Lines: 10 for 3 links" }</p>
                </div>
            </div>
            <div class="card">
                <h3>{ "3. Using NavItem Wrapper" }</h3>
                <p>{ "NavItem wraps links in semantic <li>:" }</p>
                <NavList>
                    <NavItem>
                        <NavLink<ListsRoute> to={ListsRoute::Home}>{ "Normal" }</NavLink<ListsRoute>>
                    </NavItem>
                    <NavItem disabled=true>
                        <NavLink<ListsRoute> to={ListsRoute::Products}>{ "Disabled" }</NavLink<ListsRoute>>
                    </NavItem>
                </NavList>
                <div class="info-box">
                    <strong>{ "Code:" }</strong>
                    <pre>{ r#"<NavItem disabled=true>
    <NavLink<Route> to={Route::Products}>{ "Disabled" }</NavLink<Route>>
</NavItem>"# }</pre>
                </div>
            </div>
        </div>
    }
}
