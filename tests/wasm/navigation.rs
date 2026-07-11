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

#[function_component]
fn HistoryButtons() -> Html {
    let nav = use_navigation::<TestRoute>();
    let push = nav
        .push_callback(TestRoute::Docs)
        .reform(|_: MouseEvent| ());
    let replace = nav
        .replace_callback(TestRoute::About)
        .reform(|_: MouseEvent| ());
    let go_minus_one = nav.go_callback(-1).reform(|_: MouseEvent| ());
    let back = nav.go_back.clone().reform(|_: MouseEvent| ());
    let forward = nav.go_forward.clone().reform(|_: MouseEvent| ());
    html! {
        <>
            <button id="push" onclick={push}>{ "push" }</button>
            <button id="replace" onclick={replace}>{ "replace" }</button>
            <button id="go" onclick={go_minus_one}>{ "go" }</button>
            <button id="back" onclick={back}>{ "back" }</button>
            <button id="forward" onclick={forward}>{ "forward" }</button>
        </>
    }
}

#[function_component]
fn HistoryApp() -> Html {
    html! {
        <BrowserRouter>
            <HistoryButtons />
        </BrowserRouter>
    }
}

fn click_by_id(id: &str) {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("button {id} should render"))
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();
}

#[wasm_bindgen_test]
async fn replace_callback_rewrites_the_current_entry() {
    navigate("/");
    let root = fresh_root();
    yew::Renderer::<HistoryApp>::with_root(root).render();
    wait_for_render().await;

    click_by_id("push");
    wait_for_render().await;
    assert_eq!(location_path(), "/docs");

    click_by_id("replace");
    wait_for_render().await;
    assert_eq!(location_path(), "/about");

    click_by_id("back");
    wait_for_render().await;
    assert_eq!(
        location_path(),
        "/",
        "replace must not add a history entry, so back skips /docs"
    );
}

#[wasm_bindgen_test]
async fn go_back_and_forward_walk_the_history_stack() {
    navigate("/");
    let root = fresh_root();
    yew::Renderer::<HistoryApp>::with_root(root).render();
    wait_for_render().await;

    click_by_id("push");
    wait_for_render().await;
    assert_eq!(location_path(), "/docs");

    click_by_id("back");
    wait_for_render().await;
    assert_eq!(location_path(), "/");

    click_by_id("forward");
    wait_for_render().await;
    assert_eq!(location_path(), "/docs");

    click_by_id("go");
    wait_for_render().await;
    assert_eq!(location_path(), "/", "go(-1) behaves like back");
}
