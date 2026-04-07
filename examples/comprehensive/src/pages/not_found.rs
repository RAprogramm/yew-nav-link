use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::NavLink;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div class="card not-found-card">
            <h2 class="not-found-code">{ "404" }</h2>
            <p class="not-found-text">{ "Page not found" }</p>
            <NavLink<Route> to={Route::Home}>{ "\u{2190} Back to Overview" }</NavLink<Route>>
        </div>
    }
}
