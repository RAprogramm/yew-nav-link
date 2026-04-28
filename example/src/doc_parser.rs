//! Parses `//!` doc comments from Rust source files at compile time
//! and renders them as Yew HTML with tables, code blocks, and syntax
//! highlighting.

use yew::prelude::*;

use crate::code_utils::CopyCode;

// ── Data types ─────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub struct DocSection {
    pub title: String,
    pub lines: Vec<String>
}

#[derive(Clone, Debug, PartialEq)]
pub struct DocBlock {
    pub title:    String,
    pub overview: Vec<String>,
    pub sections: Vec<DocSection>
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

        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            match &mut current_section {
                Some(sec) => sec.lines.push(line.clone()),
                None => overview.push(line.clone())
            }
            continue;
        }

        if !in_code_block {
            if let Some(header) = trimmed.strip_prefix("# ") {
                if is_first_header {
                    is_first_header = false;
                    title = header.to_string();
                    continue;
                }
                if let Some(sec) = current_section.take() {
                    sections.push(sec);
                }
                current_section = Some(DocSection {
                    title: header.to_string(),
                    lines: Vec::new()
                });
                continue;
            }
        }

        match &mut current_section {
            Some(sec) => sec.lines.push(line.clone()),
            None => overview.push(line.clone())
        }
    }
    if let Some(sec) = current_section {
        sections.push(sec);
    }

    DocBlock {
        title,
        overview,
        sections
    }
}

// ── Renderer ───────────────────────────────────────────────────

#[derive(Properties, PartialEq, Clone)]
pub struct DocRendererProps {
    pub doc: DocBlock
}

#[function_component]
pub fn DocRenderer(props: &DocRendererProps) -> Html {
    let block = &props.doc;
    let overview_html = render_content(&block.overview);

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
                _ => render_content(&sec.lines)
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

// ── Content renderer (handles mixed text + code) ───────────────

fn render_content(lines: &[String]) -> Html {
    #[derive(Debug)]
    enum Segment {
        Text(Vec<String>),
        Code(Vec<String>, String)
    }

    let mut segments: Vec<Segment> = Vec::new();
    let mut text_buf: Vec<String> = Vec::new();
    let mut code_buf: Vec<String> = Vec::new();
    let mut in_code = false;
    let mut code_lang = String::new();

    for line in lines {
        let t = line.trim();
        if let Some(lang) = t.strip_prefix("```") {
            if !in_code {
                if !text_buf.is_empty() {
                    segments.push(Segment::Text(text_buf.drain(..).collect()));
                }
                in_code = true;
                code_lang = lang.to_string();
            } else {
                in_code = false;
                segments.push(Segment::Code(
                    code_buf.drain(..).collect(),
                    code_lang.clone()
                ));
                code_lang.clear();
            }
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
                Segment::Code(code_lines, lang) => {
                    let code = code_lines.join("\n");
                    let language = lang.clone();
                    html! { <CopyCode {code} {language} /> }
                }
            })}
        </>
    }
}

// ── Text block (paragraphs + lists + tables) ───────────────────

fn render_text_block(lines: &[String]) -> Html {
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

fn is_numbered_item(line: &str) -> bool {
    let trimmed = line.trim();
    let mut chars = trimmed.chars();
    let first = match chars.next() {
        Some(c) if c.is_ascii_digit() => c,
        _ => return false
    };
    let mut rest = &trimmed[first.len_utf8()..];
    while rest.starts_with(|c: char| c.is_ascii_digit()) {
        rest = &rest[1..];
    }
    rest.starts_with(". ") || rest == "."
}

fn strip_list_prefix(line: &str) -> &str {
    let trimmed = line.trim();
    if let Some(rest) = trimmed.strip_prefix("- ") {
        return rest;
    }
    if let Some(dot_pos) = trimmed.find(". ") {
        let prefix = &trimmed[..dot_pos];
        if prefix.chars().all(|c| c.is_ascii_digit()) {
            return &trimmed[dot_pos + 2..];
        }
    }
    trimmed
}

fn render_single_block(lines: &[&str]) -> Html {
    if lines
        .iter()
        .all(|l| l.contains('|') && !l.starts_with("- "))
    {
        return render_inline_table(lines);
    }

    let has_bullet = lines.iter().any(|l| l.starts_with("- "));
    let has_numbered = lines.iter().any(|l| is_numbered_item(l));

    if has_bullet || has_numbered {
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
        <div class="table-wrapper">
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
        </div>
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
        <div class="table-wrapper">
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
        </div>
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
                if inner == '`' {
                    break;
                }
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
            chars.next();
            let mut code = String::new();
            for inner in chars.by_ref() {
                if inner == '`' {
                    break;
                }
                code.push(inner);
            }
            if chars.peek() == Some(&']') {
                chars.next();
            }
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

// ── Code section renderer ──────────────────────────────────────

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

    html! { <CopyCode {code} /> }
}
