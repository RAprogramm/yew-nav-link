//! Tooltip overlay for live demo link hover.
//!
//! Wraps demo content and shows a tooltip near hovered links
//! with info about the link (text, route, classes, active state).

use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
struct TooltipInfo {
    text: String,
    href: String,
    classes: String,
    is_active: bool,
    x: i32,
    y: i32,
}

/// Wraps live demo content. Hover on links shows a tooltip with info.
/// Clicking links does nothing (no navigation).
#[function_component]
pub fn DemoBox(props: &DemoBoxProps) -> Html {
    let tooltip = use_state(|| None::<TooltipInfo>);

    let on_mouse_over = {
        let tooltip = tooltip.clone();
        Callback::from(move |e: MouseEvent| {
            let target: HtmlElement = match e.target_dyn_into() {
                Some(t) => t,
                None => return,
            };

            if let Ok(Some(a)) = target.closest("a") {
                let href = a.get_attribute("href").unwrap_or_default();
                let text = a.text_content().unwrap_or_default();
                let classes = a.get_attribute("class").unwrap_or_default();
                let is_active = classes.contains("active");

                let x = e.client_x();
                let y = e.client_y();

                tooltip.set(Some(TooltipInfo {
                    text: text.trim().to_string(),
                    href,
                    classes,
                    is_active,
                    x,
                    y,
                }));
            }
        })
    };

    let on_mouse_out = {
        let tooltip = tooltip.clone();
        Callback::from(move |_: MouseEvent| {
            tooltip.set(None);
        })
    };

    let on_click = Callback::from(|e: MouseEvent| {
        let target: HtmlElement = match e.target_dyn_into() {
            Some(t) => t,
            None => return,
        };
        if target.closest("a").ok().flatten().is_some() {
            e.prevent_default();
            e.stop_propagation();
        }
    });

    let tooltip_html = if let Some(info) = (*tooltip).clone() {
        let badge_cls = if info.is_active {
            "tooltip-badge active"
        } else {
            "tooltip-badge"
        };
        let style = format!("left:{}px; top:{}px;", info.x + 12, info.y + 20,);
        html! {
            <div class="tooltip-card" {style}>
                <span class={badge_cls}>
                    { if info.is_active { "ACTIVE" } else { "INACTIVE" } }
                </span>
                <div class="tooltip-row">
                    <span class="tooltip-label">{ "text" }</span>
                    <code>{ &info.text }</code>
                </div>
                <div class="tooltip-row">
                    <span class="tooltip-label">{ "route" }</span>
                    <code>{ &info.href }</code>
                </div>
                <div class="tooltip-row">
                    <span class="tooltip-label">{ "class" }</span>
                    <code>{ if info.classes.is_empty() { "(none)" } else { &info.classes } }</code>
                </div>
            </div>
        }
    } else {
        html! {}
    };

    html! {
        <div class="demo" onmouseover={on_mouse_over} onmouseout={on_mouse_out} onclick={on_click}>
            <div class="demo-label">{ "Live Demo" }</div>
            { for props.children.iter() }
            { tooltip_html }
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct DemoBoxProps {
    pub children: Children,
}
