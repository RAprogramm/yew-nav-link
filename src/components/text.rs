//! # NavText
//!
//! Static text label component for navigation items.
//! Renders a `<span>` with the `nav-text` class.
//!
//! # Example
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::{NavItem, NavLink, NavList, NavText};
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
//!             <NavItem><NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>></NavItem>
//!             <NavText text="v0.6.0" />
//!         </NavList>
//!     }
//! }
//! ```
//!
//! # CSS Classes
//!
//! | Class | Condition |
//! |-------|-----------|
//! | `nav-text` | Always applied |
//!
//! # Props
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `text` | `&'static str` | — | Static text content (required) |
//! | `classes` | `Classes` | — | Additional CSS classes |

use yew::prelude::*;

/// Properties for the [`NavText`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `text` | `&'static str` | — | Static text content (required) |
/// | `classes` | `Classes` | — | Additional CSS classes |
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavTextProps {
    /// Additional CSS classes applied to the text element.
    #[prop_or_default]
    pub classes: Classes,

    /// Static text content to display.
    pub text: &'static str
}

/// Text label component for navigation items.
///
/// Renders a `<span>` with the `nav-text` class.
#[function_component]
pub fn NavText(props: &NavTextProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-text");

    html! {
        <span {classes}>
            { props.text }
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_text_props() {
        let props = NavTextProps {
            classes: Classes::default(),
            text:    "Hello"
        };

        assert_eq!(props.text, "Hello");
    }

    #[test]
    fn nav_text_clone() {
        let props1 = NavTextProps {
            classes: Classes::from("custom"),
            text:    "Text"
        };

        let props2 = props1.clone();
        assert_eq!(props1.text, props2.text);
    }
}
