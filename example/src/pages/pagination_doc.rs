use yew::prelude::*;
use yew_nav_link::components::{PageItem, PageLink, Pagination};

use crate::doc_page::{DemoCard, DocPage, Tip};

const SRC: &str = include_str!("../../../src/components/pagination.rs");

const CODE_PAGINATION: &str = "\
use yew_nav_link::components::Pagination;

#[component]
fn App() -> Html {
    let page = use_state(|| 1u32);
    let on_change = {
        let page = page.clone();
        Callback::from(move |p: u32| page.set(p))
    };

    html! {
        <Pagination
            current_page={*page}
            total_pages={20}
            siblings={2}
            show_prev_next={true}
            on_page_change={Some(on_change)}
        />
    }
}";

#[function_component]
pub fn PaginationDoc() -> Html {
    let page = use_state(|| 1u32);

    let on_change = {
        let page = page.clone();
        Callback::from(move |p: u32| page.set(p))
    };

    html! {
        <DocPage source={SRC.to_string()}>

            <DemoCard
                title="Interactive Pagination"
                description={html!{
                    <>
                        { "Click page buttons. Current: " }
                        <strong>{ (*page).to_string() }</strong>
                    </>
                }}
                code={CODE_PAGINATION.to_string()}
                tip={html!{
                    <Tip>
                        { "The " }<code>{ "siblings" }</code>
                        { " prop controls pages on each side of current. " }
                        <code>{ "0" }</code>{ " in the vector = ellipsis." }
                    </Tip>
                }}
            >
                <Pagination
                    current_page={*page}
                    total_pages={20}
                    siblings={2}
                    show_prev_next={true}
                    on_page_change={Some(on_change)}
                />
            </DemoCard>

            <DemoCard
                title="PageItem & PageLink"
                description={html!{
                    <>
                        { "Build custom pagination with lower-level components:" }
                    </>
                }}
                code={"use yew_nav_link::components::{PageItem, PageLink};\n\nhtml! {\n  <ul class=\"pagination\">\n    <PageItem page={1} active=true>\n      <PageLink href={Some(\"/page/1\")}>{ \"1\" }</PageLink>\n    </PageItem>\n    <PageItem page={2}>\n      <PageLink href={Some(\"/page/2\")}>{ \"2\" }</PageLink>\n    </PageItem>\n  </ul>\n}".to_string()}
                tip={html!{
                    <Tip>
                        { "Set " }<code>{ "active=true" }</code>{ " or " }<code>{ "disabled=true" }</code>
                        { " on items. Use " }<code>{ "href={None}" }</code>{ " for non-clickable items." }
                    </Tip>
                }}
            >
                <nav><ul class="pagination">
                    <PageItem page={1} disabled=true>
                        <PageLink href={None}>{ "prev" }</PageLink>
                    </PageItem>
                    <PageItem page={1} active=true>
                        <PageLink href={Some("/page/1")}>{ "1" }</PageLink>
                    </PageItem>
                    <PageItem page={2}>
                        <PageLink href={Some("/page/2")}>{ "2" }</PageLink>
                    </PageItem>
                    <PageItem page={3}>
                        <PageLink href={Some("/page/3")}>{ "3" }</PageLink>
                    </PageItem>
                </ul></nav>
            </DemoCard>

        </DocPage>
    }
}
