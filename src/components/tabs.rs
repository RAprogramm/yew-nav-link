//! Tab navigation container component.

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavTabsProps {
    #[prop_or_default]
    pub classes: Classes,

    #[prop_or("tablist")]
    pub role: &'static str,

    #[prop_or_default]
    pub id: Option<&'static str>,

    #[prop_or_default]
    pub full_width: bool,

    pub children: Children,
}

#[function_component]
pub fn NavTabs(props: &NavTabsProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-tabs");

    if props.full_width {
        classes.push("nav-tabs-fill");
    }

    html! {
        <ul
            {classes}
            id={props.id}
            role={props.role}
        >
            { for props.children.iter() }
        </ul>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_tabs_props_default() {
        let props = NavTabsProps {
            classes: Classes::default(),
            role: "tablist",
            id: None,
            full_width: false,
            children: Children::new(vec![]),
        };

        assert_eq!(props.role, "tablist");
        assert!(!props.full_width);
    }

    #[test]
    fn nav_tabs_full_width() {
        let props = NavTabsProps {
            classes: Classes::default(),
            role: "tablist",
            id: Some("main-tabs"),
            full_width: true,
            children: Children::new(vec![]),
        };

        assert!(props.full_width);
        assert_eq!(props.id, Some("main-tabs"));
    }
}
