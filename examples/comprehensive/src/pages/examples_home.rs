use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
struct ExampleCardProps {
    route: Route,
    title: String,
    desc: String,
    tag: String,
    tag_color: String,
    lines: String,
    concepts: Vec<&'static str>,
}

#[function_component]
fn ExampleCard(props: &ExampleCardProps) -> Html {
    let route = props.route.clone();
    html! {
        <a href={route.to_path()} style="text-decoration:none;">
            <div class="example-card-full">
                <div class="example-card-header">
                    <span class={format!("example-tag tag-{}", props.tag_color)}>{ &props.tag }</span>
                    <span class="example-lines">{ &props.lines }</span>
                </div>
                <h4>{ &props.title }</h4>
                <p>{ &props.desc }</p>
                <div class="example-concepts">
                    { for props.concepts.iter().map(|c| html! {
                        <span class="example-concept-pill">{ c }</span>
                    })}
                </div>
            </div>
        </a>
    }
}

#[function_component]
pub fn ExamplesHome() -> Html {
    html! {
        <div>
            // ── Hero ──────────────────────────────────────────
            <div class="hero">
                <h1>{ "Examples" }</h1>
                <p>
                    { "Production-ready patterns for " }
                    <code>{ "yew-nav-link" }</code>
                    { ". Each example is a standalone application demonstrating real-world usage." }
                </p>
            </div>

            // ── Architecture Overview ─────────────────────────
            <div class="card">
                <h2>{ "Library Architecture" }</h2>
                <p>
                    { "yew-nav-link sits between your application and " }
                    <code>{ "yew-router" }</code>
                    { ", providing a thin abstraction layer that adds automatic active state detection." }
                </p>
                <div class="architecture-diagram">
                    <div class="arch-layer arch-app">
                        <div class="arch-layer-title">{ "Your Application" }</div>
                        <div class="arch-layer-items">
                            <span class="arch-item">{ "Pages" }</span>
                            <span class="arch-item">{ "Layout" }</span>
                            <span class="arch-item">{ "State" }</span>
                        </div>
                    </div>
                    <div class="arch-arrow">{ "\u{2193}" }</div>
                    <div class="arch-layer arch-nav">
                        <div class="arch-layer-title">{ "yew-nav-link" }</div>
                        <div class="arch-layer-items">
                            <span class="arch-item">{ "NavLink" }</span>
                            <span class="arch-item">{ "NavList" }</span>
                            <span class="arch-item">{ "NavTabs" }</span>
                            <span class="arch-item">{ "Pagination" }</span>
                            <span class="arch-item">{ "Hooks" }</span>
                            <span class="arch-item">{ "Macros" }</span>
                        </div>
                    </div>
                    <div class="arch-arrow">{ "\u{2193}" }</div>
                    <div class="arch-layer arch-router">
                        <div class="arch-layer-title">{ "yew-router" }</div>
                        <div class="arch-layer-items">
                            <span class="arch-item">{ "Routable" }</span>
                            <span class="arch-item">{ "BrowserRouter" }</span>
                            <span class="arch-item">{ "Switch" }</span>
                            <span class="arch-item">{ "use_route" }</span>
                        </div>
                    </div>
                    <div class="arch-arrow">{ "\u{2193}" }</div>
                    <div class="arch-layer arch-dom">
                        <div class="arch-layer-title">{ "DOM" }</div>
                        <div class="arch-layer-items">
                            <span class="arch-item">{ "<a>" }</span>
                            <span class="arch-item">{ "nav-link" }</span>
                            <span class="arch-item">{ "active" }</span>
                        </div>
                    </div>
                </div>
            </div>

            // ── Active State Detection Flow ───────────────────
            <div class="card">
                <h2>{ "Active State Detection" }</h2>
                <p>
                    { "When a " }<code>{ "NavLink<R>" }</code>
                    { " renders, it subscribes to route changes and compares its target path against the current URL." }
                </p>
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

            // ── Getting Started ───────────────────────────────
            <div class="card">
                <h2>{ "Getting Started" }</h2>
                <div class="examples-grid">
                    <ExampleCard
                        route={Route::BasicExample}
                        title="Basic"
                        desc="The simplest introduction. Demonstrates both component syntax (<NavLink<R>>) and function syntax (nav_link()) with a minimal 3-page app."
                        tag="starter"
                        tag_color="blue"
                        lines="124 LOC"
                        concepts={vec!["Component Syntax", "Function Syntax", "Auto Active", "Type-Safe Routing"]}
                    />
                    <ExampleCard
                        route={Route::WithMacrosExample}
                        title="With Macros"
                        desc="Enable the macros feature to reduce boilerplate by ~75%. Declarative macros generate navigation structures from simple route-label pairs."
                        tag="macros"
                        tag_color="purple"
                        lines="~80 LOC"
                        concepts={vec!["nav_list!", "nav_menu!", "nav_tabs!", "breadcrumbs!", "nav_pagination!"]}
                    />
                </div>
            </div>

            // ── CSS Framework Integration ─────────────────────
            <div class="card">
                <h2>{ "CSS Framework Integration" }</h2>
                <div class="examples-grid">
                    <ExampleCard
                        route={Route::BootstrapExample}
                        title="Bootstrap 5"
                        desc="Zero-config Bootstrap compatibility. NavLink outputs nav-link and active classes by default — exactly what Bootstrap expects for navbar styling."
                        tag="bootstrap"
                        tag_color="indigo"
                        lines="128 LOC"
                        concepts={vec!["Zero Config", "Navbar Integration", "Card Components", "Alert Styling"]}
                    />
                    <ExampleCard
                        route={Route::TailwindExample}
                        title="Tailwind CSS"
                        desc="Dashboard sidebar with Tailwind @apply directives. Shows how to map nav-link and active classes to Tailwind utility classes for custom styling."
                        tag="tailwind"
                        tag_color="cyan"
                        lines="108 LOC"
                        concepts={vec!["@apply Directives", "Sidebar Layout", "Grid Cards", "Dark Theme"]}
                    />
                </div>
            </div>

            // ── Advanced Patterns ─────────────────────────────
            <div class="card">
                <h2>{ "Advanced Patterns" }</h2>
                <div class="examples-grid">
                    <ExampleCard
                        route={Route::NestedRoutesExample}
                        title="Nested Routes"
                        desc="Multi-level navigation with partial matching, multiple route enums, and sub-navigation. Demonstrates complex routing architectures for large applications."
                        tag="advanced"
                        tag_color="orange"
                        lines="222 LOC"
                        concepts={vec!["partial=true", "Multiple Enums", "Wildcard Routes", "Sub-Navigation"]}
                    />
                </div>
            </div>

            // ── Performance & Memory ──────────────────────────
            <div class="card">
                <h2>{ "Performance Characteristics" }</h2>
                <table class="doc-table">
                    <thead>
                        <tr>
                            <th>{ "Metric" }</th>
                            <th>{ "Value" }</th>
                            <th>{ "Details" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>{ "Re-renders" }</code></td>
                            <td>{ "On route change only" }</td>
                            <td>{ "NavLink subscribes to route changes; no polling or intervals" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Allocations" }</code></td>
                            <td>{ "~2 per NavLink" }</td>
                            <td>{ "One String for path comparison, one Classes for CSS output" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Bundle Size" }</code></td>
                            <td>{ "~8 KB (gzipped)" }</td>
                            <td>{ "Minimal overhead over yew-router; tree-shaking removes unused components" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Compile Time" }</code></td>
                            <td>{ "Negligible" }</td>
                            <td>{ "No procedural macros in core; declarative macros expand at compile time" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Memory" }</code></td>
                            <td>{ "O(n) where n = NavLink count" }</td>
                            <td>{ "Each NavLink holds a route clone and a subscription handle" }</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            // ── Running Examples ──────────────────────────────
            <div class="card">
                <h2>{ "Running Examples" }</h2>
                <div class="code-block">
                    <pre><code class="language-bash">{ r#"# Install prerequisites (once)
rustup target add wasm32-unknown-unknown
cargo install trunk

# Run any example
cd examples/basic
trunk serve

# Open http://127.0.0.1:8080 in your browser"# }</code></pre>
                </div>
            </div>
        </div>
    }
}
