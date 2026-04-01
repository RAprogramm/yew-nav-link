use crate::demo_popup::DemoBox;
use crate::doc_parser::{parse_doc_block, DocRenderer};
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::hooks::{
    use_breadcrumbs, use_is_active, use_is_exact_active, use_is_partial_active, use_route_info,
};
use yew_router::prelude::*;

const HOOKS_SRC: &str = include_str!("../../../../src/hooks/route.rs");

#[function_component]
pub fn HooksDoc() -> Html {
    let doc = parse_doc_block(HOOKS_SRC);

    let current = use_route_info::<Route>();
    let current_path = current.as_ref().map(|r| r.to_path()).unwrap_or("/".into());

    let is_home_active = use_is_active(Route::Home);
    let is_home_exact = use_is_exact_active(Route::Home);
    let is_hooks_partial = use_is_partial_active(Route::HooksDoc);

    let crumbs = use_breadcrumbs::<Route>();

    html! {
        <div>
            <DocRenderer {doc} />

            <div class="card">
                <h3>{ "Live — Current Route" }</h3>
                <DemoBox>
                    <p>{ "Route: " }<code>{ format!("{:?}", current) }</code></p>
                    <p>{ "Path: " }<code>{ &current_path }</code></p>
                    <p style="margin-top:12px; color: var(--text-dim); font-size: 13px;">
                        { "Click links in sidebar to change route." }
                    </p>
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "Live — use_is_active" }</h3>
                <DemoBox>
                    <p>{ "Is Home active? " }
                        <strong style={if is_home_active { "color: var(--green)" } else { "color: var(--red)" }}>
                            { if is_home_active { "Yes" } else { "No" } }
                        </strong>
                    </p>
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "Live — use_is_exact_active" }</h3>
                <DemoBox>
                    <p>{ "Is Home exactly active? " }
                        <strong style={if is_home_exact { "color: var(--green)" } else { "color: var(--red)" }}>
                            { if is_home_exact { "Yes" } else { "No" } }
                        </strong>
                    </p>
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "Live — use_is_partial_active" }</h3>
                <DemoBox>
                    <p>{ "Is Hooks partially active? " }
                        <strong style={if is_hooks_partial { "color: var(--green)" } else { "color: var(--red)" }}>
                            { if is_hooks_partial { "Yes" } else { "No" } }
                        </strong>
                    </p>
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "Live — Breadcrumbs" }</h3>
                <DemoBox>
                    <nav aria-label="breadcrumb">
                        <ol class="breadcrumb-list">
                            { for crumbs.iter().map(|crumb| {
                                html! {
                                    <li class={if crumb.is_active { "breadcrumb-item active" } else { "breadcrumb-item" }}>
                                        { &crumb.label }
                                    </li>
                                }
                            })}
                        </ol>
                    </nav>
                </DemoBox>
            </div>
        </div>
    }
}
