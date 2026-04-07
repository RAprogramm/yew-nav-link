//! # NavTab
//!
//! A single tab button within a [`NavTabs`](super::NavTabs) container.
//! Renders a `<li>` with a `<button>` that has proper ARIA attributes
//! (`role="tab"`, `aria-selected`, `aria-controls`).
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
//!         <NavTabs id="my-tabs">
//!             <NavTab active=true id="tab-1" panel_id="panel-1" onclick={None}>
//!                 { "Overview" }
//!             </NavTab>
//!             <NavTab active=false id="tab-2" panel_id="panel-2" onclick={None}>
//!                 { "Details" }
//!             </NavTab>
//!             <NavTab active=false disabled=true onclick={None}>
//!                 { "Disabled" }
//!             </NavTab>
//!         </NavTabs>
//!     }
//! }
//! ```
//!
//! # CSS Classes
//!
//! | Class | Condition |
//! |-------|-----------|
//! | `nav-tab` | Always applied |
//! | `active` | Applied when `active` is `true` |
//! | `disabled` | Applied when `disabled` is `true` |
//!
//! # Props
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `active` | `bool` | — | Whether this tab is selected (required) |
//! | `disabled` | `bool` | `false` | Whether this tab is disabled |
//! | `id` | `Option<&'static str>` | `None` | Tab button id |
//! | `panel_id` | `Option<&'static str>` | `None` | aria-controls target |
//! | `onclick` | `Option<Callback<MouseEvent>>` | — | Click handler (required) |
//! | `classes` | `Classes` | — | Additional CSS classes |
//! | `children` | `Children` | — | Tab content |

use yew::prelude::*;

/// Properties for the [`NavTab`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `active` | `bool` | — | Whether this tab is selected (required) |
/// | `disabled` | `bool` | `false` | Whether this tab is disabled |
/// | `id` | `Option<&'static str>` | `None` | Tab button id |
/// | `panel_id` | `Option<&'static str>` | `None` | aria-controls target |
/// | `onclick` | `Option<Callback<MouseEvent>>` | — | Click handler (required) |
/// | `classes` | `Classes` | — | Additional CSS classes |
/// | `children` | `Children` | — | Tab content |
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavTabProps {
    /// Additional CSS classes applied to the tab.
    #[prop_or_default]
    pub classes: Classes,

    /// Whether this tab is currently selected.
    pub active: bool,

    /// Whether this tab is disabled.
    #[prop_or_default]
    pub disabled: bool,

    /// Optional `id` attribute for the tab button.
    #[prop_or_default]
    pub id: Option<&'static str>,

    /// Optional `aria-controls` referencing the associated panel `id`.
    #[prop_or_default]
    pub panel_id: Option<&'static str>,

    /// Content rendered inside the tab button.
    #[prop_or_default]
    pub children: Children,

    /// Click handler invoked when the tab is selected.
    pub onclick: Option<Callback<MouseEvent>>
}

/// A single tab button within a [`NavTabs`](super::NavTabs) container.
///
/// # CSS Classes
///
/// - `nav-tab` - Always applied
/// - `active` - Applied when `active` is `true`
/// - `disabled` - Applied when `disabled` is `true`
#[function_component]
pub fn NavTab(props: &NavTabProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-tab");

    if props.active {
        classes.push("active");
    }

    if props.disabled {
        classes.push("disabled");
    }

    let onclick = props.onclick.clone();
    let onclick = onclick.map(|cb| {
        move |e: MouseEvent| {
            e.prevent_default();
            cb.emit(e);
        }
    });

    html! {
        <li {classes} role="presentation">
            <button
                type="button"
                role="tab"
                id={props.id}
                aria-selected={props.active.to_string()}
                aria-controls={props.panel_id}
                disabled={props.disabled}
                onclick={onclick}
            >
                { for props.children.iter() }
            </button>
        </li>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_tab_active() {
        let props = NavTabProps {
            classes:  Classes::default(),
            active:   true,
            disabled: false,
            id:       None,
            panel_id: None,
            children: Children::new(vec![]),
            onclick:  None
        };

        assert!(props.active);
        assert!(!props.disabled);
    }

    #[test]
    fn nav_tab_disabled() {
        let props = NavTabProps {
            classes:  Classes::default(),
            active:   false,
            disabled: true,
            id:       None,
            panel_id: None,
            children: Children::new(vec![]),
            onclick:  None
        };

        assert!(props.disabled);
        assert!(!props.active);
    }
}
