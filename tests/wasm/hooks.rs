// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Browser tests that mount every reactive hook inside a real router and
//! assert on the rendered output, so the hook bodies actually execute
//! (the native suite can only construct unexecuted hook objects).

use std::rc::Rc;

use wasm_bindgen_test::*;
use yew::prelude::*;
use yew_nav_link::{
    BreadcrumbLabelProvider, BreadcrumbLabelProviderContext, use_breadcrumbs, use_is_active,
    use_is_exact_active, use_is_partial_active, use_query_params, use_route_info
};
use yew_router::prelude::*;

use super::common::{TestRoute, document, fresh_root, navigate, wait_for_render};

wasm_bindgen_test_configure!(run_in_browser);

#[function_component]
fn ActiveProbe() -> Html {
    let exact_docs = use_is_exact_active(TestRoute::Docs);
    let active_api = use_is_active(TestRoute::DocsApi);
    let partial_docs = use_is_partial_active(TestRoute::Docs);
    let partial_home = use_is_partial_active(TestRoute::Home);
    html! {
        <p id="probe">
            { format!("{exact_docs}|{active_api}|{partial_docs}|{partial_home}") }
        </p>
    }
}

#[function_component]
fn ActiveApp() -> Html {
    html! {
        <BrowserRouter>
            <ActiveProbe />
        </BrowserRouter>
    }
}

fn probe_text() -> String {
    document()
        .get_element_by_id("probe")
        .expect("probe should render")
        .text_content()
        .unwrap_or_default()
}

#[wasm_bindgen_test]
async fn active_hooks_reflect_a_nested_route() {
    navigate("/docs/api");
    let root = fresh_root();
    yew::Renderer::<ActiveApp>::with_root(root).render();
    wait_for_render().await;

    assert_eq!(
        probe_text(),
        "false|true|true|false",
        "exact(Docs)|active(DocsApi)|partial(Docs)|partial(Home) at /docs/api"
    );
}

#[function_component]
fn RouteInfoProbe() -> Html {
    let route = use_route_info::<TestRoute>();
    let query = use_query_params();
    html! {
        <p id="probe">
            { format!(
                "{}?lang={}",
                route.map_or_else(|| "none".to_string(), |r| r.to_path()),
                query.get("lang").unwrap_or("none")
            ) }
        </p>
    }
}

#[function_component]
fn RouteInfoApp() -> Html {
    html! {
        <BrowserRouter>
            <RouteInfoProbe />
        </BrowserRouter>
    }
}

#[wasm_bindgen_test]
async fn route_info_and_query_params_read_the_current_url() {
    navigate("/docs?lang=rust");
    let root = fresh_root();
    yew::Renderer::<RouteInfoApp>::with_root(root).render();
    wait_for_render().await;

    assert_eq!(probe_text(), "/docs?lang=rust");
}

#[function_component]
fn BreadcrumbProbe() -> Html {
    let crumbs = use_breadcrumbs::<TestRoute>();
    let rendered: Vec<String> = crumbs
        .iter()
        .map(|item| {
            format!(
                "{}@{}{}",
                item.label,
                item.route.to_path(),
                if item.is_active { "!" } else { "" }
            )
        })
        .collect();
    html! { <p id="probe">{ rendered.join(",") }</p> }
}

#[function_component]
fn BreadcrumbApp() -> Html {
    html! {
        <BrowserRouter>
            <BreadcrumbProbe />
        </BrowserRouter>
    }
}

#[wasm_bindgen_test]
async fn breadcrumbs_resolve_each_prefix_to_its_route() {
    navigate("/docs/api");
    let root = fresh_root();
    yew::Renderer::<BreadcrumbApp>::with_root(root).render();
    wait_for_render().await;

    assert_eq!(
        probe_text(),
        "/@/,/docs@/docs,/docs/api@/docs/api!",
        "default labels are the paths and only the last crumb is active"
    );
}

struct StaticLabels;

impl BreadcrumbLabelProvider for StaticLabels {
    fn label_for_path(&self, path: &str) -> String {
        match path {
            "/" => "Home".to_string(),
            "/docs" => "Docs".to_string(),
            "/docs/api" => "API".to_string(),
            other => other.to_string()
        }
    }
}

#[function_component]
fn ProvidedBreadcrumbApp() -> Html {
    let context = BreadcrumbLabelProviderContext::new(Rc::new(StaticLabels));
    html! {
        <BrowserRouter>
            <ContextProvider<BreadcrumbLabelProviderContext> {context}>
                <BreadcrumbProbe />
            </ContextProvider<BreadcrumbLabelProviderContext>>
        </BrowserRouter>
    }
}

#[wasm_bindgen_test]
async fn breadcrumb_labels_come_from_the_injected_provider() {
    navigate("/docs/api");
    let root = fresh_root();
    yew::Renderer::<ProvidedBreadcrumbApp>::with_root(root).render();
    wait_for_render().await;

    assert_eq!(probe_text(), "Home@/,Docs@/docs,API@/docs/api!");
}
