//! Navigation item component.
//!
//! Wraps navigation links in a semantic `<li>` element.
//!
//! # Example
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::{NavList, NavItem, NavLink};
//! use yew_router::prelude::*;
//!
//! #[derive(Clone, PartialEq, Routable)]
//! enum Route {
//!     #[at("/")]
//!     Home,
//! }
//!
//! #[component]
//! fn Navigation() -> Html {
//!     html! {
//!         <NavList>
//!             <NavItem>
//!                 <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
//!             </NavItem>
//!         </NavList>
//!     }
//! }
//! ```
//!
//! # CSS Classes
//!
//! | Class | Description |
//! |-------|-------------|
//! | `nav-item` | Base class for the list item |

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavItemProps {
    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub children: Children,
}

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
            classes: Classes::default(),
            disabled: false,
            children: Children::new(vec![]),
        };

        assert!(props.classes.is_empty());
        assert!(!props.disabled);
    }

    #[test]
    fn nav_item_props_disabled() {
        let props = NavItemProps {
            classes: Classes::default(),
            disabled: true,
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }

    #[test]
    fn nav_item_props_with_class() {
        let props = NavItemProps {
            classes: Classes::from("my-item"),
            disabled: false,
            children: Children::new(vec![]),
        };

        assert!(props.classes.contains("my-item"));
    }

    #[test]
    fn nav_item_props_clone() {
        let props1 = NavItemProps {
            classes: Classes::from("test"),
            disabled: true,
            children: Children::new(vec![]),
        };

        let props2 = props1.clone();
        assert_eq!(props1.disabled, props2.disabled);
        assert_eq!(props1.classes, props2.classes);
    }

    #[test]
    fn nav_item_disabled_state() {
        let enabled = NavItemProps {
            classes: Classes::default(),
            disabled: false,
            children: Children::new(vec![]),
        };

        let disabled = NavItemProps {
            classes: Classes::default(),
            disabled: true,
            children: Children::new(vec![]),
        };

        assert_ne!(enabled, disabled);
    }
}
