//! Navigation example without macros - full syntax.
//!
//! This example shows how to build navigation using the full component syntax.
//! No procedural macros are used.
//!
//! Lines of code: ~180
//! Macros used: 0

use yew::prelude::*;
use yew_nav_link::{NavLink, NavList, NavItem, NavDivider, nav_link, Match};
use yew_router::prelude::*;

mod routes;
mod components;

use routes::AppRoute;

#[component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <div class="app-container">
                <header class="main-header">
                    <h1>{ "Navigation - No Macros" }</h1>
                </header>
                <main class="main-content container">
                    <Switch<AppRoute> render={switch} />
                </main>
                <footer class="main-footer">
                    <p>{ "Full syntax example - no macros" }</p>
                </footer>
            </div>
        </BrowserRouter>
    }
}

fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! { <components::home::Home /> },
        AppRoute::Links => html! { <components::links::Links /> },
        AppRoute::Lists => html! { <components::lists::Lists /> },
        AppRoute::Container => html! { <components::container::Container /> },
        AppRoute::Hooks => html! { <components::hooks::Hooks /> },
        AppRoute::NotFound => html! { <components::not_found::NotFound /> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
