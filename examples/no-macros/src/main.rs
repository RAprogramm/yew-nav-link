//! yew-nav-link — Interactive Demo
//!
//! Comprehensive showcase of every component and hook in the library.
//! Deploy this to GitHub Pages with `trunk build --release`.

use yew::prelude::*;
use yew_nav_link::NavLink;
use yew_router::prelude::*;

mod pages;
mod routes;

use routes::Route;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <div class="layout">
                <Sidebar />
                <main class="main">
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <pages::Home /> },
        Route::NavLinkDoc => html! { <pages::NavLinkDoc /> },
        Route::NavListDoc => html! { <pages::NavListDoc /> },
        Route::NavDividerDoc => html! { <pages::NavDividerDoc /> },
        Route::BadgeDoc => html! { <pages::BadgeDoc /> },
        Route::DropdownDoc => html! { <pages::DropdownDoc /> },
        Route::IconDoc => html! { <pages::IconDoc /> },
        Route::TabsDoc => html! { <pages::TabsDoc /> },
        Route::PaginationDoc => html! { <pages::PaginationDoc /> },
        Route::HooksDoc => html! { <pages::HooksDoc /> },
        Route::BreadcrumbsDoc => html! { <pages::BreadcrumbsDoc /> },
        Route::UtilsDoc => html! { <pages::UtilsDoc /> },
        Route::NotFound => html! { <pages::NotFound /> },
    }
}

#[function_component]
fn Sidebar() -> Html {
    html! {
        <aside class="sidebar">
            <div class="sidebar-logo">
                { "yew-nav-link" }
            </div>
            <div class="sidebar-version">{ "v0.6.0 — Interactive Demo" }</div>

            <div class="sidebar-section">
                <div class="sidebar-section-title">{ "Getting Started" }</div>
                <nav>
                    <NavLink<Route> to={Route::Home}>{ "Overview" }</NavLink<Route>>
                </nav>
            </div>

            <div class="sidebar-section">
                <div class="sidebar-section-title">{ "Components" }</div>
                <nav>
                    <NavLink<Route> to={Route::NavLinkDoc}>{ "NavLink" }</NavLink<Route>>
                    <NavLink<Route> to={Route::NavListDoc}>{ "NavList & NavItem" }</NavLink<Route>>
                    <NavLink<Route> to={Route::NavDividerDoc}>{ "NavDivider" }</NavLink<Route>>
                    <NavLink<Route> to={Route::BadgeDoc}>{ "NavBadge / NavHeader / NavText" }</NavLink<Route>>
                    <NavLink<Route> to={Route::DropdownDoc}>{ "NavDropdown" }</NavLink<Route>>
                    <NavLink<Route> to={Route::IconDoc}>{ "NavIcon" }</NavLink<Route>>
                    <NavLink<Route> to={Route::TabsDoc}>{ "NavTabs / NavTab" }</NavLink<Route>>
                    <NavLink<Route> to={Route::PaginationDoc}>{ "Pagination" }</NavLink<Route>>
                </nav>
            </div>

            <div class="sidebar-section">
                <div class="sidebar-section-title">{ "Hooks & Utilities" }</div>
                <nav>
                    <NavLink<Route> to={Route::HooksDoc}>{ "Route Hooks" }</NavLink<Route>>
                    <NavLink<Route> to={Route::BreadcrumbsDoc}>{ "Breadcrumbs" }</NavLink<Route>>
                    <NavLink<Route> to={Route::UtilsDoc}>{ "Path Utilities" }</NavLink<Route>>
                </nav>
            </div>
        </aside>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
