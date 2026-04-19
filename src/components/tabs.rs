//! # `NavTabs`
//!
//! Tab navigation container that wraps [`NavTab`](super::NavTab) items.
//! Renders a `<ul>` with `role="tablist"` and optional full-width layout.
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
//!             <NavTab active=true onclick={None}>{ "Tab 1" }</NavTab>
//!             <NavTab active=false onclick={None}>{ "Tab 2" }</NavTab>
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
//! | `role` | `&'static str` | `"tablist"` | ARIA role |
//! | `id` | `Option<&'static str>` | `None` | Container id |
//! | `classes` | `Classes` | — | Additional CSS classes |
//! | `children` | `Children` | — | Tab items |

use yew::prelude::*;

/// Properties for the [`NavTabs`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `full_width` | `bool` | `false` | Stretch tabs to fill width |
/// | `role` | `&'static str` | `"tablist"` | ARIA role |
/// | `id` | `Option<&'static str>` | `None` | Container id |
/// | `classes` | `Classes` | — | Additional CSS classes |
/// | `children` | `Children` | — | Tab items |
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavTabsProps {
    /// Additional CSS classes applied to the tabs container.
    #[prop_or_default]
    pub classes: Classes,

    /// ARIA role for the tab list. Defaults to `"tablist"`.
    #[prop_or("tablist")]
    pub role: &'static str,

    /// Optional `id` attribute for the tabs container.
    #[prop_or_default]
    pub id: Option<&'static str>,

    /// Whether tabs should stretch to fill the full width of the container.
    #[prop_or_default]
    pub full_width: bool,

    /// Tab items rendered inside the container.
    pub children: Children
}

/// Tab navigation container that wraps [`NavTab`](super::NavTab) items.
///
/// Renders a `<ul>` element with ARIA `role="tablist"`.
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

    html! {
        <ul
            {classes}
            id={props.id}
            role={props.role}
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
            role:       "tablist",
            id:         None,
            full_width: false,
            children:   Children::new(vec![])
        };

        assert_eq!(props.role, "tablist");
        assert!(!props.full_width);
    }

    #[test]
    fn nav_tabs_full_width() {
        let props = NavTabsProps {
            classes:    Classes::default(),
            role:       "tablist",
            id:         Some("main-tabs"),
            full_width: true,
            children:   Children::new(vec![])
        };

        assert!(props.full_width);
        assert_eq!(props.id, Some("main-tabs"));
    }
}
