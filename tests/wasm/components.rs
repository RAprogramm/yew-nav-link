// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Browser tests locking in that render-only components emit a real `class`
//! attribute — not the bogus `classes` attribute produced by the `html!`
//! `{classes}` shorthand. See `tests/html_class_shorthand_guard.rs` for the
//! static counterpart.

use wasm_bindgen_test::*;
use yew::prelude::*;
use yew_nav_link::{NavBadge, NavItem};

use super::common::{document, fresh_root, wait_for_render};

wasm_bindgen_test_configure!(run_in_browser);

#[function_component]
fn ItemApp() -> Html {
    html! { <NavItem>{ "Home" }</NavItem> }
}

#[function_component]
fn BadgeApp() -> Html {
    html! { <NavBadge>{ "9" }</NavBadge> }
}

#[wasm_bindgen_test]
async fn nav_item_emits_class_not_classes() {
    let root = fresh_root();
    yew::Renderer::<ItemApp>::with_root(root).render();
    wait_for_render().await;

    let li = document()
        .query_selector("li")
        .unwrap()
        .expect("NavItem should render an <li>");

    assert_eq!(
        li.get_attribute("class").as_deref(),
        Some("nav-item"),
        "NavItem must expose its base class via the `class` attribute"
    );
    assert!(
        li.get_attribute("classes").is_none(),
        "NavItem must not render a bogus `classes` attribute"
    );
}

#[wasm_bindgen_test]
async fn nav_badge_emits_class_not_classes() {
    let root = fresh_root();
    yew::Renderer::<BadgeApp>::with_root(root).render();
    wait_for_render().await;

    let span = document()
        .query_selector("span")
        .unwrap()
        .expect("NavBadge should render a <span>");

    let class = span
        .get_attribute("class")
        .expect("NavBadge must expose its classes via the `class` attribute");
    assert!(
        class.contains("nav-badge"),
        "NavBadge `class` should contain the base class, got {class:?}"
    );
    assert!(
        span.get_attribute("classes").is_none(),
        "NavBadge must not render a bogus `classes` attribute"
    );
}
