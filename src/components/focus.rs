// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Shared focus-management helpers for keyboard-navigable components
//! ([`NavDropdown`](super::NavDropdown), [`NavTabs`](super::NavTabs)).

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, Node};
use yew::prelude::*;

use crate::utils::{KeyboardNavConfig, handle_arrow_key, handle_home_end};

/// Collects the elements under `root` matching `selector`, in DOM order.
pub(super) fn focusable_elements(root: &NodeRef, selector: &str) -> Vec<HtmlElement> {
    root.cast::<Element>()
        .and_then(|element| element.query_selector_all(selector).ok())
        .map(|list| {
            (0..list.length())
                .filter_map(|index| list.item(index))
                .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
                .collect()
        })
        .unwrap_or_default()
}

/// Returns the index of the currently focused element within `items`.
pub(super) fn focused_position(items: &[HtmlElement]) -> Option<usize> {
    let active = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.active_element())
        .map(JsCast::unchecked_into::<Node>)?;
    items
        .iter()
        .position(|item| item.is_same_node(Some(&active)))
}

/// Computes the next focus index for arrow-key/Home/End navigation with
/// wrapping enabled.
///
/// When focus is not currently on one of the items (`position` is `None`),
/// the forward arrow and `Home` land on the first item and the backward
/// arrow and `End` on the last, instead of skipping relative to a phantom
/// index.
pub(super) fn next_focus_index(
    key: &str,
    position: Option<usize>,
    total: usize,
    vertical: bool
) -> Option<usize> {
    let config = KeyboardNavConfig {
        wrap: true,
        vertical
    };
    let forward = if vertical { "ArrowDown" } else { "ArrowRight" };
    let backward = if vertical { "ArrowUp" } else { "ArrowLeft" };
    match (key, position) {
        ("Home" | "End", _) => handle_home_end(key, position.unwrap_or(0), total),
        (k, None) if k == forward => Some(0),
        (k, None) if k == backward => total.checked_sub(1),
        (_, Some(current)) => handle_arrow_key(key, current, total, &config),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_focus_index_forward_without_position_lands_on_first() {
        assert_eq!(next_focus_index("ArrowDown", None, 3, true), Some(0));
        assert_eq!(next_focus_index("ArrowRight", None, 3, false), Some(0));
    }

    #[test]
    fn next_focus_index_backward_without_position_lands_on_last() {
        assert_eq!(next_focus_index("ArrowUp", None, 3, true), Some(2));
        assert_eq!(next_focus_index("ArrowLeft", None, 3, false), Some(2));
    }

    #[test]
    fn next_focus_index_arrow_keys_move_relative_to_position() {
        assert_eq!(next_focus_index("ArrowDown", Some(0), 3, true), Some(1));
        assert_eq!(next_focus_index("ArrowUp", Some(0), 3, true), Some(2));
        assert_eq!(next_focus_index("ArrowRight", Some(2), 3, false), Some(0));
        assert_eq!(next_focus_index("ArrowLeft", Some(1), 3, false), Some(0));
    }

    #[test]
    fn next_focus_index_home_end_ignore_position() {
        assert_eq!(next_focus_index("Home", Some(2), 3, false), Some(0));
        assert_eq!(next_focus_index("End", None, 3, true), Some(2));
    }

    #[test]
    fn next_focus_index_ignores_cross_orientation_arrows() {
        assert_eq!(next_focus_index("ArrowRight", Some(1), 3, true), None);
        assert_eq!(next_focus_index("ArrowDown", Some(1), 3, false), None);
        assert_eq!(next_focus_index("ArrowRight", None, 3, true), None);
    }

    #[test]
    fn next_focus_index_other_keys_do_nothing() {
        assert_eq!(next_focus_index("Enter", None, 3, false), None);
        assert_eq!(next_focus_index("Tab", Some(1), 3, false), None);
    }
}
