use crate::demo_popup::DemoBox;
use crate::doc_parser::{parse_doc_block, DocRenderer};
use yew::prelude::*;
use yew_nav_link::components::{PageItem, PageLink, Pagination};

const PAGINATION_SRC: &str = include_str!("../../../../src/components/pagination.rs");

#[function_component]
pub fn PaginationDoc() -> Html {
    let doc = parse_doc_block(PAGINATION_SRC);
    let page = use_state(|| 1u32);

    let on_change = {
        let page = page.clone();
        Callback::from(move |p: u32| page.set(p))
    };

    html! {
        <div>
            <DocRenderer {doc} />

            <div class="card">
                <h3>{ "Live Demo — Interactive Pagination" }</h3>
                <p>{ "Current page: " }<strong>{ (*page).to_string() }</strong></p>
                <DemoBox>
                    <Pagination
                        current_page={*page}
                        total_pages={20}
                        siblings={2}
                        show_prev_next={true}
                        on_page_change={Some(on_change)}
                    />
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "Live Demo — First/Last Buttons" }</h3>
                <DemoBox>
                    <Pagination
                        current_page={5}
                        total_pages={15}
                        show_first_last={true}
                        on_page_change={None}
                    />
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "Live Demo — PageItem & PageLink" }</h3>
                <DemoBox>
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
                </DemoBox>
            </div>
        </div>
    }
}
