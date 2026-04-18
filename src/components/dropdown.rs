//! # `NavDropdown`
//!
//! Collapsible dropdown menu for grouping related navigation items.
//! Renders a `<li>` with a toggle button and a nested `<ul>` menu.
//!
//! # Example
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::{
//!     NavItem, NavLink, NavList,
//!     components::{NavDropdown, NavDropdownDivider, NavDropdownItem}
//! };
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Routable)]
//! # enum Route {
//! #     #[at("/")]
//! #     Home,
//! #     #[at("/settings")]
//! #     Settings,
//! # }
//! #[component]
//! fn Nav() -> Html {
//!     html! {
//!         <NavList>
//!             <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
//!             <NavDropdown toggle_text="Settings">
//!                 <NavDropdownItem>
//!                     <NavLink<Route> to={Route::Settings}>{ "Profile" }</NavLink<Route>>
//!                 </NavDropdownItem>
//!                 <NavDropdownDivider />
//!                 <NavDropdownItem disabled=true>
//!                     { "Admin" }
//!                 </NavDropdownItem>
//!             </NavDropdown>
//!         </NavList>
//!     }
//! }
//! ```
//!
//! # CSS Classes
//!
//! | Class | Condition |
//! |-------|-----------|
//! | `nav-dropdown` | Always on container `<li>` |
//! | `nav-dropdown-toggle` | Toggle button |
//! | `nav-dropdown-menu` | Inner `<ul>` |
//! | `nav-dropdown-caret` | Caret indicator |
//! | `nav-dropdown-item` | Menu items |
//! | `nav-dropdown-divider` | Separator |
//! | `disabled` | Applied to disabled items |
//!
//! # Props
//!
//! **`NavDropdown`:**
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `toggle_text` | `&'static str` | `"dropdown"` | Toggle button label |
//! | `id` | `Option<&'static str>` | `None` | Element id |
//! | `classes` | `Classes` | — | Additional CSS classes |
//! | `children` | `Children` | — | Menu content |
//!
//! **`NavDropdownItem`:**
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `disabled` | `bool` | `false` | Disable the item |
//! | `classes` | `Classes` | — | Additional CSS classes |
//! | `children` | `Children` | — | Item content |
//!
//! **`NavDropdownDivider`:**
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `classes` | `Classes` | — | Additional CSS classes |

use yew::prelude::*;

/// Properties for the [`NavDropdown`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `toggle_text` | `&'static str` | `"dropdown"` | Toggle button label |
/// | `id` | `Option<&'static str>` | `None` | Element id |
/// | `classes` | `Classes` | — | Additional CSS classes |
/// | `children` | `Children` | — | Menu content |
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct NavDropdownProps {
    /// Additional CSS classes applied to the dropdown container.
    #[prop_or_default]
    pub classes: Classes,

    /// Text displayed on the dropdown toggle button.
    #[prop_or("dropdown")]
    pub toggle_text: &'static str,

    /// Optional `id` attribute for the dropdown element.
    #[prop_or_default]
    pub id: Option<&'static str>,

    /// Content rendered inside the dropdown menu.
    #[prop_or_default]
    pub children: Children
}

/// Collapsible dropdown menu for grouping navigation links.
///
/// # CSS Classes
///
/// - `nav-dropdown` - Container `<li>` element
/// - `nav-dropdown-toggle` - Toggle button
/// - `nav-dropdown-menu` - Inner `<ul>` menu
/// - `nav-dropdown-caret` - Caret indicator
#[function_component]
pub fn NavDropdown(props: &NavDropdownProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-dropdown");

    let open = use_state(|| false);

    let on_toggle = {
        let open = open.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            open.set(!*open);
        })
    };

    let menu_class = if *open {
        "nav-dropdown-menu open"
    } else {
        "nav-dropdown-menu"
    };

    html! {
        <li {classes} role="presentation">
            <button
                type="button"
                class="nav-dropdown-toggle"
                aria-expanded={if *open { "true" } else { "false" }}
                aria-haspopup="true"
                onclick={on_toggle}
            >
                { props.toggle_text }
                <span class="nav-dropdown-caret">{" ▼"}</span>
            </button>
            <ul class={menu_class} role="menu">
                { for props.children.iter() }
            </ul>
        </li>
    }
}

/// Properties for the [`NavDropdownItem`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `disabled` | `bool` | `false` | Disable the item |
/// | `classes` | `Classes` | — | Additional CSS classes |
/// | `children` | `Children` | — | Item content |
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct NavDropdownItemProps {
    /// Additional CSS classes applied to the item.
    #[prop_or_default]
    pub classes: Classes,

    /// Whether the dropdown item is disabled.
    #[prop_or_default]
    pub disabled: bool,

    /// Content rendered inside the item.
    pub children: Children
}

/// A single item within a [`NavDropdown`] menu.
///
/// # CSS Classes
///
/// - `nav-dropdown-item` - Always applied
/// - `disabled` - Applied when `disabled` is `true`
#[function_component]
pub fn NavDropdownItem(props: &NavDropdownItemProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-dropdown-item");

    if props.disabled {
        classes.push("disabled");
    }

    html! {
        <li class={classes} role="menuitem">
            { for props.children.iter() }
        </li>
    }
}

/// Properties for the [`NavDropdownDivider`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `classes` | `Classes` | — | Additional CSS classes |
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct NavDropdownDividerProps {
    /// Additional CSS classes applied to the divider.
    #[prop_or_default]
    pub classes: Classes
}

/// Visual separator between items in a [`NavDropdown`] menu.
///
/// Renders a `<li>` element with `role="separator"`.
#[function_component]
pub fn NavDropdownDivider(props: &NavDropdownDividerProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-dropdown-divider");

    html! {
        <li class={classes} role="separator" />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_dropdown_props_default() {
        let props = NavDropdownProps {
            classes:     Classes::default(),
            toggle_text: "Menu",
            id:          None,
            children:    Children::new(vec![])
        };

        assert_eq!(props.toggle_text, "Menu");
        assert!(props.id.is_none());
    }

    #[test]
    fn nav_dropdown_item_default() {
        let props = NavDropdownItemProps {
            classes:  Classes::default(),
            disabled: false,
            children: Children::new(vec![])
        };

        assert!(!props.disabled);
    }

    #[test]
    fn nav_dropdown_item_disabled() {
        let props = NavDropdownItemProps {
            classes:  Classes::default(),
            disabled: true,
            children: Children::new(vec![])
        };

        assert!(props.disabled);
    }

    #[test]
    fn nav_dropdown_divider_props() {
        let props = NavDropdownDividerProps {
            classes: Classes::default()
        };

        assert!(props.classes.is_empty());
    }
}
