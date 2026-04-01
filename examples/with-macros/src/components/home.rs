//! Home page component.

use yew::prelude::*;
use yew_nav_link::NavLink;
use yew_router::prelude::*;

use crate::routes::AppRoute;

#[component]
pub fn Home() -> Html {
    html! {
        <div class="home-page">
            <h2>{ "Navigation - With Macros" }</h2>
            <div class="card">
                <h3>{ "This example uses procedural macros" }</h3>
                <p>{ "Procedural macros significantly reduce boilerplate code." }</p>
                <div class="info-box">
                    <strong>{ "Code Statistics:" }</strong>
                    <ul>
                        <li>{ "Total lines: ~80" }</li>
                        <li>{ "Macros used: 4+" }</li>
                        <li>{ "Components: 6 (same as no-macros)" }</li>
                    </ul>
                </div>
            </div>
            <div class="card">
                <h3>{ "Macros Available:" }</h3>
                <ul>
                    <li><code>{ "nav_list! - Create navigation lists" }</code></li>
                    <li><code>{ "nav_links! - Create multiple links" }</code></li>
                    <li><code>{ "nav_menu! - Create nested menus" }</code></li>
                    <li><code>{ "nav_tabs! - Create tabbed navigation" }</code></li>
                    <li><code>{ "breadcrumbs! - Create breadcrumbs" }</code></li>
                    <li><code>{ "nav_pagination! - Create pagination" }</code></li>
                </ul>
            </div>
            <div class="card">
                <h3>{ "Compare with Full Syntax" }</h3>
                <p>{ "See the <code>no-macros</code> example to see the full syntax without macros!" }</p>
                <NavLink<AppRoute> to={AppRoute::Links}>{ "Compare the difference" }</NavLink<AppRoute>>
            </div>
        </div>
    }
}
