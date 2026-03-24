//! Header component for navigation sections.

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavHeaderProps {
    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub text: Option<&'static str>,

    #[prop_or_default]
    pub children: Children,
}

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
