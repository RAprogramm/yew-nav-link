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
                <h3>{ "Interactive Pagination — Live Demo" }</h3>
                <p>{ "Click page buttons to navigate. Current: " }<strong>{ (*page).to_string() }</strong></p>
                <DemoBox>
                    <Pagination
                        current_page={*page}
                        total_pages={20}
                        siblings={2}
                        show_prev_next={true}
                        on_page_change={Some(on_change)}
                    />
                </DemoBox>
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "The " }<code>{ "siblings" }</code>
                    { " prop controls how many pages appear on each side of the current page. " }
                    <code>{ "0" }</code>{ " in the vector = ellipsis gap." }
                </p>
            </div>

            <div class="card">
                <h3>{ "First/Last Buttons — Live Demo" }</h3>
                <p>{ "Use " }<code>{ "show_first_last=true" }</code>{ " for quick jumps:" }</p>
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
                <h3>{ "PageItem & PageLink — Live Demo" }</h3>
                <p>{ "Build custom pagination with lower-level components:" }</p>
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
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Set " }<code>{ "active=true" }</code>{ " or " }<code>{ "disabled=true" }</code>
                    { " on " }<code>{ "<PageItem>" }</code>{ ". " }
                    { "Use " }<code>{ "href={None}" }</code>{ " for non-clickable items like ellipsis." }
                </p>
            </div>
        </div>
    }
}
