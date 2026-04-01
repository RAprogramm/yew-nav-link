//! Not found page component.

use yew::prelude::*;
use yew_nav_link::NavLink;

use crate::routes::AppRoute;

#[component]
pub fn NotFound() -> Html {
    html! {
        <div class="not-found-page">
            <h2>{ "404 - Page Not Found" }</h2>
            <div class="card">
                <p>{ "The page you're looking for doesn't exist." }</p>
                <NavLink<AppRoute> to={AppRoute::Home}>{ "← Back to Home" }</NavLink<AppRoute>>
            </div>
        </div>
    }
}
