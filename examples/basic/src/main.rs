//! Basic example demonstrating yew-nav-link usage.
//!
//! Run with: `trunk serve` from the examples/basic directory.

use yew::prelude::*;
use yew_nav_link::{Match, NavLink, nav_link};
use yew_router::prelude::*;

/// Application routes.
#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// Main application component.
#[component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Navigation />
            <main>
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}

/// Navigation bar using NavLink component.
#[component]
fn Navigation() -> Html {
    html! {
        <nav>
            <ul>
                // Method 1: Component syntax (recommended for complex content)
                <li>
                    <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                </li>
                <li>
                    <NavLink<Route> to={Route::About}>{ "About" }</NavLink<Route>>
                </li>
                // Method 2: Function syntax (convenient for text-only links)
                <li>{ nav_link(Route::Contact, "Contact", Match::Exact) }</li>
            </ul>
        </nav>
    }
}

/// Route switch function.
fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::About => html! { <AboutPage /> },
        Route::Contact => html! { <ContactPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}

#[component]
fn HomePage() -> Html {
    html! {
        <>
            <h1>{ "Home" }</h1>
            <p>{ "Welcome to yew-nav-link example!" }</p>
            <div class="card">
                <p>{ "Click the navigation links above to see the active state change automatically." }</p>
                <p>{ "The current page link will be highlighted with a blue background." }</p>
            </div>
        </>
    }
}

#[component]
fn AboutPage() -> Html {
    html! {
        <>
            <h1>{ "About" }</h1>
            <p>{ "yew-nav-link provides automatic active state detection for navigation links." }</p>
            <div class="card">
                <h3>{ "Features:" }</h3>
                <ul>
                    <li>{ "Automatic 'active' class when route matches" }</li>
                    <li>{ "Type-safe routing with Yew Router" }</li>
                    <li>{ "Works with Bootstrap, Tailwind, and custom CSS" }</li>
                </ul>
            </div>
        </>
    }
}

#[component]
fn ContactPage() -> Html {
    html! {
        <>
            <h1>{ "Contact" }</h1>
            <p>{ "This page was linked using the nav_link() function syntax." }</p>
            <div class="card">
                <p>{ "Both NavLink component and nav_link function work identically." }</p>
                <p>{ "Choose the syntax that fits your use case." }</p>
            </div>
        </>
    }
}

#[component]
fn NotFoundPage() -> Html {
    html! {
        <>
            <h1>{ "404 - Not Found" }</h1>
            <p>{ "The page you're looking for doesn't exist." }</p>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
