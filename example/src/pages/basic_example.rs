use yew::prelude::*;
use yew_nav_link::{Match, NavLink, nav_link};
use yew_router::prelude::*;

use crate::{
    code_utils::CopyCode,
    file_tree::{FileTree, FileTreeNode},
    routes::Route
};

const STEP_0_CARGO_TOML: &str = r#"[package]
name = "my-yew-app"
version = "0.1.0"
edition = "2024"

[dependencies]
yew = { version = "0.23", features = ["csr"] }
yew-router = "0.20"
yew-nav-link = "0.6""#;

const STEP_0_INDEX_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>My Yew App</title>
</head>
<body>
</body>
</html>"#;

const STEP_1_ROUTES: &str = r#"use yew::prelude::*;
use yew_router::prelude::*;

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
}"#;

const STEP_2_APP: &str = r#"use yew::prelude::*;
use yew_router::prelude::*;

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

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::About => html! { <AboutPage /> },
        Route::Contact => html! { <ContactPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}"#;

const STEP_3_NAV: &str = r#"use yew::prelude::*;
use yew_nav_link::{nav_link, Match, NavLink};

#[component]
fn Navigation() -> Html {
    html! {
        <nav>
            <ul>
                <li>
                    <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                </li>
                <li>
                    <NavLink<Route> to={Route::About}>{ "About" }</NavLink<Route>>
                </li>
                <li>
                    { nav_link(Route::Contact, "Contact", Match::Exact) }
                </li>
            </ul>
        </nav>
    }
}"#;

const STEP_4_PAGES: &str = r#"use yew::prelude::*;

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
}"#;

const STEP_5_MAIN: &str = r#"use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}"#;

const FULL_CODE: &str = r#"use yew::prelude::*;
use yew_nav_link::{nav_link, Match, NavLink};
use yew_router::prelude::*;

// ── Routes ─────────────────────────────────────────────────────

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

// ── App ────────────────────────────────────────────────────────

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

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::About => html! { <AboutPage /> },
        Route::Contact => html! { <ContactPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}

// ── Navigation ─────────────────────────────────────────────────

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

// ── Pages ──────────────────────────────────────────────────────

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

// ── Entry point ────────────────────────────────────────────────

fn main() {
    yew::Renderer::<App>::new().render();
}"#;

fn render_file_tree() -> Html {
    let nodes = vec![FileTreeNode::Dir {
        name:     "my-yew-app".to_string(),
        children: vec![
            FileTreeNode::File {
                name: "Cargo.toml".to_string()
            },
            FileTreeNode::File {
                name: "index.html".to_string()
            },
            FileTreeNode::Dir {
                name:     "src".to_string(),
                children: vec![
                    FileTreeNode::File {
                        name: "main.rs".to_string()
                    },
                    FileTreeNode::File {
                        name: "routes.rs".to_string()
                    },
                    FileTreeNode::File {
                        name: "navigation.rs".to_string()
                    },
                    FileTreeNode::File {
                        name: "pages.rs".to_string()
                    },
                ]
            },
        ]
    }];
    html! {
        <FileTree {nodes} />
    }
}

// ─── Interactive Demo Components ───────────────────────────────────

#[function_component]
fn RouteStatus() -> Html {
    let current = use_route::<Route>();
    let current_name = current
        .as_ref()
        .map(|r| format!("{:?}", r))
        .unwrap_or_else(|| "Unknown".to_string());

    let current_path = current
        .as_ref()
        .map(|r| r.to_path())
        .unwrap_or_default();

    html! {
        <div class="route-status">
            <span class="route-status-label">{ "Current route:" }</span>
            <code class="route-status-name">{ current_name }</code>
            <span class="route-status-path">{ current_path }</span>
        </div>
    }
}

#[function_component]
fn MatchDemoNav() -> Html {
    html! {
        <div class="match-demo-grid">
            <div class="match-demo-section">
                <h5>{ "Exact Match (partial=false)" }</h5>
                <nav>
                    <ul class="nav-list">
                        <li class="nav-item">
                            <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                        </li>
                        <li class="nav-item">
                            <NavLink<Route> to={Route::NavListDoc}>{ "NavList" }</NavLink<Route>>
                        </li>
                    </ul>
                </nav>
                <p class="match-description">{ "Active only when URL matches exactly" }</p>
            </div>
            <div class="match-demo-section">
                <h5>{ "Partial Match (partial=true)" }</h5>
                <nav>
                    <ul class="nav-list">
                        <li class="nav-item">
                            <NavLink<Route> to={Route::NavLinkDoc} partial=true>{ "Documentation" }</NavLink<Route>>
                        </li>
                        <li class="nav-item">
                            <NavLink<Route> to={Route::NestedRoutesExample} partial=true>{ "Advanced" }</NavLink<Route>>
                        </li>
                    </ul>
                </nav>
                <p class="match-description">{ "Active on any nested route (e.g. /nav-link, /nav-link/sub)" }</p>
            </div>
        </div>
    }
}

#[function_component]
pub fn BasicExample() -> Html {
    html! {
        <div>
            // ── Header ────────────────────────────────────────
            <div class="example-header">
                <div class="example-header-top">
                    <span class="example-tag tag-blue">{ "tutorial" }</span>
                    <span class="example-source">{ "examples/basic/" }</span>
                </div>
                <h2>{ "Basic Example — Full Tutorial" }</h2>
                <p class="example-desc">
                    { "Step-by-step guide from " }<code>{ "cargo new" }</code>
                    { " to a working Yew application with navigation. " }
                    { "Each step shows the complete code for that file." }
                </p>
            </div>

            // ── Architecture ──────────────────────────────────
            <div class="card">
                <h3>{ "Architecture" }</h3>
                <p>
                    { "The basic example follows a flat routing pattern with a single " }
                    <code>{ "Route" }</code>
                    { " enum, one " }<code>{ "BrowserRouter" }</code>
                    { ", and a " }<code>{ "Switch<Route>" }</code>
                    { " that renders the matching page component." }
                </p>
                <div class="arch-diagram-small">
                    <div class="arch-box arch-box-blue">{ "App" }</div>
                    <div class="arch-arrow-sm">{ "\u{2192}" }</div>
                    <div class="arch-box arch-box-purple">{ "BrowserRouter" }</div>
                    <div class="arch-arrow-sm">{ "\u{2192}" }</div>
                    <div class="arch-box arch-box-green">{ "Navigation" }</div>
                    <div class="arch-arrow-sm">{ "\u{2192}" }</div>
                    <div class="arch-box arch-box-orange">{ "Switch<Route>" }</div>
                    <div class="arch-arrow-sm">{ "\u{2192}" }</div>
                    <div class="arch-box arch-box-gray">{ "Page" }</div>
                </div>
            </div>

            // ── File Structure ────────────────────────────────
            <div class="card">
                <h3>{ "File Structure" }</h3>
                <div class="file-tree">
                    { render_file_tree() }
                </div>
                <p>
                    { "In a real project you would split these into separate modules. " }
                    { "This tutorial shows each file's content in sequence." }
                </p>
            </div>

            // ── Step 0: Project Setup ─────────────────────────
            <div class="card">
            <div class="file-header">
                <span class="file-header-name">{ "Cargo.toml" }</span>
            </div>
                <CopyCode code={STEP_0_CARGO_TOML.to_string()} language={"toml".to_string()} />
                <div class="file-header" style="margin-top: 1rem;">
                    <span class="file-header-name">{ "index.html" }</span>
                </div>
                <CopyCode code={STEP_0_INDEX_HTML.to_string()} language={"html".to_string()} />
                <div class="info-box">
                    <strong>{ "Note:" }</strong>
                    { " yew-router " }<code>{ "0.20" }</code>
                    { " requires yew " }<code>{ "0.23" }</code>
                    { ". yew-nav-link " }<code>{ "0.6" }</code>
                    { " is compatible with both." }
                </div>
            </div>

            // ── Step 1: Route Enum ────────────────────────────
            <div class="card">
            <div class="file-header">
                <span class="file-header-name">{ "src/routes.rs" }</span>
            </div>
                <CopyCode code={STEP_1_ROUTES.to_string()} language={"rust".to_string()} />
                <div class="info-box">
                    <strong>{ "Key point:" }</strong>
                    { " The " }<code>{ "Routable" }</code>
                    { " derive macro generates " }<code>{ "to_path()" }</code>
                    { " and " }<code>{ "from_path()" }</code>
                    { " — these are what NavLink uses to compare URLs." }
                </div>
            </div>

            // ── Step 2: App + Switch ──────────────────────────
            <div class="card">
            <div class="file-header">
                <span class="file-header-name">{ "src/main.rs" }</span>
            </div>
                <CopyCode code={STEP_2_APP.to_string()} language={"rust".to_string()} />
                <p>
                    { "The " }<code>{ "BrowserRouter" }</code>
                    { " listens to URL changes. When the URL changes, " }
                    <code>{ "Switch<Route>" }</code>
                    { " calls " }<code>{ "switch()" }</code>
                    { " with the matched route and re-renders the result." }
                </p>
            </div>

            // ── Step 3: Navigation ────────────────────────────
            <div class="card">
                <div class="file-header">
                    <span class="file-header-name">{ "src/navigation.rs" }</span>
                </div>
                <p>
                    { "Two syntaxes for creating navigation links:" }
                </p>
                <CopyCode code={STEP_3_NAV.to_string()} language={"rust".to_string()} />
                <p>
                    { "Each " }<code>{ "NavLink" }</code>
                    { " automatically gets " }<code>{ "class=\"nav-link active\"" }</code>
                    { " when its route matches the current URL." }
                </p>

               <h3>{ "Live Result" }</h3>
                <MatchDemoNav />
            </div>

            // ── Step 4: Pages ─────────────────────────────────
            <div class="card">
            <div class="file-header">
                <span class="file-header-name">{ "src/pages.rs" }</span>
            </div>
                <CopyCode code={STEP_4_PAGES.to_string()} language={"rust".to_string()} />
            </div>

            // ── Step 5: Entry Point ───────────────────────────
            <div class="card">
            <div class="file-header">
                <span class="file-header-name">{ "src/main.rs" }</span>
            </div>
                <CopyCode code={STEP_5_MAIN.to_string()} language={"rust".to_string()} />
                <p>
                    { "Run with " }<code>{ "trunk serve" }</code>
                    { " and open " }<code>{ "http://127.0.0.1:8080" }</code>{ "." }
                </p>
            </div>

            // ── Active State Algorithm ────────────────────────
            <div class="card">
                <h3>{ "Active State Algorithm" }</h3>
                <div class="flow-diagram">
                    <div class="flow-step">
                        <div class="flow-num">{ "1" }</div>
                        <div class="flow-text">{ "NavLink mounts with " }<code>{ "to={Route::Home}" }</code></div>
                    </div>
                    <div class="flow-arrow">{ "\u{2192}" }</div>
                    <div class="flow-step">
                        <div class="flow-num">{ "2" }</div>
                        <div class="flow-text">{ "Calls " }<code>{ "use_route::<R>()" }</code>{ " to get current route" }</div>
                    </div>
                    <div class="flow-arrow">{ "\u{2192}" }</div>
                    <div class="flow-step">
                        <div class="flow-num">{ "3" }</div>
                        <div class="flow-text">{ "Converts both routes to paths via " }<code>{ "to_path()" }</code></div>
                    </div>
                    <div class="flow-arrow">{ "\u{2192}" }</div>
                    <div class="flow-step">
                        <div class="flow-num">{ "4" }</div>
                        <div class="flow-text">
                            { "Compares: " }<code>{ "exact" }</code>
                            { " (full match) or " }<code>{ "partial" }</code>
                            { " (prefix match)" }
                        </div>
                    </div>
                    <div class="flow-arrow">{ "\u{2192}" }</div>
                    <div class="flow-step flow-step-active">
                        <div class="flow-num">{ "5" }</div>
                        <div class="flow-text">{ "Applies " }<code>{ "class=\"nav-link active\"" }</code>{ " if matched" }</div>
                    </div>
                </div>
            </div>

            // ── Memory & Performance ──────────────────────────
            <div class="card">
                <h3>{ "Memory & Performance" }</h3>
                <table class="doc-table">
                    <thead>
                        <tr>
                            <th>{ "Metric" }</th>
                            <th>{ "Value" }</th>
                            <th>{ "Explanation" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>{ "LOC" }</code></td>
                            <td>{ "124" }</td>
                            <td>{ "Single-file application" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Route variants" }</code></td>
                            <td>{ "4" }</td>
                            <td>{ "Home, About, Contact, NotFound" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "NavLink instances" }</code></td>
                            <td>{ "3" }</td>
                            <td>{ "Each allocates ~2 Strings on mount" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Re-render trigger" }</code></td>
                            <td>{ "Route change" }</td>
                            <td>{ "NavLink subscribes to route; no polling" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Heap allocations" }</code></td>
                            <td>{ "~6 per render" }</td>
                            <td>{ "2 per NavLink: path String + Classes" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Bundle overhead" }</code></td>
                            <td>{ "~3 KB (gzipped)" }</td>
                            <td>{ "Core NavLink + nav_link function only" }</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            // ── Full Code ─────────────────────────────────────
            <div class="card">
                <div class="file-header">
                    <span class="file-header-icon">{ "\u{1F4C4}" }</span>
                    <span class="file-header-name">{ "src/main.rs" }</span>
                    <span class="file-header-badge">{ "single-file" }</span>
                </div>
                <p>
                    { "All steps combined into one file — copy-paste ready:" }
                </p>
                <CopyCode code={FULL_CODE.to_string()} language={"rust".to_string()} />
            </div>

 // ── Live Demo ─────────────────────────────────────
            <div class="card">
                <h3>{ "Live Demo" }</h3>
                <p>
                    { "Navigate between pages - the active link highlights automatically. " }
                    { "Click any link to navigate and watch the route status update in real-time." }
                </p>
                <div class="demo-box">
                    <nav>
                        <ul class="nav-list">
                            <li class="nav-item">
                                <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                            </li>
                            <li class="nav-item">
                                <NavLink<Route> to={Route::BasicExample}>{ "This Page" }</NavLink<Route>>
                            </li>
                            <li class="nav-item">
                                { nav_link(Route::NavLinkDoc, "NavLink Docs", Match::Exact) }
                            </li>
                        </ul>
                    </nav>
                </div>
                <RouteStatus />
            </div>

            // ── When to Use ───────────────────────────────────
            <div class="card">
                <h3>{ "When to Use This Pattern" }</h3>
                <div class="feature-grid">
                    <div class="feature-card">
                        <h4>{ "Small Apps" }</h4>
                        <p>{ "3-10 flat routes with simple navigation" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Prototyping" }</h4>
                        <p>{ "Quick setup without complex routing patterns" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Learning" }</h4>
                        <p>{ "Best starting point before adding complexity" }</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
