use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

fn code_block(lines: &[&str]) -> Html {
    html! {
        <pre><code>
            { for lines.iter().map(|l| html! { <>{ l }{"\n"}</> }) }
        </code></pre>
    }
}

#[function_component]
pub fn Home() -> Html {
    html! {
        <div>
            <div class="hero">
                <h1>{ "yew-nav-link" }</h1>
                <p>{ "Navigation components for Yew — automatic active state, ARIA semantics, keyboard nav." }</p>
                <div class="hero-badges">
                    <span class="badge badge-accent">{ "Yew 0.23" }</span>
                    <span class="badge badge-green">{ "129 tests" }</span>
                    <span class="badge badge-purple">{ "MIT" }</span>
                </div>
            </div>

            <div class="card">
                <h2>{ "Quick Start" }</h2>
                { code_block(&[
                    "[dependencies]",
                    "yew-nav-link = \"0.6\"",
                ]) }

                <h3>{ "Component Syntax" }</h3>
                { code_block(&[
                    "use yew_nav_link::NavLink;",
                    "use yew_router::prelude::*;",
                    "",
                    "#[component]",
                    "fn Menu() -> Html {",
                    "    html! {",
                    "        <NavLink<Route> to={Route::Home}>",
                    "            { \"Home\" }",
                    "        </NavLink<Route>>",
                    "    }",
                    "}",
                ]) }

                <h3>{ "Function Syntax" }</h3>
                { code_block(&[
                    "use yew_nav_link::{nav_link, Match};",
                    "",
                    "html! {",
                    "    { nav_link(Route::Home, \"Home\", Match::Exact) }",
                    "    { nav_link(Route::Docs, \"Docs\", Match::Partial) }",
                    "}",
                ]) }
            </div>

            <div class="card">
                <h2>{ "All Components" }</h2>
                <p>{ "Interactive demos with live code examples:" }</p>
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

            <div class="card">
                <h2>{ "Hooks & Utilities" }</h2>
                <div class="feature-grid">
                    <FeatureCard route={Route::HooksDoc} title="Route Hooks" desc="use_route_info, use_is_active, use_is_exact_active, use_is_partial_active." />
                    <FeatureCard route={Route::BreadcrumbsDoc} title="Breadcrumbs" desc="Auto-generated breadcrumb trail." />
                    <FeatureCard route={Route::UtilsDoc} title="Path Utilities" desc="is_absolute, join_paths, normalize_path." />
                </div>
            </div>
        </div>
    }
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
