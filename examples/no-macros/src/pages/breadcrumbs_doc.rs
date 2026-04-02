use crate::demo_popup::DemoBox;
use crate::doc_parser::{copy_to_clipboard, highlight_rust, parse_doc_block, DocRenderer};
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::hooks::use_breadcrumbs;

const BREADCRUMBS_SRC: &str = include_str!("../../../../src/breadcrumbs.rs");

const CODE_SIMPLE: &str = "\
use yew_nav_link::hooks::use_breadcrumbs;

#[component]
fn BreadcrumbTrail() -> Html {
    let crumbs = use_breadcrumbs::<Route>();

    html! {
        <nav aria-label=\"breadcrumb\">
            <ol class=\"breadcrumb-list\">
                { for crumbs.iter().map(|c| html! {
                    <li class={if c.is_active {
                        \"breadcrumb-item active\"
                    } else {
                        \"breadcrumb-item\"
                    }}>
                        { &c.label }
                    </li>
                })}
            </ol>
        </nav>
    }
}";

const CODE_STYLED: &str = "\
use yew_nav_link::hooks::use_breadcrumbs;

#[component]
fn BreadcrumbStyled() -> Html {
    let crumbs = use_breadcrumbs::<Route>();

    html! {
        <nav aria-label=\"breadcrumb\">
            <ol class=\"breadcrumb-list\">
                { for crumbs.iter().enumerate().map(|(i, c)| {
                    let cls = if c.is_active {
                        \"breadcrumb-item active\".to_string()
                    } else {
                        \"breadcrumb-item\".to_string()
                    };
                    html! {
                        <li class={cls}>
                            if i > 0 {
                                <span class=\"breadcrumb-sep\">{ \"→\" }</span>
                            }
                            { &c.label }
                        </li>
                    }
                })}
            </ol>
        </nav>
    }
}";

#[function_component]
pub fn BreadcrumbsDoc() -> Html {
    let doc = parse_doc_block(BREADCRUMBS_SRC);
    let crumbs = use_breadcrumbs::<Route>();

    html! {
        <div>
            <DocRenderer {doc} />

            // ── Card 1: Quick Start ───────────────────────
            <div class="card">
                <h3>{ "Quick Start — Code + Preview" }</h3>
                <p>{ "Minimal example. Copy the code, see it work below:" }</p>
                <CopyCode code={CODE_SIMPLE.to_string()} />
                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <nav aria-label="breadcrumb">
                        <ol class="breadcrumb-list">
                            { for crumbs.iter().map(|crumb| {
                                html! {
                                    <li class={if crumb.is_active { "breadcrumb-item active" } else { "breadcrumb-item" }}>
                                        { &crumb.label }
                                    </li>
                                }
                            })}
                        </ol>
                    </nav>
                </DemoBox>
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Navigate with the sidebar — the trail rebuilds automatically." }
                </p>
            </div>

            // ── Card 2: Styled with separators ────────────
            <div class="card">
                <h3>{ "Styled with Separators — Code + Preview" }</h3>
                <p>{ "Add arrows between items with " }<code>{ ".enumerate()" }</code>{ ":" }</p>
                <CopyCode code={CODE_STYLED.to_string()} />
                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <nav aria-label="breadcrumb">
                        <ol class="breadcrumb-list">
                            { for crumbs.iter().enumerate().map(|(i, crumb)| {
                                let cls = if crumb.is_active { "breadcrumb-item active" } else { "breadcrumb-item" };
                                html! {
                                    <li class={cls}>
                                        if i > 0 {
                                            <span style="margin:0 0.375rem; color:var(--text-muted);">{ "→" }</span>
                                        }
                                        { &crumb.label }
                                    </li>
                                }
                            })}
                        </ol>
                    </nav>
                </DemoBox>
            </div>
        </div>
    }
}

// ── Reusable code block with copy button ───────────────────────

#[derive(Properties, PartialEq, Clone)]
struct CopyCodeProps {
    code: String,
}

#[function_component]
fn CopyCode(props: &CopyCodeProps) -> Html {
    let copied = use_state(|| false);
    let code = props.code.clone();

    let on_copy = {
        let copied = copied.clone();
        let code = code.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            let copied = copied.clone();
            let code = code.clone();
            wasm_bindgen_futures::spawn_local(async move {
                copy_to_clipboard(&code);
                copied.set(true);
                gloo_timers::future::TimeoutFuture::new(1500).await;
                copied.set(false);
            });
        })
    };

    let btn_text = if *copied { "✓ Copied" } else { "Copy" };
    let btn_class = if *copied { "code-copy copied" } else { "code-copy" };

    html! {
        <div class="code-block">
            <button class={btn_class} onclick={on_copy}>{ btn_text }</button>
            <pre><code>{ highlight_rust(&props.code) }</code></pre>
        </div>
    }
}
