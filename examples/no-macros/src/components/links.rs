//! NavLink examples - showing full syntax.

use yew::prelude::*;
use yew_nav_link::{NavLink, nav_link, Match};
use yew_router::prelude::*;

use crate::routes::{AppRoute, LinksRoute};

#[component]
pub fn Links() -> Html {
    html! {
        <div class="links-page">
            <h2>{ "NavLink Examples" }</h2>
            <div class="card">
                <h3>{ "1. NavLink Component Syntax (Explicit)" }</h3>
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
                    <strong>{ "Code:" }</strong>
                    <pre>{ r#"NavLink<LinksRoute> to={LinksRoute::Home}>{ "Home" }</NavLink<LinksRoute>"# }</pre>
                    <p>{ "Lines: 3 per link" }</p>
                </div>
            </div>
            <div class="card">
                <h3>{ "2. nav_link Function Syntax" }</h3>
                <p>{ "For text-only links, use the nav_link function:" }</p>
                <nav>
                    <ul class="nav-list">
                        <li>{ nav_link(LinksRoute::Home, "Home", Match::Exact) }</li>
                        <li>{ nav_link(LinksRoute::About, "About", Match::Exact) }</li>
                        <li>{ nav_link(LinksRoute::Contact, "Contact", Match::Exact) }</li>
                    </ul>
                </nav>
                <div class="info-box">
                    <strong>{ "Code:" }</strong>
                    <pre>{ r#"nav_link(LinksRoute::Home, "Home", Match::Exact)"# }</pre>
                    <p>{ "Lines: 1 per link" }</p>
                </div>
            </div>
            <div class="card">
                <h3>{ "3. NavLink with Match::Partial" }</h3>
                <p>{ "Use partial matching for nested routes:" }</p>
                <nav>
                    <ul class="nav-list">
                        <li class="nav-item">
                            <NavLink<LinksRoute> to={LinksRoute::Home}>{ "Home" }</NavLink<LinksRoute>>
                        </li>
                        <li class="nav-item">
                            <NavLink<LinksRoute> to={LinksRoute::About} partial=true>{ "About (Partial)" }</NavLink<LinksRoute>>
                        </li>
                    </ul>
                </nav>
                <div class="info-box">
                    <strong>{ "Code:" }</strong>
                    <pre>{ r#"<NavLink<Route> to={Route::Variant} partial=true>{ "Link" }</NavLink<Route>>"# }</pre>
                </div>
            </div>
            <div class="card">
                <h3>{ "Back to Home" }</h3>
                <NavLink<AppRoute> to={AppRoute::Home}>{ "← Back to Home" }</NavLink<AppRoute>>
            </div>
        </div>
    }
}
