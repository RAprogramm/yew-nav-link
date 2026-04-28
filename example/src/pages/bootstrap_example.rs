use yew::prelude::*;
use yew_nav_link::NavLink;

use crate::{code_utils::CopyCode, demo_popup::DemoBox, routes::Route};

const CODE_BOOTSTRAP_NAVBAR: &str = r##"use yew::prelude::*;
use yew_nav_link::NavLink;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/nav-link")]
    NavLinkDoc,
    #[at("/bootstrap")]
    BootstrapExample,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[component]
fn BootstrapNav() -> Html {
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
            <div class="container-fluid">
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
                            <NavLink<Route> to={Route::Home}>
                                { "Home" }
                            </NavLink<Route>>
                        </li>
                        <li class="nav-item">
                            <NavLink<Route> to={Route::NavLinkDoc}>
                                { "Docs" }
                            </NavLink<Route>>
                        </li>
                        <li class="nav-item">
                            <NavLink<Route> to={Route::BootstrapExample}>
                                { "Bootstrap" }
                            </NavLink<Route>>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}"##;

const CODE_PILL_NAV: &str = r##"#[component]
fn PillNav() -> Html {
    html! {
        <ul class="nav nav-pills mb-3">
            <li class="nav-item">
                <NavLink<Route> to={Route::Home}>
                    { "Home" }
                </NavLink<Route>>
            </li>
            <li class="nav-item">
                <NavLink<Route> to={Route::NavLinkDoc}>
                    { "Documentation" }
                </NavLink<Route>>
            </li>
            <li class="nav-item">
                <NavLink<Route> to={Route::BootstrapExample}>
                    { "Bootstrap" }
                </NavLink<Route>>
            </li>
        </ul>
    }
}"##;

const CODE_TABS_NAV: &str = r##"#[component]
fn TabNav() -> Html {
    html! {
        <ul class="nav nav-tabs">
            <li class="nav-item">
                <NavLink<Route> to={Route::Home}>
                    { "Home" }
                </NavLink<Route>>
            </li>
            <li class="nav-item">
                <NavLink<Route> to={Route::NavLinkDoc}>
                    { "API Reference" }
                </NavLink<Route>>
            </li>
            <li class="nav-item">
                <NavLink<Route> to={Route::BootstrapExample}>
                    { "Examples" }
                </NavLink<Route>>
            </li>
        </ul>
    }
}"##;

const CODE_VERTICAL_NAV: &str = r##"#[component]
fn VerticalNav() -> Html {
    html! {
        <div class="d-flex">
            <div class="d-flex flex-column flex-shrink-0 p-3 bg-light"
                 style="width: 280px;">
                <span class="fs-4">{ "Sidebar" }</span>
                <hr />
                <ul class="nav nav-pills flex-column mb-auto">
                    <li class="nav-item">
                        <NavLink<Route> to={Route::Home}>
                            { "\u{1F3E0}  Home" }
                        </NavLink<Route>>
                    </li>
                    <li>
                        <NavLink<Route> to={Route::NavLinkDoc}>
                            { "\u{1F4D6}  Docs" }
                        </NavLink<Route>>
                    </li>
                    <li>
                        <NavLink<Route> to={Route::BootstrapExample}>
                            { "\u{1F3A8}  Bootstrap" }
                        </NavLink<Route>>
                    </li>
                </ul>
            </div>
        </div>
    }
}"##;

// ─── Configurable Nav Items ────────────────────────────────────────

#[derive(Clone, PartialEq, Debug)]
struct NavMenuItem {
    label: &'static str,
    route: Route,
    emoji: &'static str,
}

impl NavMenuItem {
    const fn new(label: &'static str, route: Route, emoji: &'static str) -> Self {
        NavMenuItem { label, route, emoji }
    }
}

fn nav_link_text(text: &str) -> Html {
    html! { <span>{ text }</span> }
}

fn render_nav_item(item: &NavMenuItem) -> Html {
    let text = format!("{} {}", item.emoji, item.label);
    let children = nav_link_text(&text);
    match item.route {
        Route::Home => html! {
            <li class="nav-item"><NavLink<Route> to={Route::Home}>{ children }</NavLink<Route>></li>
        },
        Route::NavLinkDoc => html! {
            <li class="nav-item"><NavLink<Route> to={Route::NavLinkDoc}>{ children }</NavLink<Route>></li>
        },
        Route::BootstrapExample => html! {
            <li class="nav-item"><NavLink<Route> to={Route::BootstrapExample}>{ children }</NavLink<Route>></li>
        },
        Route::NavListDoc => html! {
            <li class="nav-item"><NavLink<Route> to={Route::NavListDoc}>{ children }</NavLink<Route>></li>
        },
        Route::DropdownDoc => html! {
            <li class="nav-item"><NavLink<Route> to={Route::DropdownDoc}>{ children }</NavLink<Route>></li>
        },
        Route::TabsDoc => html! {
            <li class="nav-item"><NavLink<Route> to={Route::TabsDoc}>{ children }</NavLink<Route>></li>
        },
        Route::HooksDoc => html! {
            <li class="nav-item"><NavLink<Route> to={Route::HooksDoc}>{ children }</NavLink<Route>></li>
        },
        Route::PaginationDoc => html! {
            <li class="nav-item"><NavLink<Route> to={Route::PaginationDoc}>{ children }</NavLink<Route>></li>
        },
        Route::BreadcrumbsDoc => html! {
            <li class="nav-item"><NavLink<Route> to={Route::BreadcrumbsDoc}>{ children }</NavLink<Route>></li>
        },
        Route::ExamplesHome => html! {
            <li class="nav-item"><NavLink<Route> to={Route::ExamplesHome}>{ children }</NavLink<Route>></li>
        },
        _ => html! {
            <li class="nav-item"><NavLink<Route> to={Route::Home}>{ children }</NavLink<Route>></li>
        },
    }
}

#[function_component]
pub fn BootstrapExample() -> Html {
    html! {
        <div>
            // ── Header ────────────────────────────────────────────
            <div class="example-header">
                <div class="example-header-top">
                    <span class="example-tag tag-orange">{ "bootstrap" }</span>
                    <span class="example-source">{ "examples/bootstrap/" }</span>
                </div>
                <h2>{ "Bootstrap 5 Integration" }</h2>
                <p class="example-desc">
                    { "Demonstrates seamless integration of " }
                    <code>{ "NavLink<Route>" }</code>
                    { " with Bootstrap 5 navigation components. NavLink outputs " }
                    <code>{ "nav-link" }</code>
                    { " and " }<code>{ "active" }</code>
                    { " classes by default — exactly what Bootstrap expects for navbars, pills, tabs, and vertical sidebars." }
                </p>
            </div>

            // ── Architecture ──────────────────────────────────────
            <div class="card">
                <h3>{ "Architecture" }</h3>
                <p>
                    { "Bootstrap 5 relies on specific CSS class combinations to style navigation. " }
                    { "The " }<code>{ "NavLink" }</code>
                    { " component maps directly onto this system: it always applies " }
                    <code>{ "nav-link" }</code>
                    { " as the base class, and appends " }
                    <code>{ "active" }</code>
                    { " when the route matches. This produces the exact markup Bootstrap expects — zero configuration needed." }
                </p>
                <div class="arch-diagram-small">
                    <div class="arch-box arch-box-blue">{ "NavLink<Route>" }</div>
                    <div class="arch-arrow-sm">{ "\u{2192}" }</div>
                    <div class="arch-box arch-box-purple">{ "nav-link" }</div>
                    <div class="arch-arrow-sm">{ "\u{2192}" }</div>
                    <div class="arch-box arch-box-purple">{ "+ active" }</div>
                    <div class="arch-arrow-sm">{ "\u{2192}" }</div>
                    <div class="arch-box arch-box-green">{ "Bootstrap CSS" }</div>
                </div>
            </div>

            // ── CSS Class Pipeline ────────────────────────────────
            <div class="card">
                <h3>{ "CSS Class Pipeline" }</h3>
                <p>
                    { "Understanding how classes flow from NavLink to rendered HTML is key to Bootstrap integration." }
                </p>
                <div class="code-block">
                    <pre><code class="language-text">{ r##"┌─────────────────────────────────────────────────────────────────┐
│  NavLink<Route>                                                     │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │  Internal Logic                                               │  │
│  │  1. Always apply "nav-link" as base class                     │  │
│  │  2. If current route == target route → append "active"        │  │
│  │  3. If no match → "nav-link" only                             │  │
│  └───────────────────────────┬───────────────────────────────────┘  │
│                              ▼                                      │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │  Rendered <a> element                                         │  │
│  │  <a class="nav-link active" href="/bootstrap">Bootstrap</a>   │  │
│  │  <a class="nav-link" href="/">Home</a>                        │  │
│  └───────────────────────────┬───────────────────────────────────┘  │
│                              ▼                                      │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │  Bootstrap 5 CSS                                              │  │
│  │  .nav-link       → base link styling                          │  │
│  │  .nav-link.active → highlighted/selected state                │  │
│  └───────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘"## }</code></pre>
                </div>
            </div>

            // ── Bootstrap Navbar ──────────────────────────────────
            <div class="card">
                <h3>{ "1. Bootstrap Navbar" }</h3>
                <p>
                    { "The most common Bootstrap navigation pattern. Each " }<code>{ "NavLink" }</code>
                    { " sits inside a " }<code>{ "<li class=\"nav-item\">" }</code>
                    { ". NavLink automatically outputs " }<code>{ "class=\"nav-link\"" }</code>
                    { " or " }<code>{ "class=\"nav-link active\"" }</code>
                    { " — exactly what Bootstrap expects. No extra props needed." }
                </p>
                <CopyCode code={CODE_BOOTSTRAP_NAVBAR.to_string()} />

                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                        <div class="container-fluid">
                            <span class="navbar-brand">{ "MyApp" }</span>
                            <ul class="navbar-nav">
                                <li class="nav-item">
                                    <NavLink<Route> to={Route::Home}>
                                        { "Home" }
                                    </NavLink<Route>>
                                </li>
                                <li class="nav-item">
                                    <NavLink<Route> to={Route::NavLinkDoc}>
                                        { "Docs" }
                                    </NavLink<Route>>
                                </li>
                                <li class="nav-item">
                                    <NavLink<Route> to={Route::BootstrapExample}>
                                        { "Bootstrap" }
                                    </NavLink<Route>>
                                </li>
                            </ul>
                        </div>
                    </nav>
                </DemoBox>
            </div>

            // ── Nav Pills ─────────────────────────────────────────
            <div class="card">
                <h3>{ "2. Nav Pills" }</h3>
                <p>
                    { "Bootstrap's pill-style navigation uses " }<code>{ "nav-pills" }</code>
                    { " on the parent " }<code>{ "<ul>" }</code>
                    { ". The " }<code>{ "NavLink" }</code>
                    { " integration is identical — the same automatic " }
                    <code>{ "nav-link" }</code>
                    { " and " }<code>{ "active" }</code>
                    { " classes produce the correct pill styling." }
                </p>
                <CopyCode code={CODE_PILL_NAV.to_string()} />

                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <ul class="nav nav-pills">
                        <li class="nav-item">
                            <NavLink<Route> to={Route::Home}>
                                { "Home" }
                            </NavLink<Route>>
                        </li>
                        <li class="nav-item">
                            <NavLink<Route> to={Route::NavLinkDoc}>
                                { "Documentation" }
                            </NavLink<Route>>
                        </li>
                        <li class="nav-item">
                            <NavLink<Route> to={Route::BootstrapExample}>
                                { "Bootstrap" }
                            </NavLink<Route>>
                        </li>
                    </ul>
                </DemoBox>
            </div>

            // ── Nav Tabs ──────────────────────────────────────────
            <div class="card">
                <h3>{ "3. Nav Tabs" }</h3>
                <p>
                    { "Tab-style navigation uses " }<code>{ "nav-tabs" }</code>
                    { " on the parent. The active tab is indicated by the " }
                    <code>{ "active" }</code>
                    { " class that NavLink automatically applies when the route matches." }
                </p>
                <CopyCode code={CODE_TABS_NAV.to_string()} />

                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <ul class="nav nav-tabs">
                        <li class="nav-item">
                            <NavLink<Route> to={Route::Home}>
                                { "Home" }
                            </NavLink<Route>>
                        </li>
                        <li class="nav-item">
                            <NavLink<Route> to={Route::NavLinkDoc}>
                                { "API Reference" }
                            </NavLink<Route>>
                        </li>
                        <li class="nav-item">
                            <NavLink<Route> to={Route::BootstrapExample}>
                                { "Examples" }
                            </NavLink<Route>>
                        </li>
                    </ul>
                </DemoBox>
            </div>

            // ── Vertical Sidebar ──────────────────────────────────
            <div class="card">
                <h3>{ "4. Vertical Sidebar" }</h3>
                <p>
                    { "Vertical navigation sidebars use " }<code>{ "flex-column" }</code>
                    { " on the " }<code>{ "<ul>" }</code>
                    { " element. NavLink works identically — the same class pipeline produces the correct active highlighting." }
                </p>
                <CopyCode code={CODE_VERTICAL_NAV.to_string()} />

                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <div class="d-flex">
                        <div class="d-flex flex-column flex-shrink-0 p-3 bg-light"
                             style="width: 280px;">
                            <span class="fs-4">{ "Sidebar" }</span>
                            <hr />
                            <ul class="nav nav-pills flex-column mb-auto">
                                <li class="nav-item">
                                    <NavLink<Route> to={Route::Home}>
                                        { "Home" }
                                    </NavLink<Route>>
                                </li>
                                <li>
                                    <NavLink<Route> to={Route::NavLinkDoc}>
                                        { "Docs" }
                                    </NavLink<Route>>
                                </li>
                                <li>
                                    <NavLink<Route> to={Route::BootstrapExample}>
                                        { "Bootstrap" }
                                    </NavLink<Route>>
                                </li>
                            </ul>
                        </div>
                    </div>
                </DemoBox>
            </div>

            // ── Memory & Performance ──────────────────────────────
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
                            <td>{ "~200" }</td>
                            <td>{ "Full example with 4 nav patterns" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "CSS framework" }</code></td>
                            <td>{ "Bootstrap 5.3" }</td>
                            <td>{ "Loaded via CDN; ~220 KB CSS (gzipped)" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "NavLink per pattern" }</code></td>
                            <td>{ "3" }</td>
                            <td>{ "Each clones route once per render for comparison" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Heap allocations" }</code></td>
                            <td>{ "~0 per NavLink" }</td>
                            <td>{ "Uses static &'static str for classes; no String allocs" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Re-render trigger" }</code></td>
                            <td>{ "Route change" }</td>
                            <td>{ "Event-driven; no polling or mutation observers" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "DOM mutations on nav" }</code></td>
                            <td>{ "O(n)" }</td>
                            <td>{ "n = number of NavLink instances; each updates class once" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Bundle overhead" }</code></td>
                            <td>{ "~3 KB (gzipped)" }</td>
                            <td>{ "NavLink core only; Bootstrap CSS is external" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Bootstrap JS required?" }</code></td>
                            <td>{ "Optional" }</td>
                            <td>{ "Only needed for collapsible navbar toggler" }</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            // ── Integration Details ───────────────────────────────
            <div class="card">
                <h3>{ "Integration Details" }</h3>
                <div class="code-block">
                    <pre><code class="language-text">{ r##"Bootstrap Component    Parent <ul> class     NavLink output     Active class
─────────────────────────────────────────────────────────────────────────────
Navbar                   navbar-nav            nav-link         active
Nav Pills                nav nav-pills         nav-link         active
Nav Tabs                 nav nav-tabs          nav-link         active
Vertical Sidebar         nav nav-pills         nav-link         active
                         flex-column

Key observations:
1. All Bootstrap nav patterns expect "nav-link" as the base class
2. All use "active" as the active state class
3. NavLink outputs "nav-link" always, "nav-link active" when matched
4. The parent <ul> class determines the visual style (pills, tabs, etc.)
5. Zero configuration needed — just drop NavLink into Bootstrap markup"## }</code></pre>
                </div>
            </div>

            // ── When to Use This Pattern ──────────────────────────
            <div class="card">
                <h3>{ "When to Use This Pattern" }</h3>
                <div class="feature-grid">
                    <div class="feature-card">
                        <h4>{ "Bootstrap Projects" }</h4>
                        <p>{ "Ideal for applications already using Bootstrap 5 that need router-aware navigation with automatic active states" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Admin Dashboards" }</h4>
                        <p>{ "Vertical sidebar navigation with Bootstrap pills is a common admin dashboard pattern that integrates seamlessly" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Content Sites" }</h4>
                        <p>{ "Tab and pill navigation for documentation sites, blogs, or content-heavy applications with Bootstrap styling" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Responsive Navbars" }</h4>
                        <p>{ "Full Bootstrap navbar with collapsible mobile menu, where NavLink handles active state without JavaScript" }</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
