//! Navigation list container component.
//!
//! Provides a semantic `<ul>` wrapper for navigation links with proper
//! ARIA attributes and class propagation.
//!
//! # Example
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::{NavList, NavLink};
//! use yew_router::prelude::*;
//!
//! #[derive(Clone, PartialEq, Routable)]
//! enum Route {
//!     #[at("/")]
//!     Home,
//!     #[at("/about")]
//!     About,
//! }
//!
//! #[component]
//! fn Navigation() -> Html {
//!     html! {
//!         <NavList>
//!             <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
//!             <NavLink<Route> to={Route::About}>{ "About" }</NavLink<Route>>
//!         </NavList>
//!     }
//! }
//! ```
//!
//! # CSS Classes
//!
//! | Class | Description |
//! |-------|-------------|
//! | `nav-list` | Base class for the list container |

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavListProps {
    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub id: Option<&'static str>,

    #[prop_or_default]
    pub aria_label: Option<&'static str>,

    pub children: Children,
}

#[function_component]
pub fn NavList(props: &NavListProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-list");

    let aria_label = props.aria_label.unwrap_or("navigation");

    html! {
        <ul
            id={props.id}
            class={classes}
            role="list"
            aria-label={aria_label}
        >
            { for props.children.iter() }
        </ul>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_list_props_default() {
        let props = NavListProps {
            classes: Classes::default(),
            id: None,
            aria_label: None,
            children: Children::new(vec![]),
        };

        assert!(props.classes.is_empty());
        assert!(props.id.is_none());
        assert!(props.aria_label.is_none());
    }

    #[test]
    fn nav_list_props_with_values() {
        let props = NavListProps {
            classes: Classes::from("custom-class"),
            id: Some("nav-id"),
            aria_label: Some("navigation"),
            children: Children::new(vec![]),
        };

        assert!(props.classes.contains("custom-class"));
        assert_eq!(props.id, Some("nav-id"));
        assert_eq!(props.aria_label, Some("navigation"));
    }

    #[test]
    fn nav_list_props_clone() {
        let props1 = NavListProps {
            classes: Classes::from("test"),
            id: Some("id"),
            aria_label: Some("label"),
            children: Children::new(vec![]),
        };

        let props2 = props1.clone();
        assert_eq!(props1.classes, props2.classes);
        assert_eq!(props1.id, props2.id);
        assert_eq!(props1.aria_label, props2.aria_label);
    }
}
