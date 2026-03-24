//! Tab item component.

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavTabProps {
    #[prop_or_default]
    pub classes: Classes,

    pub active: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub id: Option<&'static str>,

    #[prop_or_default]
    pub panel_id: Option<&'static str>,

    #[prop_or_default]
    pub children: Children,

    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component]
pub fn NavTab(props: &NavTabProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-tab");

    if props.active {
        classes.push("active");
    }

    if props.disabled {
        classes.push("disabled");
    }

    let onclick = props.onclick.clone();
    let onclick = onclick.map(|cb| {
        move |e: MouseEvent| {
            e.prevent_default();
            cb.emit(e);
        }
    });

    html! {
        <li {classes} role="presentation">
            <button
                type="button"
                role="tab"
                id={props.id}
                aria-selected={props.active.to_string()}
                aria-controls={props.panel_id}
                disabled={props.disabled}
                onclick={onclick}
            >
                { for props.children.iter() }
            </button>
        </li>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_tab_active() {
        let props = NavTabProps {
            classes: Classes::default(),
            active: true,
            disabled: false,
            id: None,
            panel_id: None,
            children: Children::new(vec![]),
            onclick: None,
        };

        assert!(props.active);
        assert!(!props.disabled);
    }

    #[test]
    fn nav_tab_disabled() {
        let props = NavTabProps {
            classes: Classes::default(),
            active: false,
            disabled: true,
            id: None,
            panel_id: None,
            children: Children::new(vec![]),
            onclick: None,
        };

        assert!(props.disabled);
        assert!(!props.active);
    }
}
