// SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Shared helpers for browser tests: fresh DOM root, render flush, route
//! enum.

use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, Element, HtmlElement, window};
use yew_router::prelude::Routable;

#[derive(Clone, PartialEq, Debug, Routable)]
pub enum TestRoute {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/docs")]
    Docs,
    #[at("/docs/api")]
    DocsApi
}

pub fn document() -> Document {
    window().unwrap().document().unwrap()
}

pub fn body() -> HtmlElement {
    document().body().unwrap()
}

/// Allocates a fresh `<div>` under `<body>` for the next render.
pub fn fresh_root() -> Element {
    let root = document().create_element("div").unwrap();
    body().append_child(&root).unwrap();
    root
}

/// Pushes `path` into the history stack so `BrowserRouter` reads it on the
/// next render. Yew's router subscribes to history changes, so this also
/// triggers a re-render of any mounted app.
pub fn navigate(path: &str) {
    window()
        .unwrap()
        .history()
        .unwrap()
        .push_state_with_url(&JsValue::NULL, "", Some(path))
        .unwrap();
}

/// Yew renders asynchronously. Wait long enough for the scheduler to flush
/// before reading the DOM.
pub async fn wait_for_render() {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        let cb = Closure::once_into_js(move || {
            resolve.call0(&JsValue::NULL).unwrap();
        });
        window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 50)
            .unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}
