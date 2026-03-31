//! Badge component for navigation items.

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavBadgeProps {
    #[prop_or_default]
    pub classes: Classes,

    #[prop_or("primary")]
    pub variant: &'static str,

    #[prop_or_default]
    pub pill: bool,

    #[prop_or_default]
    pub children: Children
}

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
            classes:  Classes::default(),
            variant:  "primary",
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
            variant:  "success",
            pill:     true,
            children: Children::new(vec![])
        };

        let props2 = props1.clone();
        assert_eq!(props1.variant, props2.variant);
        assert_eq!(props1.pill, props2.pill);
    }
}
