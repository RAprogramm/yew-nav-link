//! Navigation divider component.
//!
//! Provides a semantic separator for navigation lists.
//!
//! # Example
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::{NavDivider, NavItem, NavLink, NavList};
//! use yew_router::prelude::*;
//!
//! #[derive(Clone, PartialEq, Routable)]
//! enum Route {
//!     #[at("/")]
//!     Home,
//!     #[at("/about")]
//!     About
//! }
//!
//! #[component]
//! fn Navigation() -> Html {
//!     html! {
//!         <NavList>
//!             <NavItem>
//!                 <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
//!             </NavItem>
//!             <NavDivider />
//!             <NavItem>
//!                 <NavLink<Route> to={Route::About}>{ "About" }</NavLink<Route>>
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
//! | `nav-divider` | Base class for the divider |

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct NavDividerProps {
    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub vertical: bool,

    #[prop_or_default]
    pub text: Option<&'static str>
}

#[function_component]
pub fn NavDivider(props: &NavDividerProps) -> Html {
    let mut classes = props.classes.clone();

    classes.push("nav-divider");
    if props.vertical {
        classes.push("nav-divider-vertical");
    }

    if let Some(text) = props.text {
        html! {
            <li {classes} role="separator">
                <span class="nav-divider-text">{ text }</span>
            </li>
        }
    } else {
        html! {
            <li {classes} role="separator" />
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_divider_props_default() {
        let props = NavDividerProps {
            classes:  Classes::default(),
            vertical: false,
            text:     None
        };

        assert!(!props.vertical);
        assert!(props.text.is_none());
    }

    #[test]
    fn nav_divider_props_vertical() {
        let props = NavDividerProps {
            classes:  Classes::default(),
            vertical: true,
            text:     None
        };

        assert!(props.vertical);
    }

    #[test]
    fn nav_divider_props_with_text() {
        let props = NavDividerProps {
            classes:  Classes::default(),
            vertical: false,
            text:     Some("Or")
        };

        assert_eq!(props.text, Some("Or"));
    }

    #[test]
    fn nav_divider_props_clone() {
        let props1 = NavDividerProps {
            classes:  Classes::from("custom"),
            vertical: true,
            text:     Some("Divider")
        };

        let props2 = props1.clone();
        assert_eq!(props1.vertical, props2.vertical);
        assert_eq!(props1.text, props2.text);
        assert_eq!(props1.classes, props2.classes);
    }

    #[test]
    fn nav_divider_equality() {
        let default = NavDividerProps::default();
        let with_class = NavDividerProps {
            classes: Classes::from("custom"),
            ..Default::default()
        };

        assert_ne!(default.classes, with_class.classes);
    }
}
