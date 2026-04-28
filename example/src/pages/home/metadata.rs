//! Parses Cargo.toml metadata at compile time.

const CARGO_TOML: &str = include_str!("../../../../Cargo.toml");

pub fn parse_toml<'a>(source: &'a str, key: &str) -> &'a str {
    for line in source.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix(key) {
            let val = rest.trim().trim_matches('"');
            if let Some(end) = val.find(" #") {
                return val[..end].trim();
            }
            return val;
        }
    }
    "?"
}

pub fn parse_dep_version<'a>(source: &'a str, dep: &str) -> &'a str {
    let prefix = format!("{} = ", dep);
    for line in source.lines() {
        let t = line.trim();
        if t.starts_with(&prefix) {
            let rest = &t[prefix.len()..];
            if rest.starts_with('"') {
                if let Some(end) = rest[1..].find('"') {
                    return &rest[1..end + 1];
                }
            }
            if let Some(start) = rest.find("version = \"") {
                let after = &rest[start + 11..];
                if let Some(end) = after.find('"') {
                    return &after[..end];
                }
            }
        }
    }
    "?"
}

#[derive(Clone, PartialEq)]
pub struct CrateMeta {
    pub version:       String,
    pub license:       String,
    pub msrv:          String,
    pub desc:          String,
    pub edition:       String,
    pub yew_ver:       String,
    pub router_ver:    String,
    pub homepage:      String,
    pub repository:    String,
    pub documentation: String
}

pub fn crate_meta() -> CrateMeta {
    CrateMeta {
        version:       parse_toml(CARGO_TOML, "version =").to_string(),
        license:       parse_toml(CARGO_TOML, "license =").to_string(),
        msrv:          parse_toml(CARGO_TOML, "rust-version =").to_string(),
        desc:          parse_toml(CARGO_TOML, "description =").to_string(),
        edition:       parse_toml(CARGO_TOML, "edition =").to_string(),
        yew_ver:       parse_dep_version(CARGO_TOML, "yew").to_string(),
        router_ver:    parse_dep_version(CARGO_TOML, "yew-router").to_string(),
        homepage:      parse_toml(CARGO_TOML, "homepage =").to_string(),
        repository:    parse_toml(CARGO_TOML, "repository =").to_string(),
        documentation: parse_toml(CARGO_TOML, "documentation =").to_string()
    }
}
