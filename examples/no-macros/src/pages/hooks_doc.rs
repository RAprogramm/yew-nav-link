use crate::demo_popup::DemoBox;
use crate::doc_parser::{parse_doc_block, DocRenderer};
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::hooks::{use_breadcrumbs, use_is_active, use_is_partial_active, use_route_info};
use yew_router::prelude::*;

const HOOKS_SRC: &str = include_str!("../../../../src/hooks/route.rs");

#[function_component]
pub fn HooksDoc() -> Html {
    let doc = parse_doc_block(HOOKS_SRC);

    let current = use_route_info::<Route>();
    let current_path = current.as_ref().map(|r| r.to_path()).unwrap_or("/".into());

    let is_home_active = use_is_active(Route::Home);
    let is_hooks_partial = use_is_partial_active(Route::HooksDoc);
    let crumbs = use_breadcrumbs::<Route>();

    html! {
        <div>
            <DocRenderer {doc} />

            // ── use_route_info ────────────────────────────
            <div class="card">
                <h3>{ "use_route_info — Live" }</h3>
                <p>{ "Returns the current route, or " }<code>{ "None" }</code>{ " if no route matches:" }</p>
                <DemoBox>
                    <p>{ "Route: " }<code>{ format!("{:?}", current) }</code></p>
                    <p>{ "Path: " }<code>{ &current_path }</code></p>
                </DemoBox>
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Navigate with the sidebar to see the value change." }
                </p>
            </div>

            // ── use_is_active ─────────────────────────────
            <div class="card">
                <h3>{ "use_is_active — Live" }</h3>
                <p>{ "Returns " }<code>{ "true" }</code>{ " when the route matches exactly:" }</p>
                <DemoBox>
                    <p>{ "Is Home active? " }
                        <strong style={if is_home_active { "color: var(--green)" } else { "color: var(--red)" }}>
                            { if is_home_active { "Yes" } else { "No" } }
                        </strong>
                    </p>
                </DemoBox>
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Go to the Home page to see " }<code>{ "true" }</code>{ "." }
                </p>
            </div>

            // ── use_is_partial_active ─────────────────────
            <div class="card">
                <h3>{ "use_is_partial_active — Live" }</h3>
                <p>{ "Returns " }<code>{ "true" }</code>{ " when the current path starts with the target:" }</p>
                <DemoBox>
                    <p>{ "Is /hooks partially active on this page? " }
                        <strong style={if is_hooks_partial { "color: var(--green)" } else { "color: var(--red)" }}>
                            { if is_hooks_partial { "Yes" } else { "No" } }
                        </strong>
                    </p>
                </DemoBox>
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "This page is " }<code>{ "/hooks" }</code>
                    { ", so " }<code>{ "use_is_partial_active(Route::HooksDoc)" }</code>
                    { " returns " }<code>{ "true" }</code>
                    { ". Also true on " }<code>{ "/hooks/anything" }</code>{ "." }
                </p>
            </div>

            // ── use_breadcrumbs ───────────────────────────
            <div class="card">
                <h3>{ "use_breadcrumbs — Live" }</h3>
                <p>{ "Returns a " }<code>{ "Vec<BreadcrumbItem<R>>" }</code>
                { " with route hierarchy:" }</p>
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
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Each item has " }<code>{ "route" }</code>{ ", " }<code>{ "label" }</code>
                    { ", and " }<code>{ "is_active" }</code>
                    { " (true for the current page)." }
                </p>
            </div>
        </div>
    }
}
