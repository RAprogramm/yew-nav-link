//! # NavDivider
//!
//! Semantic separator for dividing sections within a navigation list.
//! Renders a `<li>` with `role="separator"` and an optional text label.
//!
//! # Example
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::{NavDivider, NavItem, NavLink, NavList};
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Routable)]
//! # enum Route {
//! #     #[at("/")]
//! #     Home,
//! #     #[at("/about")]
//! #     About,
//! # }
//! #[component]
//! fn Nav() -> Html {
//!     html! {
//!         <NavList>
//!             <NavItem><NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>></NavItem>
//!             <NavDivider />
//!             <NavItem><NavLink<Route> to={Route::About}>{ "About" }</NavLink<Route>></NavItem>
//!         </NavList>
//!     }
//! }
//! ```
//!
//! # CSS Classes
//!
//! | Class | Condition |
//! |-------|-----------|
//! | `nav-divider` | Always applied |
//! | `nav-divider-vertical` | Applied when `vertical` is `true` |
//! | `nav-divider-text` | Inner text span when `text` is set |
//!
//! # Props
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `text` | `Option<&'static str>` | `None` | Optional text label |
//! | `vertical` | `bool` | `false` | Vertical divider style |
//! | `classes` | `Classes` | — | Additional CSS classes |

use yew::prelude::*;

/// Properties for the [`NavDivider`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `text` | `Option<&'static str>` | `None` | Optional text label |
/// | `vertical` | `bool` | `false` | Vertical divider style |
/// | `classes` | `Classes` | — | Additional CSS classes |
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct NavDividerProps {
    /// Additional CSS classes applied to the divider.
    #[prop_or_default]
    pub classes: Classes,

    /// Whether to render a vertical divider instead of horizontal.
    #[prop_or_default]
    pub vertical: bool,

    /// Optional text label displayed in the center of the divider.
    #[prop_or_default]
    pub text: Option<&'static str>,
}

/// Semantic separator for dividing sections within a navigation list.
///
/// Renders a `<li>` element with `role="separator"`.
///
/// # CSS Classes
///
/// - `nav-divider` - Always applied
/// - `nav-divider-vertical` - Applied when `vertical` is `true`
/// - `nav-divider-text` - Applied to inner text span when `text` is set
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
            classes: Classes::default(),
            vertical: false,
            text: None,
        };

        assert!(!props.vertical);
        assert!(props.text.is_none());
    }

    #[test]
    fn nav_divider_props_vertical() {
        let props = NavDividerProps {
            classes: Classes::default(),
            vertical: true,
            text: None,
        };

        assert!(props.vertical);
    }

    #[test]
    fn nav_divider_props_with_text() {
        let props = NavDividerProps {
            classes: Classes::default(),
            vertical: false,
            text: Some("Or"),
        };

        assert_eq!(props.text, Some("Or"));
    }

    #[test]
    fn nav_divider_props_clone() {
        let props1 = NavDividerProps {
            classes: Classes::from("custom"),
            vertical: true,
            text: Some("Divider"),
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
