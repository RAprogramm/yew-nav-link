use yew::prelude::*;

use crate::doc_page::DocPage;

const SRC: &str = include_str!("../../../src/utils/path.rs");

#[function_component]
pub fn UtilsDoc() -> Html {
    html! {
        <DocPage source={SRC.to_string()}>

            <div class="card">
                <h3>{ "is_absolute" }</h3>
                <p>{ "Check if a path starts with " }<code>{ "/" }</code>{ ":" }</p>
                <table class="doc-table">
                    <thead><tr><th>{ "Call" }</th><th>{ "Result" }</th></tr></thead>
                    <tbody>
                        <tr><td><code>{ "is_absolute(\"/foo\")" }</code></td><td><strong>{ "true" }</strong></td></tr>
                        <tr><td><code>{ "is_absolute(\"foo/bar\")" }</code></td><td><strong style="color:var(--red)">{ "false" }</strong></td></tr>
                        <tr><td><code>{ "is_absolute(\"/\")" }</code></td><td><strong>{ "true" }</strong></td></tr>
                    </tbody>
                </table>
            </div>

            <div class="card">
                <h3>{ "join_paths" }</h3>
                <p>{ "Join a base path with a child. If child is absolute, it replaces the base:" }</p>
                <table class="doc-table">
                    <thead><tr><th>{ "Call" }</th><th>{ "Result" }</th></tr></thead>
                    <tbody>
                        <tr><td><code>{ "join_paths(\"/base\", \"child\")" }</code></td><td><code>{ "/base/child" }</code></td></tr>
                        <tr><td><code>{ "join_paths(\"/base\", \"/abs\")" }</code></td><td><code>{ "/abs" }</code></td></tr>
                    </tbody>
                </table>
            </div>

            <div class="card">
                <h3>{ "normalize_path" }</h3>
                <p>{ "Collapse duplicate slashes and remove trailing " }<code>{ "/" }</code>{ ":" }</p>
                <table class="doc-table">
                    <thead><tr><th>{ "Call" }</th><th>{ "Result" }</th></tr></thead>
                    <tbody>
                        <tr><td><code>{ "normalize_path(\"//foo//bar//\")" }</code></td><td><code>{ "/foo/bar" }</code></td></tr>
                        <tr><td><code>{ "normalize_path(\"/foo/\")" }</code></td><td><code>{ "/foo" }</code></td></tr>
                    </tbody>
                </table>
            </div>

        </DocPage>
    }
}
