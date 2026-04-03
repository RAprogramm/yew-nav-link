use crate::demo_popup::DemoBox;
use crate::doc_page::CopyCode;
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{nav_link, Match, NavLink};

const CODE_ROUTE_ENUM: &str = r#"use yew::prelude::*;
use yew_nav_link::{NavLink, Match, nav_link};
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

const CODE_COMPONENT_SYNTAX: &str = r#"#[component]
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

const CODE_SWITCH: &str = r#"fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::About => html! { <AboutPage /> },
        Route::Contact => html! { <ContactPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}"#;

#[function_component]
pub fn BasicExample() -> Html {
    html! {
        <div>
            // ── Header ────────────────────────────────────────
            <div class="example-header">
                <div class="example-header-top">
                    <span class="example-tag tag-blue">{ "starter" }</span>
                    <span class="example-source">{ "examples/basic/" }</span>
                </div>
                <h2>{ "Basic Example" }</h2>
                <p class="example-desc">
                    { "The simplest introduction to " }
                    <code>{ "yew-nav-link" }</code>
                    { ". Demonstrates both component and function syntax with a minimal 3-page application." }
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

            // ── Route Definition ──────────────────────────────
            <div class="card">
                <h3>{ "1. Route Enum" }</h3>
                <p>
                    { "Routes are defined as a Rust enum deriving " }<code>{ "Routable" }</code>
                    { ". Each variant is annotated with " }<code>{ "#[at(\"path\")]" }</code>
                    { " to specify its URL pattern. The " }<code>{ "#[not_found]" }</code>
                    { " attribute marks the 404 fallback." }
                </p>
                <CopyCode code={CODE_ROUTE_ENUM.to_string()} />
                <div class="info-box">
                    <strong>{ "Key point:" }</strong>
                    { " The " }<code>{ "Routable" }</code>
                    { " derive macro generates " }<code>{ "to_path()" }</code>
                    { " and " }<code>{ "from_path()" }</code>
                    { " implementations used by NavLink for path comparison." }
                </div>
            </div>

            // ── Component Syntax ──────────────────────────────
            <div class="card">
                <h3>{ "2. Component Syntax" }</h3>
                <p>
                    { "The " }<code>{ "NavLink<R>" }</code>
                    { " component wraps Yew Router's " }<code>{ "Link<R>" }</code>
                    { " and adds automatic active state detection. Use it when you need custom children (icons, badges, nested elements)." }
                </p>
                <CopyCode code={CODE_COMPONENT_SYNTAX.to_string()} />

                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <nav>
                        <ul class="nav-list">
                            <li class="nav-item">
                                <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                            </li>
                            <li class="nav-item">
                                <NavLink<Route> to={Route::NavLinkDoc}>{ "NavLink" }</NavLink<Route>>
                            </li>
                            <li class="nav-item">
                                <NavLink<Route> to={Route::NavListDoc}>{ "NavList" }</NavLink<Route>>
                            </li>
                        </ul>
                    </nav>
                </DemoBox>
            </div>

            // ── Function Syntax ───────────────────────────────
            <div class="card">
                <h3>{ "3. Function Syntax" }</h3>
                <p>
                    { "For text-only links, use " }<code>{ "nav_link()" }</code>
                    { " — a convenience function that creates a " }<code>{ "NavLink" }</code>
                    { " with the specified match mode." }
                </p>

                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <nav>
                        <ul class="nav-list">
                            <li class="nav-item">
                                { nav_link(Route::Home, "Home (Exact)", Match::Exact) }
                            </li>
                            <li class="nav-item">
                                { nav_link(Route::BasicExample, "This Page (Exact)", Match::Exact) }
                            </li>
                            <li class="nav-item">
                                { nav_link(Route::NavLinkDoc, "NavLink (Partial)", Match::Partial) }
                            </li>
                        </ul>
                    </nav>
                </DemoBox>
            </div>

            // ── Switch ────────────────────────────────────────
            <div class="card">
                <h3>{ "4. Route Switching" }</h3>
                <p>
                    { "The " }<code>{ "switch" }</code>
                    { " function maps each route variant to its page component. " }
                    <code>{ "Switch<Route>" }</code>
                    { " calls this function whenever the URL changes." }
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

            // ── Active State Algorithm ────────────────────────
            <div class="card">
                <h3>{ "Active State Algorithm" }</h3>
                <div class="code-block">
                    <pre><code class="language-text">{ r#"1. NavLink<R> mounts with to={Route::Home}
2. Calls use_route::<R>() → gets current route
3. Converts both to paths:
   - target: route.to_path() → "/"
   - current: current_route.to_path() → "/"
4. Compares:
   - Exact: target == current → "/" == "/" → true → active
   - Partial: current.starts_with(target) → true → active
5. Renders <a class="nav-link active" href="/">Home</a>"# }</code></pre>
                </div>
            </div>

            // ── When to Use ───────────────────────────────────
            <div class="card">
                <h3>{ "When to Use This Pattern" }</h3>
                <div class="feature-grid">
                    <div class="feature-card">
                        <h4>{ "Small Apps" }</h4>
                        <p>{ "Perfect for applications with 3-10 flat routes and simple navigation" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Prototyping" }</h4>
                        <p>{ "Quickly set up navigation without complex routing patterns" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Learning" }</h4>
                        <p>{ "Best starting point to understand how NavLink works before adding complexity" }</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
