// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Root-cause guard against a silent Yew footgun.
//!
//! In `html!`, the shorthand `<el {classes}>` expands to `classes={classes}`.
//! Yew only maps a prop literally named `class` onto the DOM `class`
//! attribute, so `{classes}` renders a bogus `classes="…"` attribute that
//! browsers ignore for styling — the element silently loses every CSS class.
//! `Classes` implements `IntoPropValue<Option<AttrValue>>`, so the mistake
//! compiles cleanly and no type check catches it.
//!
//! This test scans the crate source and fails if the shorthand ever
//! reappears, forcing the correct `class={classes}` form. It runs natively in
//! `cargo nextest`, so every CI platform enforces it without a browser.

use std::{fs, path::Path};

const SHORTHAND: &str = "{classes}";
const CORRECT_PREFIX: &str = "class={classes}";

fn collect_rs_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    let entries = fs::read_dir(dir).expect("src/ must be readable");
    for entry in entries {
        let path = entry.expect("dir entry must be readable").path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn no_bare_classes_shorthand_in_src() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    collect_rs_files(&src, &mut files);
    assert!(!files.is_empty(), "expected to scan at least one src file");

    let mut violations: Vec<String> = Vec::new();
    for file in &files {
        let contents = fs::read_to_string(file).expect("source file must be readable");
        for (line_no, line) in contents.lines().enumerate() {
            let mut search_from = 0;
            while let Some(offset) = line[search_from..].find(SHORTHAND) {
                let idx = search_from + offset;
                let preceded_by_class = idx >= CORRECT_PREFIX.len() - SHORTHAND.len()
                    && line[..idx].ends_with("class=");
                if !preceded_by_class {
                    violations.push(format!(
                        "{}:{}: `{}` renders a bogus `classes` attribute; use `class={{classes}}`",
                        file.display(),
                        line_no + 1,
                        line.trim()
                    ));
                }
                search_from = idx + SHORTHAND.len();
            }
        }
    }

    assert!(
        violations.is_empty(),
        "found {} bare `{{classes}}` shorthand(s); the CSS class would be silently dropped:\n{}",
        violations.len(),
        violations.join("\n")
    );
}
