use yew::prelude::*;
use yew_nav_link::hooks::use_breadcrumbs;

use crate::{
    doc_page::{DemoCard, DocPage, Tip},
    routes::Route
};

const BREADCRUMBS_SRC: &str = include_str!("../../../src/hooks/route_info/mod.rs");

const CODE_CUSTOM_PROVIDER: &str = r#"
use yew_nav_link::hooks::BreadcrumbLabelProvider;

#[derive(Clone)]
struct MyBreadcrumbProvider;

impl BreadcrumbLabelProvider for MyBreadcrumbProvider {
    fn label_for_path(&self, path: &str) -> String {
        match path {
            "/" => "🏠 Home".to_string(),
            "/about" => "📖 About Us".to_string(),
            "/docs" => "📚 Documentation".to_string(),
            other => other.to_string(),
        }
    }
}
"#;

#[function_component]
pub fn CustomBreadcrumbs() -> Html {
    let crumbs = use_breadcrumbs::<Route>();

    html! {
        <DocPage source={BREADCRUMBS_SRC.to_string()}>

            <div class="alert alert-info" style="margin-bottom: 2rem;">
                <strong>{ "New in v0.8.0" }</strong>
                { " — Custom breadcrumb label providers for formatted route names." }
            </div>

            <DemoCard
                title="BreadcrumbLabelProvider Trait"
                description={html!{
                    <>
                        { "Implement " }<code>{ "BreadcrumbLabelProvider" }</code>
                        { " to customize how paths are labeled in breadcrumbs:" }
                    </>
                }}
                code={CODE_CUSTOM_PROVIDER.to_string()}
                tip={html!{
                    <Tip>
                        { "The " }<code>{ "label_for_path" }</code>
                        { " method receives the URL path and returns a display label." }
                    </Tip>
                }}
            >
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
            </DemoCard>

            <DemoCard
                title="Use Cases"
                description={html!{
                    <>
                        { "Common scenarios for custom breadcrumb providers:" }
                    </>
                }}
                code={"// Add icons/emoji\n\"/\" => \"🏠 Home\".to_string()\n\n// Format paths\n\"/users/123\" => \"User #123\".to_string()\n\n// Translate labels\n\"/about\" => \"О нас\".to_string()".to_string()}
                tip={html! { <></> }}
            >
                <ul class="use-cases-list">
                    <li>
                        <strong>{ "🎨 Visual Enhancement" }</strong>
                        { " — Add emoji or icon prefixes to make breadcrumbs more visually distinct" }
                    </li>
                    <li>
                        <strong>{ "📊 Data Formatting" }</strong>
                        { " — Format paths like \"/users/123\" into \"User #123\"" }
                    </li>
                    <li>
                        <strong>{ "🌐 Internationalization" }</strong>
                        { " — Translate path segments based on current locale" }
                    </li>
                    <li>
                        <strong>{ "📦 Dynamic Content" }</strong>
                        { " — Fetch and display related data for paths" }
                    </li>
                </ul>
            </DemoCard>

            <DemoCard
                title="Example Implementation"
                description={html!{
                    <>
                        { "A practical example with emoji and path matching:" }
                    </>
                }}
                code={r#"
#[derive(Clone)]
struct EmojiBreadcrumbProvider;

impl BreadcrumbLabelProvider for EmojiBreadcrumbProvider {
    fn label_for_path(&self, path: &str) -> String {
        match path {
            "/" => "🏠 Homepage".to_string(),
            "/docs" => "📚 Documentation".to_string(),
            "/docs/api" => "📖 API Reference".to_string(),
            "/settings" => "⚙️ Settings".to_string(),
            "/settings/profile" => "👤 Profile".to_string(),
            other => other.to_string(),
        }
    }
}
"#.to_string()}
                tip={html! { <></> }}
            >
                <p>{ "The trait is simple to implement — just match paths and return friendly labels." }</p>
            </DemoCard>
        </DocPage>
    }
}
