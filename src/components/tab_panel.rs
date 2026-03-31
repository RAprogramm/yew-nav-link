//! Tab panel component.

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavTabPanelProps {
    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub id: Option<&'static str>,

    #[prop_or_default]
    pub labelled_by: Option<&'static str>,

    pub hidden: bool,

    pub children: Children
}

#[function_component]
pub fn NavTabPanel(props: &NavTabPanelProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("nav-tab-panel");

    html! {
        <div
            {classes}
            id={props.id}
            role="tabpanel"
            aria-labelledby={props.labelled_by}
            hidden={props.hidden}
        >
            { for props.children.iter() }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_tab_panel_hidden() {
        let props = NavTabPanelProps {
            classes:     Classes::default(),
            id:          Some("panel-1"),
            labelled_by: Some("tab-1"),
            hidden:      true,
            children:    Children::new(vec![])
        };

        assert!(props.hidden);
        assert_eq!(props.id, Some("panel-1"));
    }

    #[test]
    fn nav_tab_panel_visible() {
        let props = NavTabPanelProps {
            classes:     Classes::default(),
            id:          Some("panel-1"),
            labelled_by: Some("tab-1"),
            hidden:      false,
            children:    Children::new(vec![])
        };

        assert!(!props.hidden);
    }
}
