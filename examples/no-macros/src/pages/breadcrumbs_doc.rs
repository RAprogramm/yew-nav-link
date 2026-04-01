use crate::demo_popup::DemoBox;
use crate::doc_parser::{parse_doc_block, DocRenderer};
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::hooks::use_breadcrumbs;

const HOOKS_SRC: &str = include_str!("../../../../src/hooks/route.rs");

#[function_component]
pub fn BreadcrumbsDoc() -> Html {
    let doc = parse_doc_block(HOOKS_SRC);
    let crumbs = use_breadcrumbs::<Route>();

    html! {
        <div>
            <DocRenderer {doc} />

            <div class="card">
                <h3>{ "Live — Breadcrumbs" }</h3>
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
            </div>
        </div>
    }
}
