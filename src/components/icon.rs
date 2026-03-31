//! Icon component for navigation items.
//!
//! Provides a simple way to add icons to navigation elements.

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavIconProps {
    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub name: Option<&'static str>,

    #[prop_or_default]
    pub size: NavIconSize,

    #[prop_or_default]
    pub children: Children
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NavIconSize {
    Small,
    #[default]
    Medium,
    Large
}

impl NavIconSize {
    fn as_class(&self) -> &'static str {
        match self {
            NavIconSize::Small => "nav-icon-sm",
            NavIconSize::Medium => "nav-icon-md",
            NavIconSize::Large => "nav-icon-lg"
        }
    }
}

#[function_component]
pub fn NavIcon(props: &NavIconProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-icon");
    classes.push(props.size.as_class());

    if let Some(name) = props.name {
        html! {
            <i {classes} aria-hidden="true">
                { name }
            </i>
        }
    } else {
        html! {
            <i {classes} aria-hidden="true">
                { for props.children.iter() }
            </i>
        }
    }
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavLinkWithIconProps {
    #[prop_or_default]
    pub classes: Classes,

    pub icon: NavIconSize,

    pub children: Children
}

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
