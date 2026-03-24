//! Dropdown navigation component.
//!
//! Provides a collapsible dropdown menu for navigation.

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavDropdownProps {
    #[prop_or_default]
    pub classes: Classes,

    #[prop_or("dropdown")]
    pub toggle_text: &'static str,

    #[prop_or_default]
    pub id: Option<&'static str>,

    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn NavDropdown(props: &NavDropdownProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-dropdown");

    html! {
        <li {classes} role="presentation" data-bs-toggle="dropdown">
            <button
                type="button"
                class="nav-dropdown-toggle"
                aria-expanded="false"
                aria-haspopup="true"
            >
                { props.toggle_text }
                <span class="nav-dropdown-caret">{" ▼"}</span>
            </button>
            <ul class="nav-dropdown-menu" role="menu">
                { for props.children.iter() }
            </ul>
        </li>
    }
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavDropdownItemProps {
    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub disabled: bool,

    pub children: Children,
}

#[function_component]
pub fn NavDropdownItem(props: &NavDropdownItemProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-dropdown-item");

    if props.disabled {
        classes.push("disabled");
    }

    html! {
        <li {classes} role="menuitem">
            <a href="#" onclick={move |e: MouseEvent| e.prevent_default()}>
                { for props.children.iter() }
            </a>
        </li>
    }
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavDropdownDividerProps {
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component]
pub fn NavDropdownDivider(props: &NavDropdownDividerProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-dropdown-divider");

    html! {
        <li {classes} role="separator" />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_dropdown_props_default() {
        let props = NavDropdownProps {
            classes: Classes::default(),
            toggle_text: "Menu",
            id: None,
            children: Children::new(vec![]),
        };

        assert_eq!(props.toggle_text, "Menu");
        assert!(props.id.is_none());
    }

    #[test]
    fn nav_dropdown_item_default() {
        let props = NavDropdownItemProps {
            classes: Classes::default(),
            disabled: false,
            children: Children::new(vec![]),
        };

        assert!(!props.disabled);
    }

    #[test]
    fn nav_dropdown_item_disabled() {
        let props = NavDropdownItemProps {
            classes: Classes::default(),
            disabled: true,
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }

    #[test]
    fn nav_dropdown_divider_props() {
        let props = NavDropdownDividerProps {
            classes: Classes::default(),
        };

        assert!(props.classes.is_empty());
    }
}
