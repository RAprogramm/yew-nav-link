//! Build script: parses `src/` of yew-nav-link and generates
//! `generated/meta.rs` with component and hook metadata derived
//! directly from rustdoc in the library source.
//!
//! No manual doc_metadata.rs needed — everything comes from
//! the actual source.

use std::fs;
use std::path::Path;

const EXAMPLE_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn lib_src_path() -> String {
    Path::new(EXAMPLE_DIR)
        .parent()
        .unwrap()
        .join("src")
        .to_str()
        .unwrap()
        .to_string()
}

// ── Main entry ──────────────────────────────────────────────

fn main() {
    let lib_src = lib_src_path();
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed={}", lib_src);

    let components = gather_components(&lib_src);
    let hooks = gather_hooks(&lib_src);
    let out = generate_code(&components, &hooks);

    let out_path = Path::new(EXAMPLE_DIR).join("src/generated/mod.rs");
    fs::create_dir_all(out_path.parent().unwrap()).ok();
    fs::write(&out_path, out).unwrap();
}

// ── Parsing helpers ─────────────────────────────────────────

/// Read all `//!` doc lines from a file.
fn read_doc_block(file: &Path) -> Vec<String> {
    let src = fs::read_to_string(file).unwrap_or_default();
    src.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            trimmed
                .strip_prefix("//!")
                .map(|rest| rest.strip_prefix(' ').unwrap_or(rest).to_string())
        })
        .collect()
}

/// Given a file path and line index (0-based), read `///` doc comments
/// immediately above that line, skipping `#[hook]` attributes.
fn read_doc_above(file: &Path, line_idx: usize) -> Vec<String> {
    let src = fs::read_to_string(file).unwrap_or_default();
    let lines: Vec<&str> = src.lines().collect();
    if line_idx == 0 || line_idx >= lines.len() {
        return Vec::new();
    }

    let mut doc_lines: Vec<String> = Vec::new();
    let mut i = line_idx - 1;

    // Skip any `#[hook]` or `#[component]` attributes
    while i > 0 {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("#[hook") || trimmed.starts_with("#[component") || trimmed.starts_with("#[derive") {
            i -= 1;
            continue;
        }
        if trimmed.starts_with("///") {
            let inner = trimmed[3..].trim();
            doc_lines.insert(0, inner.to_string());
            i -= 1;
        } else {
            break;
        }
    }

    doc_lines
}

/// Extract the name from a function signature like `pub fn use_foo<R>() -> T`.
fn extract_fn_name(line: &str) -> String {
    line.trim_start_matches("pub fn ")
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

/// Extract return type: after `->` take first token.
fn extract_return_type(line: &str) -> String {
    if let Some(pos) = line.find("->") {
        let after = line[pos + 2..].trim();
        after
            .split(|c: char| !c.is_alphanumeric() && c != '}' && c != '>')
            .next()
            .unwrap_or("")
            .to_string()
    } else {
        String::new()
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Remove lines that are inside ``` code blocks from doc comment lines.
fn filter_code_blocks(lines: &[String]) -> Vec<String> {
    let mut in_code = false;
    let mut result = Vec::new();
    for line in lines {
        if line.contains("```") {
            in_code = !in_code;
            continue;
        }
        if !in_code {
            result.push(line.clone());
        }
    }
    result
}

fn simplify_markdown(s: &str) -> String {
    let s = s.replace('`', "");

    // Strip inline links [text](url) -> text
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '[' {
            let mut buf = String::new();
            while let Some(ch) = chars.next() {
                if ch == ']' {
                    break;
                }
                buf.push(ch);
            }
            result.push_str(&buf);
            if chars.peek() == Some(&'(') {
                chars.next();
                let mut depth = 1;
                while let Some(ch) = chars.next() {
                    if ch == ')' {
                        depth -= 1;
                        if depth == 0 { break; }
                    }
                    if ch == '(' {
                        depth += 1;
                    }
                }
            }
        } else {
            result.push(c);
        }
    }

    result.split_whitespace().collect::<Vec<_>>().join(" ")
}

// ── Component gathering ─────────────────────────────────────

#[derive(Clone, Debug)]
struct CompMeta {
    name: String,
    category: String,
    summary: String,
}

fn gather_components(lib_src: &str) -> Vec<CompMeta> {
    let components_dir = Path::new(lib_src).join("components");
    let mut result: Vec<CompMeta> = Vec::new();

    if !components_dir.is_dir() {
        return result;
    }

    for entry in fs::read_dir(&components_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_file() || path.extension().map_or(true, |e| e != "rs") {
            continue;
        }

        let file_name = path.file_stem().unwrap().to_str().unwrap();

        // Skip internal files, mod.rs, page_link/page_item/pagination_page, dropdown
        // Skip internal helpers and mod.rs
        // dropdown.rs defines NavDropdown, NavDropdownItem, NavDropdownDivider
        // tab_item.rs defines NavTab, tab_panel.rs defines NavTabPanel
        match file_name {
            "mod" | "page_link" | "page_item" | "pagination_page" | "dropdown" => {
                continue;
            }
            _ => {}
        }

        let doc_lines = read_doc_block(&path);
        let name = match extract_title(&doc_lines) {
            Some(t) => t,
            None => capitalize(file_name),
        };

        let category = match file_name {
            "badge" | "header" | "text" | "icon" => "Visual",
            "tabs" => "Navigation",
            "pagination" => "Navigation",
            _ => "Visual",
        };

        let summary = extract_summary_from_doc(&doc_lines);

        result.push(CompMeta {
            name,
            category: category.to_string(),
            summary,
        });
    }

    result.sort_by(|a, b| a.name.cmp(&b.name));
    result
}

fn extract_title(lines: &[String]) -> Option<String> {
    lines.iter().find_map(|l| {
        l.strip_prefix("# ").map(|inner| inner.trim().trim_matches('`').to_string())
    })
}

/// Extract a short summary from `//!` doc comment lines.
/// Strategy: take lines between `//! # Title` and `//! ``` ` or `//! # SubHeader`,
/// skip code blocks entirely.
fn extract_summary_from_doc(lines: &[String]) -> String {
    let mut in_title = false;
    let mut in_code = false;
    let mut result: Vec<String> = Vec::new();

    for line in lines {
        let trimmed = line.trim();

        // Track code blocks
        if trimmed.starts_with("```") {
            in_code = !in_code;
            continue;
        }

        if in_code {
            continue; // skip inside code blocks
        }

        if trimmed.starts_with("# ") && trimmed.len() > 2 {
            // Could be a title or sub-header
            if !in_title {
                in_title = true;
                continue; // skip title line itself
            }
            // Sub-header (e.g. `//! # CSS Classes`) — stop
            break;
        }

        if in_title {
            if !trimmed.is_empty() {
                result.push(line.clone());
            } else if !result.is_empty() {
                // First blank line after content = end of summary
                break;
            }
        }
    }

    let joined = result.join(" ");
    simplify_markdown(&joined)
}

// ── Hook gathering ──────────────────────────────────────────

#[derive(Clone, Debug)]
struct HookMeta {
    name: String,
    return_type: String,
    summary: String,
}

fn gather_hooks(lib_src: &str) -> Vec<HookMeta> {
    let hooks_dir = Path::new(lib_src).join("hooks");
    let mut result: Vec<HookMeta> = Vec::new();

    if !hooks_dir.is_dir() {
        return result;
    }

    collect_hooks_recursive(&hooks_dir, &mut result);
    result.sort_by(|a, b| a.name.cmp(&b.name));
    result
}

fn collect_hooks_recursive(dir: &Path, result: &mut Vec<HookMeta>) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            collect_hooks_recursive(&path, result);
            continue;
        }

        if !path.is_file() || path.extension().map_or(true, |e| e != "rs") {
            continue;
        }

        let src = fs::read_to_string(&path).unwrap_or_default();
        let lines: Vec<&str> = src.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Look for pub fn use_ with #[hook] on previous non-blank non-attrs line
            if !trimmed.starts_with("pub fn use_") {
                continue;
            }

            // Find the actual #[hook] attribute above, skipping other attributes
            let has_hook = find_hook_attr(&lines, i);
            if !has_hook {
                continue;
            }

            let name = extract_fn_name(trimmed);
            if name.is_empty() {
                continue;
            }

            // Avoid duplicates (some files might be parsed twice)
            if result.iter().any(|h| h.name == name) {
                continue;
            }

            // Collect doc comments above
            let doc_lines = read_doc_above(&path, i);

            let return_type = extract_return_type(trimmed);
            let summary = filter_code_blocks(&doc_lines)
                .iter()
                .filter(|l| !l.starts_with("#"))
                .take(8)
                .cloned()
                .collect::<Vec<_>>()
                .join(" ");

            result.push(HookMeta {
                name,
                return_type,
                summary: simplify_markdown(&summary),
            });
        }
    }
}

/// Check if there's a #[hook] attribute somewhere above this function.
fn find_hook_attr(lines: &[&str], fn_line: usize) -> bool {
    let mut i = fn_line;
    while i > 0 {
        i -= 1;
        let trimmed = lines[i].trim();
        if trimmed.starts_with("#[hook]") {
            return true;
        }
        // Stop at any non-attribute, non-blank line
        if !trimmed.is_empty() && !trimmed.starts_with('#') && !trimmed.starts_with("//") {
            break;
        }
    }
    false
}

// ── Code generation ─────────────────────────────────────────

fn generate_code(components: &[CompMeta], hooks: &[HookMeta]) -> String {
    let mut out = String::new();

    out.push_str("// Auto-generated by example/build.rs — do not edit.\n");
    out.push_str("// Driven by rustdoc in yew-nav-link/src/.\n\n");

    // ── CompItem ──
    out.push_str("#[derive(Clone, Debug)]\npub struct CompItem {\n");
    out.push_str("    pub name: &'static str,\n");
    out.push_str("    pub summary: &'static str,\n");
    out.push_str("    pub category: &'static str,\n}\n\n");

    out.push_str("pub static COMPONENTS: &[CompItem] = &[\n");
    for comp in components {
        out.push_str(&format!(
            "    CompItem {{ name: {:?}, summary: {:?}, category: {:?} }},\n",
            comp.name, comp.summary, comp.category
        ));
    }
    out.push_str("];\n\n");

    // ── HookItem ──
    out.push_str("#[derive(Clone, Debug)]\npub struct HookItem {\n");
    out.push_str("    pub name: &'static str,\n");
    out.push_str("    pub return_type: &'static str,\n");
    out.push_str("    pub summary: &'static str,\n}\n\n");

    out.push_str("pub static HOOKS: &[HookItem] = &[\n");
    for hook in hooks {
        out.push_str(&format!(
            "    HookItem {{ name: {:?}, return_type: {:?}, summary: {:?} }},\n",
            hook.name, hook.return_type, hook.summary
        ));
    }
    out.push_str("];\n\n");

    // ── Sidebar ──
    out.push_str("#[derive(Clone, Debug)]\npub struct SidebarSection {\n");
    out.push_str("    pub title: &'static str,\n");
    out.push_str("    pub items: &'static [SidebarItem],\n}\n\n");

    out.push_str("#[derive(Clone, Debug)]\npub enum SidebarTarget {\n");
    out.push_str("    Overview,\n");
    out.push_str("    CompPage(u32),     // doc page index\n");
    out.push_str("    CompItem(u32),     // component in COMPONENTS\n");
    out.push_str("    HookItem(u32),     // hook in HOOKS\n}\n\n");

    out.push_str("#[derive(Clone, Debug)]\npub struct SidebarItem {\n");
    out.push_str("    pub label: &'static str,\n");
    out.push_str("    pub target: SidebarTarget,\n}\n\n");

    // Build sidebar sections
    out.push_str("pub static SIDEBAR: &[SidebarSection] = &[\n");

    // Overview section
    out.push_str(r#"    SidebarSection {
        title: "Overview",
        items: &[
            SidebarItem {
                label: "Overview",
                target: SidebarTarget::Overview,
            },
        ],
    },
"#);

    // Components section
    out.push_str(r#"    SidebarSection {
        title: "Components",
        items: &["#);

    for (i, comp) in components.iter().enumerate() {
        out.push_str(&format!(
            r#"            SidebarItem {{
                label: {:?},
                target: SidebarTarget::CompItem({}),
            }},"#,
            comp.name, i
        ));
        out.push('\n');
    }
    out.push_str("        ],\n    },\n");

    // Hooks section
    out.push_str(r#"    SidebarSection {
        title: "Hooks",
        items: &["#);

    for (i, hook) in hooks.iter().enumerate() {
        out.push_str(&format!(
            r#"            SidebarItem {{
                label: {:?},
                target: SidebarTarget::HookItem({}),
            }},"#,
            hook.name, i
        ));
        out.push('\n');
    }
    out.push_str("        ],\n    },\n");

    out.push_str("];\n");

    out
}
