use yew::prelude::*;

fn code_block(lines: &[&str]) -> Html {
    html! {
        <pre><code>
            { for lines.iter().map(|l| html! { <>{ l }{"\n"}</> }) }
        </code></pre>
    }
}

#[function_component]
pub fn UtilsDoc() -> Html {
    html! {
        <div>
            <div class="card">
                <h2>{ "Path Utilities" }</h2>
                <p>{ "Helpers for working with URL paths." }</p>
            </div>

            <div class="card">
                <h3>{ "is_absolute" }</h3>
                <p>{ "Check if a path starts with " }<code>{ "/" }</code></p>
                <div class="demo">
                    <div class="demo-label">{ "Examples" }</div>
                    <table>
                        <tr><td><code>{ "is_absolute(\"/foo\")" }</code></td><td>{ " → true" }</td></tr>
                        <tr><td><code>{ "is_absolute(\"foo/bar\")" }</code></td><td>{ " → false" }</td></tr>
                        <tr><td><code>{ "is_absolute(\"/\")" }</code></td><td>{ " → true" }</td></tr>
                    </table>
                </div>
                { code_block(&[
                    "use yew_nav_link::is_absolute;",
                    "",
                    "assert!(is_absolute(\"/foo\"));",
                    "assert!(!is_absolute(\"foo/bar\"));",
                ]) }
            </div>

            <div class="card">
                <h3>{ "join_paths" }</h3>
                <p>{ "Join base and relative paths. Absolute child replaces base." }</p>
                <div class="demo">
                    <div class="demo-label">{ "Examples" }</div>
                    <table>
                        <tr><td><code>{ "join_paths(\"/base\", \"child\")" }</code></td><td>{ " → /base/child" }</td></tr>
                        <tr><td><code>{ "join_paths(\"/base\", \"/abs\")" }</code></td><td>{ " → /abs" }</td></tr>
                        <tr><td><code>{ "join_paths(\"/base/\", \"child\")" }</code></td><td>{ " → /base/child" }</td></tr>
                    </table>
                </div>
                { code_block(&[
                    "use yew_nav_link::join_paths;",
                    "",
                    "assert_eq!(join_paths(\"/base\", \"child\"), \"/base/child\");",
                    "assert_eq!(join_paths(\"/base\", \"/abs\"), \"/abs\");",
                ]) }
            </div>

            <div class="card">
                <h3>{ "normalize_path" }</h3>
                <p>{ "Remove double slashes and trailing slashes." }</p>
                <div class="demo">
                    <div class="demo-label">{ "Examples" }</div>
                    <table>
                        <tr><td><code>{ "normalize_path(\"//foo//bar//\")" }</code></td><td>{ " → /foo/bar" }</td></tr>
                        <tr><td><code>{ "normalize_path(\"/foo/\")" }</code></td><td>{ " → /foo" }</td></tr>
                        <tr><td><code>{ "normalize_path(\"/\")" }</code></td><td>{ " → /" }</td></tr>
                    </table>
                </div>
                { code_block(&[
                    "use yew_nav_link::normalize_path;",
                    "",
                    "assert_eq!(normalize_path(\"//foo//bar//\"), \"/foo/bar\");",
                    "assert_eq!(normalize_path(\"/foo/\"), \"/foo\");",
                ]) }
            </div>

            <div class="card">
                <h3>{ "Full API" }</h3>
                <table>
                    <tr><td><code>{ "is_absolute(path: &str) -> bool" }</code></td></tr>
                    <tr><td><code>{ "join_paths(base: &str, child: &str) -> String" }</code></td></tr>
                    <tr><td><code>{ "normalize_path(path: &str) -> String" }</code></td></tr>
                </table>
            </div>
        </div>
    }
}
