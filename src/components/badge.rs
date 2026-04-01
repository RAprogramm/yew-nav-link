//! # NavBadge
//!
//! Inline badge for displaying notification counts and status indicators
//! within navigation items. Renders a `<span>` with variant-based CSS classes.
//!
//! # Example
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::{NavItem, NavLink, NavList, NavBadge};
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Routable)]
//! # enum Route { #[at("/")] Home }
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
//! # CSS Classes
//!
//! | Class | Condition |
//! |-------|-----------|
//! | `nav-badge` | Always applied |
//! | `nav-badge-pill` | Applied when `pill` is `true` |
//! | `nav-badge-{variant}` | Applied based on the `variant` prop |
//!
//! # Props
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `variant` | `&'static str` | `"primary"` | Visual variant name |
//! | `pill` | `bool` | `false` | Pill-shaped corners |
//! | `classes` | `Classes` | — | Additional CSS classes |
//! | `children` | `Children` | — | Badge content |

use yew::prelude::*;

/// Properties for the [`NavBadge`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `variant` | `&'static str` | `"primary"` | Visual variant name |
/// | `pill` | `bool` | `false` | Pill-shaped corners |
/// | `classes` | `Classes` | — | Additional CSS classes |
/// | `children` | `Children` | — | Badge content |
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavBadgeProps {
    /// Additional CSS classes applied to the badge.
    #[prop_or_default]
    pub classes: Classes,

    /// Visual variant name, e.g. `"primary"`, `"success"`, `"danger"`.
    #[prop_or("primary")]
    pub variant: &'static str,

    /// Render the badge with pill-shaped (fully rounded) corners.
    #[prop_or_default]
    pub pill: bool,

    /// Content rendered inside the badge.
    #[prop_or_default]
    pub children: Children,
}

/// Badge component for displaying status or count indicators on navigation items.
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

    let variant_class = format!("nav-badge-{}", props.variant);
    classes.push(variant_class);

    html! {
        <span {classes}>
            { for props.children.iter() }
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_badge_props_default() {
        let props = NavBadgeProps {
            classes: Classes::default(),
            variant: "primary",
            pill: false,
            children: Children::new(vec![]),
        };

        assert!(!props.pill);
        assert_eq!(props.variant, "primary");
    }

    #[test]
    fn nav_badge_clone() {
        let props1 = NavBadgeProps {
            classes: Classes::from("test"),
            variant: "success",
            pill: true,
            children: Children::new(vec![]),
        };

        let props2 = props1.clone();
        assert_eq!(props1.variant, props2.variant);
        assert_eq!(props1.pill, props2.pill);
    }
}
