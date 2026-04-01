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
                <h3>{ "Live Demo — is_absolute" }</h3>
                <DemoBox>
                    <table>
                        <tr><td><code>{ "is_absolute(\"/foo\")" }</code></td><td>{ " → true" }</td></tr>
                        <tr><td><code>{ "is_absolute(\"foo/bar\")" }</code></td><td>{ " → false" }</td></tr>
                        <tr><td><code>{ "is_absolute(\"/\")" }</code></td><td>{ " → true" }</td></tr>
                    </table>
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "Live Demo — join_paths" }</h3>
                <DemoBox>
                    <table>
                        <tr><td><code>{ "join_paths(\"/base\", \"child\")" }</code></td><td>{ " → /base/child" }</td></tr>
                        <tr><td><code>{ "join_paths(\"/base\", \"/abs\")" }</code></td><td>{ " → /abs" }</td></tr>
                    </table>
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "Live Demo — normalize_path" }</h3>
                <DemoBox>
                    <table>
                        <tr><td><code>{ "normalize_path(\"//foo//bar//\")" }</code></td><td>{ " → /foo/bar" }</td></tr>
                        <tr><td><code>{ "normalize_path(\"/foo/\")" }</code></td><td>{ " → /foo" }</td></tr>
                    </table>
                </DemoBox>
            </div>
        </div>
    }
}
