// SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
// SPDX-License-Identifier: MIT

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_nav_link::{Match, NavLink, components::*, hooks::*, nav::*, nav_link, utils::*};
use yew_router::prelude::*;

// ============ ROUTES ============

#[derive(Clone, PartialEq, Debug, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/basic")]
    BasicLinks,
    #[at("/components")]
    Components,
    #[at("/tabs")]
    TabsDemo,
    #[at("/pagination")]
    PaginationDemo,
    #[at("/hooks")]
    HooksDemo,
    #[at("/utils")]
    UtilsDemo,
    #[at("/dropdown")]
    DropdownDemo,
    #[at("/blog")]
    Blog,
    #[at("/blog/:id")]
    BlogPost { id: String },
    #[at("/nested")]
    Nested,
    #[at("/nested/first")]
    NestedFirst,
    #[at("/nested/second")]
    NestedSecond,
    #[at("/query")]
    QueryDemo
}

// ============ GLOBAL STYLES ============

fn global_styles() -> Html {
    html! {
        <style>{r#"
            :root {
                --primary: #3b82f6;
                --primary-dark: #2563eb;
                --primary-light: #dbeafe;
                --success: #10b981;
                --warning: #f59e0b;
                --danger: #ef4444;
                --text: #1e293b;
                --text-muted: #64748b;
                --text-light: #94a3b8;
                --bg: #f8fafc;
                --bg-card: #ffffff;
                --border: #e2e8f0;
                --shadow: 0 1px 3px rgba(0,0,0,0.1);
                --shadow-lg: 0 10px 15px -3px rgba(0,0,0,0.1);
                --radius: 0.5rem;
                --radius-sm: 0.375rem;
            }

            * { box-sizing: border-box; margin: 0; padding: 0; }

            body {
                font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
                color: var(--text);
                background: var(--bg);
                line-height: 1.6;
            }

            .app-container { min-height: 100vh; display: flex; flex-direction: column; }

            .main-nav {
                background: var(--bg-card);
                border-bottom: 1px solid var(--border);
                padding: 0 2rem;
                position: sticky;
                top: 0;
                z-index: 100;
                box-shadow: var(--shadow);
            }

            .main-nav .nav-content {
                max-width: 1200px;
                margin: 0 auto;
                display: flex;
                align-items: center;
                gap: 0.5rem;
                padding: 0.75rem 0;
            }

            .main-nav .logo {
                font-weight: 700;
                font-size: 1.25rem;
                color: var(--primary);
                text-decoration: none;
                margin-right: 2rem;
                display: flex;
                align-items: center;
                gap: 0.5rem;
            }

            .main-nav .nav-links {
                display: flex;
                gap: 0.25rem;
                flex-wrap: wrap;
            }

            .container {
                max-width: 1200px;
                margin: 0 auto;
                padding: 2rem;
                width: 100%;
            }

            .page-header {
                margin-bottom: 2rem;
                padding-bottom: 1rem;
                border-bottom: 1px solid var(--border);
            }

            .page-header h1 {
                font-size: 2rem;
                font-weight: 700;
                color: var(--text);
                margin-bottom: 0.5rem;
            }

            .page-header p {
                color: var(--text-muted);
                font-size: 1.1rem;
            }

            .section {
                background: var(--bg-card);
                border: 1px solid var(--border);
                border-radius: var(--radius);
                padding: 1.5rem;
                margin-bottom: 1.5rem;
                box-shadow: var(--shadow);
            }

            .section-title {
                font-size: 1.25rem;
                font-weight: 600;
                margin-bottom: 1rem;
                color: var(--text);
            }

            .section-desc {
                color: var(--text-muted);
                margin-bottom: 1rem;
                font-size: 0.95rem;
            }

            .code-block {
                background: #1e293b;
                color: #e2e8f0;
                padding: 1rem;
                border-radius: var(--radius-sm);
                font-family: 'Monaco', 'Consolas', monospace;
                font-size: 0.85rem;
                overflow-x: auto;
                margin: 1rem 0;
                line-height: 1.5;
            }

            .demo-grid {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                gap: 1rem;
                margin-top: 1rem;
            }

            .card {
                background: var(--bg);
                border: 1px solid var(--border);
                border-radius: var(--radius-sm);
                padding: 1rem;
            }

            .card-title {
                font-weight: 600;
                margin-bottom: 0.5rem;
                color: var(--text);
            }

            .badge {
                display: inline-block;
                padding: 0.25rem 0.75rem;
                border-radius: 9999px;
                font-size: 0.75rem;
                font-weight: 600;
            }

            .badge-blue { background: var(--primary-light); color: var(--primary); }
            .badge-green { background: #d1fae5; color: var(--success); }
            .badge-yellow { background: #fef3c7; color: var(--warning); }
            .badge-red { background: #fee2e2; color: var(--danger); }

            .info-box {
                background: var(--primary-light);
                border-left: 4px solid var(--primary);
                padding: 1rem;
                border-radius: var(--radius-sm);
                margin: 1rem 0;
            }

            .info-box p { color: var(--primary-dark); font-size: 0.9rem; }

            .status-active { color: var(--success); font-weight: 600; }
            .status-inactive { color: var(--text-muted); }

            pre {
                background: #1e293b;
                color: #e2e8f0;
                padding: 1rem;
                border-radius: var(--radius-sm);
                overflow-x: auto;
                font-size: 0.85rem;
                line-height: 1.5;
            }

            .flex-row { display: flex; gap: 1rem; align-items: center; flex-wrap: wrap; }
            .flex-col { display: flex; flex-direction: column; gap: 0.5rem; }
            .gap-1 { gap: 1rem; }
            .mt-1 { margin-top: 1rem; }
            .mb-1 { margin-bottom: 1rem; }

            table {
                width: 100%;
                border-collapse: collapse;
                margin: 1rem 0;
            }

            th, td {
                padding: 0.75rem;
                text-align: left;
                border-bottom: 1px solid var(--border);
            }

            th {
                background: var(--bg);
                font-weight: 600;
                font-size: 0.875rem;
                color: var(--text-muted);
            }

            .breadcrumb-demo {
                display: flex;
                gap: 0.5rem;
                align-items: center;
                padding: 1rem;
                background: var(--bg);
                border-radius: var(--radius-sm);
                margin: 1rem 0;
            }

            .breadcrumb-separator {
                color: var(--text-light);
            }
        "#}</style>
    }
}

// ============ DEMO PAGES ============

#[component]
fn Navigation() -> Html {
    html! {
        <nav class="main-nav">
            <div class="nav-content">
                <NavLink<Route>
                    to={Route::Home}
                    class="logo"
                >
                    { "yew-nav-link" }
                </NavLink<Route>>

                <div class="nav-links">
                    <NavLink<Route> to={Route::BasicLinks}>{ "Basic" }</NavLink<Route>>
                    <NavLink<Route> to={Route::Components}>{ "Components" }</NavLink<Route>>
                    <NavLink<Route> to={Route::TabsDemo}>{ "Tabs" }</NavLink<Route>>
                    <NavLink<Route> to={Route::PaginationDemo}>{ "Pagination" }</NavLink<Route>>
                    <NavLink<Route> to={Route::DropdownDemo}>{ "Dropdown" }</NavLink<Route>>
                    <NavLink<Route> to={Route::HooksDemo}>{ "Hooks" }</NavLink<Route>>
                    <NavLink<Route> to={Route::UtilsDemo}>{ "Utils" }</NavLink<Route>>
                    <NavLink<Route> to={Route::Blog} partial=true>{ "Blog" }</NavLink<Route>>
                    <NavLink<Route> to={Route::Nested} partial=true>{ "Nested" }</NavLink<Route>>
                    <NavLink<Route> to={Route::QueryDemo}>{ "Query" }</NavLink<Route>>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn HomePage() -> Html {
    html! {
        <div class="container">
            <div class="page-header">
                <h1>{ "yew-nav-link 2026 Demo" }</h1>
                <p>{ "Comprehensive demonstration of all navigation components and utilities" }</p>
            </div>

            <div class="section">
                <h2 class="section-title">{ "Overview" }</h2>
                <p class="section-desc">
                    { "yew-nav-link provides enhanced navigation components for Yew applications with automatic active state detection, breadcrumbs, tabs, pagination, and more." }
                </p>

                <div class="demo-grid">
                    <div class="card">
                        <div class="card-title">{ "NavLink Component" }</div>
                        <p>{ "Automatic active state with Exact and Partial matching modes" }</p>
                        <div class="mt-1">
                            <NavLink<Route> to={Route::BasicLinks}>{ "Try Basic Demo →" }</NavLink<Route>>
                        </div>
                    </div>

                    <div class="card">
                        <div class="card-title">{ "UI Components" }</div>
                        <p>{ "Badges, icons, dropdowns, tabs, pagination, and more" }</p>
                        <div class="mt-1">
                            <NavLink<Route> to={Route::Components}>{ "Try Components →" }</NavLink<Route>>
                        </div>
                    </div>

                    <div class="card">
                        <div class="card-title">{ "Hooks" }</div>
                        <p>{ "use_is_active, use_navigation, use_query_params, use_breadcrumbs" }</p>
                        <div class="mt-1">
                            <NavLink<Route> to={Route::HooksDemo}>{ "Try Hooks →" }</NavLink<Route>>
                        </div>
                    </div>

                    <div class="card">
                        <div class="card-title">{ "URL Utilities" }</div>
                        <p>{ "Path normalization, joining, absolute URL detection" }</p>
                        <div class="mt-1">
                            <NavLink<Route> to={Route::UtilsDemo}>{ "Try Utils →" }</NavLink<Route>>
                        </div>
                    </div>
                </div>
            </div>

            <div class="section">
                <h2 class="section-title">{ "Quick Syntax Reference" }</h2>

                <div class="code-block">{r#"// Component syntax with NavLink
<NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>

// Partial matching for nested routes
<NavLink<Route> to={Route::Blog} partial=true>{ "Blog" }</NavLink<Route>>

// Function syntax for text-only links
{ nav_link(Route::About, "About", Match::Exact) }
{ nav_link(Route::Docs, "Docs", Match::Partial) }

// With custom attributes
<NavLink<Route>
    to={Route::Home}
    class="custom-class"
    style="font-weight: bold;"
>
    { "Styled Link" }
</NavLink<Route>>"#}</div>
            </div>
        </div>
    }
}

#[component]
fn BasicLinksPage() -> Html {
    let exact_active_basic = use_is_exact_active(Route::BasicLinks);
    let partial_active_blog = use_is_partial_active(Route::Blog);

    html! {
        <div class="container">
            <div class="page-header">
                <h1>{ "Basic NavLink Usage" }</h1>
                <p>{ "Learn the fundamental NavLink component with different matching modes" }</p>
            </div>

            <div class="section">
                <h2 class="section-title">{ "Component Syntax" }</h2>
                <p class="section-desc">
                    { "NavLink wraps yew-router's Link with automatic active state detection." }
                </p>

                <div class="demo-grid">
                    <div class="card">
                        <div class="card-title">{ "Exact Match (Default)" }</div>
                        <p>{ "Link is active only when route matches exactly" }</p>
                        <div class="mt-1 flex-col">
                            <NavLink<Route> to={Route::Home}>{ "Home (Exact)" }</NavLink<Route>>
                            <NavLink<Route> to={Route::BasicLinks}>{ "Basic Links (Exact)" }</NavLink<Route>>
                            <span class="section-desc">
                                { "Current: " }
                                { if exact_active_basic {
                                    html! { <span class="status-active">{ "Active" }</span> }
                                } else {
                                    html! { <span class="status-inactive">{ "Inactive" }</span> }
                                }}
                            </span>
                        </div>
                    </div>

                    <div class="card">
                        <div class="card-title">{ "Partial Match" }</div>
                        <p>{ "Link stays active for nested/child routes" }</p>
                        <div class="mt-1 flex-col">
                            <NavLink<Route> to={Route::Blog} partial=true>
                                { "Blog (Partial)" }
                            </NavLink<Route>>
                            <small class="section-desc">
                                { "Active on /blog, /blog/post-1, /blog/post-1/comments" }
                            </small>
                            <span class="section-desc">
                                { "Current: " }
                                { if partial_active_blog {
                                    html! { <span class="status-active">{ "Active" }</span> }
                                } else {
                                    html! { <span class="status-inactive">{ "Inactive" }</span> }
                                }}
                            </span>
                        </div>
                    </div>
                </div>

                <div class="code-block mt-1">{r#"// Exact match - active only on exact route
<NavLink<Route> to={Route::Home}>
    { "Home" }
</NavLink<Route>>

// Partial match - active on route and sub-routes
<NavLink<Route> to={Route::Blog} partial=true>
    { "Blog" }
</NavLink<Route>>
// Active on: /blog, /blog/*"#}</div>
            </div>

            <div class="section">
                <h2 class="section-title">{ "Function Syntax" }</h2>
                <p class="section-desc">
                    { "For simple text-only links, use the nav_link() function." }
                </p>

                <div class="demo-grid">
                    <div class="card">
                        <div class="card-title">{ "nav_link() Function" }</div>
                        <div class="flex-col">
                            { nav_link(Route::Home, "Home (Function, Exact)", Match::Exact) }
                            { nav_link(Route::BasicLinks, "Basic Links (Function, Exact)", Match::Exact) }
                            { nav_link(Route::Blog, "Blog (Function, Partial)", Match::Partial) }
                            { nav_link(Route::HooksDemo, "Hooks Demo (Function)", Match::Exact) }
                        </div>
                    </div>
                </div>

                <div class="code-block mt-1">{r#"use yew_nav_link::{nav_link, Match};

// Function syntax for text-only links
{ nav_link(Route::Home, "Home", Match::Exact) }
{ nav_link(Route::Docs, "Docs", Match::Partial) }

// Equivalent to:
// <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
// <NavLink<Route> to={Route::Docs} partial=true>{ "Docs" }</NavLink<Route>>"#}</div>
            </div>

            <div class="section">
                <h2 class="section-title">{ "Custom Styling" }</h2>
                <p class="section-desc">
                    { "NavLink applies 'nav-link' class always and 'active' when matched. Customize via CSS." }
                </p>

                <div class="code-block">{r#"// NavLink applies these CSS classes:
// - 'nav-link' - always applied
// - 'active' - when route matches

.nav-link {
    text-decoration: none;
    color: #64748b;
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
}

.nav-link:hover {
    color: #3b82f6;
    background: rgba(59, 130, 246, 0.05);
}

.nav-link.active {
    color: #3b82f6;
    background: rgba(59, 130, 246, 0.1);
    font-weight: 600;
}"#}</div>
            </div>
        </div>
    }
}

#[component]
fn ComponentsPage() -> Html {
    html! {
        <div class="container">
            <div class="page-header">
                <h1>{ "UI Components" }</h1>
                <p>{ "Badges, icons, headers, text, dividers, and dropdown components" }</p>
            </div>

            <div class="section">
                <h2 class="section-title">{ "NavBadge" }</h2>
                <p class="section-desc">{ "Display badges with counts, labels, or status indicators using children" }</p>

                <div class="demo-grid">
                    <div class="card">
                        <div class="card-title">{ "Badge Variants" }</div>
                        <div class="flex-col gap-1">
                            <div class="flex-row">
                                { "Messages " }
                                <NavBadge variant="primary">{ "5" }</NavBadge>
                            </div>
                            <div class="flex-row">
                                { "Notifications " }
                                <NavBadge variant="danger">{ "99" }</NavBadge>
                            </div>
                            <div class="flex-row">
                                { "Tasks " }
                                <NavBadge variant="success">{ "3" }</NavBadge>
                            </div>
                            <div class="flex-row">
                                { "Warnings " }
                                <NavBadge variant="warning">{ "12" }</NavBadge>
                            </div>
                            <div class="flex-row">
                                { "Pill style " }
                                <NavBadge variant="primary" pill=true>{ "New" }</NavBadge>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="code-block mt-1">{r#"use yew_nav_link::NavBadge;

// Badge with variant and content via children
<NavBadge variant="danger">{ "5" }</NavBadge>
<NavBadge variant="success" pill=true>{ "New" }</NavBadge>

// Usage in navigation
<NavLink<Route> to={Route::Home}>
    { "Messages " }
    <NavBadge variant="primary">{ "5" }</NavBadge>
</NavLink<Route>>"#}</div>
            </div>

            <div class="section">
                <h2 class="section-title">{ "NavIcon & NavLinkWithIcon" }</h2>
                <p class="section-desc">{ "Add icons to navigation links" }</p>

                <div class="demo-grid">
                    <div class="card">
                        <div class="card-title">{ "Icon with Name" }</div>
                        <div class="flex-col gap-1">
                            <div class="flex-row">
                                <NavIcon name={Some("⚙")} size={NavIconSize::Small} />
                                { " Small Icon" }
                            </div>
                            <div class="flex-row">
                                <NavIcon name={Some("⚙")} size={NavIconSize::Medium} />
                                { " Medium Icon" }
                            </div>
                            <div class="flex-row">
                                <NavIcon name={Some("⚙")} size={NavIconSize::Large} />
                                { " Large Icon" }
                            </div>
                        </div>
                    </div>

                    <div class="card">
                        <div class="card-title">{ "Icon with Children" }</div>
                        <div class="flex-col gap-1">
                            <div class="flex-row">
                                <NavIcon size={NavIconSize::Small}>{ "S" }</NavIcon>
                                { " Small" }
                            </div>
                            <div class="flex-row">
                                <NavIcon size={NavIconSize::Medium}>{ "M" }</NavIcon>
                                { " Medium" }
                            </div>
                            <div class="flex-row">
                                <NavIcon size={NavIconSize::Large}>{ "L" }</NavIcon>
                                { " Large" }
                            </div>
                        </div>
                    </div>

                    <div class="card">
                        <div class="card-title">{ "NavLinkWithIcon" }</div>
                        <p>{ "Wraps content with icon styling" }</p>
                        <div class="flex-col gap-1">
                            <NavLink<Route> to={Route::Home}>
                                <NavLinkWithIcon icon={NavIconSize::Small}>
                                    { "Home" }
                                </NavLinkWithIcon>
                            </NavLink<Route>>
                            <NavLink<Route> to={Route::Components}>
                                <NavLinkWithIcon icon={NavIconSize::Medium}>
                                    { "Components" }
                                </NavLinkWithIcon>
                            </NavLink<Route>>
                        </div>
                    </div>
                </div>

                <div class="code-block mt-1">{r#"use yew_nav_link::{NavIcon, NavIconSize, NavLinkWithIcon};

// Icon with name attribute
<NavIcon name={Some("🏠")} size={NavIconSize::Medium} />

// Icon with children
<NavIcon size={NavIconSize::Medium}>{ "⚙" }</NavIcon>

// NavLinkWithIcon wraps content with icon styling (no generic - it's not a router link)
<NavLinkWithIcon icon={NavIconSize::Small}>
    { "Link text" }
</NavLinkWithIcon>"#}</div>
            </div>

            <div class="section">
                <h2 class="section-title">{ "NavHeader & NavText" }</h2>

                <div class="demo-grid">
                    <div class="card">
                        <div class="card-title">{ "Header & Text" }</div>
                        <div class="flex-col gap-1">
                            <NavHeader>{ "Section Title" }</NavHeader>
                            <NavText text="Regular text content" />
                            <NavDivider />
                            <NavText text="More text below divider" />
                        </div>
                    </div>

                    <div class="card">
                        <div class="card-title">{ "In Navigation" }</div>
                        <NavList>
                            <NavHeader>{ "Main Menu" }</NavHeader>
                            <NavItem>
                                <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                            </NavItem>
                            <NavItem>
                                <NavLink<Route> to={Route::Components}>{ "Components" }</NavLink<Route>>
                            </NavItem>
                            <NavDivider />
                            <NavText text="Version 1.0" />
                        </NavList>
                    </div>
                </div>

                <div class="code-block mt-1">{r#"use yew_nav_link::{NavHeader, NavText, NavDivider, NavList, NavItem};

// Headers and text
<NavHeader>{ "Navigation" }</NavHeader>
<NavText text="Some description" />
<NavDivider />

// Inside navigation
<NavList>
    <NavHeader>{ "Menu" }</NavHeader>
    <NavItem>
        <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
    </NavItem>
    <NavText text="v1.0" />
</NavList>"#}</div>
            </div>

            <div class="section">
                <h2 class="section-title">{ "NavList & NavItem" }</h2>
                <p class="section-desc">{ "Structured navigation with lists and items" }</p>

                <div class="demo-grid">
                    <div class="card">
                        <div class="card-title">{ "Navigation List" }</div>
                        <NavList>
                            <NavItem>
                                <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                            </NavItem>
                            <NavItem>
                                <NavLink<Route> to={Route::Components}>{ "Components" }</NavLink<Route>>
                            </NavItem>
                            <NavDivider />
                            <NavItem>
                                <NavLink<Route> to={Route::HooksDemo}>{ "Hooks" }</NavLink<Route>>
                            </NavItem>
                        </NavList>
                    </div>
                </div>

                <div class="code-block mt-1">{r#"use yew_nav_link::{NavList, NavItem, NavDivider};

<NavList>
    <NavItem>
        <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
    </NavItem>
    <NavItem>
        <NavLink<Route> to={Route::About}>{ "About" }</NavLink<Route>>
    </NavItem>
    <NavDivider />
    <NavItem>
        <NavLink<Route> to={Route::Contact}>{ "Contact" }</NavLink<Route>>
    </NavItem>
</NavList>"#}</div>
            </div>
        </div>
    }
}

#[component]
fn TabsDemoPage() -> Html {
    let active_tab = use_state(|| 0u32);

    html! {
        <div class="container">
            <div class="page-header">
                <h1>{ "Tabs Component" }</h1>
                <p>{ "Tabbed navigation with NavTabs, NavTab, and NavTabPanel" }</p>
            </div>

            <div class="section">
                <h2 class="section-title">{ "Basic Tabs" }</h2>
                <p class="section-desc">{ "NavTabs provides tabbed interface. State is managed via active prop on each NavTab." }</p>

                <NavTabs id="demo-tabs">
                    <NavTab
                        active={*active_tab == 0}
                        onclick={Some(Callback::from({
                            let active_tab = active_tab.clone();
                            move |_: MouseEvent| { active_tab.set(0); }
                        }))}
                        panel_id={Some("panel-1")}
                    >
                        { "First Tab" }
                    </NavTab>
                    <NavTab
                        active={*active_tab == 1}
                        onclick={Some(Callback::from({
                            let active_tab = active_tab.clone();
                            move |_: MouseEvent| { active_tab.set(1); }
                        }))}
                        panel_id={Some("panel-2")}
                    >
                        { "Second Tab" }
                    </NavTab>
                    <NavTab
                        active={*active_tab == 2}
                        onclick={Some(Callback::from({
                            let active_tab = active_tab.clone();
                            move |_: MouseEvent| { active_tab.set(2); }
                        }))}
                        panel_id={Some("panel-3")}
                    >
                        { "Third Tab" }
                    </NavTab>
                </NavTabs>

                <div class="mt-1">
                    <NavTabPanel id={Some("panel-1")} labelled_by={Some("tab-1")} hidden={*active_tab != 0}>
                        <div class="card">
                            <div class="card-title">{ "Content Panel 1" }</div>
                            <p>{ "This is the content for the first tab. NavTabPanel only renders when its hidden prop is false." }</p>
                        </div>
                    </NavTabPanel>
                    <NavTabPanel id={Some("panel-2")} labelled_by={Some("tab-2")} hidden={*active_tab != 1}>
                        <div class="card">
                            <div class="card-title">{ "Content Panel 2" }</div>
                            <p>{ "Content for the second tab. Tab state is managed by setting the active prop on NavTab and hidden prop on NavTabPanel." }</p>
                        </div>
                    </NavTabPanel>
                    <NavTabPanel id={Some("panel-3")} labelled_by={Some("tab-3")} hidden={*active_tab != 2}>
                        <div class="card">
                            <div class="card-title">{ "Content Panel 3" }</div>
                            <p>{ "Content for the third tab. You control which panel is visible via the hidden prop." }</p>
                        </div>
                    </NavTabPanel>
                </div>
            </div>

            <div class="section">
                <h2 class="section-title">{ "Full Width Tabs" }</h2>
                <NavTabs full_width=true>
                    <NavTab
                        active={*active_tab == 0}
                        onclick={Some(Callback::from({
                            let active_tab = active_tab.clone();
                            move |_: MouseEvent| { active_tab.set(0); }
                        }))}
                    >
                        { "Overview" }
                    </NavTab>
                    <NavTab
                        active={*active_tab == 1}
                        onclick={Some(Callback::from({
                            let active_tab = active_tab.clone();
                            move |_: MouseEvent| { active_tab.set(1); }
                        }))}
                    >
                        { "Details" }
                    </NavTab>
                    <NavTab
                        active={*active_tab == 2}
                        onclick={Some(Callback::from({
                            let active_tab = active_tab.clone();
                            move |_: MouseEvent| { active_tab.set(2); }
                        }))}
                        disabled=true
                    >
                        { "Disabled" }
                    </NavTab>
                </NavTabs>
            </div>

            <div class="section">
                <h2 class="section-title">{ "Code Example" }</h2>
                <div class="code-block">{r#"use yew_nav_link::{NavTabs, NavTab, NavTabPanel};
use yew::prelude::*;

#[component]
fn TabsExample() -> Html {
    let active_tab = use_state(|| 0u32);

    html! {
        <NavTabs id="my-tabs">
            <NavTab
                active={*active_tab == 0}
                onclick={Some(Callback::from({
                    let active_tab = active_tab.clone();
                    move |_: MouseEvent| { active_tab.set(0); }
                }))}
                panel_id={Some("panel-1")}
            >
                { "Tab 1" }
            </NavTab>

            <NavTabPanel id={Some("panel-1")} hidden={*active_tab != 0}>
                <div>{ "Content 1" }</div>
            </NavTabPanel>
        </NavTabs>
    }
}"#}</div>
            </div>

            <div class="info-box">
                <p>{ "Note: NavTabs doesn't manage state internally. You must control the active prop on NavTab and hidden prop on NavTabPanel." }</p>
            </div>
        </div>
    }
}

#[component]
fn PaginationDemoPage() -> Html {
    let current_page = use_state(|| 1u32);

    let on_page_change = {
        let current_page = current_page.clone();
        Callback::from(move |page: u32| {
            current_page.set(page);
        })
    };

    html! {
        <div class="container">
            <div class="page-header">
                <h1>{ "Pagination Component" }</h1>
                <p>{ "Page navigation with Pagination component" }</p>
            </div>

            <div class="section">
                <h2 class="section-title">{ "Pagination Demo" }</h2>
                <p class="section-desc">
                    { "Current page: " }{ *current_page }
                </p>

                <Pagination
                    current_page={*current_page}
                    total_pages={10}
                    siblings={2}
                    show_first_last=true
                    on_page_change={Some(on_page_change.clone())}
                />

                <div class="mt-1">
                    <p class="section-desc">{ "The Pagination component handles all page rendering automatically including:" }</p>
                    <ul class="mt-1">
                        <li>{ "First/last page buttons (when show_first_last=true)" }</li>
                        <li>{ "Prev/next navigation buttons" }</li>
                        <li>{ "Siblings configuration for pages around current" }</li>
                        <li>{ "Ellipsis (...) for skipped pages" }</li>
                    </ul>
                </div>
            </div>

            <div class="section">
                <h2 class="section-title">{ "Code Example" }</h2>
                <div class="code-block">{r#"use yew_nav_link::Pagination;

#[component]
fn PaginationExample() -> Html {
    let current_page = use_state(|| 1u32);

    let on_page_change = {
        let current_page = current_page.clone();
        Callback::from(move |page: u32| {
            current_page.set(page);
        })
    };

    html! {
        <Pagination
            current_page={*current_page}
            total_pages={20}
            siblings={2}
            show_prev_next={true}
            show_first_last={true}
            on_page_change={Some(on_page_change)}
        />
    }
}"#}</div>
            </div>

            <div class="info-box">
                <p>{ "Note: Pagination is a self-contained component. It renders all page buttons, prev/next, and first/last buttons based on the props you provide." }</p>
            </div>
        </div>
    }
}

#[component]
fn DropdownDemoPage() -> Html {
    html! {
        <div class="container">
            <div class="page-header">
                <h1>{ "Dropdown Component" }</h1>
                <p>{ "NavDropdown with items, dividers, and headers" }</p>
            </div>

            <div class="section">
                <h2 class="section-title">{ "Dropdown Demo" }</h2>
                <p class="section-desc">{ "NavDropdown manages its own open/close state internally" }</p>

                <div class="demo-grid">
                    <div class="card">
                        <div class="card-title">{ "Basic Dropdown" }</div>
                        <NavList>
                            <NavDropdown toggle_text="Menu">
                                <NavDropdownItem>
                                    <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                                </NavDropdownItem>
                                <NavDropdownItem>
                                    <NavLink<Route> to={Route::Components}>{ "Components" }</NavLink<Route>>
                                </NavDropdownItem>
                                <NavDropdownDivider />
                                <NavDropdownItem>
                                    <NavLink<Route> to={Route::HooksDemo}>{ "Hooks" }</NavLink<Route>>
                                </NavDropdownItem>
                            </NavDropdown>
                        </NavList>
                    </div>

                    <div class="card">
                        <div class="card-title">{ "With Disabled Items" }</div>
                        <NavList>
                            <NavDropdown toggle_text="Options">
                                <NavDropdownItem>
                                    <NavLink<Route> to={Route::Home}>{ "Profile" }</NavLink<Route>>
                                </NavDropdownItem>
                                <NavDropdownItem disabled=true>
                                    { "Admin (Disabled)" }
                                </NavDropdownItem>
                                <NavDropdownDivider />
                                <NavDropdownItem>
                                    <NavLink<Route> to={Route::UtilsDemo}>{ "Settings" }</NavLink<Route>>
                                </NavDropdownItem>
                            </NavDropdown>
                        </NavList>
                    </div>
                </div>

                <div class="code-block mt-1">{r#"use yew_nav_link::{NavDropdown, NavDropdownItem, NavDropdownDivider, NavList};

<NavList>
    <NavDropdown toggle_text="Menu">
        <NavDropdownItem>
            <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
        </NavDropdownItem>
        <NavDropdownDivider />
        <NavDropdownItem disabled=true>
            { "Disabled Item" }
        </NavDropdownItem>
    </NavDropdown>
</NavList>"#}</div>
            </div>

            <div class="info-box">
                <p>{ "Note: NavDropdown manages its own open/close state. Just provide toggle_text and children." }</p>
            </div>
        </div>
    }
}

#[component]
fn HooksDemoPage() -> Html {
    let current_route: Option<Route> = use_route_info::<Route>();
    let is_active_home = use_is_active(Route::Home);
    let is_exact_active_home = use_is_exact_active(Route::Home);
    let is_exact_active_hooks = use_is_exact_active(Route::HooksDemo);
    let is_partial_active_blog = use_is_partial_active(Route::Blog);
    let breadcrumbs: Vec<BreadcrumbItem<Route>> = use_breadcrumbs();
    let query_params = use_query_params();
    let navigation = use_navigation::<Route>();

    html! {
        <div class="container">
            <div class="page-header">
                <h1>{ "Hooks Demo" }</h1>
                <p>{ "Reactive hooks for route state, navigation, and breadcrumbs" }</p>
            </div>

            <div class="section">
                <h2 class="section-title">{ "use_route_info" }</h2>
                <p class="section-desc">{ "Get the current route (returns Option<Route>)" }</p>

                <div class="card">
                    <table>
                        <tr>
                            <th>{ "Property" }</th>
                            <th>{ "Value" }</th>
                        </tr>
                        <tr>
                            <td>{ "Current Route" }</td>
                            <td>{ format!("{:?}", current_route) }</td>
                        </tr>
                        <tr>
                            <td>{ "Is Some" }</td>
                            <td>
                                { if current_route.is_some() {
                                    html! { <span class="status-active">{ "Yes" }</span> }
                                } else {
                                    html! { <span class="status-inactive">{ "No" }</span> }
                                }}
                            </td>
                        </tr>
                    </table>
                </div>

                <div class="code-block mt-1">{r#"use yew_nav_link::use_route_info;

// Returns Option<Route> - the current matched route
let current_route: Option<Route> = use_route_info::<Route>();

if let Some(route) = current_route {
    // Use the route
    let path = route.to_path();
}"#}</div>
            </div>

            <div class="section">
                <h2 class="section-title">{ "use_is_active Hooks" }</h2>
                <p class="section-desc">{ "These hooks take a route and return bool" }</p>

                <div class="demo-grid">
                    <div class="card">
                        <div class="card-title">{ "use_is_active" }</div>
                        <p>{ "Check if a specific route is active (exact match)" }</p>
                        <div class="mt-1">
                            { "Home active: " }
                            { if is_active_home {
                                html! { <span class="status-active">{ "Yes" }</span> }
                            } else {
                                html! { <span class="status-inactive">{ "No" }</span> }
                            }}
                        </div>
                    </div>

                    <div class="card">
                        <div class="card-title">{ "use_is_exact_active" }</div>
                        <p>{ "Same as use_is_active - exact matching" }</p>
                        <div class="mt-1 flex-col">
                            <div>
                                { "Home: " }
                                { if is_exact_active_home {
                                    html! { <span class="status-active">{ "Active" }</span> }
                                } else {
                                    html! { <span class="status-inactive">{ "Inactive" }</span> }
                                }}
                            </div>
                            <div>
                                { "HooksDemo: " }
                                { if is_exact_active_hooks {
                                    html! { <span class="status-active">{ "Active" }</span> }
                                } else {
                                    html! { <span class="status-inactive">{ "Inactive" }</span> }
                                }}
                            </div>
                        </div>
                    </div>

                    <div class="card">
                        <div class="card-title">{ "use_is_partial_active" }</div>
                        <p>{ "For partial matching (e.g., Blog section)" }</p>
                        <div class="mt-1">
                            { "Blog: " }
                            { if is_partial_active_blog {
                                html! { <span class="status-active">{ "Active" }</span> }
                            } else {
                                html! { <span class="status-inactive">{ "No" }</span> }
                            }}
                        </div>
                    </div>
                </div>

                <div class="code-block mt-1">{r#"use yew_nav_link::{use_is_active, use_is_exact_active, use_is_partial_active};

// All these hooks take a route as argument and return bool
let is_home = use_is_active(Route::Home);           // exact match
let is_home = use_is_exact_active(Route::Home);    // exact match (alias)
let is_blog = use_is_partial_active(Route::Blog);  // partial match"#}</div>
            </div>

            <div class="section">
                <h2 class="section-title">{ "use_breadcrumbs" }</h2>
                <p class="section-desc">{ "Generate breadcrumb trail from current route" }</p>

                <div class="breadcrumb-demo">
                    <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                    <span class="breadcrumb-separator">{ " / " }</span>
                    <span>{ "Breadcrumbs Demo (Dynamic breadcrumbs temporarily simplified)" }</span>
                    <span class="breadcrumb-separator">{ " / " }</span>
                    <span>{ "Current Page" }</span>
                </div>
                <div class="mt-1">
                    <p class="section-desc">{ "Breadcrumbs: " }{ breadcrumbs.len() }{ " items (use_breadcrumbs() returns Vec<BreadcrumbItem<Route>>)" }</p>
                </div>

                <div class="code-block mt-1">{r#"use yew_nav_link::{use_breadcrumbs, BreadcrumbItem};

let breadcrumbs: Vec<BreadcrumbItem<Route>> = use_breadcrumbs();

// BreadcrumbItem provides:
// - route: Route - the route for this breadcrumb
// - label: String - display label
// - is_active: bool - whether this is the current page"#}</div>
            </div>

            <div class="section">
                <h2 class="section-title">{ "use_query_params" }</h2>
                <p class="section-desc">{ "Access URL query parameters reactively" }</p>

                <div class="card">
                    <p>{ "Current query params: " }</p>
                    <pre>{ format!("{:#?}", query_params) }</pre>
                    <p class="section-desc mt-1">
                        { "Try adding ?key=value to the URL" }
                    </p>
                </div>

                <div class="code-block mt-1">{r#"use yew_nav_link::use_query_params;

let query_params: std::collections::HashMap<String, String> = use_query_params();

// Access params
let search = query_params.get("q");
let page = query_params.get("page");"#}</div>
            </div>

            <div class="section">
                <h2 class="section-title">{ "use_navigation" }</h2>
                <p class="section-desc">{ "Programmatic navigation with Navigation struct" }</p>

                <div class="demo-grid">
                    <div class="card">
                        <div class="card-title">{ "Navigation Methods" }</div>
                        <div class="flex-col gap-1">
                            <button
                                onclick={Callback::from({
                                    let nav = navigation.clone();
                                    move |_| { nav.push_callback(Route::Home).emit(()); }
                                })}
                            >
                                { "Go to Home" }
                            </button>
                            <button
                                onclick={Callback::from({
                                    let nav = navigation.clone();
                                    move |_| { nav.push_callback(Route::BasicLinks).emit(()); }
                                })}
                            >
                                { "Go to Basic Links" }
                            </button>
                            <button
                                onclick={Callback::from({
                                    let nav = navigation.clone();
                                    move |_| { nav.go_back.emit(()); }
                                })}
                            >
                                { "Go Back" }
                            </button>
                            <button
                                onclick={Callback::from({
                                    let nav = navigation.clone();
                                    move |_| { nav.go_forward.emit(()); }
                                })}
                            >
                                { "Go Forward" }
                            </button>
                        </div>
                    </div>
                </div>

                <div class="code-block mt-1">{r#"use yew_nav_link::{use_navigation, Navigation};

let navigation: Navigation<Route> = use_navigation();

// Navigation provides callbacks:
// - go_back: Callback<()> - navigate back
// - go_forward: Callback<()> - navigate forward
// - push_callback(route): Callback<()> - navigate to route
// - replace_callback(route): Callback<()> - replace current entry
// - go_callback(delta): Callback<()> - go with delta"#}</div>
            </div>
        </div>
    }
}

#[component]
fn UtilsDemoPage() -> Html {
    let test_path1 = "/foo/bar/";
    let test_path2 = "/baz/qux";
    let joined = join_paths(test_path1, test_path2);
    let normalized = normalize_path("/foo/bar/../baz/");
    let is_abs = is_absolute("https://example.com/path");
    let is_rel = is_absolute("/relative/path");

    html! {
        <div class="container">
            <div class="page-header">
                <h1>{ "URL Utilities" }</h1>
                <p>{ "Path manipulation and URL helper functions" }</p>
            </div>

            <div class="section">
                <h2 class="section-title">{ "join_paths" }</h2>
                <p class="section-desc">{ "Join two path segments, handling slashes correctly" }</p>

                <div class="card">
                    <table>
                        <tr>
                            <th>{ "Input 1" }</th>
                            <th>{ "Input 2" }</th>
                            <th>{ "Result" }</th>
                        </tr>
                        <tr>
                            <td><code>{ test_path1 }</code></td>
                            <td><code>{ test_path2 }</code></td>
                            <td><code>{ joined }</code></td>
                        </tr>
                        <tr>
                            <td><code>{ "/a/b/" }</code></td>
                            <td><code>{ "/c/d" }</code></td>
                            <td><code>{ join_paths("/a/b/", "/c/d") }</code></td>
                        </tr>
                        <tr>
                            <td><code>{ "foo" }</code></td>
                            <td><code>{ "bar" }</code></td>
                            <td><code>{ join_paths("foo", "bar") }</code></td>
                        </tr>
                    </table>
                </div>

                <div class="code-block mt-1">{r#"use yew_nav_link::join_paths;

let joined = join_paths("/foo/bar/", "/baz/qux");
// Result: "/foo/bar/baz/qux"

let joined2 = join_paths("foo", "bar");
// Result: "foo/bar""#}</div>
            </div>

            <div class="section">
                <h2 class="section-title">{ "normalize_path" }</h2>
                <p class="section-desc">{ "Normalize path by resolving .. and . segments" }</p>

                <div class="card">
                    <table>
                        <tr>
                            <th>{ "Input" }</th>
                            <th>{ "Result" }</th>
                        </tr>
                        <tr>
                            <td><code>{ "/foo/bar/../baz/" }</code></td>
                            <td><code>{ normalized }</code></td>
                        </tr>
                        <tr>
                            <td><code>{ "/a/b/./c" }</code></td>
                            <td><code>{ normalize_path("/a/b/./c") }</code></td>
                        </tr>
                        <tr>
                            <td><code>{ "/a/b/c/../../d" }</code></td>
                            <td><code>{ normalize_path("/a/b/c/../../d") }</code></td>
                        </tr>
                    </table>
                </div>

                <div class="code-block mt-1">{r#"use yew_nav_link::normalize_path;

let normalized = normalize_path("/foo/bar/../baz/");
// Result: "/foo/baz/""#}</div>
            </div>

            <div class="section">
                <h2 class="section-title">{ "is_absolute" }</h2>
                <p class="section-desc">{ "Check if a URL is absolute (has scheme)" }</p>

                <div class="card">
                    <table>
                        <tr>
                            <th>{ "URL" }</th>
                            <th>{ "Is Absolute" }</th>
                        </tr>
                        <tr>
                            <td><code>{ "https://example.com/path" }</code></td>
                            <td>
                                { if is_abs {
                                    html! { <span class="status-active">{ "Yes" }</span> }
                                } else {
                                    html! { <span class="status-inactive">{ "No" }</span> }
                                }}
                            </td>
                        </tr>
                        <tr>
                            <td><code>{ "/relative/path" }</code></td>
                            <td>
                                { if is_rel {
                                    html! { <span class="status-active">{ "Yes" }</span> }
                                } else {
                                    html! { <span class="status-inactive">{ "No" }</span> }
                                }}
                            </td>
                        </tr>
                    </table>
                </div>

                <div class="code-block mt-1">{r#"use yew_nav_link::is_absolute;

let is_abs = is_absolute("https://example.com/path");
// true

let is_rel = is_absolute("/relative/path");
// false"#}</div>
            </div>
        </div>
    }
}

#[component]
fn BlogPage() -> Html {
    let posts = vec![
        ("getting-started", "Getting Started with Yew"),
        ("advanced-patterns", "Advanced Yew Patterns"),
        ("wasm-optimization", "WASM Optimization Tips"),
    ];

    html! {
        <div class="container">
            <div class="page-header">
                <h1>{ "Blog" }</h1>
                <p>{ "Nested routes demonstration - this page stays active on /blog and sub-routes" }</p>
            </div>

            <div class="info-box">
                <p>{ "The 'Blog' nav link uses partial=true, so it stays active on /blog, /blog/post-1, /blog/post-1/comments, etc." }</p>
            </div>

            <div class="section">
                <h2 class="section-title">{ "Posts" }</h2>
                <table>
                    <thead>
                        <tr>
                            <th>{ "Title" }</th>
                            <th>{ "Link" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { posts.into_iter().map(|(id, title)| {
                            html! {
                                <tr>
                                    <td>{ title }</td>
                                    <td>
                                        <NavLink<Route> to={Route::BlogPost { id: id.to_string() }}>
                                            { "Read →" }
                                        </NavLink<Route>>
                                    </td>
                                </tr>
                            }
                        }).collect::<Html>() }
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
fn BlogPostPage() -> Html {
    let route = use_route::<Route>();
    let post_id = match route {
        Some(Route::BlogPost {
            id
        }) => id,
        _ => "unknown".to_string()
    };

    html! {
        <div class="container">
            <div class="page-header">
                <h1>{ format!("Blog Post: {}", post_id) }</h1>
                <p>{ "Notice the 'Blog' nav link is still active (partial match)" }</p>
            </div>

            <div class="section">
                <p>{ "This page demonstrates partial matching. The Blog link in the main navigation remains active because it uses partial=true." }</p>

                <div class="card mt-1">
                    <div class="card-title">{ "Post Content" }</div>
                    <p>{ format!("This is the content for post: {}", post_id) }</p>
                    <div class="mt-1">
                        <NavLink<Route> to={Route::Blog}>{ "← Back to Blog" }</NavLink<Route>>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn NestedPage() -> Html {
    html! {
        <div class="container">
            <div class="page-header">
                <h1>{ "Nested Routes Demo" }</h1>
                <p>{ "Demonstrate nested navigation with partial matching" }</p>
            </div>

            <div class="info-box">
                <p>{ "The 'Nested' nav link uses partial=true and stays active on all sub-routes." }</p>
            </div>

            <div class="section">
                <h2 class="section-title">{ "Sub-Navigation" }</h2>
                <NavList>
                    <NavItem>
                        <NavLink<Route> to={Route::NestedFirst}>{ "First Sub-Page" }</NavLink<Route>>
                    </NavItem>
                    <NavItem>
                        <NavLink<Route> to={Route::NestedSecond}>{ "Second Sub-Page" }</NavLink<Route>>
                    </NavItem>
                </NavList>
            </div>

            <div class="section">
                <Switch<Route> render={|route: Route| {
                    match route {
                        Route::NestedFirst | Route::NestedSecond => html! {
                            <div class="card">
                                <h3>{ "Sub-Page Content" }</h3>
                                <p>{ "This content changes based on the nested route." }</p>
                            </div>
                        },
                        _ => html! {},
                    }
                }} />
            </div>
        </div>
    }
}

#[component]
fn QueryDemoPage() -> Html {
    let query_params = use_query_params();

    html! {
        <div class="container">
            <div class="page-header">
                <h1>{ "Query Parameters" }</h1>
                <p>{ "Demonstrate URL query parameter handling" }</p>
            </div>

            <div class="section">
                <h2 class="section-title">{ "Current Query Params" }</h2>
                <div class="card">
                    <pre>{ format!("{:#?}", query_params) }</pre>
                </div>

                <div class="info-box mt-1">
                    <p>{ "Try adding query parameters to the URL, e.g.: ?search=test&page=2" }</p>
                </div>

                <div class="code-block mt-1">{r#"// Access query params with use_query_params hook
let query_params: HashMap<String, String> = use_query_params();

// Example URL: /query?search=rust&page=2
// query_params = {"search": "rust", "page": "2"}"#}</div>
            </div>
        </div>
    }
}

// ============ APP ============

#[component]
fn App() -> Html {
    html! {
        <>
            { global_styles() }
            <BrowserRouter>
                <div class="app-container">
                    <Navigation />
                    <Switch<Route> render={|route: Route| {
                        match route {
                            Route::Home => html! { <HomePage/> },
                            Route::BasicLinks => html! { <BasicLinksPage/> },
                            Route::Components => html! { <ComponentsPage/> },
                            Route::TabsDemo => html! { <TabsDemoPage/> },
                            Route::PaginationDemo => html! { <PaginationDemoPage/> },
                            Route::DropdownDemo => html! { <DropdownDemoPage/> },
                            Route::HooksDemo => html! { <HooksDemoPage/> },
                            Route::UtilsDemo => html! { <UtilsDemoPage/> },
                            Route::Blog | Route::BlogPost { .. } => html! { <BlogPage/> },
                            Route::Nested | Route::NestedFirst | Route::NestedSecond => html! { <NestedPage/> },
                            Route::QueryDemo => html! { <QueryDemoPage/> },
                        }
                    }} />
                </div>
            </BrowserRouter>
        </>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
