// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Browser tests for `NavDropdown` keyboard interaction (#217): opening
//! focuses the first item, arrow keys move a roving focus, and Escape closes
//! the menu and returns focus to the toggle.

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit};
use yew::prelude::*;
use yew_nav_link::{
    NavLink, NavList,
    components::{NavDropdown, NavDropdownItem}
};
use yew_router::prelude::*;

use super::common::{TestRoute, document, fresh_root, wait_for_render};

wasm_bindgen_test_configure!(run_in_browser);

#[function_component]
fn DropdownApp() -> Html {
    html! {
        <BrowserRouter>
            <NavList>
                <NavDropdown toggle_text="Menu">
                    <NavDropdownItem>
                        <NavLink<TestRoute> to={TestRoute::Home}>{ "Home" }</NavLink<TestRoute>>
                    </NavDropdownItem>
                    <NavDropdownItem>
                        <NavLink<TestRoute> to={TestRoute::About}>{ "About" }</NavLink<TestRoute>>
                    </NavDropdownItem>
                </NavDropdown>
            </NavList>
        </BrowserRouter>
    }
}

fn active_element() -> Option<HtmlElement> {
    document()
        .active_element()
        .and_then(|el| el.dyn_into::<HtmlElement>().ok())
}

fn press(target: &HtmlElement, key: &str) {
    let init = KeyboardEventInit::new();
    init.set_key(key);
    init.set_bubbles(true);
    let event = KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap();
    target.dispatch_event(&event).unwrap();
}

#[wasm_bindgen_test]
async fn keyboard_opens_moves_focus_and_escapes() {
    let root = fresh_root();
    yew::Renderer::<DropdownApp>::with_root(root).render();
    wait_for_render().await;

    let toggle = document()
        .query_selector(".nav-dropdown-toggle")
        .unwrap()
        .expect("toggle should render")
        .dyn_into::<HtmlElement>()
        .unwrap();

    assert_eq!(
        toggle.get_attribute("aria-expanded").as_deref(),
        Some("false")
    );

    toggle.click();
    wait_for_render().await;
    assert_eq!(
        toggle.get_attribute("aria-expanded").as_deref(),
        Some("true")
    );

    let links = document()
        .query_selector_all(".nav-dropdown-menu a[href]")
        .unwrap();
    let first = links.item(0).unwrap().dyn_into::<HtmlElement>().unwrap();
    let second = links.item(1).unwrap().dyn_into::<HtmlElement>().unwrap();

    assert!(
        active_element().is_some_and(|el| el.is_same_node(Some(first.as_ref()))),
        "opening the menu should focus the first item"
    );

    press(&first, "ArrowDown");
    wait_for_render().await;
    assert!(
        active_element().is_some_and(|el| el.is_same_node(Some(second.as_ref()))),
        "ArrowDown should move focus to the second item"
    );

    press(&second, "Escape");
    wait_for_render().await;
    assert_eq!(
        toggle.get_attribute("aria-expanded").as_deref(),
        Some("false")
    );
    assert!(
        active_element().is_some_and(|el| el.is_same_node(Some(toggle.as_ref()))),
        "Escape should close the menu and return focus to the toggle"
    );
}
