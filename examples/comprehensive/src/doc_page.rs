//! Reusable documentation page template.
//!
//! Components for building consistent doc pages across the example app.

use crate::demo_popup::DemoBox;
use crate::doc_parser::{parse_doc_block, DocRenderer};
use yew::prelude::*;

/// Re-exported from `code_utils` for backwards compatibility.
pub use crate::code_utils::CopyCode;

// ── DocPage: auto-parses source + renders docs ──────────────────

#[derive(Properties, PartialEq, Clone)]
pub struct DocPageProps {
    /// Source file content (from `include_str!()`).
    pub source: String,
    /// Page title override (if empty, uses parsed title).
    #[prop_or_default]
    pub title: String,
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
        <p class="tip-text">
            { for props.children.iter() }
        </p>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct TipProps {
    pub children: Children,
}
