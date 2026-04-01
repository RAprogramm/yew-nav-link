//! yew-nav-link — Interactive Demo
//!
//! Comprehensive showcase of every component and hook in the library.
//! Deploy this to GitHub Pages with `trunk build --release`.

use yew::prelude::*;
use yew_nav_link::NavLink;
use yew_router::prelude::*;

mod demo_popup;
mod doc_parser;
mod pages;
mod routes;

use routes::Route;

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

    let open_class = if *menu_open { " open" } else { "" };
    let overlay_class = if *menu_open {
        "overlay visible"
    } else {
        "overlay"
    };

    html! {
        <BrowserRouter>
            <div class="topbar">
                <button class="hamburger" onclick={toggle.clone()}>
                    <span class="hamburger-line" />
                    <span class="hamburger-line" />
                    <span class="hamburger-line" />
                </button>
                <div class="topbar-brand">{ "yew-nav-link" }</div>
                <div class="topbar-spacer" />
            </div>

            <div class={format!("sidebar{}", open_class)}>
                <SidebarContent on_link_click={close.clone()} />
            </div>

            <div class={overlay_class} onclick={close.clone()} />

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
        Route::NotFound => html! { <pages::NotFound /> },
    }
}

#[derive(Properties, PartialEq, Clone)]
struct SidebarProps {
    on_link_click: Callback<MouseEvent>,
}

#[function_component]
fn SidebarContent(props: &SidebarProps) -> Html {
    let onclick = props.on_link_click.clone();

    html! {
        <div class="sidebar-inner">
            <div class="sidebar-header">
                <div class="sidebar-logo">{ "yew-nav-link" }</div>
                <div class="sidebar-version">{ "v0.6.0" }</div>
            </div>

            <div class="sidebar-section">
                <div class="sidebar-section-title">{ "Getting Started" }</div>
                <nav>
                    <NavLink<Route> to={Route::Home} onclick={onclick.clone()}>{ "Overview" }</NavLink<Route>>
                </nav>
            </div>

            <div class="sidebar-section">
                <div class="sidebar-section-title">{ "Components" }</div>
                <nav>
                    <NavLink<Route> to={Route::NavLinkDoc} onclick={onclick.clone()}>{ "NavLink" }</NavLink<Route>>
                    <NavLink<Route> to={Route::NavListDoc} onclick={onclick.clone()}>{ "NavList & NavItem" }</NavLink<Route>>
                    <NavLink<Route> to={Route::NavDividerDoc} onclick={onclick.clone()}>{ "NavDivider" }</NavLink<Route>>
                    <NavLink<Route> to={Route::BadgeDoc} onclick={onclick.clone()}>{ "Badge / Header / Text" }</NavLink<Route>>
                    <NavLink<Route> to={Route::DropdownDoc} onclick={onclick.clone()}>{ "NavDropdown" }</NavLink<Route>>
                    <NavLink<Route> to={Route::IconDoc} onclick={onclick.clone()}>{ "NavIcon" }</NavLink<Route>>
                    <NavLink<Route> to={Route::TabsDoc} onclick={onclick.clone()}>{ "NavTabs / NavTab" }</NavLink<Route>>
                    <NavLink<Route> to={Route::PaginationDoc} onclick={onclick.clone()}>{ "Pagination" }</NavLink<Route>>
                </nav>
            </div>

            <div class="sidebar-section">
                <div class="sidebar-section-title">{ "Hooks & Utilities" }</div>
                <nav>
                    <NavLink<Route> to={Route::HooksDoc} onclick={onclick.clone()}>{ "Route Hooks" }</NavLink<Route>>
                    <NavLink<Route> to={Route::BreadcrumbsDoc} onclick={onclick.clone()}>{ "Breadcrumbs" }</NavLink<Route>>
                    <NavLink<Route> to={Route::UtilsDoc} onclick={onclick.clone()}>{ "Path Utilities" }</NavLink<Route>>
                </nav>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
