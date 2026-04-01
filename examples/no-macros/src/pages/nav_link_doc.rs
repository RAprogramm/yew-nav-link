use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{nav_link, Match, NavLink};

fn code_block(lines: &[&str]) -> Html {
    html! {
        <pre><code>
            { for lines.iter().map(|l| html! { <>{ l }{"\n"}</> }) }
        </code></pre>
    }
}

#[function_component]
pub fn NavLinkDoc() -> Html {
    html! {
        <div>
            <div class="card">
                <h2>{ "NavLink" }</h2>
                <p>
                    { "Wraps " }<code>{ "yew-router" }</code>{ " Link with automatic " }
                    <code>{ "active" }</code>{ " CSS class when the route matches." }
                </p>
            </div>

            <div class="card">
                <h3>{ "Component Syntax — Exact Match" }</h3>
                <p>{ "NavLink adds the active class when the current route matches exactly:" }</p>

                <div class="demo">
                    <div class="demo-label">{ "Live Demo" }</div>
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
                </div>

                { code_block(&[
                    "use yew_nav_link::NavLink;",
                    "use yew_router::prelude::*;",
                    "",
                    "html! {",
                    "    <nav>",
                    "        <NavLink<Route> to={Route::Home}>",
                    "            { \"Home\" }",
                    "        </NavLink<Route>>",
                    "    </nav>",
                    "}",
                ]) }
            </div>

            <div class="card">
                <h3>{ "Partial Matching" }</h3>
                <p>
                    { "Use " }<code>{ "partial=true" }</code>{ " to keep parent links active on nested routes." }
                </p>

                <div class="demo">
                    <div class="demo-label">{ "Live Demo" }</div>
                    <nav>
                        <ul class="nav-list">
                            <li class="nav-item">
                                <NavLink<Route> to={Route::Home} partial=true>
                                    { "Home (partial) — stays active on nested" }
                                </NavLink<Route>>
                            </li>
                        </ul>
                    </nav>
                </div>

                { code_block(&[
                    "<NavLink<Route> to={Route::Docs} partial=true>",
                    "    { \"Docs\" }",
                    "</NavLink<Route>>",
                    "",
                    "// Active on /docs, /docs/api, /docs/guide, ...",
                ]) }
            </div>

            <div class="card">
                <h3>{ "Function Syntax — nav_link()" }</h3>
                <p>
                    { "For text-only links, use " }<code>{ "nav_link()" }</code>{ " with " }
                    <code>{ "Match" }</code>{ " mode:" }
                </p>

                <div class="demo">
                    <div class="demo-label">{ "Live Demo" }</div>
                    <nav>
                        <ul class="nav-list">
                            <li>{ nav_link(Route::Home, "Home (Exact)", Match::Exact) }</li>
                            <li>{ nav_link(Route::NavLinkDoc, "NavLink Doc (Exact)", Match::Exact) }</li>
                        </ul>
                    </nav>
                </div>

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
                <h3>{ "Props" }</h3>
                <table>
                    <tr>
                        <td><code>{ "to" }</code></td>
                        <td>{ "Route value (required)" }</td>
                    </tr>
                    <tr>
                        <td><code>{ "partial" }</code></td>
                        <td>{ "bool — Enable partial path matching (default: false)" }</td>
                    </tr>
                    <tr>
                        <td><code>{ "classes" }</code></td>
                        <td>{ "Classes — Additional CSS classes" }</td>
                    </tr>
                </table>
            </div>
        </div>
    }
}
