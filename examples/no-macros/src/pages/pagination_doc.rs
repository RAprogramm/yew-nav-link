use yew::prelude::*;
use yew_nav_link::components::{PageItem, PageLink, Pagination};

fn code_block(lines: &[&str]) -> Html {
    html! {
        <pre><code>
            { for lines.iter().map(|l| html! { <>{ l }{"\n"}</> }) }
        </code></pre>
    }
}

#[function_component]
pub fn PaginationDoc() -> Html {
    let page = use_state(|| 1u32);

    let on_change = {
        let page = page.clone();
        Callback::from(move |p: u32| page.set(p))
    };

    html! {
        <div>
            <div class="card">
                <h2>{ "Pagination" }</h2>
                <p>{ "Page navigation with prev/next, ellipsis, and first/last buttons." }</p>
            </div>

            <div class="card">
                <h3>{ "Interactive Pagination" }</h3>
                <p>{ "Current page: " }<strong>{ (*page).to_string() }</strong></p>
                <div class="demo">
                    <div class="demo-label">{ "Live Demo — click pages" }</div>
                    <Pagination
                        current_page={*page}
                        total_pages={20}
                        siblings={2}
                        show_prev_next={true}
                        on_page_change={Some(on_change)}
                    />
                </div>

                { code_block(&[
                    "use yew_nav_link::components::Pagination;",
                    "",
                    "let page = use_state(|| 1u32);",
                    "let on_change = {",
                    "    let page = page.clone();",
                    "    Callback::from(move |p: u32| page.set(p))",
                    "};",
                    "",
                    "<Pagination",
                    "    current_page={*page}",
                    "    total_pages={20}",
                    "    siblings={2}",
                    "    show_prev_next={true}",
                    "    on_page_change={Some(on_change)}",
                    "/>",
                ]) }
            </div>

            <div class="card">
                <h3>{ "With First/Last Buttons" }</h3>
                <div class="demo">
                    <div class="demo-label">{ "Live Demo" }</div>
                    <Pagination
                        current_page={5}
                        total_pages={15}
                        show_first_last={true}
                        on_page_change={None}
                    />
                </div>
                { code_block(&[
                    "<Pagination",
                    "    current_page={5}",
                    "    total_pages={15}",
                    "    show_first_last={true}",
                    "    on_page_change={None}",
                    "/>",
                ]) }
            </div>

            <div class="card">
                <h3>{ "PageItem & PageLink" }</h3>
                <p>{ "Build custom pagination manually:" }</p>
                <div class="demo">
                    <div class="demo-label">{ "Live Demo" }</div>
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
                        <PageItem page={0}>
                            <PageLink href={None}>{ "next" }</PageLink>
                        </PageItem>
                    </ul></nav>
                </div>
                { code_block(&[
                    "use yew_nav_link::components::{PageItem, PageLink};",
                    "",
                    "<PageItem page={1} active=true>",
                    "    <PageLink href={Some(\"/page/1\")}>",
                    "        { \"1\" }",
                    "    </PageLink>",
                    "</PageItem>",
                ]) }
            </div>

            <div class="card">
                <h3>{ "Props" }</h3>
                <p><strong>{ "Pagination" }</strong></p>
                <table>
                    <tr><td><code>{ "current_page" }</code></td><td>{ "u32 (default: 1)" }</td></tr>
                    <tr><td><code>{ "total_pages" }</code></td><td>{ "u32 (default: 10)" }</td></tr>
                    <tr><td><code>{ "siblings" }</code></td><td>{ "u32 — pages around current (default: 1)" }</td></tr>
                    <tr><td><code>{ "show_prev_next" }</code></td><td>{ "bool (default: true)" }</td></tr>
                    <tr><td><code>{ "show_first_last" }</code></td><td>{ "bool (default: false)" }</td></tr>
                    <tr><td><code>{ "on_page_change" }</code></td><td>{ "Option<Callback<u32>>" }</td></tr>
                </table>
            </div>
        </div>
    }
}
