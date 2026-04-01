use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::hooks::use_breadcrumbs;

fn code_block(lines: &[&str]) -> Html {
    html! {
        <pre><code>
            { for lines.iter().map(|l| html! { <>{ l }{"\n"}</> }) }
        </code></pre>
    }
}

#[function_component]
pub fn BreadcrumbsDoc() -> Html {
    let crumbs = use_breadcrumbs::<Route>();

    html! {
        <div>
            <div class="card">
                <h2>{ "Breadcrumbs" }</h2>
                <p>{ "Auto-generated breadcrumb trail from the current route." }</p>
            </div>

            <div class="card">
                <h3>{ "Live Breadcrumbs" }</h3>
                <div class="demo">
                    <div class="demo-label">{ "Live — based on current route" }</div>
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
                </div>

                { code_block(&[
                    "use yew_nav_link::hooks::use_breadcrumbs;",
                    "",
                    "let crumbs = use_breadcrumbs::<Route>();",
                    "",
                    "html! {",
                    "    <nav aria-label=\"breadcrumb\">",
                    "        <ol>",
                    "            { for crumbs.iter().map(|crumb| {",
                    "                html! {",
                    "                    <li class={if crumb.is_active {",
                    "                        \"active\"",
                    "                    } else { \"\" }}>",
                    "                        { &crumb.label }",
                    "                    </li>",
                    "                }",
                    "            })}",
                    "        </ol>",
                    "    </nav>",
                    "}",
                ]) }
            </div>

            <div class="card">
                <h3>{ "BreadcrumbItem Fields" }</h3>
                <table>
                    <tr><td><code>{ "route" }</code></td><td>{ "The route value" }</td></tr>
                    <tr><td><code>{ "label" }</code></td><td>{ "Display text (path)" }</td></tr>
                    <tr><td><code>{ "is_active" }</code></td><td>{ "bool — whether this is the current route" }</td></tr>
                </table>
            </div>
        </div>
    }
}
