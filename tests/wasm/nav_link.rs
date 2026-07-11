// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Browser tests for `NavLink`. Each `#[wasm_bindgen_test]` renders a
//! small Yew app under `BrowserRouter`, waits for Yew to flush, and reads
//! the resulting DOM directly.

use wasm_bindgen_test::*;
use yew::prelude::*;
use yew_nav_link::NavLink;
use yew_router::prelude::*;

use super::common::{TestRoute, fresh_root, navigate, wait_for_render};

wasm_bindgen_test_configure!(run_in_browser);

#[function_component]
fn TwoLinkApp() -> Html {
    html! {
        <BrowserRouter>
            <NavLink<TestRoute> to={TestRoute::Home}>{ "Home" }</NavLink<TestRoute>>
            <NavLink<TestRoute> to={TestRoute::About}>{ "About" }</NavLink<TestRoute>>
        </BrowserRouter>
    }
}

#[function_component]
fn CustomClassApp() -> Html {
    html! {
        <BrowserRouter>
            <NavLink<TestRoute> to={TestRoute::Home} class="menu-item" active_class="is-current">
                { "Home" }
            </NavLink<TestRoute>>
        </BrowserRouter>
    }
}

#[function_component]
fn PartialApp() -> Html {
    html! {
        <BrowserRouter>
            <NavLink<TestRoute> to={TestRoute::Docs} partial=true>{ "Docs" }</NavLink<TestRoute>>
        </BrowserRouter>
    }
}

#[function_component]
fn BasenameApp() -> Html {
    html! {
        <BrowserRouter basename="/app">
            <NavLink<TestRoute> to={TestRoute::Home}>{ "Home" }</NavLink<TestRoute>>
        </BrowserRouter>
    }
}

#[wasm_bindgen_test]
async fn active_link_at_home_carries_aria_current_and_active_class() {
    navigate("/");
    let root = fresh_root();
    yew::Renderer::<TwoLinkApp>::with_root(root.clone()).render();
    wait_for_render().await;

    let links = root.get_elements_by_tag_name("a");
    assert_eq!(links.length(), 2, "expected two anchors");

    let home = links.item(0).unwrap();
    let about = links.item(1).unwrap();

    assert_eq!(
        home.get_attribute("class").as_deref(),
        Some("nav-link active"),
        "home link should carry the default base + active class"
    );
    assert_eq!(
        home.get_attribute("aria-current").as_deref(),
        Some("page"),
        "active link should advertise aria-current to assistive tech"
    );
    assert_eq!(
        about.get_attribute("class").as_deref(),
        Some("nav-link"),
        "inactive link should carry only the base class"
    );
    assert!(
        about.get_attribute("aria-current").is_none(),
        "inactive link must not set aria-current"
    );

    root.remove();
}

#[wasm_bindgen_test]
async fn custom_class_and_active_class_replace_defaults() {
    navigate("/");
    let root = fresh_root();
    yew::Renderer::<CustomClassApp>::with_root(root.clone()).render();
    wait_for_render().await;

    let link = root.get_elements_by_tag_name("a").item(0).unwrap();
    assert_eq!(
        link.get_attribute("class").as_deref(),
        Some("menu-item is-current"),
        "custom class slots must fully replace the defaults"
    );

    root.remove();
}

#[wasm_bindgen_test]
async fn partial_match_keeps_parent_active_under_nested_route() {
    navigate("/docs/api");
    let root = fresh_root();
    yew::Renderer::<PartialApp>::with_root(root.clone()).render();
    wait_for_render().await;

    let link = root.get_elements_by_tag_name("a").item(0).unwrap();
    assert_eq!(
        link.get_attribute("class").as_deref(),
        Some("nav-link active"),
        "partial=true should keep Docs active while at /docs/api"
    );
    assert_eq!(
        link.get_attribute("aria-current").as_deref(),
        Some("page"),
        "partial-matched active link should still announce aria-current"
    );

    navigate("/");
    root.remove();
}

#[wasm_bindgen_test]
async fn inactive_link_emits_relative_href_for_its_target() {
    navigate("/");
    let root = fresh_root();
    yew::Renderer::<TwoLinkApp>::with_root(root.clone()).render();
    wait_for_render().await;

    let links = root.get_elements_by_tag_name("a");
    let about = links.item(1).unwrap();
    assert_eq!(
        about.get_attribute("href").as_deref(),
        Some("/about"),
        "href should match the target route's path"
    );

    root.remove();
}

#[wasm_bindgen_test]
async fn href_includes_router_basename() {
    navigate("/app/");
    let root = fresh_root();
    yew::Renderer::<BasenameApp>::with_root(root.clone()).render();
    wait_for_render().await;

    let link = root.get_elements_by_tag_name("a").item(0).unwrap();
    let href = link.get_attribute("href").unwrap_or_default();
    assert!(
        href.starts_with("/app"),
        "href must include the basename, got {href:?}"
    );

    navigate("/");
    root.remove();
}

/// Dispatches an untrusted `click` carrying the given modifier flags on
/// `target`, with a window-level guard that always cancels the browser's
/// default action last so the test page never actually navigates away.
///
/// Returns `true` when some earlier handler (i.e. `NavLink`'s onclick) had
/// already called `prevent_default` by the time the event bubbled to
/// `window`.
fn click_with_modifiers(target: &web_sys::Element, ctrl: bool, meta: bool) -> bool {
    use std::{cell::Cell, rc::Rc};

    use wasm_bindgen::{JsCast, prelude::Closure};

    let window = web_sys::window().unwrap();
    let seen_prevented = Rc::new(Cell::new(false));
    let guard = {
        let seen_prevented = Rc::clone(&seen_prevented);
        Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
            seen_prevented.set(event.default_prevented());
            event.prevent_default();
        })
    };
    window
        .add_event_listener_with_callback("click", guard.as_ref().unchecked_ref())
        .unwrap();

    let init = web_sys::MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_ctrl_key(ctrl);
    init.set_meta_key(meta);
    let event = web_sys::MouseEvent::new_with_mouse_event_init_dict("click", &init).unwrap();
    target.dispatch_event(&event).unwrap();

    window
        .remove_event_listener_with_callback("click", guard.as_ref().unchecked_ref())
        .unwrap();
    seen_prevented.get()
}

#[wasm_bindgen_test]
async fn plain_click_navigates_through_the_router() {
    navigate("/");
    let root = fresh_root();
    yew::Renderer::<TwoLinkApp>::with_root(root.clone()).render();
    wait_for_render().await;

    let about = root.get_elements_by_tag_name("a").item(1).unwrap();
    let prevented = click_with_modifiers(&about, false, false);
    wait_for_render().await;

    assert!(prevented, "plain click must be intercepted by NavLink");
    let path = web_sys::window().unwrap().location().pathname().unwrap();
    assert_eq!(path, "/about", "plain click must push the target route");
}

#[wasm_bindgen_test]
async fn modifier_clicks_fall_through_to_the_browser() {
    navigate("/");
    let root = fresh_root();
    yew::Renderer::<TwoLinkApp>::with_root(root.clone()).render();
    wait_for_render().await;

    let about = root.get_elements_by_tag_name("a").item(1).unwrap();
    let ctrl_prevented = click_with_modifiers(&about, true, false);
    wait_for_render().await;
    let meta_prevented = click_with_modifiers(&about, false, true);
    wait_for_render().await;

    assert!(
        !ctrl_prevented && !meta_prevented,
        "ctrl/meta clicks must keep the browser default (open in new tab)"
    );
    let path = web_sys::window().unwrap().location().pathname().unwrap();
    assert_eq!(path, "/", "ctrl/meta clicks must not be routed by NavLink");
}
