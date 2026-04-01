use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::NavLink;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div class="card" style="text-align:center; padding:60px 20px;">
            <h2 style="font-size:48px; margin-bottom:16px;">{ "404" }</h2>
            <p style="color:var(--text-dim); margin-bottom:24px;">{ "Page not found" }</p>
            <NavLink<Route> to={Route::Home}>{ "← Back to Overview" }</NavLink<Route>>
        </div>
    }
}
