//! Counts public API items by scanning source files at build time.

const SOURCES: &[&str] = &[
    include_str!("../../../../../src/lib.rs"),
    include_str!("../../../../../src/active_link/nav_link.rs"),
    include_str!("../../../../../src/attrs.rs"),
    include_str!("../../../../../src/errors.rs"),
    include_str!("../../../../../src/components/mod.rs"),
    include_str!("../../../../../src/components/badge.rs"),
    include_str!("../../../../../src/components/dropdown.rs"),
    include_str!("../../../../../src/components/header.rs"),
    include_str!("../../../../../src/components/icon.rs"),
    include_str!("../../../../../src/components/page_item.rs"),
    include_str!("../../../../../src/components/page_link.rs"),
    include_str!("../../../../../src/components/pagination.rs"),
    include_str!("../../../../../src/components/pagination_page.rs"),
    include_str!("../../../../../src/components/tab_item.rs"),
    include_str!("../../../../../src/components/tab_panel.rs"),
    include_str!("../../../../../src/components/tabs.rs"),
    include_str!("../../../../../src/components/text.rs"),
    include_str!("../../../../../src/nav/divider.rs"),
    include_str!("../../../../../src/nav/item.rs"),
    include_str!("../../../../../src/nav/list.rs"),
    include_str!("../../../../../src/hooks/route_info/mod.rs"),
    include_str!("../../../../../src/utils/path.rs"),
    include_str!("../../../../../src/utils/keyboard/config.rs"),
    include_str!("../../../../../src/utils/keyboard/direction.rs"),
    include_str!("../../../../../src/utils/keyboard/handlers.rs"),
    include_str!("../../../../../src/utils/url/codec.rs"),
    include_str!("../../../../../src/utils/url/parts.rs"),
    include_str!("../../../../../src/utils/url/query.rs")
];

pub struct ApiStats {
    pub fns:     usize,
    pub structs: usize,
    pub enums:   usize
}

pub fn count_api_items() -> ApiStats {
    let mut fns = 0usize;
    let mut structs = 0usize;
    let mut enums = 0usize;
    for src in SOURCES {
        for line in src.lines() {
            let t = line.trim();
            if t.starts_with("pub fn ") || t.starts_with("pub async fn ") {
                fns += 1;
            }
            if t.starts_with("pub struct ") {
                structs += 1;
            }
            if t.starts_with("pub enum ") {
                enums += 1;
            }
        }
    }
    ApiStats {
        fns,
        structs,
        enums
    }
}
