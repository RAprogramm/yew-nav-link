//! # NavHeader
//!
//! Section header for labeling groups within a navigation list.
//! Renders a `<li>` with `role="presentation"` and the `nav-header` class.
//!
//! # Example
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::{NavItem, NavLink, NavList, NavHeader};
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Routable)]
//! # enum Route { #[at("/")] Home, #[at("/about")] About }
//! #[component]
//! fn Nav() -> Html {
//!     html! {
//!         <NavList>
//!             <NavHeader text="Main" />
//!             <NavItem><NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>></NavItem>
//!             <NavHeader text="Info" />
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
//! | `nav-header` | Always applied |
//! | `nav-header-text` | Inner text span |
//!
//! # Props
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `text` | `Option<&'static str>` | `None` | Header text label |
//! | `classes` | `Classes` | — | Additional CSS classes |
//! | `children` | `Children` | — | Content when `text` is `None` |

use yew::prelude::*;

/// Properties for the [`NavHeader`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `text` | `Option<&'static str>` | `None` | Header text label |
/// | `classes` | `Classes` | — | Additional CSS classes |
/// | `children` | `Children` | — | Content when `text` is `None` |
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavHeaderProps {
    /// Additional CSS classes applied to the header.
    #[prop_or_default]
    pub classes: Classes,

    /// Text label displayed in the header. If set, overrides `children`.
    #[prop_or_default]
    pub text: Option<&'static str>,

    /// Content rendered inside the header when `text` is `None`.
    #[prop_or_default]
    pub children: Children,
}

/// Header component for labeling sections within a navigation list.
///
/// Renders a `<li>` with `role="presentation"` and the `nav-header` class.
#[function_component]
pub fn NavHeader(props: &NavHeaderProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-header");

    if let Some(text) = props.text {
        html! {
            <li {classes} role="presentation">
                <span class="nav-header-text">{ text }</span>
            </li>
        }
    } else {
        html! {
            <li {classes} role="presentation">
                { for props.children.iter() }
            </li>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_header_props_with_text() {
        let props = NavHeaderProps {
            classes: Classes::default(),
            text: Some("Header"),
            children: Children::new(vec![]),
        };

        assert_eq!(props.text, Some("Header"));
    }

    #[test]
    fn nav_header_clone() {
        let props1 = NavHeaderProps {
            classes: Classes::default(),
            text: Some("Header"),
            children: Children::new(vec![]),
        };

        let props2 = props1.clone();
        assert_eq!(props1.text, props2.text);
    }
}
