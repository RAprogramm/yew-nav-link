use yew::prelude::*;
use yew_nav_link::hooks::{use_breadcrumbs, use_is_active, use_is_partial_active, use_route_info};
use yew_router::Routable;

use crate::{
    doc_page::{DemoCard, DocPage, Tip},
    routes::Route
};

const SRC: &str = include_str!("../../../src/hooks/route_info/mod.rs");

const CODE_ROUTE_INFO: &str = "\
use yew_nav_link::hooks::use_route_info;

# #[derive(Clone, PartialEq, Debug, Routable)]
# enum Route { #[at(\"/\")] Home }
#[component]
fn CurrentPage() -> Html {
    let route = use_route_info::<Route>();
    html! { <p>{ format!(\"Route: {:?}\", route) }</p> }
}";

const CODE_IS_ACTIVE: &str = r#"
use yew_nav_link::hooks::use_is_active;

# #[derive(Clone, PartialEq, Debug, Routable)]
# enum Route { #[at("/")] Home }
#[component]
fn Nav() -> Html {
    let active = use_is_active(Route::Home);
    html! {
        <span style={if active { "color: var(--green)" } else { "color: var(--red)" }}>
            { "Home" }
        </span>
    }
}"#;

const CODE_PARTIAL: &str = "\
use yew_nav_link::hooks::use_is_partial_active;

# #[derive(Clone, PartialEq, Debug, Routable)]
# enum Route { #[at(\"/docs\")] Docs }
#[component]
fn Sidebar() -> Html {
    let docs_active = use_is_partial_active(Route::Docs);
    html! {
        <a class={if docs_active { \"active\" } else { \"\" }}>
            { \"Documentation\" }
        </a>
    }
}";

#[function_component]
pub fn HooksDoc() -> Html {
    let current = use_route_info::<Route>();
    let current_path = current.as_ref().map(|r| r.to_path()).unwrap_or("/".into());

    let is_home_active = use_is_active(Route::Home);
    let is_hooks_partial = use_is_partial_active(Route::HooksDoc);
    let crumbs = use_breadcrumbs::<Route>();

    html! {
        <DocPage source={SRC.to_string()}>

            <DemoCard
                title="use_route_info"
                description={html!{
                    <>
                        { "Returns " }<code>{ "Option<R>" }</code>
                        { " — the current route or " }<code>{ "None" }</code>{ ":" }
                    </>
                }}
                code={CODE_ROUTE_INFO.to_string()}
                tip={html!{
                    <Tip>
                        { "Navigate with the sidebar to see the value change." }
                    </Tip>
                }}
            >
                <p>{ "Route: " }<code>{ format!("{:?}", current) }</code></p>
                <p>{ "Path: " }<code>{ &current_path }</code></p>
            </DemoCard>

            <DemoCard
                title="use_is_active"
                description={html!{
                    <>
                        { "Returns " }<code>{ "true" }</code>
                        { " when the route matches exactly:" }
                    </>
                }}
                code={CODE_IS_ACTIVE.to_string()}
                tip={html!{
                    <Tip>
                        { "Go to Home page to see " }<code>{ "true" }</code>{ "." }
                    </Tip>
                }}
            >
                <p>{ "Is Home active? " }
                    <strong style={if is_home_active { "color: var(--green)" } else { "color: var(--red)" }}>
                        { if is_home_active { "Yes" } else { "No" } }
                    </strong>
                </p>
            </DemoCard>

            <DemoCard
                title="use_is_partial_active"
                description={html!{
                    <>
                        { "Returns " }<code>{ "true" }</code>
                        { " when the current path starts with the target:" }
                    </>
                }}
                code={CODE_PARTIAL.to_string()}
                tip={html!{
                    <Tip>
                        { "This page is " }<code>{ "/hooks" }</code>
                        { ", so it matches " }<code>{ "/hooks" }</code>
                        { " via prefix. Also true on " }<code>{ "/hooks/anything" }</code>{ "." }
                    </Tip>
                }}
            >
                <p>{ "Is /hooks partially active? " }
                    <strong style={if is_hooks_partial { "color: var(--green)" } else { "color: var(--red)" }}>
                        { if is_hooks_partial { "Yes" } else { "No" } }
                    </strong>
                </p>
            </DemoCard>

            <DemoCard
                title="use_breadcrumbs"
                description={html!{
                    <>
                        { "Returns " }<code>{ "Vec<BreadcrumbItem<R>>" }</code>
                        { " with route hierarchy:" }
                    </>
                }}
                code={"use yew_nav_link::hooks::use_breadcrumbs;\n\nlet crumbs = use_breadcrumbs::<Route>();\n// crumbs[0].label = \"/\"\n// crumbs[1].label = \"/hooks\"\n// crumbs[1].is_active = true".to_string()}
                tip={html!{
                    <Tip>
                        { "Each item has " }<code>{ "route" }</code>{ ", " }<code>{ "label" }</code>
                        { ", and " }<code>{ "is_active" }</code>{ "." }
                    </Tip>
                }}
            >
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
            </DemoCard>

        </DocPage>
    }
}
