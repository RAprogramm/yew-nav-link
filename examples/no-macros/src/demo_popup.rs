//! Popup overlay for live demo link clicks.
//!
//! Wraps demo content and intercepts `<a>` clicks to show link info
//! instead of navigating.

use web_sys::HtmlElement;
use yew::prelude::*;

/// Info displayed in the popup when a link is clicked in a demo.
#[derive(Clone, PartialEq, Debug)]
struct PopupInfo {
    text: String,
    href: String,
    classes: String,
    is_active: bool,
}

/// Wraps live demo content. Clicks on links show a popup instead of navigating.
#[function_component]
pub fn DemoBox(props: &DemoBoxProps) -> Html {
    let popup = use_state(|| None::<PopupInfo>);

    let on_click = {
        let popup = popup.clone();
        Callback::from(move |e: MouseEvent| {
            // Find the closest <a> element
            let target: HtmlElement = e.target_dyn_into().unwrap();
            let el = target.closest("a").ok().flatten();
            if let Some(a) = el {
                e.prevent_default();
                e.stop_propagation();

                let href = a.get_attribute("href").unwrap_or_default();
                let text = a.text_content().unwrap_or_default();
                let classes = a.get_attribute("class").unwrap_or_default();
                let is_active = classes.contains("active");

                popup.set(Some(PopupInfo {
                    text: text.trim().to_string(),
                    href,
                    classes,
                    is_active,
                }));
            }
        })
    };

    let close = {
        let popup = popup.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            popup.set(None);
        })
    };

    let overlay = if let Some(info) = (*popup).clone() {
        let _cls = if info.is_active { "popup-active" } else { "" };
        let badge_class = format!("popup-badge {}", _cls);
        html! {
            <div class="popup-overlay" onclick={close.clone()}>
                <div class="popup-card" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                    <div class="popup-header">
                        <span class={badge_class}>
                            { if info.is_active { "ACTIVE" } else { "INACTIVE" } }
                        </span>
                        <button class="popup-close" onclick={close}>{ "×" }</button>
                    </div>
                    <h3 class="popup-title">{ "Link Clicked" }</h3>
                    <div class="popup-row">
                        <span class="popup-label">{ "Text" }</span>
                        <code>{ &info.text }</code>
                    </div>
                    <div class="popup-row">
                        <span class="popup-label">{ "Route" }</span>
                        <code>{ &info.href }</code>
                    </div>
                    <div class="popup-row">
                        <span class="popup-label">{ "Classes" }</span>
                        <code>
                            { if info.classes.is_empty() { "(none)" } else { &info.classes } }
                        </code>
                    </div>
                    <div class="popup-row">
                        <span class="popup-label">{ "Active" }</span>
                        <span class={if info.is_active { "popup-yes" } else { "popup-no" }}>
                            { if info.is_active { "yes" } else { "no" } }
                        </span>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    };

    html! {
        <div class="demo" onclick={on_click}>
            <div class="demo-label">{ "Live Demo" }</div>
            { for props.children.iter() }
            { overlay }
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct DemoBoxProps {
    pub children: Children,
}
