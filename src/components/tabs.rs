// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! # `NavTabs`
//!
//! Tab navigation container that wraps [`NavTab`](super::NavTab) items.
//! Renders a `<ul>` with `role="tablist"`, optional full-width layout, and
//! WAI-ARIA tabs-pattern keyboard support: arrow keys move focus between
//! enabled tabs with wrap-around, `Home`/`End` jump to the first/last tab,
//! and the active tab is the only one in the tab sequence (roving
//! tabindex on [`NavTab`](super::NavTab)).
//!
//! # Example
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::components::{NavTab, NavTabs};
//!
//! #[component]
//! fn TabBar() -> Html {
//!     html! {
//!         <NavTabs id="main-tabs">
//!             <NavTab active=true>{ "Tab 1" }</NavTab>
//!             <NavTab active=false>{ "Tab 2" }</NavTab>
//!         </NavTabs>
//!     }
//! }
//! ```
//!
//! # CSS Classes
//!
//! | Class | Condition |
//! |-------|-----------|
//! | `nav-tabs` | Always applied |
//! | `nav-tabs-fill` | Applied when `full_width` is `true` |
//!
//! # Props
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `full_width` | `bool` | `false` | Stretch tabs to fill width |
//! | `vertical` | `bool` | `false` | Vertical tablist (`aria-orientation` + up/down arrows) |
//! | `role` | `AttrValue` | `"tablist"` | ARIA role |
//! | `id` | `Option<AttrValue>` | `None` | Container id |
//! | `classes` | `Classes` | — | Additional CSS classes |
//! | `children` | `Children` | — | Tab items |

use web_sys::KeyboardEvent;
use yew::prelude::*;

use super::focus::{focusable_elements, focused_position, next_focus_index};

/// Properties for the [`NavTabs`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `full_width` | `bool` | `false` | Stretch tabs to fill width |
/// | `vertical` | `bool` | `false` | Vertical tablist (`aria-orientation` + up/down arrows) |
/// | `role` | `AttrValue` | `"tablist"` | ARIA role |
/// | `id` | `Option<AttrValue>` | `None` | Container id |
/// | `classes` | `Classes` | — | Additional CSS classes |
/// | `children` | `Children` | — | Tab items |
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavTabsProps {
    /// Additional CSS classes applied to the tabs container.
    #[prop_or_default]
    pub classes: Classes,

    /// ARIA role for the tab list. Defaults to `"tablist"`.
    #[prop_or(AttrValue::Static("tablist"))]
    pub role: AttrValue,

    /// Optional `id` attribute for the tabs container.
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Whether tabs should stretch to fill the full width of the container.
    #[prop_or_default]
    pub full_width: bool,

    /// Whether the tablist is vertical: emits `aria-orientation="vertical"`
    /// and switches keyboard navigation to the up/down arrows.
    #[prop_or_default]
    pub vertical: bool,

    /// Tab items rendered inside the container.
    pub children: Children
}

/// Tab navigation container that wraps [`NavTab`](super::NavTab) items.
///
/// Renders a `<ul>` element with ARIA `role="tablist"` and implements the
/// keyboard interaction of the WAI-ARIA tabs pattern: `ArrowRight` /
/// `ArrowLeft` (or `ArrowDown`/`ArrowUp` when `vertical`) move focus over
/// the enabled tabs with wrap-around, and `Home`/`End` jump to the first
/// and last tab. Activation stays with the consumer via each tab's
/// `onclick` (manual activation model).
///
/// # CSS Classes
///
/// - `nav-tabs` - Always applied
/// - `nav-tabs-fill` - Applied when `full_width` is `true`
#[function_component]
pub fn NavTabs(props: &NavTabsProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-tabs");

    if props.full_width {
        classes.push("nav-tabs-fill");
    }

    let list_ref = use_node_ref();
    let vertical = props.vertical;

    let onkeydown = {
        let list_ref = list_ref.clone();
        Callback::from(move |event: KeyboardEvent| {
            let key = event.key();
            let tabs = focusable_elements(&list_ref, "[role='tab']:not([disabled])");
            if tabs.is_empty() {
                return;
            }
            let position = focused_position(&tabs);
            if let Some(tab) =
                next_focus_index(&key, position, tabs.len(), vertical).and_then(|i| tabs.get(i))
            {
                event.prevent_default();
                let _ = tab.focus();
            }
        })
    };

    let aria_orientation = props.vertical.then_some("vertical");

    html! {
        <ul
            ref={list_ref}
            class={classes}
            id={props.id.clone()}
            role={props.role.clone()}
            aria-orientation={aria_orientation}
            onkeydown={onkeydown}
        >
            { for props.children.iter() }
        </ul>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_tabs_props_default() {
        let props = NavTabsProps {
            classes:    Classes::default(),
            role:       AttrValue::Static("tablist"),
            id:         None,
            full_width: false,
            vertical:   false,
            children:   Children::new(vec![])
        };

        assert_eq!(props.role, "tablist");
        assert!(!props.full_width);
    }

    #[test]
    fn nav_tabs_full_width() {
        let props = NavTabsProps {
            classes:    Classes::default(),
            role:       AttrValue::Static("tablist"),
            id:         Some(AttrValue::Static("main-tabs")),
            full_width: true,
            vertical:   false,
            children:   Children::new(vec![])
        };

        assert!(props.full_width);
        assert_eq!(props.id.as_deref(), Some("main-tabs"));
    }
}
