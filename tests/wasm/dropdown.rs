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

#[function_component]
fn RichDropdownApp() -> Html {
    html! {
        <BrowserRouter>
            <NavList>
                <NavDropdown toggle_text="Tools" id="tools-menu">
                    <NavDropdownItem>
                        <NavLink<TestRoute> to={TestRoute::Home}>{ "Home" }</NavLink<TestRoute>>
                    </NavDropdownItem>
                    <NavDropdownItem disabled=true>
                        <NavLink<TestRoute> to={TestRoute::Docs}>{ "Docs" }</NavLink<TestRoute>>
                    </NavDropdownItem>
                    <NavDropdownItem>
                        <NavLink<TestRoute> to={TestRoute::About}>{ "About" }</NavLink<TestRoute>>
                    </NavDropdownItem>
                </NavDropdown>
            </NavList>
        </BrowserRouter>
    }
}

fn toggle() -> HtmlElement {
    document()
        .query_selector(".nav-dropdown-toggle")
        .unwrap()
        .expect("toggle should render")
        .dyn_into::<HtmlElement>()
        .unwrap()
}

fn menu_link(text: &str) -> HtmlElement {
    let list = document()
        .query_selector_all(".nav-dropdown-menu a[href]")
        .unwrap();
    (0..list.length())
        .filter_map(|index| list.item(index))
        .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
        .find(|link| link.text_content().as_deref() == Some(text))
        .unwrap_or_else(|| panic!("menu link {text} should render"))
}

fn assert_focused(element: &HtmlElement, context: &str) {
    assert!(
        active_element().is_some_and(|el| el.is_same_node(Some(element.as_ref()))),
        "{context}"
    );
}

#[wasm_bindgen_test]
async fn menu_id_wires_the_toggle_aria_controls() {
    let root = fresh_root();
    yew::Renderer::<RichDropdownApp>::with_root(root).render();
    wait_for_render().await;

    assert_eq!(
        toggle().get_attribute("aria-controls").as_deref(),
        Some("tools-menu")
    );
    let menu = document()
        .get_element_by_id("tools-menu")
        .expect("menu should carry the id prop");
    assert!(menu.class_name().contains("nav-dropdown-menu"));
}

#[wasm_bindgen_test]
async fn arrow_up_on_closed_toggle_opens_and_focuses_the_last_item() {
    let root = fresh_root();
    yew::Renderer::<RichDropdownApp>::with_root(root).render();
    wait_for_render().await;

    let toggle = toggle();
    let _ = toggle.focus();
    press(&toggle, "ArrowUp");
    wait_for_render().await;

    assert_eq!(
        toggle.get_attribute("aria-expanded").as_deref(),
        Some("true")
    );
    assert_focused(
        &menu_link("About"),
        "ArrowUp from the closed toggle should focus the last enabled item"
    );
}

#[wasm_bindgen_test]
async fn keyboard_navigation_skips_disabled_items_and_wraps() {
    let root = fresh_root();
    yew::Renderer::<RichDropdownApp>::with_root(root).render();
    wait_for_render().await;

    toggle().click();
    wait_for_render().await;
    let home = menu_link("Home");
    let about = menu_link("About");
    assert_focused(&home, "opening by click focuses the first enabled item");

    press(&home, "ArrowDown");
    wait_for_render().await;
    assert_focused(
        &about,
        "ArrowDown must skip the link inside the disabled item"
    );

    press(&about, "ArrowDown");
    wait_for_render().await;
    assert_focused(&home, "ArrowDown from the last item wraps to the first");

    press(&home, "End");
    wait_for_render().await;
    assert_focused(&about, "End jumps to the last enabled item");

    press(&about, "Home");
    wait_for_render().await;
    assert_focused(&home, "Home jumps to the first enabled item");
}

#[wasm_bindgen_test]
async fn disclosure_markup_has_no_menu_roles() {
    let root = fresh_root();
    yew::Renderer::<RichDropdownApp>::with_root(root).render();
    wait_for_render().await;

    assert!(
        document()
            .query_selector("[role='menu'], [role='menuitem']")
            .unwrap()
            .is_none(),
        "disclosure navigation must not use menu roles"
    );
    assert!(!toggle().has_attribute("aria-haspopup"));

    let disabled_item = document()
        .query_selector(".nav-dropdown-item.disabled")
        .unwrap()
        .expect("disabled item should render");
    assert_eq!(
        disabled_item.get_attribute("aria-disabled").as_deref(),
        Some("true")
    );
}
