//! Text component for navigation items.

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavTextProps {
    #[prop_or_default]
    pub classes: Classes,

    pub text: &'static str
}

#[function_component]
pub fn NavText(props: &NavTextProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-text");

    html! {
        <span {classes}>
            { props.text }
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_text_props() {
        let props = NavTextProps {
            classes: Classes::default(),
            text:    "Hello"
        };

        assert_eq!(props.text, "Hello");
    }

    #[test]
    fn nav_text_clone() {
        let props1 = NavTextProps {
            classes: Classes::from("custom"),
            text:    "Text"
        };

        let props2 = props1.clone();
        assert_eq!(props1.text, props2.text);
    }
}
