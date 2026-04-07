//! # NavItem
//!
//! Semantic list item wrapper for navigation links.
//! Renders a `<li>` with `role="listitem"` and optional disabled state.
//!
//! # Example
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::{NavItem, NavLink, NavList};
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Routable)]
//! # enum Route {
//! #     #[at("/")]
//! #     Home,
//! # }
//! #[component]
//! fn Nav() -> Html {
//!     html! {
//!         <NavList>
//!             <NavItem>
//!                 <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
//!             </NavItem>
//!             <NavItem disabled=true>
//!                 { "Settings (disabled)" }
//!             </NavItem>
//!         </NavList>
//!     }
//! }
//! ```
//!
//! # CSS Classes
//!
//! | Class | Condition |
//! |-------|-----------|
//! | `nav-item` | Always applied |
//! | `disabled` | Applied when `disabled` is `true` |
//!
//! # Props
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `disabled` | `bool` | `false` | Disabled state |
//! | `classes` | `Classes` | — | Additional CSS classes |
//! | `children` | `Children` | — | Item content |

use yew::prelude::*;

/// Properties for the [`NavItem`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `disabled` | `bool` | `false` | Disabled state |
/// | `classes` | `Classes` | — | Additional CSS classes |
/// | `children` | `Children` | — | Item content |
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavItemProps {
    /// Additional CSS classes applied to the item.
    #[prop_or_default]
    pub classes: Classes,

    /// Whether the navigation item is disabled.
    #[prop_or_default]
    pub disabled: bool,

    /// Content rendered inside the item, typically a
    /// [`NavLink`](crate::NavLink).
    #[prop_or_default]
    pub children: Children
}

/// Semantic list item wrapper for navigation links.
///
/// Renders a `<li>` element with `role="listitem"`.
///
/// # CSS Classes
///
/// - `nav-item` - Always applied
/// - `disabled` - Applied when `disabled` is `true`
#[function_component]
pub fn NavItem(props: &NavItemProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-item");

    if props.disabled {
        classes.push("disabled");
        html! {
            <li {classes} role="listitem" aria-disabled="true">
                { for props.children.iter() }
            </li>
        }
    } else {
        html! {
            <li {classes} role="listitem">
                { for props.children.iter() }
            </li>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_item_props_default() {
        let props = NavItemProps {
            classes:  Classes::default(),
            disabled: false,
            children: Children::new(vec![])
        };

        assert!(props.classes.is_empty());
        assert!(!props.disabled);
    }

    #[test]
    fn nav_item_props_disabled() {
        let props = NavItemProps {
            classes:  Classes::default(),
            disabled: true,
            children: Children::new(vec![])
        };

        assert!(props.disabled);
    }

    #[test]
    fn nav_item_props_with_class() {
        let props = NavItemProps {
            classes:  Classes::from("my-item"),
            disabled: false,
            children: Children::new(vec![])
        };

        assert!(props.classes.contains("my-item"));
    }

    #[test]
    fn nav_item_props_clone() {
        let props1 = NavItemProps {
            classes:  Classes::from("test"),
            disabled: true,
            children: Children::new(vec![])
        };

        let props2 = props1.clone();
        assert_eq!(props1.disabled, props2.disabled);
        assert_eq!(props1.classes, props2.classes);
    }

    #[test]
    fn nav_item_disabled_state() {
        let enabled = NavItemProps {
            classes:  Classes::default(),
            disabled: false,
            children: Children::new(vec![])
        };

        let disabled = NavItemProps {
            classes:  Classes::default(),
            disabled: true,
            children: Children::new(vec![])
        };

        assert_ne!(enabled, disabled);
    }
}
