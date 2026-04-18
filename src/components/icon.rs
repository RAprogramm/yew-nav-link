//! # `NavIcon`
//!
//! Icon wrapper for embedding icons in navigation items.
//! Renders an `<i>` element with `aria-hidden="true"` and a configurable size
//! class.
//!
//! # Example
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::{
//!     NavItem, NavLink, NavList,
//!     components::{NavIcon, NavIconSize}
//! };
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
//!                     <NavIcon name="home" size={NavIconSize::Small} />
//!                     { " Home" }
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
//! | `nav-icon` | Always applied |
//! | `nav-icon-sm` | Size is `Small` |
//! | `nav-icon-md` | Size is `Medium` (default) |
//! | `nav-icon-lg` | Size is `Large` |
//! | `nav-link-with-icon` | Applied to `NavLinkWithIcon` |
//!
//! # Props
//!
//! **`NavIcon`:**
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `name` | `Option<&'static str>` | `None` | Icon name |
//! | `size` | `NavIconSize` | `Medium` | Size variant |
//! | `classes` | `Classes` | — | Additional CSS classes |
//! | `children` | `Children` | — | Content when `name` is `None` |
//!
//! **`NavLinkWithIcon`:**
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `icon` | `NavIconSize` | — | Size of the icon (required) |
//! | `classes` | `Classes` | — | Additional CSS classes |
//! | `children` | `Children` | — | Content to wrap |

use yew::prelude::*;

/// Properties for the [`NavIcon`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `name` | `Option<&'static str>` | `None` | Icon name |
/// | `size` | `NavIconSize` | `Medium` | Size variant |
/// | `classes` | `Classes` | — | Additional CSS classes |
/// | `children` | `Children` | — | Content when `name` is `None` |
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavIconProps {
    /// Additional CSS classes applied to the icon.
    #[prop_or_default]
    pub classes: Classes,

    /// Optional icon name rendered as text content.
    #[prop_or_default]
    pub name: Option<&'static str>,

    /// Size variant of the icon.
    #[prop_or_default]
    pub size: NavIconSize,

    /// Content rendered inside the icon when `name` is `None`.
    #[prop_or_default]
    pub children: Children
}

/// Available size variants for [`NavIcon`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NavIconSize {
    /// Small icon (`nav-icon-sm`).
    Small,
    /// Medium icon, the default size (`nav-icon-md`).
    #[default]
    Medium,
    /// Large icon (`nav-icon-lg`).
    Large
}

impl NavIconSize {
    fn as_class(self) -> &'static str {
        match self {
            NavIconSize::Small => "nav-icon-sm",
            NavIconSize::Medium => "nav-icon-md",
            NavIconSize::Large => "nav-icon-lg"
        }
    }
}

/// Icon component for embedding icons within navigation elements.
///
/// Renders an `<i>` element with `aria-hidden="true"` and a size class.
///
/// # CSS Classes
///
/// - `nav-icon` - Always applied
/// - `nav-icon-sm`, `nav-icon-md`, `nav-icon-lg` - Size variant
#[function_component]
pub fn NavIcon(props: &NavIconProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-icon");
    classes.push(props.size.as_class());

    if let Some(name) = props.name {
        html! {
            <i class={classes} aria-hidden="true">
                { name }
            </i>
        }
    } else {
        html! {
            <i class={classes} aria-hidden="true">
                { for props.children.iter() }
            </i>
        }
    }
}

/// Properties for the [`NavLinkWithIcon`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `icon` | `NavIconSize` | — | Size of the icon (required) |
/// | `classes` | `Classes` | — | Additional CSS classes |
/// | `children` | `Children` | — | Content to wrap |
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavLinkWithIconProps {
    /// Additional CSS classes applied to the wrapper.
    #[prop_or_default]
    pub classes: Classes,

    /// Size of the associated icon.
    pub icon: NavIconSize,

    /// Content rendered inside the span.
    pub children: Children
}

/// Wraps content alongside an icon within a navigation link.
///
/// # CSS Classes
///
/// - `nav-link-with-icon` - Always applied
#[function_component]
pub fn NavLinkWithIcon(props: &NavLinkWithIconProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-link-with-icon");

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
    fn nav_icon_props_default() {
        let props = NavIconProps {
            classes:  Classes::default(),
            name:     None,
            size:     NavIconSize::default(),
            children: Children::new(vec![])
        };

        assert!(props.name.is_none());
        assert_eq!(props.size, NavIconSize::Medium);
    }

    #[test]
    fn nav_icon_with_name() {
        let props = NavIconProps {
            classes:  Classes::default(),
            name:     Some("home"),
            size:     NavIconSize::Small,
            children: Children::new(vec![])
        };

        assert_eq!(props.name, Some("home"));
        assert_eq!(props.size, NavIconSize::Small);
    }

    #[test]
    fn nav_icon_size_variants() {
        assert_eq!(NavIconSize::Small.as_class(), "nav-icon-sm");
        assert_eq!(NavIconSize::Medium.as_class(), "nav-icon-md");
        assert_eq!(NavIconSize::Large.as_class(), "nav-icon-lg");
    }

    #[test]
    fn nav_icon_default() {
        assert_eq!(NavIconSize::default(), NavIconSize::Medium);
    }
}
