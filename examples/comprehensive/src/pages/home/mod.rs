mod api_stats;
mod feature_card;
mod hero;
mod metadata;

use crate::code_utils::CopyCode;
use crate::routes::Route;
use api_stats::count_api_items;
use feature_card::FeatureCard;
use hero::Hero;
use metadata::crate_meta;
use yew::prelude::*;

const BASH_CODE: &str = r#"# Install prerequisites (once)
rustup target add wasm32-unknown-unknown
cargo install trunk

# Run the comprehensive demo
cd examples/comprehensive
trunk serve

# Open http://127.0.0.1:8080 in your browser"#;

#[function_component]
pub fn Home() -> Html {
    let meta = crate_meta();
    let stats = count_api_items();

    html! {
        <div>
            <Hero meta={meta.clone()} />

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

            // ── API Stats ────────────────────────────────
            <div class="card">
                <h2>{ "API at a Glance" }</h2>
                <div class="feature-grid">
                    <div class="stat-card">
                        <div class="stat-value">{ stats.fns }</div>
                        <div class="stat-label">{ "Public Functions" }</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{ stats.structs }</div>
                        <div class="stat-label">{ "Public Structs" }</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{ stats.enums }</div>
                        <div class="stat-label">{ "Public Enums" }</div>
                    </div>
                </div>
            </div>

            // ── Components grid ────────────────────────────
            <div class="card">
                <h2>{ "Components" }</h2>
                <div class="feature-grid">
                    <FeatureCard route={Route::NavLinkDoc} title="NavLink" desc="Auto-active link with exact & partial matching." />
                    <FeatureCard route={Route::NavListDoc} title="NavList / NavItem" desc="Semantic <ul>/<li> with ARIA." />
                    <FeatureCard route={Route::NavDividerDoc} title="NavDivider" desc="Visual separators with labels." />
                    <FeatureCard route={Route::BadgeDoc} title="NavBadge / Header / Text" desc="Badges, section headers, static text." />
                    <FeatureCard route={Route::DropdownDoc} title="NavDropdown" desc="Dropdown menus with items." />
                    <FeatureCard route={Route::IconDoc} title="NavIcon" desc="Icon wrapper with size variants." />
                    <FeatureCard route={Route::TabsDoc} title="NavTabs / NavTab" desc="Tab navigation with ARIA." />
                    <FeatureCard route={Route::PaginationDoc} title="Pagination" desc="Page navigation with ellipsis." />
                </div>
            </div>

            // ── Hooks & Utilities grid ─────────────────────
            <div class="card">
                <h2>{ "Hooks & Utilities" }</h2>
                <div class="feature-grid">
                    <FeatureCard route={Route::HooksDoc} title="Route Hooks" desc="use_route_info, use_is_active, use_is_partial_active." />
                    <FeatureCard route={Route::BreadcrumbsDoc} title="Breadcrumbs" desc="Auto-generated breadcrumb trail." />
                    <FeatureCard route={Route::UtilsDoc} title="Path Utilities" desc="is_absolute, join_paths, normalize_path." />
                </div>
            </div>

            // ── Installation ───────────────────────────────
            <div class="card">
                <h2>{ "Installation" }</h2>
                <p>{ "Add to your " }<code>{ "Cargo.toml" }</code>{ ":" }</p>
                <CopyCode code={"yew-nav-link = \"0.6\"".to_string()} language={"toml".to_string()} />
                <div class="info-box info-box-spaced">
                    <strong>{ "Requirements: " }</strong>
                    { "yew 0.23+, yew-router 0.20+" }
                </div>
            </div>

            // ── Getting Started Examples ───────────────────
            <div class="card">
                <h2>{ "Getting Started" }</h2>
                <div class="examples-grid">
                    <FeatureCard route={Route::BasicExample} title="Basic" desc="Simple 3-page app with component and function syntax." />
                </div>
            </div>

            // ── CSS Framework Integration ──────────────────
            <div class="card">
                <h2>{ "CSS Framework Integration" }</h2>
                <div class="examples-grid">
                    <FeatureCard route={Route::BootstrapExample} title="Bootstrap 5" desc="Zero-config compatibility. NavLink outputs nav-link and active classes by default." />
                    <FeatureCard route={Route::TailwindExample} title="Tailwind CSS" desc="Dashboard sidebar with @apply directives for custom styling." />
                </div>
            </div>

            // ── Advanced Patterns ──────────────────────────
            <div class="card">
                <h2>{ "Advanced Patterns" }</h2>
                <div class="examples-grid">
                    <FeatureCard route={Route::NestedRoutesExample} title="Nested Routes" desc="Multi-level navigation with partial matching and sub-navigation." />
                </div>
            </div>

            // ── Performance Characteristics ────────────────
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
                            <td>{ "Core components and hooks compile with standard Rust + Yew toolchains" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Memory" }</code></td>
                            <td>{ "O(n) where n = NavLink count" }</td>
                            <td>{ "Each NavLink holds a route clone and a subscription handle" }</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            // ── Quality & Coverage ─────────────────────────
            <div class="card">
                <h2>{ "Quality & Coverage" }</h2>
                <p>
                    { "Test coverage is tracked via " }
                    <a href="https://codecov.io/gh/RAprogramm/yew-nav-link" target="_blank" rel="noopener noreferrer" class="codecov-link">
                        { "Codecov" }
                    </a>
                    { " with a target of " }<code>{ "95%+" }</code>{ " coverage." }
                </p>
                <div class="codecov-grid">
                    <div class="codecov-chart">
                        <h4>{ "Sunburst" }</h4>
                        <img
                            src="https://codecov.io/gh/RAprogramm/yew-nav-link/graphs/sunburst.svg?token=E93AERE3UC"
                            alt="Codecov Sunburst"
                            loading="lazy"
                        />
                    </div>
                    <div class="codecov-chart">
                        <h4>{ "Grid" }</h4>
                        <img
                            src="https://codecov.io/gh/RAprogramm/yew-nav-link/graphs/tree.svg?token=E93AERE3UC"
                            alt="Codecov Grid"
                            loading="lazy"
                        />
                    </div>
                    <div class="codecov-chart">
                        <h4>{ "Icicle" }</h4>
                        <img
                            src="https://codecov.io/gh/RAprogramm/yew-nav-link/graphs/icicle.svg?token=E93AERE3UC"
                            alt="Codecov Icicle"
                            loading="lazy"
                        />
                    </div>
                </div>
            </div>

            // ── Running Examples ───────────────────────────
            <div class="card">
                <h2>{ "Running Examples" }</h2>
                <CopyCode code={BASH_CODE.to_string()} language={"bash".to_string()} />
            </div>
        </div>
    }
}
