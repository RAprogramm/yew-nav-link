use crate::doc_parser::highlight_rust;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

// ── Dynamic data from repo at build time ───────────────────────

const CARGO_TOML: &str = include_str!("../../../../Cargo.toml");

/// Parses `key = "value"` from a TOML string.
fn parse_toml<'a>(source: &'a str, key: &str) -> &'a str {
    for line in source.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix(key) {
            let val = rest.trim().trim_matches('"');
            // Handle inline comments
            if let Some(end) = val.find(" #") {
                return val[..end].trim();
            }
            return val;
        }
    }
    "?"
}

/// Parses a dependency version from Cargo.toml.
/// Handles both `dep = "1.0"` and `dep = { version = "1.0", ... }` formats.
fn parse_dep_version<'a>(source: &'a str, dep: &str) -> &'a str {
    let prefix = format!("{} = ", dep);
    for line in source.lines() {
        let t = line.trim();
        if t.starts_with(&prefix) {
            let rest = &t[prefix.len()..];
            // Simple format: dep = "0.20"
            if rest.starts_with('"') {
                if let Some(end) = rest[1..].find('"') {
                    return &rest[1..end + 1];
                }
            }
            // Inline table: dep = { version = "0.20", ... }
            if let Some(start) = rest.find("version = \"") {
                let after = &rest[start + 11..];
                if let Some(end) = after.find('"') {
                    return &after[..end];
                }
            }
        }
    }
    "?"
}

fn cargo_version() -> &'static str {
    parse_toml(CARGO_TOML, "version =")
}
fn cargo_license() -> &'static str {
    parse_toml(CARGO_TOML, "license =")
}
fn cargo_msrv() -> &'static str {
    parse_toml(CARGO_TOML, "rust-version =")
}
fn cargo_desc() -> &'static str {
    parse_toml(CARGO_TOML, "description =")
}
fn cargo_edition() -> &'static str {
    parse_toml(CARGO_TOML, "edition =")
}
fn dep_yew() -> &'static str {
    parse_dep_version(CARGO_TOML, "yew")
}
fn dep_yew_router() -> &'static str {
    parse_dep_version(CARGO_TOML, "yew-router")
}

/// Counts public API items by scanning source files at build time.
fn count_public_items() -> (usize, usize, usize, usize) {
    let sources: &[&str] = &[
        include_str!("../../../../src/lib.rs"),
        include_str!("../../../../src/lib_macros.rs"),
        include_str!("../../../../src/nav_link.rs"),
        include_str!("../../../../src/attrs.rs"),
        include_str!("../../../../src/errors.rs"),
        include_str!("../../../../src/components/mod.rs"),
        include_str!("../../../../src/components/badge.rs"),
        include_str!("../../../../src/components/dropdown.rs"),
        include_str!("../../../../src/components/header.rs"),
        include_str!("../../../../src/components/icon.rs"),
        include_str!("../../../../src/components/page_item.rs"),
        include_str!("../../../../src/components/page_link.rs"),
        include_str!("../../../../src/components/pagination.rs"),
        include_str!("../../../../src/components/pagination_page.rs"),
        include_str!("../../../../src/components/tab_item.rs"),
        include_str!("../../../../src/components/tab_panel.rs"),
        include_str!("../../../../src/components/tabs.rs"),
        include_str!("../../../../src/components/text.rs"),
        include_str!("../../../../src/nav/divider.rs"),
        include_str!("../../../../src/nav/item.rs"),
        include_str!("../../../../src/nav/list.rs"),
        include_str!("../../../../src/hooks/route.rs"),
        include_str!("../../../../src/utils/path.rs"),
        include_str!("../../../../src/utils/keyboard/config.rs"),
        include_str!("../../../../src/utils/keyboard/direction.rs"),
        include_str!("../../../../src/utils/keyboard/handlers.rs"),
        include_str!("../../../../src/utils/url/codec.rs"),
        include_str!("../../../../src/utils/url/parts.rs"),
        include_str!("../../../../src/utils/url/query.rs"),
    ];
    let mut fns = 0usize;
    let mut structs = 0usize;
    let mut enums = 0usize;
    let mut macros = 0usize;
    for src in sources {
        for line in src.lines() {
            let t = line.trim();
            if t.starts_with("pub fn ") || t.starts_with("pub async fn ") {
                fns += 1;
            }
            if t.starts_with("pub struct ") {
                structs += 1;
            }
            if t.starts_with("pub enum ") {
                enums += 1;
            }
            if t.starts_with("#[macro_export]") {
                macros += 1;
            }
        }
    }
    (fns, structs, enums, macros)
}

// ── Component ──────────────────────────────────────────────────

#[function_component]
pub fn Home() -> Html {
    let version = cargo_version();
    let license = cargo_license();
    let msrv = cargo_msrv();
    let desc = cargo_desc();
    let edition = cargo_edition();
    let yew_ver = dep_yew();
    let router_ver = dep_yew_router();
    let (pub_fns, pub_structs, pub_enums, pub_macros) = count_public_items();

    html! {
        <div>
            // ── Hero ──────────────────────────────────────
            <div class="hero">
                <h1>{ "yew-nav-link" }</h1>
                <p>{ desc }</p>

                <div class="hero-badges">
                    <div class="tag tag-version">
                        <span class="tag-label">{ "version" }</span>
                        <span class="tag-value">{ version }</span>
                    </div>
                    <div class="tag tag-yew">
                        <span class="tag-label">{ "yew" }</span>
                        <span class="tag-value">{ yew_ver }</span>
                    </div>
                    <div class="tag tag-router">
                        <span class="tag-label">{ "yew-router" }</span>
                        <span class="tag-value">{ router_ver }</span>
                    </div>
                    <div class="tag tag-edition">
                        <span class="tag-label">{ "edition" }</span>
                        <span class="tag-value">{ edition }</span>
                    </div>
                    <div class="tag tag-msrv">
                        <span class="tag-label">{ "MSRV" }</span>
                        <span class="tag-value">{ msrv }</span>
                    </div>
                    <div class="tag tag-license">
                        <span class="tag-label">{ "license" }</span>
                        <span class="tag-value">{ license }</span>
                    </div>
                    <div class="tag tag-wasm">
                        <span class="tag-label">{ "target" }</span>
                        <span class="tag-value">{ "wasm" }</span>
                    </div>
                    <div class="tag tag-no-std">
                        <span class="tag-label">{ "format" }</span>
                        <span class="tag-value">{ "cdylib + rlib" }</span>
                    </div>
                </div>
            </div>

            // ── Quick Start ────────────────────────────────
            <div class="card">
                <h2>{ "Quick Start" }</h2>
                <CodeBlock code={format!(
                    "[dependencies]\nyew-nav-link = \"{}\"",
                    version
                )} />

                <h3>{ "Component Syntax" }</h3>
                <CodeBlock code={COMPONENT_SYNTAX.to_string()} />

                <h3>{ "Function Syntax" }</h3>
                <CodeBlock code={FUNCTION_SYNTAX.to_string()} />

                <h3>{ "Partial Matching" }</h3>
                <CodeBlock code={PARTIAL_SYNTAX.to_string()} />
            </div>

            // ── Stats ──────────────────────────────────────
            <div class="card">
                <h2>{ "API at a Glance" }</h2>
                <div class="feature-grid">
                    <div class="stat-card">
                        <div class="stat-value">{ pub_fns }</div>
                        <div class="stat-label">{ "Public Functions" }</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{ pub_structs }</div>
                        <div class="stat-label">{ "Public Structs" }</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{ pub_enums }</div>
                        <div class="stat-label">{ "Public Enums" }</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{ pub_macros }</div>
                        <div class="stat-label">{ "Exported Macros" }</div>
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
        </div>
    }
}

// ── Code examples from README ──────────────────────────────────

const COMPONENT_SYNTAX: &str = "\
use yew::prelude::*;
use yew_nav_link::NavLink;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at(\"/\")]
    Home,
    #[at(\"/about\")]
    About,
}

#[component]
fn Navigation() -> Html {
    html! {
        <nav>
            <NavLink<Route> to={Route::Home}>{ \"Home\" }</NavLink<Route>>
            <NavLink<Route> to={Route::About}>{ \"About\" }</NavLink<Route>>
        </nav>
    }
}";

const FUNCTION_SYNTAX: &str = "\
use yew::prelude::*;
use yew_nav_link::{nav_link, Match};
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at(\"/\")]
    Home,
    #[at(\"/docs\")]
    Docs,
}

#[component]
fn Menu() -> Html {
    html! {
        <nav>
            { nav_link(Route::Home, \"Home\", Match::Exact) }
            { nav_link(Route::Docs, \"Docs\", Match::Partial) }
        </nav>
    }
}";

const PARTIAL_SYNTAX: &str = "\
use yew::prelude::*;
use yew_nav_link::NavLink;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at(\"/docs\")]
    Docs,
    #[at(\"/docs/api\")]
    DocsApi,
}

#[component]
fn Navigation() -> Html {
    html! {
        <nav>
            // Active on /docs, /docs/api, /docs/*
            <NavLink<Route> to={Route::Docs} partial=true>
                { \"Docs\" }
            </NavLink<Route>>
        </nav>
    }
}";

// ── Helpers ────────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
struct CodeBlockProps {
    code: String,
}

#[function_component]
fn CodeBlock(props: &CodeBlockProps) -> Html {
    let copied = use_state(|| false);
    let code = props.code.clone();

    let on_copy = {
        let copied = copied.clone();
        let code = code.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            let code = code.clone();
            let copied = copied.clone();
            wasm_bindgen_futures::spawn_local(async move {
                copy_to_clipboard(&code);
                copied.set(true);
                gloo_timers::future::TimeoutFuture::new(1500).await;
                copied.set(false);
            });
        })
    };

    let btn_text = if *copied { "✓ Copied" } else { "Copy" };
    let btn_class = if *copied { "code-copy copied" } else { "code-copy" };

    html! {
        <div class="code-block">
            <button class={btn_class} onclick={on_copy}>{ btn_text }</button>
            <pre><code>{ highlight_rust(&props.code) }</code></pre>
        </div>
    }
}

fn copy_to_clipboard(text: &str) {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    let clipboard = navigator.clipboard();
    let promise = clipboard.write_text(text);
    let _ = wasm_bindgen_futures::JsFuture::from(promise);
}

#[derive(Properties, PartialEq)]
struct FeatureCardProps {
    route: Route,
    title: String,
    desc: String,
}

#[function_component]
fn FeatureCard(props: &FeatureCardProps) -> Html {
    let route = props.route.clone();
    html! {
        <a href={route.to_path()} style="text-decoration:none;">
            <div class="feature-card">
                <h4>{ &props.title }</h4>
                <p>{ &props.desc }</p>
            </div>
        </a>
    }
}
