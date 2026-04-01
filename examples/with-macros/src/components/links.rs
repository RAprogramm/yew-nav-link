//! NavLink examples with macros - but using manual syntax for now.

use yew::prelude::*;
use yew_nav_link::{NavLink, nav_link, Match};
use yew_router::prelude::*;

use crate::routes::{AppRoute, LinksRoute};

#[component]
pub fn Links() -> Html {
    html! {
        <div class="links-page">
            <h2>{ "NavLink Examples - Macros Version" }</h2>
            <div class="card">
                <h3>{ "1. NavLink Component (Manual)" }</h3>
                <p>{ "The NavLink component provides automatic active state detection:" }</p>
                <nav>
                    <ul class="nav-list">
                        <li class="nav-item">
                            <NavLink<LinksRoute> to={LinksRoute::Home}>{ "Home" }</NavLink<LinksRoute>>
                        </li>
                        <li class="nav-item">
                            <NavLink<LinksRoute> to={LinksRoute::About}>{ "About" }</NavLink<LinksRoute>>
                        </li>
                        <li class="nav-item">
                            <NavLink<LinksRoute> to={LinksRoute::Contact}>{ "Contact" }</NavLink<LinksRoute>>
                        </li>
                    </ul>
                </nav>
                <div class="info-box">
                    <strong>{ "Code (12 lines):" }</strong>
                    <pre>{ r#"<ul class="nav-list">
    <li class="nav-item">
        <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
    </li>
    ...
</ul>"# }</pre>
                    <p>{ "With macros, this would be just 3 lines using <code>nav_list!</code>" }</p>
                </div>
            </div>
            <div class="card">
                <h3>{ "2. nav_link Function" }</h3>
                <p>{ "For text-only links:" }</p>
                <nav>
                    <ul class="nav-list">
                        <li>{ nav_link(LinksRoute::Home, "Home", Match::Exact) }</li>
                        <li>{ nav_link(LinksRoute::About, "About", Match::Exact) }</li>
                        <li>{ nav_link(LinksRoute::Contact, "Contact", Match::Exact) }</li>
                    </ul>
                </nav>
                <div class="info-box">
                    <strong>{ "Code (3 lines):" }</strong>
                    <pre>{ r#"<ul class="nav-list">
    <li>{ nav_link(Route::Home, "Home", Match::Exact) }</li>
    ...
</ul>"# }</pre>
                    <p>{ "With macros, this would be just 3 lines using <code>nav_links!</code>" }</p>
                </div>
            </div>
            <div class="card">
                <h3>{ "Back to Home" }</h3>
                <NavLink<AppRoute> to={AppRoute::Home}>{ "← Back to Home" }</NavLink<AppRoute>>
            </div>
        </div>
    }
}
