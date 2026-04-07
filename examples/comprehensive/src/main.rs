//! yew-nav-link — Interactive Demo
//!
//! Comprehensive showcase of every component and hook in the library.
//! Deploy this to GitHub Pages with `trunk build --release`.

use yew::prelude::*;
use yew_nav_link::NavLink;
use yew_router::prelude::*;

mod code_utils;
mod demo_popup;
mod doc_page;
mod doc_parser;
mod file_tree;
mod pages;
mod routes;

use routes::Route;

/// Version string from crate metadata.
const SIDEBAR_VERSION: &str = env!("CARGO_PKG_VERSION");

#[function_component]
fn App() -> Html {
    let menu_open = use_state(|| false);

    let toggle = {
        let menu_open = menu_open.clone();
        Callback::from(move |_: MouseEvent| menu_open.set(!*menu_open))
    };

    let close = {
        let menu_open = menu_open.clone();
        Callback::from(move |_: MouseEvent| menu_open.set(false))
    };

    let sidebar_class = if *menu_open { "sidebar open" } else { "sidebar" };
    let overlay_class = if *menu_open { "overlay visible" } else { "overlay" };

    html! {
        <BrowserRouter>
            <div class="topbar">
                <button class="hamburger" onclick={toggle}>
                    <span class="hamburger-line" />
                    <span class="hamburger-line" />
                    <span class="hamburger-line" />
                </button>
                <div class="topbar-brand">{ "yew-nav-link" }</div>
                <div class="topbar-spacer" />
            </div>

            <div class={sidebar_class} onclick={close.clone()}>
                <SidebarContent />
            </div>

            <div class={overlay_class} onclick={close} />

            <main class="main">
                <Switch<Route> render={switch} />
            </main>
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
        Route::MacrosDoc => html! { <pages::MacrosDoc /> },
        Route::ExamplesHome => html! { <pages::ExamplesHome /> },
        Route::BasicExample => html! { <pages::BasicExample /> },
        Route::BootstrapExample => html! { <pages::BootstrapExample /> },
        Route::TailwindExample => html! { <pages::TailwindExample /> },
        Route::NestedRoutesExample => html! { <pages::NestedRoutesExample /> },
        Route::WithMacrosExample => html! { <pages::WithMacrosExample /> },
        Route::CustomCssFeatures => html! { <pages::CustomCssFeatures /> },
        Route::NavigationHooks => html! { <pages::NavigationHooks /> },
        Route::CustomBreadcrumbs => html! { <pages::CustomBreadcrumbs /> },
        Route::NotFound => html! { <pages::NotFound /> },
    }
}

#[function_component]
fn SidebarContent() -> Html {
    html! {
        <div class="sidebar-inner">
            <div class="sidebar-header">
                <div class="sidebar-logo">{ "yew-nav-link" }</div>
                <div class="sidebar-version">{ SIDEBAR_VERSION }</div>
            </div>

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
                    <NavLink<Route> to={Route::BadgeDoc}>{ "Badge / Header / Text" }</NavLink<Route>>
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
                    <NavLink<Route> to={Route::NavigationHooks}>{ "Navigation Hooks" }</NavLink<Route>>
                    <NavLink<Route> to={Route::BreadcrumbsDoc}>{ "Breadcrumbs" }</NavLink<Route>>
                    <NavLink<Route> to={Route::CustomBreadcrumbs}>{ "Custom Breadcrumbs" }</NavLink<Route>>
                    <NavLink<Route> to={Route::UtilsDoc}>{ "Path Utilities" }</NavLink<Route>>
                </nav>
            </div>

            <div class="sidebar-section">
                <div class="sidebar-section-title">{ "Macros" }</div>
                <nav>
                    <NavLink<Route> to={Route::MacrosDoc}>{ "All Macros" }</NavLink<Route>>
                </nav>
            </div>

            <div class="sidebar-section">
                <div class="sidebar-section-title">{ "New in v0.8" }</div>
                <nav>
                    <NavLink<Route> to={Route::CustomCssFeatures}>{ "Custom CSS Classes" }</NavLink<Route>>
                    <NavLink<Route> to={Route::NavigationHooks}>{ "Navigation Hooks" }</NavLink<Route>>
                    <NavLink<Route> to={Route::CustomBreadcrumbs}>{ "Custom Breadcrumbs" }</NavLink<Route>>
                </nav>
            </div>

            <div class="sidebar-section">
                <div class="sidebar-section-title">{ "Examples" }</div>
                <nav>
                    <NavLink<Route> to={Route::ExamplesHome}>{ "Overview" }</NavLink<Route>>
                    <NavLink<Route> to={Route::BasicExample}>{ "Basic" }</NavLink<Route>>
                    <NavLink<Route> to={Route::BootstrapExample}>{ "Bootstrap 5" }</NavLink<Route>>
                    <NavLink<Route> to={Route::TailwindExample}>{ "Tailwind CSS" }</NavLink<Route>>
                    <NavLink<Route> to={Route::NestedRoutesExample}>{ "Nested Routes" }</NavLink<Route>>
                    <NavLink<Route> to={Route::WithMacrosExample}>{ "With Macros" }</NavLink<Route>>
                </nav>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
