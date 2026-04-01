//! Route hooks example.

use yew::prelude::*;
use yew_nav_link::{NavLink, hooks::{use_route_info, use_is_active, use_is_exact_active, use_is_partial_active}};
use yew_router::prelude::*;

use crate::routes::{AppRoute, HookRoute};

#[component]
pub fn Hooks() -> Html {
    let current_route = use_route_info::<HookRoute>();
    let current_path = current_route.as_ref().map(|r| r.to_path()).unwrap_or_else(|| "/".to_string());

    let is_home_active = use_is_active(HookRoute::Home);
    let is_home_exact = use_is_exact_active(HookRoute::Home);
    let is_about_partial = use_is_partial_active(HookRoute::About);

    html! {
        <div class="hooks-page">
            <h2>{ "Route Hooks" }</h2>
            <div class="card">
                <h3>{ "Current Route" }</h3>
                <div class="info-box">
                    <strong>{ "Current Route:" }</strong>
                    { current_route.map(|r| format!("{:?}", r)).unwrap_or_else(|| "None".to_string()) }
                    <br/>
                    <strong>{ "Current Path:" }</strong>
                    { current_path }
                </div>
            </div>
            <div class="card">
                <h3>{ "use_route_info" }</h3>
                <p>{ "Gets the current route from the router:" }</p>
                <div class="info-box">
                    <strong>{ "Usage:" }</strong>
                    <code>{ "let current = use_route_info::<MyRoute>();" }</code>
                    <br/>
                    <strong>{ "Returns:" }</strong>
                    <code>{ "Option<MyRoute>" }</code>
                </div>
            </div>
            <div class="card">
                <h3>{ "use_is_active" }</h3>
                <p>{ "Checks if a route is currently active:" }</p>
                <div class="info-box">
                    <strong>{ "Is Home Active?" }</strong>
                    { if is_home_active { "Yes" } else { "No" } }
                </div>
            </div>
            <div class="card">
                <h3>{ "use_is_exact_active" }</h3>
                <p>{ "Checks if a route is exactly active:" }</p>
                <div class="info-box">
                    <strong>{ "Is Home Exactly Active?" }</strong>
                    { if is_home_exact { "Yes" } else { "No" } }
                </div>
            </div>
            <div class="card">
                <h3>{ "use_is_partial_active" }</h3>
                <p>{ "Checks if current route is a child:" }</p>
                <div class="info-box">
                    <strong>{ "Is About Partially Active?" }</strong>
                    { if is_about_partial { "Yes" } else { "No" } }
                </div>
            </div>
            <div class="card">
                <h3>{ "Back to Home" }</h3>
                <NavLink<AppRoute> to={AppRoute::Home}>{ "← Back to Home" }</NavLink<AppRoute>>
            </div>
        </div>
    }
}
