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
mod generated;
mod pages;
mod routes;
mod templates;

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

    let sidebar_class = if *menu_open {
        "sidebar open"
    } else {
        "sidebar"
    };
    let overlay_class = if *menu_open {
        "overlay visible"
    } else {
        "overlay"
    };

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
        Route::ExamplesHome => html! { <pages::ExamplesHome /> },
        Route::BasicExample => html! { <pages::BasicExample /> },
        Route::BootstrapExample => html! { <pages::BootstrapExample /> },
        Route::TailwindExample => html! { <pages::TailwindExample /> },
        Route::NestedRoutesExample => html! { <pages::NestedRoutesExample /> },
        Route::CustomCssFeatures => html! { <pages::CustomCssFeatures /> },
        Route::NavigationHooks => html! { <pages::NavigationHooks /> },
        Route::CustomBreadcrumbs => html! { <pages::CustomBreadcrumbs /> },
        Route::NotFound => html! { <pages::NotFound /> }
    }
}

#[function_component]
fn SidebarContent() -> Html {
    let sections_html: Html = html! {
        <>
        { for generated::SIDEBAR.iter().map(|section| {
            let title = section.title.to_string();
            let items_html: Vec<Html> = section.items.iter().map(|item| {
                let route = sidebar_target_route(&item.target);
                let label = item.label.to_string();
                html! {
                    <NavLink<Route> to={route}>
                        <>{ label }</>
                    </NavLink<Route>>
                }
            }).collect();
            html! {
                <div class="sidebar-section">
                    <div class="sidebar-section-title">{ title }</div>
                    <nav>{ for items_html.iter().cloned() }</nav>
                </div>
            }
        })}
        </>
    };

    html! {
        <div class="sidebar-inner">
            <div class="sidebar-header">
                <div class="sidebar-logo">{ "yew-nav-link" }</div>
                <div class="sidebar-version">{ SIDEBAR_VERSION }</div>
            </div>

            { sections_html }

            <div class="sidebar-section">
                <div class="sidebar-section-title">{ "Examples" }</div>
                <nav>
                    <NavLink<Route> to={Route::ExamplesHome}>{ "Overview" }</NavLink<Route>>
                    <NavLink<Route> to={Route::BasicExample}>{ "Basic" }</NavLink<Route>>
                    <NavLink<Route> to={Route::BootstrapExample}>{ "Bootstrap 5" }</NavLink<Route>>
                    <NavLink<Route> to={Route::TailwindExample}>{ "Tailwind CSS" }</NavLink<Route>>
                    <NavLink<Route> to={Route::NestedRoutesExample}>{ "Nested Routes" }</NavLink<Route>>
                </nav>
            </div>
        </div>
    }
}

fn sidebar_target_route(target: &generated::SidebarTarget) -> Route {
    match target {
        generated::SidebarTarget::Overview => Route::Home,
        generated::SidebarTarget::CompItem(i) => {
            let name = generated::COMPONENTS[*i as usize].name;
            comp_route(name)
        }
        generated::SidebarTarget::HookItem(_) => Route::HooksDoc,
        generated::SidebarTarget::CompPage(_) => Route::Home,
    }
}

fn comp_route(name: &str) -> Route {
    match name {
        "NavLink" => Route::NavLinkDoc,
        "NavList" | "NavItem" => Route::NavListDoc,
        "NavDivider" => Route::NavDividerDoc,
        "NavBadge" | "NavHeader" | "NavText" => Route::BadgeDoc,
        "NavDropdown" | "NavDropdownItem" | "NavDropdownDivider" => Route::DropdownDoc,
        "NavIcon" => Route::IconDoc,
        "NavTabs" | "NavTab" | "NavTabPanel" => Route::TabsDoc,
        "Pagination" | "PageItem" | "PageLink" => Route::PaginationDoc,
        _ => Route::Home, // fallback
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
