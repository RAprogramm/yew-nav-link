// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Browser tests for `NavTabs`/`NavTab`: tab clicks reach the consumer's
//! handler, the roving tabindex keeps only the active tab in the tab
//! sequence, and arrow/Home/End keys move focus per the ARIA tabs pattern.

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit};
use yew::prelude::*;
use yew_nav_link::components::{NavTab, NavTabs};

use super::common::{document, fresh_root, wait_for_render};

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Properties, Clone, PartialEq)]
struct TabsAppProps {
    on_select: Callback<u32>
}

#[function_component]
fn TabsApp(props: &TabsAppProps) -> Html {
    let first = props.on_select.reform(|_: MouseEvent| 0);
    let second = props.on_select.reform(|_: MouseEvent| 1);
    html! {
        <NavTabs>
            <NavTab active=true id="tab-a" onclick={first}>{ "A" }</NavTab>
            <NavTab active=false id="tab-b" onclick={second}>{ "B" }</NavTab>
            <NavTab active=false disabled=true>{ "C" }</NavTab>
        </NavTabs>
    }
}

fn render_tabs(selections: &Rc<RefCell<Vec<u32>>>) {
    let root = fresh_root();
    let on_select = {
        let selections = Rc::clone(selections);
        Callback::from(move |index: u32| selections.borrow_mut().push(index))
    };
    yew::Renderer::<TabsApp>::with_root_and_props(
        root,
        TabsAppProps {
            on_select
        }
    )
    .render();
}

fn tab(id: &str) -> HtmlElement {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("tab {id} should render"))
        .dyn_into::<HtmlElement>()
        .unwrap()
}

fn press(target: &HtmlElement, key: &str) {
    let init = KeyboardEventInit::new();
    init.set_key(key);
    init.set_bubbles(true);
    init.set_cancelable(true);
    let event = KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap();
    target.dispatch_event(&event).unwrap();
}

fn focused_id() -> Option<String> {
    document().active_element().map(|element| element.id())
}

#[wasm_bindgen_test]
async fn tab_click_reaches_the_consumer_handler() {
    let selections = Rc::new(RefCell::new(Vec::new()));
    render_tabs(&selections);
    wait_for_render().await;

    tab("tab-b").click();
    wait_for_render().await;
    assert_eq!(*selections.borrow(), vec![1]);
}

#[wasm_bindgen_test]
async fn roving_tabindex_keeps_only_the_active_tab_in_tab_order() {
    let selections = Rc::new(RefCell::new(Vec::new()));
    render_tabs(&selections);
    wait_for_render().await;

    assert_eq!(tab("tab-a").get_attribute("tabindex").as_deref(), Some("0"));
    assert_eq!(
        tab("tab-b").get_attribute("tabindex").as_deref(),
        Some("-1")
    );
}

#[wasm_bindgen_test]
async fn arrow_keys_move_focus_with_wrap_and_skip_disabled_tabs() {
    let selections = Rc::new(RefCell::new(Vec::new()));
    render_tabs(&selections);
    wait_for_render().await;

    let first = tab("tab-a");
    let _ = first.focus();
    press(&first, "ArrowRight");
    wait_for_render().await;
    assert_eq!(focused_id().as_deref(), Some("tab-b"));

    press(&tab("tab-b"), "ArrowRight");
    wait_for_render().await;
    assert_eq!(
        focused_id().as_deref(),
        Some("tab-a"),
        "the disabled third tab is skipped and focus wraps to the first"
    );

    press(&tab("tab-a"), "ArrowLeft");
    wait_for_render().await;
    assert_eq!(focused_id().as_deref(), Some("tab-b"));
}

#[wasm_bindgen_test]
async fn home_and_end_jump_to_the_first_and_last_enabled_tab() {
    let selections = Rc::new(RefCell::new(Vec::new()));
    render_tabs(&selections);
    wait_for_render().await;

    let first = tab("tab-a");
    let _ = first.focus();
    press(&first, "End");
    wait_for_render().await;
    assert_eq!(focused_id().as_deref(), Some("tab-b"));

    press(&tab("tab-b"), "Home");
    wait_for_render().await;
    assert_eq!(focused_id().as_deref(), Some("tab-a"));
}
