//! Parses `//!` doc comments from Rust source files at compile time
//! and renders them as Yew HTML with tables, code blocks, and syntax highlighting.

use yew::prelude::*;

// ── Data types ─────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub struct DocSection {
    pub title: String,
    pub lines: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DocBlock {
    pub title:    String,
    pub overview: Vec<String>,
    pub sections: Vec<DocSection>,
}

// ── Parser ─────────────────────────────────────────────────────

pub fn parse_doc_block(source: &str) -> DocBlock {
    let doc_lines: Vec<String> = source
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("//!") {
                let content = rest.strip_prefix(' ').unwrap_or(rest);
                Some(content.to_string())
            } else {
                None
            }
        })
        .collect();

    let mut title = String::new();
    let mut overview: Vec<String> = Vec::new();
    let mut sections: Vec<DocSection> = Vec::new();
    let mut current_section: Option<DocSection> = None;
    let mut in_code_block = false;
    let mut is_first_header = true;

    for line in &doc_lines {
        let trimmed = line.trim();

        // Track code fences — don't parse headers inside them
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            match &mut current_section {
                Some(sec) => sec.lines.push(line.clone()),
                None => overview.push(line.clone()),
            }
            continue;
        }

        // Only treat `# ` as header OUTSIDE code blocks
        if !in_code_block {
            if let Some(header) = trimmed.strip_prefix("# ") {
                // First header → just set title, keep content in overview
                if is_first_header {
                    is_first_header = false;
                    title = header.to_string();
                    continue;
                }
                // Subsequent headers → start new section
                if let Some(sec) = current_section.take() {
                    sections.push(sec);
                }
                current_section = Some(DocSection {
                    title: header.to_string(),
                    lines: Vec::new(),
                });
                continue;
            }
        }

        match &mut current_section {
            Some(sec) => sec.lines.push(line.clone()),
            None => overview.push(line.clone()),
        }
    }
    if let Some(sec) = current_section {
        sections.push(sec);
    }

    DocBlock { title, overview, sections }
}

// ── Renderer ───────────────────────────────────────────────────

#[derive(Properties, PartialEq, Clone)]
pub struct DocRendererProps {
    pub doc: DocBlock,
}

#[function_component]
pub fn DocRenderer(props: &DocRendererProps) -> Html {
    let block = &props.doc;
    let overview_html = render_content(&block.overview);

    let sections_html: Html = block.sections.iter().map(|sec| {
        let title = sec.title.clone();
        let content = match title.as_str() {
            "CSS Classes" | "Props" | "Variants" | "Hooks" | "Functions" | "Types" => {
                render_table_section(&sec.lines)
            }
            "Example" => render_code_section(&sec.lines),
            _ => render_content(&sec.lines),
        };
        html! {
            <div class="card">
                <h3>{ &title }</h3>
                { content }
            </div>
        }
    }).collect();

    html! {
        <div>
            <div class="card">
                <h2>{ &block.title }</h2>
                { overview_html }
            </div>
            { sections_html }
        </div>
    }
}

// ── Content renderer (handles mixed text + code) ───────────────

/// Renders lines that may contain text, lists, AND code blocks.
/// Splits on code fences and renders each segment appropriately.
fn render_content(lines: &[String]) -> Html {
    // Split into segments: Text(Vec<String>) | Code(Vec<String>)
    #[derive(Debug)]
    enum Segment {
        Text(Vec<String>),
        Code(Vec<String>),
    }

    let mut segments: Vec<Segment> = Vec::new();
    let mut text_buf: Vec<String> = Vec::new();
    let mut code_buf: Vec<String> = Vec::new();
    let mut in_code = false;

    for line in lines {
        let t = line.trim();
        if t == "```rust" || t == "```ignore" || t == "```toml" || t == "```html" {
            if !text_buf.is_empty() {
                segments.push(Segment::Text(text_buf.drain(..).collect()));
            }
            in_code = true;
        } else if t == "```" && in_code {
            in_code = false;
            segments.push(Segment::Code(code_buf.drain(..).collect()));
        } else if in_code {
            code_buf.push(line.clone());
        } else {
            text_buf.push(line.clone());
        }
    }
    if !text_buf.is_empty() {
        segments.push(Segment::Text(text_buf));
    }

    html! {
        <>
            { for segments.iter().map(|seg| match seg {
                Segment::Text(lines) => render_text_block(lines),
                Segment::Code(code_lines) => {
                    let code = code_lines.join("\n");
                    html! { <CodeBlock {code} /> }
                }
            })}
        </>
    }
}

// ── Code block with copy button ────────────────────────────────

#[derive(Properties, PartialEq, Clone)]
struct CodeBlockProps {
    code: String,
}

/// Strips `# ` prefix and cleans formatting for display.
pub fn strip_hidden_lines(code: &str) -> String {
    let stripped: Vec<&str> = code
        .lines()
        .map(|line| {
            if let Some(rest) = line.strip_prefix("# ") {
                rest
            } else {
                line
            }
        })
        .collect();

    // Add blank line before top-level attrs that follow non-empty lines
    let mut result: Vec<String> = Vec::new();
    for (i, line) in stripped.iter().enumerate() {
        let trimmed = line.trim();
        let prev = if i > 0 { stripped[i - 1].trim() } else { "" };

        // Only add blank line before #[derive or #[component at column 0
        // (not #[at( inside enums which are indented)
        if !prev.is_empty()
            && !line.starts_with(" ")
            && !line.starts_with("\t")
            && (trimmed.starts_with("#[derive") || trimmed.starts_with("#[component") || trimmed.starts_with("#[function_component"))
        {
            result.push(String::new());
        }

        result.push(line.to_string());
    }

    result.join("\n")
}

#[function_component]
fn CodeBlock(props: &CodeBlockProps) -> Html {
    let copied = use_state(|| false);
    let display_code = strip_hidden_lines(&props.code);
    let copy_code = props.code.clone();

    let on_copy = {
        let copied = copied.clone();
        let code = copy_code.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            let code = code.clone();
            let copied = copied.clone();
            wasm_bindgen_futures::spawn_local(async move {
                copy_to_clipboard(&code);
                copied.set(true);
                gloo_timers::future::TimeoutFuture::new(1500).await;
                copied.set(false);
            });
        })
    };

    let btn_text = if *copied { "✓ Copied" } else { "Copy" };
    let btn_class = if *copied { "code-copy copied" } else { "code-copy" };

    html! {
        <div class="code-block">
            <button class={btn_class} onclick={on_copy}>{ btn_text }</button>
            <pre><code>{ highlight_rust(&display_code) }</code></pre>
        </div>
    }
}

pub fn copy_to_clipboard(text: &str) {
    let Some(window) = web_sys::window() else { return };
    let navigator = window.navigator();
    let clipboard = navigator.clipboard();
    let promise = clipboard.write_text(text);
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = wasm_bindgen_futures::JsFuture::from(promise).await {
            web_sys::console::error_1(&e);
        }
    });
}

// ── Text block (paragraphs + lists + tables) ───────────────────

/// Renders text lines handling mixed paragraphs, lists, and inline tables.
/// - Lines starting with `- ` begin a new list item
/// - Continuation lines (no `- `) are appended to the previous item
/// - Empty lines separate paragraphs / list blocks
fn render_text_block(lines: &[String]) -> Html {
    // First pass: split into logical blocks by empty lines
    let mut raw_blocks: Vec<Vec<&str>> = Vec::new();
    let mut cur: Vec<&str> = Vec::new();
    for line in lines {
        let t = line.trim();
        if t.is_empty() {
            if !cur.is_empty() {
                raw_blocks.push(cur);
                cur = Vec::new();
            }
        } else {
            cur.push(t);
        }
    }
    if !cur.is_empty() {
        raw_blocks.push(cur);
    }

    html! {
        <>
            { for raw_blocks.iter().map(|block| render_single_block(block))}
        </>
    }
}

/// Checks if a line starts a numbered list item like "1. " or "23. "
fn is_numbered_item(line: &str) -> bool {
    let trimmed = line.trim();
    let mut chars = trimmed.chars();
    // Must start with digit
    let first = match chars.next() {
        Some(c) if c.is_ascii_digit() => c,
        _ => return false,
    };
    // Consume all digits
    let mut rest = &trimmed[first.len_utf8()..];
    while rest.starts_with(|c: char| c.is_ascii_digit()) {
        rest = &rest[1..];
    }
    // Must be followed by ". "
    rest.starts_with(". ") || rest == "."
}

/// Extracts the text after "1. " or "- " prefix.
fn strip_list_prefix(line: &str) -> &str {
    let trimmed = line.trim();
    if let Some(rest) = trimmed.strip_prefix("- ") {
        return rest;
    }
    // Numbered: find ". " and skip it
    if let Some(dot_pos) = trimmed.find(". ") {
        let prefix = &trimmed[..dot_pos];
        if prefix.chars().all(|c| c.is_ascii_digit()) {
            return &trimmed[dot_pos + 2..];
        }
    }
    trimmed
}

/// Renders one block (no empty lines inside) — may be a paragraph, list, or table.
fn render_single_block(lines: &[&str]) -> Html {
    // Table block: all lines contain |
    if lines.iter().all(|l| l.contains('|') && !l.starts_with("- ")) {
        return render_inline_table(lines);
    }

    // Check if any line starts with "- " or "1. " → it's a list
    let has_bullet = lines.iter().any(|l| l.starts_with("- "));
    let has_numbered = lines.iter().any(|l| is_numbered_item(l));

    if has_bullet || has_numbered {
        // Merge continuation lines into list items
        let mut items: Vec<String> = Vec::new();
        let mut is_numbered = false;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("- ") {
                items.push(trimmed.trim_start_matches("- ").to_string());
            } else if is_numbered_item(trimmed) {
                is_numbered = true;
                items.push(strip_list_prefix(trimmed).to_string());
            } else if let Some(last) = items.last_mut() {
                last.push(' ');
                last.push_str(trimmed);
            } else {
                items.insert(0, format!("__PARA__{}", trimmed));
            }
        }

        let mut para_buf: Vec<String> = Vec::new();
        let mut list_items: Vec<String> = Vec::new();
        let mut result: Vec<Html> = Vec::new();

        for item in items {
            if let Some(para) = item.strip_prefix("__PARA__") {
                para_buf.push(para.to_string());
            } else {
                if !para_buf.is_empty() {
                    let text = para_buf.join(" ");
                    result.push(html! { <p>{ render_inline(&text) }</p> });
                    para_buf.clear();
                }
                list_items.push(item);
            }
        }
        if !para_buf.is_empty() {
            let text = para_buf.join(" ");
            result.push(html! { <p>{ render_inline(&text) }</p> });
        }
        if !list_items.is_empty() {
            if is_numbered {
                result.push(html! {
                    <ol>
                        { for list_items.iter().map(|item| {
                            html! { <li>{ render_inline(item) }</li> }
                        })}
                    </ol>
                });
            } else {
                result.push(html! {
                    <ul>
                        { for list_items.iter().map(|item| {
                            html! { <li>{ render_inline(item) }</li> }
                        })}
                    </ul>
                });
            }
        }

        return html! { <>{ for result }</> };
    }

    // Plain paragraph: join all lines
    let joined = lines.join(" ");
    html! { <p>{ render_inline(&joined) }</p> }
}

// ── Table rendering ────────────────────────────────────────────

fn render_table_section(lines: &[String]) -> Html {
    let table_lines: Vec<&String> = lines.iter().filter(|l| l.contains('|')).collect();

    if table_lines.len() < 3 {
        return render_content(lines);
    }

    let header = parse_table_row(table_lines[0]);
    let rows: Vec<Vec<String>> = table_lines[1..]
        .iter()
        .filter(|l| !l.contains("---"))
        .map(|l| parse_table_row(l))
        .collect();

    html! {
        <table class="doc-table">
            <thead>
                <tr>
                    { for header.iter().map(|h| html! { <th>{ render_inline(h.trim()) }</th> }) }
                </tr>
            </thead>
            <tbody>
                { for rows.iter().map(|row| {
                    html! {
                        <tr>
                            { for row.iter().map(|cell| html! { <td>{ render_inline(cell.trim()) }</td> })}
                        </tr>
                    }
                })}
            </tbody>
        </table>
    }
}

fn parse_table_row(line: &str) -> Vec<String> {
    line.split('|')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect()
}

fn render_inline_table(lines: &[&str]) -> Html {
    let rows: Vec<Vec<String>> = lines
        .iter()
        .filter(|l| l.contains('|'))
        .filter(|l| !l.contains("---"))
        .map(|l| l.split('|').filter(|s| !s.is_empty()).map(|s| s.trim().to_string()).collect())
        .collect();

    if rows.is_empty() {
        return html! {};
    }

    html! {
        <table class="doc-table">
            <tbody>
                { for rows.iter().map(|row| {
                    html! {
                        <tr>
                            { for row.iter().map(|cell| html! { <td>{ render_inline(cell) }</td> })}
                        </tr>
                    }
                })}
            </tbody>
        </table>
    }
}

// ── Inline markdown ────────────────────────────────────────────

fn render_inline(text: &str) -> Html {
    let mut parts: Vec<Html> = Vec::new();
    let mut buf = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '`' {
            if !buf.is_empty() {
                parts.push(html! { <>{ buf.clone() }</> });
                buf.clear();
            }
            let mut code = String::new();
            for inner in chars.by_ref() {
                if inner == '`' { break; }
                code.push(inner);
            }
            let code_clone = code.clone();
            parts.push(html! { <code>{ code_clone }</code> });
        } else if c == '*' && chars.peek() == Some(&'*') {
            chars.next();
            if !buf.is_empty() {
                parts.push(html! { <>{ buf.clone() }</> });
                buf.clear();
            }
            let mut bold = String::new();
            while let Some(inner) = chars.next() {
                if inner == '*' && chars.peek() == Some(&'*') {
                    chars.next();
                    break;
                }
                bold.push(inner);
            }
            let bold_clone = bold.clone();
            parts.push(html! { <strong>{ bold_clone }</strong> });
        } else if c == '[' && chars.peek() == Some(&'`') {
            // Handle [`Type`] links → just render as code
            chars.next(); // skip `
            let mut code = String::new();
            for inner in chars.by_ref() {
                if inner == '`' { break; }
                code.push(inner);
            }
            // Skip ]
            if chars.peek() == Some(&']') { chars.next(); }
            let code_clone = code.clone();
            parts.push(html! { <code>{ code_clone }</code> });
        } else {
            buf.push(c);
        }
    }
    if !buf.is_empty() {
        parts.push(html! { <>{ buf }</> });
    }

    html! { <>{ for parts }</> }
}

// ── Legacy code section (for standalone Example sections) ──────

fn render_code_section(lines: &[String]) -> Html {
    let code: String = lines
        .iter()
        .skip_while(|l| l.trim() != "```rust" && l.trim() != "```ignore")
        .skip(1)
        .take_while(|l| l.trim() != "```")
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    if code.is_empty() {
        return html! {};
    }

    html! { <CodeBlock {code} /> }
}

// ── Syntax highlighting ────────────────────────────────────────

pub fn highlight_rust(code: &str) -> Html {
    let tokens = tokenize_rust(code);
    html! {
        <>
            { for tokens.iter().map(|(cls, txt)| {
                if cls.is_empty() {
                    html! { <>{ txt }</> }
                } else {
                    let class = cls.clone();
                    let text = txt.clone();
                    html! { <span {class}>{ text }</span> }
                }
            })}
        </>
    }
}

pub fn tokenize_rust(code: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let bytes = code.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let c = bytes[i] as char;

        if c == '/' && i + 1 < len && bytes[i + 1] as char == '/' {
            let start = i;
            while i < len && bytes[i] as char != '\n' { i += 1; }
            out.push(("cm".into(), code[start..i].to_string()));
            continue;
        }

        if c == '"' {
            let start = i;
            i += 1;
            while i < len && bytes[i] as char != '"' {
                if bytes[i] as char == '\\' { i += 1; }
                i += 1;
            }
            if i < len { i += 1; }
            out.push(("str".into(), code[start..i].to_string()));
            continue;
        }

        if c == '\'' {
            let start = i;
            i += 1;
            if i < len && bytes[i].is_ascii_alphabetic() {
                while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') { i += 1; }
                if i < len && bytes[i] as char == '\'' { i += 1; }
            } else if i < len && bytes[i] as char == '\\' {
                i += 1;
                while i < len && bytes[i] as char != '\'' { i += 1; }
                if i < len { i += 1; }
            }
            out.push(("str".into(), code[start..i].to_string()));
            continue;
        }

        if c.is_ascii_digit() {
            let start = i;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'.') { i += 1; }
            out.push(("".into(), code[start..i].to_string()));
            continue;
        }

        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') { i += 1; }
            let word = &code[start..i];

            let cls = match word {
                "fn" | "let" | "mut" | "pub" | "use" | "mod" | "struct" | "enum" | "impl"
                | "trait" | "type" | "const" | "static" | "if" | "else" | "match" | "return"
                | "for" | "while" | "loop" | "break" | "continue" | "move" | "ref" | "self"
                | "super" | "crate" | "where" | "async" | "await" | "dyn" | "as" | "in"
                | "unsafe" | "extern" | "true" | "false" => "kw",

                "String" | "Vec" | "Option" | "Result" | "bool" | "u8" | "u16" | "u32" | "u64"
                | "usize" | "i8" | "i16" | "i32" | "i64" | "isize" | "f32" | "f64" | "str"
                | "char" | "Self" => "ty",

                _ if word.starts_with(char::is_uppercase) && !word.contains('_') => "ty",
                _ if i < len && bytes[i] as char == '(' => "fn",
                _ => "",
            };

            out.push((cls.into(), word.to_string()));
            continue;
        }

        if c == '#' && i + 1 < len && bytes[i + 1] as char == '[' {
            let start = i;
            i += 1;
            while i < len && bytes[i] as char != ']' { i += 1; }
            if i < len { i += 1; }
            out.push(("pr".into(), code[start..i].to_string()));
            continue;
        }

        if c == '!' && !out.is_empty() {
            if let Some(last) = out.last() {
                if last.0.is_empty() || last.0 == "fn" {
                    out.push(("!".into(), "!".to_string()));
                    i += 1;
                    continue;
                }
            }
        }

        if "(){}[];,".contains(c) {
            out.push(("pn".into(), c.to_string()));
            i += 1;
            continue;
        }

        if "-><=+&|:".contains(c) {
            let start = i;
            while i < len && "-><=+&|:".contains(bytes[i] as char) { i += 1; }
            out.push(("".into(), code[start..i].to_string()));
            continue;
        }

        out.push(("".into(), c.to_string()));
        i += 1;
    }

    out
}
