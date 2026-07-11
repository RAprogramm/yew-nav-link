// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Browser tests for `Pagination` interaction: every control (prev/next,
//! first/last jumps, numbered pages) emits the right page number through
//! `on_page_change`, the active page stays focusable with
//! `aria-current="page"`, and ellipsis gaps render as non-interactive text.

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_nav_link::components::{Pagination, PaginationProps};

use super::common::{document, fresh_root, wait_for_render};

wasm_bindgen_test_configure!(run_in_browser);

fn render_pagination(clicks: &Rc<RefCell<Vec<u32>>>) {
    let root = fresh_root();
    let on_page_change = {
        let clicks = Rc::clone(clicks);
        Callback::from(move |page: u32| clicks.borrow_mut().push(page))
    };
    let props = PaginationProps {
        classes:         Classes::default(),
        current_page:    5,
        total_pages:     10,
        siblings:        1,
        show_first_last: true,
        show_prev_next:  true,
        on_page_change:  Some(on_page_change)
    };
    yew::Renderer::<Pagination>::with_root_and_props(root, props).render();
}

fn button_by_label(label: &str) -> HtmlElement {
    document()
        .query_selector(&format!("button[aria-label='{label}']"))
        .unwrap()
        .unwrap_or_else(|| panic!("button {label} should render"))
        .dyn_into::<HtmlElement>()
        .unwrap()
}

fn number_buttons() -> Vec<HtmlElement> {
    let list = document()
        .query_selector_all(".pagination-item button:not([aria-label])")
        .unwrap();
    (0..list.length())
        .filter_map(|index| list.item(index))
        .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
        .collect()
}

#[wasm_bindgen_test]
async fn every_control_emits_its_page_number() {
    let clicks = Rc::new(RefCell::new(Vec::new()));
    render_pagination(&clicks);
    wait_for_render().await;

    button_by_label("Previous page").click();
    button_by_label("Next page").click();
    button_by_label("First page").click();
    button_by_label("Last page").click();

    let four = number_buttons()
        .into_iter()
        .find(|button| button.text_content().as_deref() == Some("4"))
        .expect("page 4 should be in the sibling window");
    four.click();

    wait_for_render().await;
    assert_eq!(*clicks.borrow(), vec![4, 6, 1, 10, 4]);
}

#[wasm_bindgen_test]
async fn first_and_last_pages_render_exactly_once() {
    let clicks = Rc::new(RefCell::new(Vec::new()));
    render_pagination(&clicks);
    wait_for_render().await;

    let texts: Vec<String> = number_buttons()
        .into_iter()
        .filter_map(|button| button.text_content())
        .collect();
    assert_eq!(
        texts.iter().filter(|text| text.as_str() == "1").count(),
        1,
        "first page must not be duplicated by the jump button"
    );
    assert_eq!(
        texts.iter().filter(|text| text.as_str() == "10").count(),
        1,
        "last page must not be duplicated by the jump button"
    );
}

#[wasm_bindgen_test]
async fn active_page_is_focusable_and_marked_current() {
    let clicks = Rc::new(RefCell::new(Vec::new()));
    render_pagination(&clicks);
    wait_for_render().await;

    let active = document()
        .query_selector("button[aria-current='page']")
        .unwrap()
        .expect("active page should carry aria-current")
        .dyn_into::<HtmlElement>()
        .unwrap();
    assert_eq!(active.text_content().as_deref(), Some("5"));
    assert!(!active.has_attribute("disabled"));

    active.click();
    wait_for_render().await;
    assert_eq!(*clicks.borrow(), vec![5]);
}

#[wasm_bindgen_test]
async fn ellipsis_renders_as_non_interactive_text() {
    let clicks = Rc::new(RefCell::new(Vec::new()));
    render_pagination(&clicks);
    wait_for_render().await;

    let gaps = document()
        .query_selector_all(".pagination-ellipsis")
        .unwrap();
    assert_eq!(gaps.length(), 2, "window 4..6 of 10 pages has two gaps");

    let buttons_inside = document()
        .query_selector(".pagination-ellipsis button")
        .unwrap();
    assert!(buttons_inside.is_none(), "ellipsis must not be a button");
}
