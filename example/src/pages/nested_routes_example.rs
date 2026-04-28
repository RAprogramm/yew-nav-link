use yew::prelude::*;
use yew_nav_link::NavLink;

use crate::{code_utils::CopyCode, demo_popup::DemoBox, routes::Route};

const CODE_ROUTE_ENUM: &str = r#"use yew::prelude::*;
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
}"#;

const CODE_MAIN_NAV: &str = r#"#[component]
fn MainNav() -> Html {
    html! {
        <nav class="main-nav">
            <ul>
                <li><NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>></li>
                // partial=true keeps this link active on /docs, /docs/getting-started, etc.
                <li><NavLink<Route> to={Route::DocsRoot} partial=true>{ "Documentation" }</NavLink<Route>></li>
                <li><NavLink<Route> to={Route::BlogRoot} partial=true>{ "Blog" }</NavLink<Route>></li>
            </ul>
        </nav>
    }
}"#;

const CODE_SUB_NAV: &str = r#"#[component]
fn DocsSection() -> Html {
    html! {
        <>
            // Sub-navigation uses the DocsRoute enum
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
}"#;

const CODE_SWITCH: &str = r#"fn switch_main(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        // Both DocsRoot and Docs route to the same section
        Route::DocsRoot | Route::Docs => html! { <DocsSection /> },
        Route::BlogRoot | Route::Blog => html! { <BlogSection /> },
    }
}"#;

const CODE_PARTIAL_DEEP: &str = r#"// Exact match (default): active ONLY when URL === target path
<NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
// URL: "/"          → active
// URL: "/docs"      → NOT active

// Partial match: active when URL starts with target path
<NavLink<Route> to={Route::DocsRoot} partial=true>{ "Docs" }</NavLink<Route>>
// URL: "/docs"              → active
// URL: "/docs/getting-started" → active
// URL: "/docs/api/reference"   → active
// URL: "/documentation"        → NOT active (different path segment)
// URL: "/blog"                 → NOT active"#;

#[function_component]
pub fn NestedRoutesExample() -> Html {
    html! {
        <div>
            // ── Header ────────────────────────────────────────
            <div class="example-header">
                <div class="example-header-top">
                    <span class="example-tag tag-red">{ "advanced" }</span>
                    <span class="example-source">{ "examples/nested-routes/" }</span>
                </div>
                <h2>{ "Nested Routes Example" }</h2>
                <p class="example-desc">
                    { "Demonstrates multi-level routing with " }
                    <code>{ "partial=true" }</code>
                    { " for parent link persistence. Each section has its own route enum and sub-navigation, enabling scalable application architectures." }
                </p>
            </div>

            // ── Architecture ──────────────────────────────────
            <div class="card">
                <h3>{ "Architecture" }</h3>
                <p>
                    { "The nested routes pattern uses multiple route enums organized hierarchically. The top-level " }
                    <code>{ "Route" }</code>
                    { " enum captures section-level navigation with wildcard patterns, while each section defines its own " }
                    <code>{ "XxxRoute" }</code>
                    { " enum for internal navigation. Parent links use " }
                    <code>{ "partial=true" }</code>
                    { " to remain active across all nested paths." }
                </p>
                <div class="arch-diagram">
                    <div class="arch-row">
                        <div class="arch-box arch-box-blue">{ "App" }</div>
                        <div class="arch-arrow">{ "\u{2192}" }</div>
                        <div class="arch-box arch-box-purple">{ "BrowserRouter" }</div>
                        <div class="arch-arrow">{ "\u{2192}" }</div>
                        <div class="arch-box arch-box-green">{ "MainNav" }</div>
                        <div class="arch-arrow">{ "\u{2192}" }</div>
                        <div class="arch-box arch-box-orange">{ "Switch<Route>" }</div>
                    </div>
                    <div class="arch-row" style="margin-top:0.75rem;">
                        <div class="arch-box arch-box-orange">{ "DocsSection" }</div>
                        <div class="arch-arrow">{ "\u{2192}" }</div>
                        <div class="arch-box arch-box-teal">{ "SubNav<DocsRoute>" }</div>
                        <div class="arch-arrow">{ "\u{2192}" }</div>
                        <div class="arch-box arch-box-gray">{ "Switch<DocsRoute>" }</div>
                    </div>
                </div>
                <div class="info-box" style="margin-top:1rem;">
                    <strong>{ "Key insight:" }</strong>
                    { " Each section is a self-contained routing unit with its own enum, switch function, and sub-navigation. This keeps route definitions modular and prevents enum bloat." }
                </div>
            </div>

            // ── Multiple Route Enums ──────────────────────────
            <div class="card">
                <h3>{ "1. Multiple Route Enums" }</h3>
                <p>
                    { "Define separate route enums for each section. The top-level " }
                    <code>{ "Route" }</code>
                    { " uses wildcard patterns (" }
                    <code>{ "/docs/*" }</code>
                    { ") to capture all nested paths and route them to the appropriate section component." }
                </p>
                <CopyCode code={CODE_ROUTE_ENUM.to_string()} />
                <div class="info-box">
                    <strong>{ "Why multiple enums?" }</strong>
                    { " Separating route enums by section provides type safety within each context, reduces the size of any single enum, and allows independent evolution of section routes without touching the top-level router." }
                </div>
            </div>

            // ── Main Navigation ───────────────────────────────
            <div class="card">
                <h3>{ "2. Main Navigation with Partial Matching" }</h3>
                <p>
                    { "The top-level navigation uses " }
                    <code>{ "partial=true" }</code>
                    { " on section links so they remain active while the user navigates any nested route within that section." }
                </p>
                <CopyCode code={CODE_MAIN_NAV.to_string()} />

                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <nav>
                        <ul class="nav-list">
                            <li class="nav-item">
                                <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                            </li>
                            <li class="nav-item">
                                <NavLink<Route> to={Route::NavLinkDoc} partial=true>{ "Documentation" }</NavLink<Route>>
                            </li>
                            <li class="nav-item">
                                <NavLink<Route> to={Route::NestedRoutesExample} partial=true>{ "Nested Routes" }</NavLink<Route>>
                            </li>
                        </ul>
                    </nav>
                </DemoBox>
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "The \"Documentation\" and \"Nested Routes\" links use " }
                    <code>{ "partial=true" }</code>
                    { " \u{2014} they stay active on any nested path beneath their target." }
                </p>
            </div>

            // ── Sub-Navigation ────────────────────────────────
            <div class="card">
                <h3>{ "3. Sub-Navigation" }</h3>
                <p>
                    { "Each section component contains its own sub-navigation using a section-specific route enum and " }
                    <code>{ "Switch<T>" }</code>
                    { " to render the appropriate page content." }
                </p>
                <CopyCode code={CODE_SUB_NAV.to_string()} />

                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <div class="nested-sub-nav">
                        <ul class="sub-nav-list">
                            <li class="sub-nav-item">
                                <NavLink<Route> to={Route::Home}>{ "Overview" }</NavLink<Route>>
                            </li>
                            <li class="sub-nav-item">
                                <NavLink<Route> to={Route::NavLinkDoc}>{ "Getting Started" }</NavLink<Route>>
                            </li>
                            <li class="sub-nav-item">
                                <NavLink<Route> to={Route::NestedRoutesExample}>{ "API Reference" }</NavLink<Route>>
                            </li>
                        </ul>
                    </div>
                </DemoBox>
            </div>

            // ── Route Switching ───────────────────────────────
            <div class="card">
                <h3>{ "4. Route Switching" }</h3>
                <p>
                    { "The top-level switch function maps multiple route variants to the same section component. Both the exact root (" }
                    <code>{ "/docs" }</code>
                    { ") and the wildcard (" }
                    <code>{ "/docs/*" }</code>
                    { ") resolve to the same section renderer." }
                </p>
                <CopyCode code={CODE_SWITCH.to_string()} />
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
                            <td><code>{ "Route enums" }</code></td>
                            <td>{ "3" }</td>
                            <td>{ "Route, DocsRoute, BlogRoute \u{2014} isolated per section" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Route variants" }</code></td>
                            <td>{ "10" }</td>
                            <td>{ "5 top-level + 3 docs + 3 blog (excluding NotFound)" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Switch instances" }</code></td>
                            <td>{ "2" }</td>
                            <td>{ "One per active section; lazy-mounted on demand" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "NavLink subscriptions" }</code></td>
                            <td>{ "N per nav" }</td>
                            <td>{ "Each NavLink subscribes to route changes via use_route()" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Re-render scope" }</code></td>
                            <td>{ "Section-local" }</td>
                            <td>{ "Sub-nav re-renders only within its Switch context" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "partial=true cost" }</code></td>
                            <td>{ "O(1)" }</td>
                            <td>{ "Single starts_with() string comparison per NavLink on route change" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Heap per NavLink" }</code></td>
                            <td>{ "~2 Strings" }</td>
                            <td>{ "Target path + current path for comparison" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Bundle overhead" }</code></td>
                            <td>{ "~4 KB (gzipped)" }</td>
                            <td>{ "NavLink component + active detection logic" }</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            // ── Route Resolution Algorithm ────────────────────
            <div class="card">
                <h3>{ "Route Resolution Algorithm" }</h3>
                <p>
                    { "Understanding how NavLink determines active state is essential for debugging nested routing. Here is the step-by-step process:" }
                </p>
                <div class="code-block">
                    <pre><code class="language-text">{ r#"1. NavLink<R> mounts with to={Route::DocsRoot} (partial=true)
2. Subscribes to route changes via use_route::<R>()
3. On each route change:
   a. Gets current route from browser history
   b. Converts target route to path: Route::DocsRoot.to_path() → "/docs"
   c. Converts current route to path: current.to_path() → "/docs/api"
4. Match evaluation:
   - Exact mode (default): target_path == current_path
     → "/docs" == "/docs/api" → false → NOT active
   - Partial mode: current_path.starts_with(target_path)
     AND (current_path.len() == target_path.len()
          OR current_path.chars().nth(target_path.len()) == Some('/'))
     → "/docs/api".starts_with("/docs") → true
      → char at index 5 is '/' → true → ACTIVE
 5. Renders <a class="nav-link active" href="/docs">Documentation</a>"# }</code></pre>
                </div>
                <div class="info-box">
                    <strong>{ "Note:" }</strong>
                    { " The partial match checks for a path segment boundary (a " }
                    <code>{ "/" }</code>
                    { " character) to prevent false positives. This ensures " }
                    <code>{ "/docs" }</code>
                    { " matches " }
                    <code>{ "/docs/api" }</code>
                    { " but NOT " }
                    <code>{ "/documentation" }</code>
                    { "." }
                </div>
            </div>

            // ── partial=true Deep Dive ────────────────────────
            <div class="card">
                <h3>{ "partial=true Deep Dive" }</h3>
                <p>
                    { "The " }
                    <code>{ "partial" }</code>
                    { " prop is the cornerstone of nested route navigation. It controls whether a link remains active when the current URL extends beyond the target path." }
                </p>
                <CopyCode code={CODE_PARTIAL_DEEP.to_string()} />

                <h3 style="margin-top:1.5rem;">{ "Segment Boundary Detection" }</h3>
                <p>
                    { "The partial match algorithm does not use a naive " }
                    <code>{ "starts_with" }</code>
                    { " check. It enforces segment boundaries to prevent accidental matches:" }
                </p>
                <table class="doc-table">
                    <thead>
                        <tr>
                            <th>{ "Target" }</th>
                            <th>{ "Current URL" }</th>
                            <th>{ "partial=false" }</th>
                            <th>{ "partial=true" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>{ "/docs" }</code></td>
                            <td><code>{ "/docs" }</code></td>
                            <td>{ "Active" }</td>
                            <td>{ "Active" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "/docs" }</code></td>
                            <td><code>{ "/docs/api" }</code></td>
                            <td>{ "Not active" }</td>
                            <td>{ "Active" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "/docs" }</code></td>
                            <td><code>{ "/docs/api/v2" }</code></td>
                            <td>{ "Not active" }</td>
                            <td>{ "Active" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "/docs" }</code></td>
                            <td><code>{ "/documentation" }</code></td>
                            <td>{ "Not active" }</td>
                            <td>{ "Not active" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "/" }</code></td>
                            <td><code>{ "/anything" }</code></td>
                            <td>{ "Not active" }</td>
                            <td>{ "Active" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "/docs" }</code></td>
                            <td><code>{ "/blog" }</code></td>
                            <td>{ "Not active" }</td>
                            <td>{ "Not active" }</td>
                        </tr>
                    </tbody>
                </table>

                <div class="info-box" style="margin-top:1rem;">
                    <strong>{ "Root path special case:" }</strong>
                    { " When the target is " }
                    <code>{ "/" }</code>
                    { " with " }
                    <code>{ "partial=true" }</code>
                    { ", the link is active on every route since all paths start with " }
                    <code>{ "/" }</code>
                    { ". This is useful for a persistent \"Home\" link in sidebars." }
                </div>
            </div>

            // ── When to Use This Pattern ──────────────────────
            <div class="card">
                <h3>{ "When to Use This Pattern" }</h3>
                <div class="feature-grid">
                    <div class="feature-card">
                        <h4>{ "Multi-Section Apps" }</h4>
                        <p>{ "Applications with distinct areas like /docs, /blog, /admin that each have their own internal navigation and page hierarchy" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Documentation Sites" }</h4>
                        <p>{ "Technical documentation with overview, guides, API reference, and changelog sections, each with dozens of sub-pages" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Admin Dashboards" }</h4>
                        <p>{ "Complex admin panels with nested resource management (users, settings, analytics) requiring persistent section highlighting" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "E-Commerce Platforms" }</h4>
                        <p>{ "Product catalogs with category/subcategory hierarchies where breadcrumb and sidebar navigation must reflect the current depth" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Team-Owned Modules" }</h4>
                        <p>{ "Large codebases where different teams own different sections; separate route enums prevent merge conflicts and enable independent deployment" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Progressive Enhancement" }</h4>
                        <p>{ "Start with a flat Route enum, then extract section-specific enums as the application grows without rewriting the navigation layer" }</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
