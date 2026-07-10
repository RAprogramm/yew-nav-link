// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Browser tests for `use_navigation`. They prove the programmatic
//! navigation callbacks route through yew-router's `Navigator`, so a
//! configured basename is honored (the regression fixed in #214).

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_nav_link::use_navigation;
use yew_router::prelude::*;

use super::common::{TestRoute, document, fresh_root, navigate, wait_for_render};

wasm_bindgen_test_configure!(run_in_browser);

#[function_component]
fn PushButton() -> Html {
    let nav = use_navigation::<TestRoute>();
    let onclick = nav
        .push_callback(TestRoute::About)
        .reform(|_: MouseEvent| ());
    html! { <button id="go" {onclick}>{ "go" }</button> }
}

#[function_component]
fn BasenameApp() -> Html {
    html! {
        <BrowserRouter basename="/app">
            <PushButton />
        </BrowserRouter>
    }
}

fn location_path() -> String {
    web_sys::window().unwrap().location().pathname().unwrap()
}

#[wasm_bindgen_test]
async fn push_callback_prepends_router_basename() {
    navigate("/app/");
    let root = fresh_root();
    yew::Renderer::<BasenameApp>::with_root(root).render();
    wait_for_render().await;

    let button = document()
        .get_element_by_id("go")
        .expect("button should render")
        .dyn_into::<HtmlElement>()
        .unwrap();
    button.click();
    wait_for_render().await;

    assert_eq!(
        location_path(),
        "/app/about",
        "push_callback must prepend the router basename `/app`"
    );
}
