// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![no_main]

use libfuzzer_sys::fuzz_target;
use yew_nav_link::utils::QueryParams;

fuzz_target!(|data: &str| {
    let params = QueryParams::parse(data);

    let reparsed = QueryParams::parse(&params.to_query_string());
    assert_eq!(
        reparsed, params,
        "parse -> to_query_string -> parse must be a fixed point"
    );

    for key in params.keys() {
        assert!(
            params.get(key).is_some(),
            "every reported key must resolve to a value"
        );
    }
});
