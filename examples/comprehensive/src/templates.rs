//! Unified documentation templates and metadata system.
//!
//! Provides reusable components and data structures for generating
//! consistent documentation pages across the comprehensive example.
//! Designed for easy parsing and programmatic generation.

use crate::code_utils::CopyCode;
use crate::demo_popup::DemoBox;
use yew::prelude::*;

// ── Metadata Types for Auto-Generation ─────────────────────────

/// Component metadata for documentation auto-generation.
#[derive(Clone, Debug, PartialEq)]
pub struct ComponentMeta {
    /// Component name (e.g., "NavLink", "NavList")
    pub name: String,
    /// Short description for sidebar/cards
    pub description: String,
    /// Version introduced (e.g., "0.1.0", "0.8.0")
    pub since: String,
    /// Feature flags required (if any)
    pub features: Vec<String>,
    /// Related components
    pub related: Vec<String>,
}

impl ComponentMeta {
    pub fn new(name: &str, description: &str, since: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            since: since.to_string(),
            features: Vec::new(),
            related: Vec::new(),
        }
    }

    pub fn with_feature(mut self, feature: &str) -> Self {
        self.features.push(feature.to_string());
        self
    }

    pub fn with_related(mut self, related: &str) -> Self {
        self.related.push(related.to_string());
        self
    }
}

/// Hook metadata for documentation.
#[derive(Clone, Debug)]
pub struct HookMeta {
    pub name: String,
    pub description: String,
    pub since: String,
    pub return_type: String,
}

impl HookMeta {
    pub fn new(name: &str, description: &str, since: &str, return_type: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            since: since.to_string(),
            return_type: return_type.to_string(),
        }
    }
}

/// Code example metadata.
#[derive(Clone, Debug)]
pub struct ExampleMeta {
    pub title: String,
    pub description: String,
    pub code: String,
    pub language: String,
}

impl ExampleMeta {
    pub fn new(title: &str, description: &str, code: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            code: code.to_string(),
            language: "rust".to_string(),
        }
    }

    pub fn with_language(mut self, language: &str) -> Self {
        self.language = language.to_string();
        self
    }
}

// ── Unified DocPage Template ───────────────────────────────────

#[derive(Properties, PartialEq, Clone)]
pub struct DocPageProps {
    /// Source file content (from `include_str!()`).
    pub source: String,
    /// Page title override (if empty, uses parsed title).
    #[prop_or_default]
    pub title: String,
    /// Component metadata for display.
    #[prop_or_default]
    pub meta: Option<ComponentMeta>,
    /// Additional cards rendered after the parsed docs.
    #[prop_or_default]
    pub children: Children,
}

/// Documentation page that auto-parses `//!` doc comments.
#[function_component]
pub fn DocPage(props: &DocPageProps) -> Html {
    use crate::doc_parser::{parse_doc_block, DocRenderer};

    let doc = parse_doc_block(&props.source);

    let header_html = if let Some(meta) = &props.meta {
        html! {
            <DocPageHeader
                title={doc.title.clone()}
                meta={meta.clone()}
            />
        }
    } else {
        html! {
            <div class="card">
                <h2>{ &doc.title }</h2>
            </div>
        }
    };

    html! {
        <div class="doc-page">
            { header_html }
            <DocRenderer {doc} />
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct DocPageHeaderProps {
    title: String,
    meta: ComponentMeta,
}

#[function_component]
fn DocPageHeader(props: &DocPageHeaderProps) -> Html {
    let features_badges: Html = props
        .meta
        .features
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let feature = f.clone();
            html! {
                <Badge variant="warning" key={i}>
                    { feature }
                </Badge>
            }
        })
        .collect();

    let since_text = format!("Since v{}", props.meta.since);

    html! {
        <div class="card doc-page-header">
            <div class="doc-page-header-content">
                <h2 class="doc-page-title">{ &props.title }</h2>
                <p class="doc-page-description">{ &props.meta.description }</p>
                <div class="doc-page-meta">
                    <Badge variant="info">{ since_text }</Badge>
                    { features_badges }
                </div>
            </div>
        </div>
    }
}

// ── Unified DemoCard Template ──────────────────────────────────

#[derive(Properties, PartialEq, Clone)]
pub struct DemoCardProps {
    /// Section title.
    pub title: String,
    /// Description shown above the code block.
    pub description: Html,
    /// Code string (rendered with syntax highlighting + copy button).
    pub code: String,
    /// Code language for syntax highlighting.
    #[prop_or_default]
    pub language: String,
    /// Explanation shown below the live demo.
    #[prop_or_default]
    pub tip: Option<Html>,
    /// Live demo content.
    pub children: Children,
}

/// Card with code block + live demo + description.
#[function_component]
pub fn DemoCard(props: &DemoCardProps) -> Html {
    let tip_html = if let Some(tip) = &props.tip {
        html! { <p class="tip-text">{ tip.clone() }</p> }
    } else {
        html! {}
    };

    html! {
        <div class="card demo-card">
            <h3>{ &props.title }</h3>
            <p>{ props.description.clone() }</p>
            <CopyCode code={props.code.clone()} language={props.language.clone()} />
            <h4 class="demo-label">{ "Live Demo" }</h4>
            <DemoBox>
                { for props.children.iter() }
            </DemoBox>
            { tip_html }
        </div>
    }
}

// ── Badge Component ────────────────────────────────────────────

#[derive(Properties, PartialEq, Clone)]
pub struct BadgeProps {
    pub children: Html,
    #[prop_or_default]
    pub variant: String,
}

#[function_component]
pub fn Badge(props: &BadgeProps) -> Html {
    let class = if props.variant.is_empty() {
        "badge".to_string()
    } else {
        format!("badge badge-{}", props.variant)
    };
    html! {
        <span class={class}>
            { props.children.clone() }
        </span>
    }
}

// ── Tip Helper ─────────────────────────────────────────────────

#[derive(Properties, PartialEq, Clone)]
pub struct TipProps {
    pub children: Children,
}

/// Returns a `<p>` with muted styling for tips/explanations.
#[function_component]
pub fn Tip(props: &TipProps) -> Html {
    html! {
        <p class="tip-text">
            { for props.children.iter() }
        </p>
    }
}

// ── InfoBox Component ──────────────────────────────────────────

#[derive(Properties, PartialEq, Clone)]
pub struct InfoBoxProps {
    pub children: Html,
    #[prop_or_default]
    pub title: Option<String>,
}

#[function_component]
pub fn InfoBox(props: &InfoBoxProps) -> Html {
    let title_html = if let Some(title) = &props.title {
        html! { <strong>{ title.clone() }{" "}</strong> }
    } else {
        html! {}
    };

    html! {
        <div class="info-box">
            { title_html }
            { props.children.clone() }
        </div>
    }
}

// ── ApiTable Component ─────────────────────────────────────────

#[derive(Properties, PartialEq, Clone)]
pub struct ApiTableProps {
    pub title: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[function_component]
pub fn ApiTable(props: &ApiTableProps) -> Html {
    let headers: Html = props
        .columns
        .iter()
        .map(|h| html! { <th>{ h.clone() }</th> })
        .collect();

    let rows_html: Html = props
        .rows
        .iter()
        .map(|row| {
            let cells: Html = row
                .iter()
                .map(|cell| html! { <td>{ cell.clone() }</td> })
                .collect();
            html! { <tr>{ cells }</tr> }
        })
        .collect();

    html! {
        <div class="card">
            <h3>{ &props.title }</h3>
            <div class="table-wrapper">
                <table class="doc-table">
                    <thead>
                        <tr>{ headers }</tr>
                    </thead>
                    <tbody>
                        { rows_html }
                    </tbody>
                </table>
            </div>
        </div>
    }
}

// ── Deployment Badge Component ─────────────────────────────────

#[derive(Properties, PartialEq, Clone)]
pub struct DeploymentBadgeProps {
    /// Repository owner/name (e.g., "yew-nav-link/yew-nav-link")
    #[prop_or_default]
    pub repo: String,
    /// Branch or tag for deployment
    #[prop_or_default]
    pub branch: String,
    /// Deployment status (for future use)
    #[prop_or_default]
    pub status: DeploymentStatus,
}

#[derive(Clone, PartialEq, Default)]
pub enum DeploymentStatus {
    #[default]
    Success,
    Building,
    Failed,
}

impl DeploymentStatus {
    fn color(&self) -> &'static str {
        match self {
            DeploymentStatus::Success => "var(--green)",
            DeploymentStatus::Building => "var(--yellow)",
            DeploymentStatus::Failed => "var(--red)",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            DeploymentStatus::Success => "Deployed",
            DeploymentStatus::Building => "Building...",
            DeploymentStatus::Failed => "Failed",
        }
    }
}

#[function_component]
pub fn DeploymentBadge(props: &DeploymentBadgeProps) -> Html {
    let gh_pages_url = format!("https://{}/docs", props.repo);

    let status_color = props.status.color().to_string();

    html! {
        <a
            href={gh_pages_url}
            target="_blank"
            rel="noopener noreferrer"
            class="deployment-badge"
            title={format!("View deployed documentation")}
        >
            <span class="deployment-badge-dot" style={format!("background-color: {}", status_color)} />
            <span class="deployment-badge-text">{ props.status.label() }</span>
            <svg class="deployment-badge-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                <polyline points="15 3 21 3 21 9"></polyline>
                <line x1="10" y1="14" x2="21" y2="3"></line>
            </svg>
        </a>
    }
}

// ── Version Badge Component ────────────────────────────────────

#[derive(Properties, PartialEq, Clone)]
pub struct VersionBadgeProps {
    pub version: String,
    #[prop_or_default]
    pub latest: bool,
}

#[function_component]
pub fn VersionBadge(props: &VersionBadgeProps) -> Html {
    let variant = if props.latest { "success" } else { "info" };

    html! {
        <Badge variant={variant.to_string()}>
            { format!("v{}", props.version) }
        </Badge>
    }
}
