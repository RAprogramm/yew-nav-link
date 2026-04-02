use crate::doc_page::{DemoCard, DocPage, Tip};
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::hooks::use_breadcrumbs;

const BREADCRUMBS_SRC: &str = include_str!("../../../../src/breadcrumbs.rs");

const CODE_SIMPLE: &str = "\
use yew_nav_link::hooks::use_breadcrumbs;

# #[derive(Clone, PartialEq, Debug, Routable)]
# enum Route {
#     #[at(\"/\")]
#     Home,
# }
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

# #[derive(Clone, PartialEq, Debug, Routable)]
# enum Route {
#     #[at(\"/\")]
#     Home,
# }
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
    let crumbs = use_breadcrumbs::<Route>();

    html! {
        <DocPage source={BREADCRUMBS_SRC.to_string()}>

            <DemoCard
                title="Quick Start"
                description={html!{ "Minimal example. Copy the code, see it work below:" }}
                code={CODE_SIMPLE.to_string()}
                tip={html!{
                    <Tip>
                        { "Navigate with the sidebar — the trail rebuilds automatically." }
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
                title="Styled with Separators"
                description={html!{
                    <>
                        { "Add arrows between items with " }<code>{ ".enumerate()" }</code>{ ":" }
                    </>
                }}
                code={CODE_STYLED.to_string()}
                tip={html! { <></> }}
            >
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
            </DemoCard>

        </DocPage>
    }
}
