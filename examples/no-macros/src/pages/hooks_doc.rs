use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::hooks::{
    use_is_active, use_is_exact_active, use_is_partial_active, use_route_info,
};
use yew_router::prelude::*;

fn code_block(lines: &[&str]) -> Html {
    html! {
        <pre><code>
            { for lines.iter().map(|l| html! { <>{ l }{"\n"}</> }) }
        </code></pre>
    }
}

#[function_component]
pub fn HooksDoc() -> Html {
    let current = use_route_info::<Route>();
    let current_path = current.as_ref().map(|r| r.to_path()).unwrap_or("/".into());

    let is_home_active = use_is_active(Route::Home);
    let is_home_exact = use_is_exact_active(Route::Home);
    let is_hooks_partial = use_is_partial_active(Route::HooksDoc);

    html! {
        <div>
            <div class="card">
                <h2>{ "Route Hooks" }</h2>
                <p>{ "Reactive hooks for working with the current route state." }</p>
            </div>

            <div class="card">
                <h3>{ "Current Route Info" }</h3>
                <div class="demo">
                    <div class="demo-label">{ "Live — current route" }</div>
                    <p>{ "Route: " }<code>{ format!("{:?}", current) }</code></p>
                    <p>{ "Path: " }<code>{ &current_path }</code></p>
                </div>
                { code_block(&[
                    "use yew_nav_link::hooks::use_route_info;",
                    "",
                    "let current = use_route_info::<Route>();",
                    "// Returns Option<Route>",
                ]) }
            </div>

            <div class="card">
                <h3>{ "use_is_active" }</h3>
                <p>{ "True when the route matches exactly:" }</p>
                <div class="demo">
                    <div class="demo-label">{ "Live" }</div>
                    <p>{ "Is Home active? " }
                        <strong style={if is_home_active { "color: var(--green)" } else { "color: var(--red)" }}>
                            { if is_home_active { "Yes" } else { "No" } }
                        </strong>
                    </p>
                </div>
                { code_block(&[
                    "use yew_nav_link::hooks::use_is_active;",
                    "",
                    "let active = use_is_active(Route::Home);",
                    "// Returns bool",
                ]) }
            </div>

            <div class="card">
                <h3>{ "use_is_exact_active" }</h3>
                <div class="demo">
                    <div class="demo-label">{ "Live" }</div>
                    <p>{ "Is Home exactly active? " }
                        <strong style={if is_home_exact { "color: var(--green)" } else { "color: var(--red)" }}>
                            { if is_home_exact { "Yes" } else { "No" } }
                        </strong>
                    </p>
                </div>
                { code_block(&[
                    "use yew_nav_link::hooks::use_is_exact_active;",
                    "",
                    "let exact = use_is_exact_active(Route::Home);",
                ]) }
            </div>

            <div class="card">
                <h3>{ "use_is_partial_active" }</h3>
                <p>{ "True when current route is a child of the given route:" }</p>
                <div class="demo">
                    <div class="demo-label">{ "Live" }</div>
                    <p>{ "Is Hooks partially active? " }
                        <strong style={if is_hooks_partial { "color: var(--green)" } else { "color: var(--red)" }}>
                            { if is_hooks_partial { "Yes" } else { "No" } }
                        </strong>
                    </p>
                </div>
                { code_block(&[
                    "use yew_nav_link::hooks::use_is_partial_active;",
                    "",
                    "let partial = use_is_partial_active(Route::Docs);",
                    "// True on /docs, /docs/api, /docs/guide",
                ]) }
            </div>
        </div>
    }
}
