//! # `NavList`
//!
//! A `<ul>` wrapper for your navigation links. Renders with `role="list"`
//! and an `aria-label` for screen readers. Use this as the outer container
//! whenever you build a nav menu — it keeps your markup semantic and
//! accessible by default.
//!
//! # Quick Start
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::{NavLink, NavList};
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
//!         <NavList aria_label="Main Navigation">
//!             <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
//!             <NavLink<Route> to={Route::About}>{ "About" }</NavLink<Route>>
//!         </NavList>
//!     }
//! }
//! ```
//!
//! # CSS Classes
//!
//! | Class | When Applied |
//! |-------|--------------|
//! | `nav-list` | Always |
//!
//! # Props
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `aria_label` | `Option<&'static str>` | `"navigation"` | Screen reader label |
//! | `id` | `Option<&'static str>` | `None` | Element id |
//! | `classes` | `Classes` | — | Additional CSS classes |
//! | `children` | `Children` | — | List items |
//!
//! # How It Works
//!
//! `NavList` renders a `<ul>` with `role="list"`. You place `<NavLink>` or
//! `<NavItem>` children inside — each one renders as an `<li>` with
//! `role="listitem"`. The `aria-label` is set automatically (defaults to
//! `"navigation"`).
//!
//! # Memory & Performance
//!
//! `NavList` is a thin wrapper — no state, no subscriptions, no allocations
//! beyond rendering the children. It's as cheap as a plain `<ul>` in HTML.

use yew::prelude::*;

/// Properties for the [`NavList`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `aria_label` | `Option<&'static str>` | `"navigation"` | ARIA label |
/// | `id` | `Option<&'static str>` | `None` | Element id |
/// | `classes` | `Classes` | — | Additional CSS classes |
/// | `children` | `Children` | — | Navigation items |
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavListProps {
    /// Additional CSS classes applied to the list.
    #[prop_or_default]
    pub classes: Classes,

    /// Optional `id` attribute for the list element.
    #[prop_or_default]
    pub id: Option<&'static str>,

    /// `aria-label` for screen readers. Defaults to `"navigation"`.
    #[prop_or_default]
    pub aria_label: Option<&'static str>,

    /// Navigation items rendered inside the list.
    pub children: Children
}

/// Semantic `<ul>` container for navigation items.
///
/// Renders with `role="list"` and an `aria-label` for accessibility.
///
/// # CSS Classes
///
/// - `nav-list` - Always applied
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
            classes:    Classes::default(),
            id:         None,
            aria_label: None,
            children:   Children::new(vec![])
        };

        assert!(props.classes.is_empty());
        assert!(props.id.is_none());
        assert!(props.aria_label.is_none());
    }

    #[test]
    fn nav_list_props_with_values() {
        let props = NavListProps {
            classes:    Classes::from("custom-class"),
            id:         Some("nav-id"),
            aria_label: Some("navigation"),
            children:   Children::new(vec![])
        };

        assert!(props.classes.contains("custom-class"));
        assert_eq!(props.id, Some("nav-id"));
        assert_eq!(props.aria_label, Some("navigation"));
    }

    #[test]
    fn nav_list_props_clone() {
        let props1 = NavListProps {
            classes:    Classes::from("test"),
            id:         Some("id"),
            aria_label: Some("label"),
            children:   Children::new(vec![])
        };

        let props2 = props1.clone();
        assert_eq!(props1.classes, props2.classes);
        assert_eq!(props1.id, props2.id);
        assert_eq!(props1.aria_label, props2.aria_label);
    }
}
