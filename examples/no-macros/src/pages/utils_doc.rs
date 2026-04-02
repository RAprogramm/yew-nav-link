use crate::demo_popup::DemoBox;
use crate::doc_parser::{parse_doc_block, DocRenderer};
use yew::prelude::*;

const PATH_SRC: &str = include_str!("../../../../src/utils/path.rs");

#[function_component]
pub fn UtilsDoc() -> Html {
    let doc = parse_doc_block(PATH_SRC);

    html! {
        <div>
            <DocRenderer {doc} />

            <div class="card">
                <h3>{ "is_absolute — Live Demo" }</h3>
                <p>{ "Check if a path starts with " }<code>{ "/" }</code>{ ":" }</p>
                <DemoBox>
                    <table class="doc-table">
                        <tbody>
                            <tr><td><code>{ "is_absolute(\"/foo\")" }</code></td><td>{ " → " }<strong>{ "true" }</strong></td></tr>
                            <tr><td><code>{ "is_absolute(\"foo/bar\")" }</code></td><td>{ " → " }<strong style="color:var(--red)">{ "false" }</strong></td></tr>
                            <tr><td><code>{ "is_absolute(\"/\")" }</code></td><td>{ " → " }<strong>{ "true" }</strong></td></tr>
                        </tbody>
                    </table>
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "join_paths — Live Demo" }</h3>
                <p>{ "Join a base path with a child. If child is absolute, it replaces the base:" }</p>
                <DemoBox>
                    <table class="doc-table">
                        <tbody>
                            <tr><td><code>{ "join_paths(\"/base\", \"child\")" }</code></td><td>{ " → " }<code>{ "/base/child" }</code></td></tr>
                            <tr><td><code>{ "join_paths(\"/base\", \"/abs\")" }</code></td><td>{ " → " }<code>{ "/abs" }</code></td></tr>
                        </tbody>
                    </table>
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "normalize_path — Live Demo" }</h3>
                <p>{ "Collapse duplicate slashes and remove trailing " }<code>{ "/" }</code>{ ":" }</p>
                <DemoBox>
                    <table class="doc-table">
                        <tbody>
                            <tr><td><code>{ "normalize_path(\"//foo//bar//\")" }</code></td><td>{ " → " }<code>{ "/foo/bar" }</code></td></tr>
                            <tr><td><code>{ "normalize_path(\"/foo/\")" }</code></td><td>{ " → " }<code>{ "/foo" }</code></td></tr>
                        </tbody>
                    </table>
                </DemoBox>
            </div>
        </div>
    }
}
