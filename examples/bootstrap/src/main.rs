//! Bootstrap integration example for yew-nav-link.
//!
//! Demonstrates how NavLink works seamlessly with Bootstrap's nav components.
//! The `nav-link` and `active` classes are Bootstrap-compatible by default.
//!
//! Run with: `trunk serve` from the examples/bootstrap directory.

use yew::prelude::*;
use yew_nav_link::NavLink;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/products")]
    Products,
    #[at("/pricing")]
    Pricing,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Navbar />
            <div class="container py-4">
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}

/// Bootstrap navbar with NavLink integration.
/// NavLink outputs `nav-link` class by default - perfect for Bootstrap!
#[component]
fn Navbar() -> Html {
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-primary">
            <div class="container">
                <a class="navbar-brand" href="#">{ "MyApp" }</a>
                <button
                    class="navbar-toggler"
                    type="button"
                    data-bs-toggle="collapse"
                    data-bs-target="#navbarNav"
                >
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarNav">
                    <ul class="navbar-nav">
                        <li class="nav-item">
                            <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                        </li>
                        <li class="nav-item">
                            <NavLink<Route> to={Route::Products}>{ "Products" }</NavLink<Route>>
                        </li>
                        <li class="nav-item">
                            <NavLink<Route> to={Route::Pricing}>{ "Pricing" }</NavLink<Route>>
                        </li>
                        <li class="nav-item">
                            <NavLink<Route> to={Route::Contact}>{ "Contact" }</NavLink<Route>>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <div class="card">
                <div class="card-body">
                    <h1 class="card-title">{ "Welcome" }</h1>
                    <p class="card-text">{ "This example shows yew-nav-link with Bootstrap 5." }</p>
                    <p class="card-text text-muted">
                        { "The NavLink component outputs " }
                        <code>{ "nav-link" }</code>
                        { " and " }
                        <code>{ "active" }</code>
                        { " classes - Bootstrap-compatible by default!" }
                    </p>
                </div>
            </div>
        },
        Route::Products => html! {
            <div class="card">
                <div class="card-body">
                    <h1 class="card-title">{ "Products" }</h1>
                    <p class="card-text">{ "Browse our product catalog." }</p>
                </div>
            </div>
        },
        Route::Pricing => html! {
            <div class="card">
                <div class="card-body">
                    <h1 class="card-title">{ "Pricing" }</h1>
                    <p class="card-text">{ "View our pricing plans." }</p>
                </div>
            </div>
        },
        Route::Contact => html! {
            <div class="card">
                <div class="card-body">
                    <h1 class="card-title">{ "Contact" }</h1>
                    <p class="card-text">{ "Get in touch with us." }</p>
                </div>
            </div>
        },
        Route::NotFound => html! {
            <div class="alert alert-warning">
                <h4 class="alert-heading">{ "404 - Not Found" }</h4>
                <p>{ "The page you're looking for doesn't exist." }</p>
            </div>
        },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
