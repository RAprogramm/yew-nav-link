//! Navigation example with macros - minimal syntax.
//!
//! This example shows how to build navigation using procedural macros.
//! Macros significantly reduce boilerplate code.
//!
//! Lines of code: ~80
//! Macros used: 4 (nav_list, nav_links, nav_menu, nav_tabs)

use yew::prelude::*;
use yew_nav_link::NavLink;
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
                    <h1>{ "Navigation - With Macros" }</h1>
                </header>
                <main class="main-content container">
                    <Switch<AppRoute> render={switch} />
                </main>
                <footer class="main-footer">
                    <p>{ "Macros example - reduced code" }</p>
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
