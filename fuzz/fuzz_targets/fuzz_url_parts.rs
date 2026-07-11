// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![no_main]

use libfuzzer_sys::fuzz_target;
use yew_nav_link::utils::{UrlParts, percent_decode};

fuzz_target!(|data: &str| {
    let parts = UrlParts::parse(data);

    assert!(
        parts.path.is_empty() || parts.path.starts_with('/'),
        "a non-empty path must be rooted, got {:?}",
        parts.path
    );
    if let Some(query) = &parts.query {
        assert!(
            !query.contains('#'),
            "the fragment must be split off before the query"
        );
    }
    let _ = parts.query_params();

    let _ = percent_decode(data);
});
