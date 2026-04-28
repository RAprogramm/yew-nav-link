//! Shared code rendering utilities.
//!
//! Provides syntax highlighting, code formatting, and clipboard copy
//! functionality used by both the doc parser and demo pages.

use yew::prelude::*;

/// Duration (ms) for the "Copied" feedback badge.
pub const COPIED_FEEDBACK_MS: u32 = 1500;

// ── Clipboard ──────────────────────────────────────────────────

/// Copies text to the system clipboard.
///
/// Spawns an async task; errors are logged to the browser console.
/// Safe to call from synchronous contexts.
pub fn copy_to_clipboard(text: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let navigator = window.navigator();
    let clipboard = navigator.clipboard();
    let promise = clipboard.write_text(text);
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = wasm_bindgen_futures::JsFuture::from(promise).await {
            web_sys::console::error_1(&e);
        }
    });
}

// ── Code formatting ────────────────────────────────────────────

/// Strips `# ` prefix (Rust doctest hidden lines) and adds spacing
/// before top-level attributes for readability.
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

    let mut result: Vec<String> = Vec::new();
    for (i, line) in stripped.iter().enumerate() {
        let trimmed = line.trim();
        let prev = if i > 0 { stripped[i - 1].trim() } else { "" };

        if !prev.is_empty()
            && !line.starts_with(' ')
            && !line.starts_with('\t')
            && (trimmed.starts_with("#[derive")
                || trimmed.starts_with("#[component")
                || trimmed.starts_with("#[function_component"))
        {
            result.push(String::new());
        }

        result.push(line.to_string());
    }

    result.join("\n")
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
                    let class = (*cls).to_string();
                    let text = txt.clone();
                    html! { <span {class}>{ text }</span> }
                }
            })}
        </>
    }
}

pub fn highlight_css(code: &str) -> Html {
    let tokens = tokenize_css(code);
    html! {
        <>
            { for tokens.iter().map(|(cls, txt)| {
                if cls.is_empty() {
                    html! { <>{ txt }</> }
                } else {
                    let class = (*cls).to_string();
                    let text = txt.clone();
                    html! { <span {class}>{ text }</span> }
                }
            })}
        </>
    }
}

pub fn tokenize_css(code: &str) -> Vec<(&'static str, String)> {
    let mut out: Vec<(&'static str, String)> = Vec::new();
    let bytes = code.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let c = bytes[i] as char;

        if c == '/' && i + 1 < len && bytes[i + 1] as char == '*' {
            let start = i;
            i += 2;
            while i + 1 < len && !(bytes[i] as char == '*' && bytes[i + 1] as char == '/') {
                i += 1;
            }
            if i + 1 < len {
                i += 2;
            }
            out.push(("cm", code[start..i].to_string()));
            continue;
        }

        if c == '.' || c == '#' {
            let start = i;
            i += 1;
            while i < len
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'-')
            {
                i += 1;
            }
            out.push(("sl", code[start..i].to_string()));
            continue;
        }

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
            out.push(("str", code[start..i].to_string()));
            continue;
        }

        if c.is_ascii_digit() {
            let start = i;
            while i < len
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'.' || bytes[i] == b'%')
            {
                i += 1;
            }
            out.push(("num", code[start..i].to_string()));
            continue;
        }

        if c == ':' && (i == 0 || bytes[i - 1] as char != ':') {
            let start = i;
            i += 1;
            while i < len && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            let val_start = i;
            while i < len && bytes[i] as char != ';' && bytes[i] as char != '}' {
                i += 1;
            }
            if i > val_start {
                out.push(("", code[start..val_start].to_string()));
                out.push(("pr", code[val_start..i].to_string()));
            } else {
                out.push(("", code[start..i].to_string()));
            }
            continue;
        }

        if c == '{' || c == '}' || c == ';' {
            out.push(("pn", c.to_string()));
            i += 1;
            continue;
        }

        out.push(("", c.to_string()));
        i += 1;
    }

    out
}

pub fn tokenize_rust(code: &str) -> Vec<(&'static str, String)> {
    let mut out: Vec<(&'static str, String)> = Vec::new();
    let bytes = code.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let c = bytes[i] as char;

        if c == '/' && i + 1 < len && bytes[i + 1] as char == '/' {
            let start = i;
            while i < len && bytes[i] as char != '\n' {
                i += 1;
            }
            out.push(("cm", code[start..i].to_string()));
            continue;
        }

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
            out.push(("str", code[start..i].to_string()));
            continue;
        }

        if c == '\'' {
            let start = i;
            i += 1;
            if i < len && bytes[i].is_ascii_alphabetic() {
                while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
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
            out.push(("str", code[start..i].to_string()));
            continue;
        }

        if c.is_ascii_digit() {
            let start = i;
            while i < len
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'.')
            {
                i += 1;
            }
            out.push(("", code[start..i].to_string()));
            continue;
        }

        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            let word = &code[start..i];

            let cls: &'static str = match word {
                "fn" | "let" | "mut" | "pub" | "use" | "mod" | "struct" | "enum" | "impl"
                | "trait" | "type" | "const" | "static" | "if" | "else" | "match" | "return"
                | "for" | "while" | "loop" | "break" | "continue" | "move" | "ref" | "self"
                | "super" | "crate" | "where" | "async" | "await" | "dyn" | "as" | "in"
                | "unsafe" | "extern" | "true" | "false" => "kw",

                "String" | "Vec" | "Option" | "Result" | "bool" | "u8" | "u16" | "u32" | "u64"
                | "usize" | "i8" | "i16" | "i32" | "i64" | "isize" | "f32" | "f64" | "str"
                | "char" | "Self" => "ty",

                _ if word.starts_with(char::is_uppercase) && !word.contains('_') => "ty",
                _ if i < len && bytes[i] as char == '(' => "kw",
                _ => ""
            };

            out.push((cls, word.to_string()));
            continue;
        }

        if c == '#' && i + 1 < len && bytes[i + 1] as char == '[' {
            let start = i;
            i += 1;
            while i < len && bytes[i] as char != ']' {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            out.push(("pr", code[start..i].to_string()));
            continue;
        }

        if c == '!' && !out.is_empty() {
            if let Some(last) = out.last() {
                if last.0.is_empty() || last.0 == "kw" {
                    out.push(("!", "!".to_string()));
                    i += 1;
                    continue;
                }
            }
        }

        if "(){}[];,".contains(c) {
            out.push(("pn", c.to_string()));
            i += 1;
            continue;
        }

        if "-><=+&|:".contains(c) {
            let start = i;
            while i < len && "-><=+&|:".contains(bytes[i] as char) {
                i += 1;
            }
            out.push(("", code[start..i].to_string()));
            continue;
        }

        out.push(("", c.to_string()));
        i += 1;
    }

    out
}

// ── CopyCode component ─────────────────────────────────────────

#[derive(Properties, PartialEq, Clone)]
pub struct CopyCodeProps {
    pub code:     String,
    #[prop_or_default]
    pub language: String
}

/// Syntax-highlighted code block with a "Copy" button.
///
/// Lines starting with `# ` (Rust hidden doctest lines) are stripped from
/// the display but included in the copied text for compilability.
#[function_component]
pub fn CopyCode(props: &CopyCodeProps) -> Html {
    let copied = use_state(|| false);

    let display_code = strip_hidden_lines(&props.code);

    let on_copy = {
        let copied = copied.clone();
        let code = props.code.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            let code = code.clone();
            let copied = copied.clone();
            wasm_bindgen_futures::spawn_local(async move {
                copy_to_clipboard(&code);
                copied.set(true);
                gloo_timers::future::TimeoutFuture::new(COPIED_FEEDBACK_MS).await;
                copied.set(false);
            });
        })
    };

    let btn_text = if *copied { "\u{2713} Copied" } else { "Copy" };
    let btn_class = if *copied {
        "code-copy copied"
    } else {
        "code-copy"
    };

    let highlighted = match props.language.as_str() {
        "css" => highlight_css(&display_code),
        "html" => highlight_html(&display_code),
        "bash" => highlight_bash(&display_code),
        _ => highlight_rust(&display_code)
    };

    html! {
        <div class="code-block">
            <button class={btn_class} onclick={on_copy}>{ btn_text }</button>
            <pre><code>{ highlighted }</code></pre>
        </div>
    }
}

// ── HTML highlighter ────────────────────────────────────────────

pub fn highlight_html(code: &str) -> Html {
    let tokens = tokenize_html(code);
    html! {
        <>
            { for tokens.iter().map(|(cls, txt)| {
                if cls.is_empty() {
                    html! { <>{ txt }</> }
                } else {
                    let class = (*cls).to_string();
                    let text = txt.clone();
                    html! { <span {class}>{ text }</span> }
                }
            })}
        </>
    }
}

pub fn tokenize_html(code: &str) -> Vec<(&'static str, String)> {
    let mut out: Vec<(&'static str, String)> = Vec::new();
    let bytes = code.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let c = bytes[i] as char;

        if c == '<' && i + 1 < len && bytes[i + 1] as char == '!' {
            let start = i;
            i += 2;
            while i < len && bytes[i] as char != '>' {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            out.push(("cm", code[start..i].to_string()));
            continue;
        }

        if c == '<' {
            let start = i;
            i += 1;
            if i < len && bytes[i] as char == '/' {
                i += 1;
            }
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'-') {
                i += 1;
            }
            out.push(("tn", code[start..i].to_string()));

            while i < len && bytes[i] as char != '>' {
                if bytes[i] as char == '"' {
                    let s = i;
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
                    out.push(("str", code[s..i].to_string()));
                } else if bytes[i].is_ascii_whitespace() {
                    let s = i;
                    while i < len && bytes[i].is_ascii_whitespace() {
                        i += 1;
                    }
                    out.push(("", code[s..i].to_string()));
                } else if bytes[i] as char == '=' {
                    out.push(("pn", "=".to_string()));
                    i += 1;
                } else {
                    let s = i;
                    while i < len
                        && bytes[i] as char != '>'
                        && bytes[i] as char != '"'
                        && !bytes[i].is_ascii_whitespace()
                    {
                        i += 1;
                    }
                    out.push(("at", code[s..i].to_string()));
                }
            }
            if i < len {
                out.push(("pn", ">".to_string()));
                i += 1;
            }
            continue;
        }

        out.push(("", c.to_string()));
        i += 1;
    }

    out
}

// ── Bash highlighter ────────────────────────────────────────────

pub fn highlight_bash(code: &str) -> Html {
    let tokens = tokenize_bash(code);
    html! {
        <>
            { for tokens.iter().map(|(cls, txt)| {
                if cls.is_empty() {
                    html! { <>{ txt }</> }
                } else {
                    let class = (*cls).to_string();
                    let text = txt.clone();
                    html! { <span {class}>{ text }</span> }
                }
            })}
        </>
    }
}

pub fn tokenize_bash(code: &str) -> Vec<(&'static str, String)> {
    let mut out: Vec<(&'static str, String)> = Vec::new();
    let bytes = code.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let c = bytes[i] as char;

        if c == '#' {
            let start = i;
            while i < len && bytes[i] as char != '\n' {
                i += 1;
            }
            out.push(("cm", code[start..i].to_string()));
            continue;
        }

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
            out.push(("str", code[start..i].to_string()));
            continue;
        }

        if c.is_ascii_digit() {
            let start = i;
            while i < len && bytes[i].is_ascii_alphanumeric() {
                i += 1;
            }
            out.push(("num", code[start..i].to_string()));
            continue;
        }

        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < len
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'-')
            {
                i += 1;
            }
            let word = &code[start..i];
            let cls: &'static str = match word {
                "cd" | "cargo" | "trunk" | "rustup" | "rm" | "mkdir" | "ls" | "cat" => "kw",
                "install" | "serve" | "build" | "add" | "check" | "test" | "target" => "fn",
                _ => ""
            };
            out.push((cls, word.to_string()));
            continue;
        }

        out.push(("", c.to_string()));
        i += 1;
    }

    out
}
