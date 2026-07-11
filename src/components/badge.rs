// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! # `NavBadge`
//!
//! Small colored label for showing counts, statuses, or labels inside
//! navigation links. Renders a `<span>` you can place anywhere inside
//! a `NavLink` or `NavItem`.
//!
//! # Quick Start
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::{NavBadge, NavItem, NavLink, NavList};
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
//!                 <NavLink<Route> to={Route::Home}>
//!                     { "Messages " }
//!                     <NavBadge variant="danger">{ "3" }</NavBadge>
//!                 </NavLink<Route>>
//!             </NavItem>
//!         </NavList>
//!     }
//! }
//! ```
//!
//! Available variants: `"primary"`, `"success"`, `"warning"`, `"danger"`.
//! Set `pill=true` for fully rounded corners.
//!
//! # CSS Classes
//!
//! | Class | When Applied |
//! |-------|--------------|
//! | `nav-badge` | Always |
//! | `nav-badge-pill` | When `pill` is `true` |
//! | `nav-badge-{variant}` | Based on the `variant` prop |
//!
//! # Props
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `variant` | `AttrValue` | `"primary"` | Color variant |
//! | `pill` | `bool` | `false` | Rounded pill shape |
//! | `classes` | `Classes` | — | Additional CSS classes |
//! | `children` | `Children` | — | Badge content |

use yew::prelude::*;

/// Properties for the [`NavBadge`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `variant` | `AttrValue` | `"primary"` | Visual variant name |
/// | `pill` | `bool` | `false` | Pill-shaped corners |
/// | `classes` | `Classes` | — | Additional CSS classes |
/// | `children` | `Children` | — | Badge content |
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavBadgeProps {
    /// Additional CSS classes applied to the badge.
    #[prop_or_default]
    pub classes: Classes,

    /// Visual variant name, e.g. `"primary"`, `"success"`, `"danger"`.
    #[prop_or(AttrValue::Static("primary"))]
    pub variant: AttrValue,

    /// Render the badge with pill-shaped (fully rounded) corners.
    #[prop_or_default]
    pub pill: bool,

    /// Content rendered inside the badge.
    #[prop_or_default]
    pub children: Children
}

/// Badge component for displaying status or count indicators on navigation
/// items.
///
/// # CSS Classes
///
/// - `nav-badge` - Always applied
/// - `nav-badge-pill` - Applied when `pill` is `true`
/// - `nav-badge-{variant}` - Applied based on the `variant` prop
#[function_component]
pub fn NavBadge(props: &NavBadgeProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-badge");

    if props.pill {
        classes.push("nav-badge-pill");
    }

    classes.push(variant_class(&props.variant));

    html! {
        <span class={classes}>
            { for props.children.iter() }
        </span>
    }
}

/// Maps a `variant` prop value to its precomputed CSS class. Unknown variants
/// fall back to `nav-badge-primary` (the documented default).
const fn variant_class(variant: &str) -> &'static str {
    match variant.as_bytes() {
        b"success" => "nav-badge-success",
        b"warning" => "nav-badge-warning",
        b"danger" => "nav-badge-danger",
        _ => "nav-badge-primary"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_badge_props_default() {
        let props = NavBadgeProps {
            classes:  Classes::default(),
            variant:  AttrValue::Static("primary"),
            pill:     false,
            children: Children::new(vec![])
        };

        assert!(!props.pill);
        assert_eq!(props.variant, "primary");
    }

    #[test]
    fn nav_badge_clone() {
        let props1 = NavBadgeProps {
            classes:  Classes::from("test"),
            variant:  AttrValue::Static("success"),
            pill:     true,
            children: Children::new(vec![])
        };

        let props2 = props1.clone();
        assert_eq!(props1.variant, props2.variant);
        assert_eq!(props1.pill, props2.pill);
    }

    #[test]
    fn variant_class_maps_documented_variants() {
        assert_eq!(variant_class("primary"), "nav-badge-primary");
        assert_eq!(variant_class("success"), "nav-badge-success");
        assert_eq!(variant_class("warning"), "nav-badge-warning");
        assert_eq!(variant_class("danger"), "nav-badge-danger");
    }

    #[test]
    fn variant_class_unknown_falls_back_to_primary() {
        assert_eq!(variant_class(""), "nav-badge-primary");
        assert_eq!(variant_class("unknown"), "nav-badge-primary");
        assert_eq!(variant_class("PRIMARY"), "nav-badge-primary");
    }
}
