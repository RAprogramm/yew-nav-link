//! Parses `//!` doc comments from Rust source files at compile time
//! and renders them as Yew HTML with tables, code blocks, and syntax highlighting.

use yew::prelude::*;

/// A parsed documentation section.
#[derive(Clone, Debug, PartialEq)]
pub struct DocSection {
    pub title: String,
    pub lines: Vec<String>,
}

/// A complete parsed `//!` doc block.
#[derive(Clone, Debug, PartialEq)]
pub struct DocBlock {
    pub title: String,
    pub overview: Vec<String>,
    pub sections: Vec<DocSection>,
}

/// Parses a `//!` doc block from a Rust source string.
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

    for line in &doc_lines {
        if let Some(header) = line.strip_prefix("# ") {
            if let Some(sec) = current_section.take() {
                sections.push(sec);
            }
            if title.is_empty() {
                title = header.to_string();
            }
            current_section = Some(DocSection {
                title: header.to_string(),
                lines: Vec::new(),
            });
        } else {
            match &mut current_section {
                Some(sec) => sec.lines.push(line.clone()),
                None => overview.push(line.clone()),
            }
        }
    }
    if let Some(sec) = current_section {
        sections.push(sec);
    }

    DocBlock {
        title,
        overview,
        sections,
    }
}

/// Renders a [`DocBlock`] as Yew HTML.
#[function_component]
pub fn DocRenderer(props: &DocRendererProps) -> Html {
    let block = &props.doc;

    let overview_html = render_doc_text(&block.overview);

    let sections_html: Html = block
        .sections
        .iter()
        .map(|sec| {
            let title = sec.title.clone();
            let content = match title.as_str() {
                "CSS Classes" | "Props" | "Variants" | "Hooks" | "Functions" | "Types" => {
                    render_table_section(&sec.lines)
                }
                "Example" => render_code_section(&sec.lines),
                _ => render_doc_text(&sec.lines),
            };
            html! {
                <div class="card">
                    <h3>{ &title }</h3>
                    { content }
                </div>
            }
        })
        .collect();

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

#[derive(Properties, PartialEq, Clone)]
pub struct DocRendererProps {
    pub doc: DocBlock,
}

// ── Inline markdown rendering ──────────────────────────────────────────

/// Renders doc lines with inline markdown (backtick code, bold, lists, paragraphs).
fn render_doc_text(lines: &[String]) -> Html {
    let mut blocks: Vec<Vec<&str>> = Vec::new();
    let mut cur: Vec<&str> = Vec::new();

    for line in lines {
        let t = line.trim();
        if t.is_empty() {
            if !cur.is_empty() {
                blocks.push(cur);
                cur = Vec::new();
            }
        } else {
            cur.push(t);
        }
    }
    if !cur.is_empty() {
        blocks.push(cur);
    }

    html! {
        <>
            { for blocks.iter().map(|block| {
                // Check if it's a list block
                if block.iter().all(|l| l.starts_with("- ") || l.starts_with("| ")) {
                    if block[0].starts_with("| ") {
                        // Inline table inside a non-table section
                        return render_inline_table(block);
                    }
                    return html! {
                        <ul>
                            { for block.iter().map(|l| {
                                let text = l.trim_start_matches("- ");
                                html! { <li>{ render_inline(text) }</li> }
                            })}
                        </ul>
                    };
                }
                let joined = block.join(" ");
                html! { <p>{ render_inline(&joined) }</p> }
            })}
        </>
    }
}

/// Renders an inline markdown string (backtick code, bold, plain text).
fn render_inline(text: &str) -> Html {
    let mut parts: Vec<Html> = Vec::new();
    let mut buf = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '`' {
            // Flush plain text
            if !buf.is_empty() {
                parts.push(html! { <>{ buf.clone() }</> });
                buf.clear();
            }
            // Collect code span
            let mut code = String::new();
            for inner in chars.by_ref() {
                if inner == '`' {
                    break;
                }
                code.push(inner);
            }
            let code_clone = code.clone();
            parts.push(html! { <code>{ code_clone }</code> });
        } else if c == '*' && chars.peek() == Some(&'*') {
            chars.next(); // consume second *
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
        } else {
            buf.push(c);
        }
    }
    if !buf.is_empty() {
        parts.push(html! { <>{ buf }</> });
    }

    html! { <>{ for parts }</> }
}

/// Renders a small inline table from lines like `| key | value |`.
fn render_inline_table(lines: &[&str]) -> Html {
    let rows: Vec<Vec<String>> = lines
        .iter()
        .filter(|l| l.contains('|'))
        .filter(|l| !l.contains("---"))
        .map(|l| {
            l.split('|')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().to_string())
                .collect()
        })
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

// ── Table rendering ────────────────────────────────────────────────────

fn render_table_section(lines: &[String]) -> Html {
    let table_lines: Vec<&String> = lines.iter().filter(|l| l.contains('|')).collect();

    if table_lines.len() < 3 {
        return render_doc_text(lines);
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

// ── Code block + syntax highlighting ───────────────────────────────────

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

    html! {
        <pre><code>{ highlight_rust(&code) }</code></pre>
    }
}

/// Simple Rust syntax highlighter — wraps tokens in `<span class="...">`.
fn highlight_rust(code: &str) -> Html {
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

fn tokenize_rust(code: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let bytes = code.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let c = bytes[i] as char;

        // Line comment
        if c == '/' && i + 1 < len && bytes[i + 1] as char == '/' {
            let start = i;
            while i < len && bytes[i] as char != '\n' {
                i += 1;
            }
            out.push(("cm".into(), code[start..i].to_string()));
            continue;
        }

        // String literal
        if c == '"' {
            let start = i;
            i += 1;
            while i < len && bytes[i] as char != '"' {
                if bytes[i] as char == '\\' {
                    i += 1;
                }
                i += 1;
            }
            if i < len {
                i += 1;
            }
            out.push(("str".into(), code[start..i].to_string()));
            continue;
        }

        // Lifetime or char literal
        if c == '\'' {
            let start = i;
            i += 1;
            if i < len && bytes[i].is_ascii_alphabetic() {
                // Could be lifetime 'a or char 'a'
                while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                // If followed by ', it's a char literal
                if i < len && bytes[i] as char == '\'' {
                    i += 1;
                }
            } else if i < len && bytes[i] as char == '\\' {
                i += 1;
                while i < len && bytes[i] as char != '\'' {
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
            }
            out.push(("str".into(), code[start..i].to_string()));
            continue;
        }

        // Numbers
        if c.is_ascii_digit() {
            let start = i;
            while i < len
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'.')
            {
                i += 1;
            }
            out.push(("".into(), code[start..i].to_string()));
            continue;
        }

        // Identifiers / keywords
        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
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

        // Attribute macros
        if c == '#' && i + 1 < len && bytes[i + 1] as char == '[' {
            let start = i;
            i += 1;
            while i < len && bytes[i] as char != ']' {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            out.push(("pr".into(), code[start..i].to_string()));
            continue;
        }

        // Macro invocations (word followed by !)
        if c == '!' && !out.is_empty() {
            // Check if previous token was an identifier
            if let Some(last) = out.last() {
                if last.0.is_empty() || last.0 == "fn" {
                    out.push(("!".into(), "!".to_string()));
                    i += 1;
                    continue;
                }
            }
        }

        // Punctuation
        if "(){}[];,".contains(c) {
            out.push(("pn".into(), c.to_string()));
            i += 1;
            continue;
        }

        // Operators
        if "-><=+&|:".contains(c) {
            let start = i;
            while i < len && "-><=+&|:".contains(bytes[i] as char) {
                i += 1;
            }
            out.push(("".into(), code[start..i].to_string()));
            continue;
        }

        // Whitespace / other
        out.push(("".into(), c.to_string()));
        i += 1;
    }

    out
}
