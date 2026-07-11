// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! # `NavDropdown`
//!
//! Collapsible dropdown menu for grouping related navigation items.
//! Renders a `<li>` with a toggle button and a nested `<ul>` menu.
//!
//! # Example
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::{
//!     NavItem, NavLink, NavList,
//!     components::{NavDropdown, NavDropdownDivider, NavDropdownItem}
//! };
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Routable)]
//! # enum Route {
//! #     #[at("/")]
//! #     Home,
//! #     #[at("/settings")]
//! #     Settings,
//! # }
//! #[component]
//! fn Nav() -> Html {
//!     html! {
//!         <NavList>
//!             <NavItem>
//!                 <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
//!             </NavItem>
//!             <NavDropdown toggle_text="Settings">
//!                 <NavDropdownItem>
//!                     <NavLink<Route> to={Route::Settings}>{ "Profile" }</NavLink<Route>>
//!                 </NavDropdownItem>
//!                 <NavDropdownDivider />
//!                 <NavDropdownItem disabled=true>
//!                     { "Admin" }
//!                 </NavDropdownItem>
//!             </NavDropdown>
//!         </NavList>
//!     }
//! }
//! ```
//!
//! # CSS Classes
//!
//! | Class | Condition |
//! |-------|-----------|
//! | `nav-dropdown` | Always on container `<li>` |
//! | `nav-dropdown-toggle` | Toggle button |
//! | `nav-dropdown-menu` | Inner `<ul>` |
//! | `nav-dropdown-caret` | Caret indicator |
//! | `nav-dropdown-item` | Menu items |
//! | `nav-dropdown-divider` | Separator |
//! | `disabled` | Applied to disabled items |
//!
//! # Props
//!
//! **`NavDropdown`:**
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `toggle_text` | `&'static str` | `"dropdown"` | Toggle button label |
//! | `id` | `Option<&'static str>` | `None` | Menu `<ul>` id, also wired to the toggle's `aria-controls` |
//! | `classes` | `Classes` | — | Additional CSS classes |
//! | `children` | `Children` | — | Menu content |
//!
//! **`NavDropdownItem`:**
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `disabled` | `bool` | `false` | Disable the item |
//! | `classes` | `Classes` | — | Additional CSS classes |
//! | `children` | `Children` | — | Item content |
//!
//! **`NavDropdownDivider`:**
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `classes` | `Classes` | — | Additional CSS classes |

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, Node};
use yew::prelude::*;

use crate::utils::{KeyboardNavConfig, handle_arrow_key, handle_home_end};

/// Collects the focusable elements (links and enabled buttons) inside the
/// dropdown menu, in DOM order.
///
/// Elements nested in a disabled [`NavDropdownItem`] are excluded, so
/// keyboard navigation never lands on a link the item's disabled state is
/// supposed to neutralize.
fn menu_items(menu_ref: &NodeRef) -> Vec<HtmlElement> {
    menu_ref
        .cast::<Element>()
        .and_then(|menu| {
            menu.query_selector_all("a[href], button:not([disabled])")
                .ok()
        })
        .map(|list| {
            (0..list.length())
                .filter_map(|index| list.item(index))
                .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
                .filter(|element| {
                    element
                        .closest(".nav-dropdown-item.disabled")
                        .ok()
                        .flatten()
                        .is_none()
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Properties for the [`NavDropdown`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `toggle_text` | `&'static str` | `"dropdown"` | Toggle button label |
/// | `id` | `Option<&'static str>` | `None` | Menu `<ul>` id, also wired to the toggle's `aria-controls` |
/// | `classes` | `Classes` | — | Additional CSS classes |
/// | `children` | `Children` | — | Menu content |
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct NavDropdownProps {
    /// Additional CSS classes applied to the dropdown container.
    #[prop_or_default]
    pub classes: Classes,

    /// Text displayed on the dropdown toggle button.
    #[prop_or("dropdown")]
    pub toggle_text: &'static str,

    /// Optional `id` for the menu `<ul>`; when set, the toggle references it
    /// via `aria-controls`.
    #[prop_or_default]
    pub id: Option<&'static str>,

    /// Content rendered inside the dropdown menu.
    #[prop_or_default]
    pub children: Children
}

/// Collapsible dropdown menu for grouping navigation links.
///
/// # Keyboard & accessibility
///
/// Implements the WAI-ARIA disclosure-navigation pattern: the toggle carries
/// `aria-expanded` (plus `aria-controls` when `id` is set) and the menu stays
/// a plain list of links in the normal tab order — no `menu`/`menuitem`
/// roles, which the APG reserves for application menus rather than site
/// navigation.
///
/// Arrow-key support is layered on top as the optional enhancement the APG
/// describes: opening the menu focuses the first item (`ArrowUp` on a closed
/// toggle opens it and focuses the last), `ArrowDown`/`ArrowUp` (wrapping)
/// and `Home`/`End` move focus over the menu's links, `Escape` closes the
/// menu and returns focus to the toggle, and moving focus out of the
/// dropdown (tabbing away or clicking elsewhere) dismisses it.
///
/// # CSS Classes
///
/// - `nav-dropdown` - Container `<li>` element
/// - `nav-dropdown-toggle` - Toggle button
/// - `nav-dropdown-menu` - Inner `<ul>` menu
/// - `nav-dropdown-caret` - Caret indicator
#[function_component]
pub fn NavDropdown(props: &NavDropdownProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-dropdown");

    let open = use_state(|| false);
    let open_focus_last = use_mut_ref(|| false);
    let container_ref = use_node_ref();
    let toggle_ref = use_node_ref();
    let menu_ref = use_node_ref();

    let on_toggle = {
        let open = open.clone();
        let open_focus_last = open_focus_last.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            *open_focus_last.borrow_mut() = false;
            open.set(!*open);
        })
    };

    let on_keydown = {
        let open = open.clone();
        let open_focus_last = open_focus_last.clone();
        let toggle_ref = toggle_ref.clone();
        let menu_ref = menu_ref.clone();
        Callback::from(move |event: KeyboardEvent| {
            let key = event.key();

            if key == "Escape" && *open {
                event.prevent_default();
                open.set(false);
                if let Some(toggle) = toggle_ref.cast::<HtmlElement>() {
                    let _ = toggle.focus();
                }
                return;
            }

            if !matches!(key.as_str(), "ArrowDown" | "ArrowUp" | "Home" | "End") {
                return;
            }

            let items = menu_items(&menu_ref);
            if items.is_empty() {
                return;
            }

            event.prevent_default();
            if !*open {
                *open_focus_last.borrow_mut() = key == "ArrowUp";
                open.set(true);
                return;
            }

            let active = web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| document.active_element())
                .map(JsCast::unchecked_into::<Node>);
            let position = active
                .as_ref()
                .and_then(|node| items.iter().position(|item| item.is_same_node(Some(node))));

            if let Some(item) =
                next_focus_index(&key, position, items.len()).and_then(|index| items.get(index))
            {
                let _ = item.focus();
            }
        })
    };

    let on_focusout = {
        let open = open.clone();
        let container_ref = container_ref.clone();
        Callback::from(move |event: FocusEvent| {
            if !*open {
                return;
            }
            let stays_inside = match (container_ref.cast::<Node>(), event.related_target()) {
                (Some(container), Some(related)) => related
                    .dyn_into::<Node>()
                    .is_ok_and(|node| container.contains(Some(&node))),
                _ => false
            };
            if !stays_inside {
                open.set(false);
            }
        })
    };

    {
        let menu_ref = menu_ref.clone();
        use_effect_with(*open, move |is_open| {
            if *is_open {
                let items = menu_items(&menu_ref);
                let focus_last = std::mem::take(&mut *open_focus_last.borrow_mut());
                let target = if focus_last {
                    items.last()
                } else {
                    items.first()
                };
                if let Some(item) = target {
                    let _ = item.focus();
                }
            }
            || ()
        });
    }

    let menu_class = if *open {
        "nav-dropdown-menu open"
    } else {
        "nav-dropdown-menu"
    };

    html! {
        <li
            ref={container_ref}
            class={classes}
            onkeydown={on_keydown}
            onfocusout={on_focusout}
        >
            <button
                ref={toggle_ref}
                type="button"
                class="nav-dropdown-toggle"
                aria-expanded={if *open { "true" } else { "false" }}
                aria-controls={props.id}
                onclick={on_toggle}
            >
                { props.toggle_text }
                <span class="nav-dropdown-caret" aria-hidden="true">{" ▼"}</span>
            </button>
            <ul ref={menu_ref} id={props.id} class={menu_class}>
                { for props.children.iter() }
            </ul>
        </li>
    }
}

/// Computes the next focus index for the optional arrow-key enhancement.
///
/// When focus is not currently on a menu item (`position` is `None`),
/// `ArrowDown`/`Home` land on the first item and `ArrowUp`/`End` on the last,
/// instead of skipping relative to a phantom index.
fn next_focus_index(key: &str, position: Option<usize>, total: usize) -> Option<usize> {
    let config = KeyboardNavConfig {
        wrap:     true,
        vertical: true
    };
    match (key, position) {
        ("Home" | "End", _) => handle_home_end(key, position.unwrap_or(0), total),
        ("ArrowDown", None) => Some(0),
        ("ArrowUp", None) => total.checked_sub(1),
        (_, Some(current)) => handle_arrow_key(key, current, total, &config),
        _ => None
    }
}

/// Properties for the [`NavDropdownItem`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `disabled` | `bool` | `false` | Disable the item |
/// | `classes` | `Classes` | — | Additional CSS classes |
/// | `children` | `Children` | — | Item content |
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct NavDropdownItemProps {
    /// Additional CSS classes applied to the item.
    #[prop_or_default]
    pub classes: Classes,

    /// Whether the dropdown item is disabled.
    #[prop_or_default]
    pub disabled: bool,

    /// Content rendered inside the item.
    pub children: Children
}

/// A single item within a [`NavDropdown`] menu.
///
/// Renders a plain `<li>`; a disabled item additionally carries
/// `aria-disabled="true"` and its links are skipped by the dropdown's
/// keyboard navigation.
///
/// # CSS Classes
///
/// - `nav-dropdown-item` - Always applied
/// - `disabled` - Applied when `disabled` is `true`
#[function_component]
pub fn NavDropdownItem(props: &NavDropdownItemProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-dropdown-item");

    if props.disabled {
        classes.push("disabled");
    }

    let aria_disabled = props.disabled.then_some("true");

    html! {
        <li class={classes} aria-disabled={aria_disabled}>
            { for props.children.iter() }
        </li>
    }
}

/// Properties for the [`NavDropdownDivider`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `classes` | `Classes` | — | Additional CSS classes |
#[derive(Properties, Clone, PartialEq, Eq, Debug, Default)]
pub struct NavDropdownDividerProps {
    /// Additional CSS classes applied to the divider.
    #[prop_or_default]
    pub classes: Classes
}

/// Visual separator between items in a [`NavDropdown`] menu.
///
/// Renders a `<li>` element with `role="separator"`.
#[function_component]
pub fn NavDropdownDivider(props: &NavDropdownDividerProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-dropdown-divider");

    html! {
        <li class={classes} role="separator" />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_dropdown_props_default() {
        let props = NavDropdownProps {
            classes:     Classes::default(),
            toggle_text: "Menu",
            id:          None,
            children:    Children::new(vec![])
        };

        assert_eq!(props.toggle_text, "Menu");
        assert!(props.id.is_none());
    }

    #[test]
    fn nav_dropdown_item_default() {
        let props = NavDropdownItemProps {
            classes:  Classes::default(),
            disabled: false,
            children: Children::new(vec![])
        };

        assert!(!props.disabled);
    }

    #[test]
    fn nav_dropdown_item_disabled() {
        let props = NavDropdownItemProps {
            classes:  Classes::default(),
            disabled: true,
            children: Children::new(vec![])
        };

        assert!(props.disabled);
    }

    #[test]
    fn nav_dropdown_divider_props() {
        let props = NavDropdownDividerProps {
            classes: Classes::default()
        };

        assert!(props.classes.is_empty());
    }

    #[test]
    fn nav_dropdown_with_custom_id() {
        let props = NavDropdownProps {
            classes:     Classes::default(),
            toggle_text: "Menu",
            id:          Some("my-dropdown"),
            children:    Children::new(vec![])
        };

        assert_eq!(props.id, Some("my-dropdown"));
    }

    #[test]
    fn nav_dropdown_item_with_classes() {
        let mut classes = Classes::new();
        classes.push("custom-item");
        let props = NavDropdownItemProps {
            classes,
            disabled: false,
            children: Children::new(vec![])
        };

        assert!(props.classes.contains("custom-item"));
    }

    #[test]
    fn nav_dropdown_disabled_item() {
        let props = NavDropdownItemProps {
            classes:  Classes::default(),
            disabled: true,
            children: Children::new(vec![])
        };

        assert!(props.disabled);
    }

    #[test]
    fn next_focus_index_arrow_down_without_position_lands_on_first() {
        assert_eq!(next_focus_index("ArrowDown", None, 3), Some(0));
    }

    #[test]
    fn next_focus_index_arrow_up_without_position_lands_on_last() {
        assert_eq!(next_focus_index("ArrowUp", None, 3), Some(2));
    }

    #[test]
    fn next_focus_index_arrow_keys_move_relative_to_position() {
        assert_eq!(next_focus_index("ArrowDown", Some(0), 3), Some(1));
        assert_eq!(next_focus_index("ArrowUp", Some(0), 3), Some(2));
        assert_eq!(next_focus_index("ArrowDown", Some(2), 3), Some(0));
    }

    #[test]
    fn next_focus_index_home_end_ignore_position() {
        assert_eq!(next_focus_index("Home", Some(2), 3), Some(0));
        assert_eq!(next_focus_index("End", None, 3), Some(2));
    }

    #[test]
    fn next_focus_index_other_keys_do_nothing() {
        assert_eq!(next_focus_index("Enter", None, 3), None);
        assert_eq!(next_focus_index("Tab", Some(1), 3), None);
    }

    #[test]
    fn nav_dropdown_with_children() {
        let children = Children::new(vec![html! { <div>{ "child" }</div> }]);
        let props = NavDropdownProps {
            classes: Classes::default(),
            toggle_text: "Test",
            id: None,
            children
        };

        assert_eq!(props.children.len(), 1);
    }
}
