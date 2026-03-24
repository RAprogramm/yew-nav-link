//! Page link component.

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct PageLinkProps {
    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub href: Option<&'static str>,

    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn PageLink(props: &PageLinkProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("page-link");

    if let Some(href) = props.href {
        html! {
            <a {href} {classes}>
                { for props.children.iter() }
            </a>
        }
    } else {
        html! {
            <span {classes}>
                { for props.children.iter() }
            </span>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_link_with_href() {
        let props = PageLinkProps {
            classes: Classes::default(),
            href: Some("/page/2"),
            children: Children::new(vec![]),
        };

        assert_eq!(props.href, Some("/page/2"));
    }

    #[test]
    fn page_link_without_href() {
        let props = PageLinkProps {
            classes: Classes::default(),
            href: None,
            children: Children::new(vec![]),
        };

        assert!(props.href.is_none());
    }
}
