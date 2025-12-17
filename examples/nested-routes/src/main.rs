//! Nested routes example for yew-nav-link.
//!
//! Demonstrates using NavLink with nested routing patterns:
//! - Main navigation (top-level routes)
//! - Sub-navigation (nested routes within sections)
//!
//! Run with: `trunk serve` from the examples/nested-routes directory.

use yew::prelude::*;
use yew_nav_link::NavLink;
use yew_router::prelude::*;

/// Top-level application routes.
#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/docs")]
    DocsRoot,
    #[at("/docs/*")]
    Docs,
    #[at("/blog")]
    BlogRoot,
    #[at("/blog/*")]
    Blog,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// Documentation section routes.
#[derive(Clone, PartialEq, Routable)]
enum DocsRoute {
    #[at("/docs")]
    Overview,
    #[at("/docs/getting-started")]
    GettingStarted,
    #[at("/docs/api")]
    Api,
    #[not_found]
    #[at("/docs/404")]
    NotFound,
}

/// Blog section routes.
#[derive(Clone, PartialEq, Routable)]
enum BlogRoute {
    #[at("/blog")]
    Latest,
    #[at("/blog/archive")]
    Archive,
    #[at("/blog/categories")]
    Categories,
    #[not_found]
    #[at("/blog/404")]
    NotFound,
}

#[component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <MainNav />
            <Switch<Route> render={switch_main} />
        </BrowserRouter>
    }
}

/// Main navigation bar.
/// Uses `partial=true` so section links stay active on nested routes.
#[component]
fn MainNav() -> Html {
    html! {
        <nav class="main-nav">
            <ul>
                <li><NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>></li>
                <li><NavLink<Route> to={Route::DocsRoot} partial=true>{ "Documentation" }</NavLink<Route>></li>
                <li><NavLink<Route> to={Route::BlogRoot} partial=true>{ "Blog" }</NavLink<Route>></li>
            </ul>
        </nav>
    }
}

fn switch_main(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::DocsRoot | Route::Docs => html! { <DocsSection /> },
        Route::BlogRoot | Route::Blog => html! { <BlogSection /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}

#[component]
fn HomePage() -> Html {
    html! {
        <div class="content">
            <h1>{ "Nested Routes Example" }</h1>
            <p>{ "This example demonstrates using NavLink with nested routing patterns." }</p>
            <div class="info-box">
                <strong>{ "Key concepts:" }</strong>
                <ul>
                    <li>{ "Main navigation uses top-level Route enum" }</li>
                    <li>{ "Each section has its own nested route enum (DocsRoute, BlogRoute)" }</li>
                    <li>{ "NavLink automatically detects active state at each level" }</li>
                </ul>
            </div>
            <p>{ "Click 'Documentation' or 'Blog' to see sub-navigation in action." }</p>
        </div>
    }
}

/// Documentation section with sub-navigation.
#[component]
fn DocsSection() -> Html {
    html! {
        <>
            <nav class="sub-nav">
                <ul>
                    <li><NavLink<DocsRoute> to={DocsRoute::Overview}>{ "Overview" }</NavLink<DocsRoute>></li>
                    <li><NavLink<DocsRoute> to={DocsRoute::GettingStarted}>{ "Getting Started" }</NavLink<DocsRoute>></li>
                    <li><NavLink<DocsRoute> to={DocsRoute::Api}>{ "API Reference" }</NavLink<DocsRoute>></li>
                </ul>
            </nav>
            <div class="content">
                <Switch<DocsRoute> render={switch_docs} />
            </div>
        </>
    }
}

fn switch_docs(route: DocsRoute) -> Html {
    match route {
        DocsRoute::Overview => html! {
            <>
                <h1>{ "Documentation Overview" }</h1>
                <p>{ "Welcome to the documentation section." }</p>
                <p>{ "Notice how the 'Documentation' link in the main nav stays active," }</p>
                <p>{ "while the sub-navigation tabs change based on the current page." }</p>
            </>
        },
        DocsRoute::GettingStarted => html! {
            <>
                <h1>{ "Getting Started" }</h1>
                <h2>{ "Installation" }</h2>
                <p>{ "Add yew-nav-link to your Cargo.toml dependencies." }</p>
                <h2>{ "Basic Usage" }</h2>
                <p>{ "Import NavLink and use it in your navigation components." }</p>
            </>
        },
        DocsRoute::Api => html! {
            <>
                <h1>{ "API Reference" }</h1>
                <h2>{ "NavLink<R>" }</h2>
                <p>{ "The main component for navigation links with active state." }</p>
                <h2>{ "nav_link()" }</h2>
                <p>{ "Helper function for creating text-only navigation links." }</p>
            </>
        },
        DocsRoute::NotFound => html! {
            <p>{ "Documentation page not found." }</p>
        },
    }
}

/// Blog section with sub-navigation.
#[component]
fn BlogSection() -> Html {
    html! {
        <>
            <nav class="sub-nav">
                <ul>
                    <li><NavLink<BlogRoute> to={BlogRoute::Latest}>{ "Latest Posts" }</NavLink<BlogRoute>></li>
                    <li><NavLink<BlogRoute> to={BlogRoute::Archive}>{ "Archive" }</NavLink<BlogRoute>></li>
                    <li><NavLink<BlogRoute> to={BlogRoute::Categories}>{ "Categories" }</NavLink<BlogRoute>></li>
                </ul>
            </nav>
            <div class="content">
                <Switch<BlogRoute> render={switch_blog} />
            </div>
        </>
    }
}

fn switch_blog(route: BlogRoute) -> Html {
    match route {
        BlogRoute::Latest => html! {
            <>
                <h1>{ "Latest Posts" }</h1>
                <p>{ "Recent blog entries would appear here." }</p>
            </>
        },
        BlogRoute::Archive => html! {
            <>
                <h1>{ "Archive" }</h1>
                <p>{ "Browse posts by date." }</p>
            </>
        },
        BlogRoute::Categories => html! {
            <>
                <h1>{ "Categories" }</h1>
                <p>{ "Browse posts by category." }</p>
            </>
        },
        BlogRoute::NotFound => html! {
            <p>{ "Blog page not found." }</p>
        },
    }
}

#[component]
fn NotFoundPage() -> Html {
    html! {
        <div class="content">
            <h1>{ "404 - Not Found" }</h1>
            <p>{ "The page you're looking for doesn't exist." }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
