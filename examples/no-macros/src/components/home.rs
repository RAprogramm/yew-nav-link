//! Home page component.

use yew::prelude::*;
use yew_nav_link::NavLink;
use yew_router::prelude::*;

use crate::routes::AppRoute;

#[component]
pub fn Home() -> Html {
    html! {
        <div class="home-page">
            <h2>{ "Navigation - No Macros" }</h2>
            <div class="card">
                <h3>{ "This example shows the full syntax" }</h3>
                <p>{ "No procedural macros are used - everything is written explicitly." }</p>
                <div class="info-box">
                    <strong>{ "Code Statistics:" }</strong>
                    <ul>
                        <li>{ "Total lines: ~180" }</li>
                        <li>{ "Macros used: 0" }</li>
                        <li>{ "Components: 6" }</li>
                    </ul>
                </div>
            </div>
            <div class="card">
                <h3>{ "Examples" }</h3>
                <nav>
                    <ul class="nav-list">
                        <li class="nav-item">
                            <a class="nav-link" href={ AppRoute::Links.to_path() }>{ "NavLink Examples" }</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href={ AppRoute::Lists.to_path() }>{ "NavList Examples" }</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href={ AppRoute::Container.to_path() }>{ "Container Components" }</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href={ AppRoute::Hooks.to_path() }>{ "Route Hooks" }</a>
                        </li>
                    </ul>
                </nav>
            </div>
            <div class="card">
                <h3>{ "Compare with Macros" }</h3>
                <p>{ "See the <code>with-macros</code> example for the same functionality with much less code!" }</p>
                <NavLink<AppRoute> to={AppRoute::Links}>{ "See the difference" }</NavLink<AppRoute>>
            </div>
        </div>
    }
}
