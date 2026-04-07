//! # NavTabPanel
//!
//! Content panel associated with a [`NavTab`](super::NavTab).
//! Renders a `<div>` with `role="tabpanel"` and `aria-labelledby`.
//!
//! # Example
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::components::NavTabPanel;
//!
//! #[component]
//! fn Panels() -> Html {
//!     html! {
//!         <>
//!             <NavTabPanel id="panel-1" labelled_by="tab-1" hidden={false}>
//!                 <p>{ "Panel 1 content" }</p>
//!             </NavTabPanel>
//!             <NavTabPanel id="panel-2" labelled_by="tab-2" hidden={true}>
//!                 <p>{ "Panel 2 content" }</p>
//!             </NavTabPanel>
//!         </>
//!     }
//! }
//! ```
//!
//! # CSS Classes
//!
//! | Class | Condition |
//! |-------|-----------|
//! | `nav-tab-panel` | Always applied |
//!
//! # Props
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `hidden` | `bool` | — | Whether the panel is hidden (required) |
//! | `id` | `Option<&'static str>` | `None` | Panel element id |
//! | `labelled_by` | `Option<&'static str>` | `None` | aria-labelledby target |
//! | `classes` | `Classes` | — | Additional CSS classes |
//! | `children` | `Children` | — | Panel content |

use yew::prelude::*;

/// Properties for the [`NavTabPanel`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `hidden` | `bool` | — | Whether the panel is hidden (required) |
/// | `id` | `Option<&'static str>` | `None` | Panel element id |
/// | `labelled_by` | `Option<&'static str>` | `None` | aria-labelledby target |
/// | `classes` | `Classes` | — | Additional CSS classes |
/// | `children` | `Children` | — | Panel content |
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavTabPanelProps {
    /// Additional CSS classes applied to the panel.
    #[prop_or_default]
    pub classes: Classes,

    /// Optional `id` attribute for the panel element.
    #[prop_or_default]
    pub id: Option<&'static str>,

    /// Optional `aria-labelledby` referencing the tab button `id`.
    #[prop_or_default]
    pub labelled_by: Option<&'static str>,

    /// Whether the panel is hidden.
    pub hidden: bool,

    /// Content rendered inside the panel.
    pub children: Children
}

/// Tab panel component that holds the content for a single tab.
///
/// Renders a `<div>` with `role="tabpanel"`.
///
/// # CSS Classes
///
/// - `nav-tab-panel` - Always applied
#[function_component]
pub fn NavTabPanel(props: &NavTabPanelProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-tab-panel");

    html! {
        <div
            {classes}
            id={props.id}
            role="tabpanel"
            aria-labelledby={props.labelled_by}
            hidden={props.hidden}
        >
            { for props.children.iter() }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_tab_panel_hidden() {
        let props = NavTabPanelProps {
            classes:     Classes::default(),
            id:          Some("panel-1"),
            labelled_by: Some("tab-1"),
            hidden:      true,
            children:    Children::new(vec![])
        };

        assert!(props.hidden);
        assert_eq!(props.id, Some("panel-1"));
    }

    #[test]
    fn nav_tab_panel_visible() {
        let props = NavTabPanelProps {
            classes:     Classes::default(),
            id:          Some("panel-1"),
            labelled_by: Some("tab-1"),
            hidden:      false,
            children:    Children::new(vec![])
        };

        assert!(!props.hidden);
    }
}
