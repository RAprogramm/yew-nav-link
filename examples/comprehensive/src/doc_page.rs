//! Reusable documentation page template.
//!
//! Components for building consistent doc pages across the example app.

use crate::doc_parser::{copy_to_clipboard, highlight_rust, parse_doc_block, strip_hidden_lines, DocRenderer};
use crate::demo_popup::DemoBox;
use yew::prelude::*;

// ── DocPage: auto-parses source + renders docs ──────────────────

#[derive(Properties, PartialEq, Clone)]
pub struct DocPageProps {
    /// Source file content (from `include_str!()`).
    pub source:   String,
    /// Page title override (if empty, uses parsed title).
    #[prop_or_default]
    pub title:    String,
    /// Additional cards rendered after the parsed docs.
    #[prop_or_default]
    pub children: Children,
}

/// Documentation page that auto-parses `//!` doc comments and renders
/// overview, sections, tables, and code blocks. Append custom cards
/// via `children`.
#[function_component]
pub fn DocPage(props: &DocPageProps) -> Html {
    let doc = parse_doc_block(&props.source);

    html! {
        <div>
            <DocRenderer {doc} />
            { for props.children.iter() }
        </div>
    }
}

// ── CopyCode: code block with copy button ───────────────────────

#[derive(Properties, PartialEq, Clone)]
pub struct CopyCodeProps {
    /// Code string to display and copy.
    pub code: String,
}

/// Syntax-highlighted code block with a "Copy" button.
///
/// Lines starting with `# ` (Rust hidden doctest lines) are stripped from
/// the display but included in the copied text for compilability.
#[function_component]
pub fn CopyCode(props: &CopyCodeProps) -> Html {
    let copied = use_state(|| false);

    // Display: strip `# ` prefix and format
    let display_code = strip_hidden_lines(&props.code);

    // Copy: keep full compilable code
    let copy_code = props.code.clone();

    let on_copy = {
        let copied = copied.clone();
        let code = copy_code.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            let code = code.clone();
            let copied = copied.clone();
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
            <pre><code>{ highlight_rust(&display_code) }</code></pre>
        </div>
    }
}

// ── DemoCard: code + live demo in one card ──────────────────────

#[derive(Properties, PartialEq, Clone)]
pub struct DemoCardProps {
    /// Section title.
    pub title: String,
    /// Description shown above the code block.
    pub description: Html,
    /// Code string (rendered with syntax highlighting + copy button).
    pub code: String,
    /// Explanation shown below the live demo.
    #[prop_or_default]
    pub tip: Html,
    /// Live demo content.
    pub children: Children,
}

/// Card with code block + live demo + description.
#[function_component]
pub fn DemoCard(props: &DemoCardProps) -> Html {
    html! {
        <div class="card">
            <h3>{ &props.title }</h3>
            <p>{ props.description.clone() }</p>
            <CopyCode code={props.code.clone()} />
            <h3>{ "Live Result" }</h3>
            <DemoBox>
                { for props.children.iter() }
            </DemoBox>
            { props.tip.clone() }
        </div>
    }
}

// ── Tip helper: muted text with explanation ─────────────────────

/// Returns a `<p>` with muted styling for tips/explanations.
#[function_component]
pub fn Tip(props: &TipProps) -> Html {
    html! {
        <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
            { for props.children.iter() }
        </p>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct TipProps {
    pub children: Children,
}
