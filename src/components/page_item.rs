//! Page item component.

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct PageItemProps {
    #[prop_or_default]
    pub classes: Classes,

    pub page: u32,

    #[prop_or(false)]
    pub active: bool,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn PageItem(props: &PageItemProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("page-item");

    if props.active {
        classes.push("active");
    }

    if props.disabled {
        classes.push("disabled");
    }

    html! {
        <li {classes}>
            <span class="page-link">
                { for props.children.iter() }
            </span>
        </li>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_item_props() {
        let props = PageItemProps {
            classes: Classes::default(),
            page: 1,
            active: false,
            disabled: false,
            children: Children::new(vec![]),
        };

        assert_eq!(props.page, 1);
        assert!(!props.active);
        assert!(!props.disabled);
    }
}
