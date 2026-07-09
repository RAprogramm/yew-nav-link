// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![no_main]

use libfuzzer_sys::fuzz_target;
use yew_nav_link::utils::{is_absolute, normalize_path};

fuzz_target!(|data: &[u8]| {
    let Ok(s) = std::str::from_utf8(data) else {
        return;
    };

    let first = normalize_path(s);
    let second = normalize_path(&first);
    assert_eq!(first, second, "normalize_path must be idempotent");

    if is_absolute(s) && !first.is_empty() {
        assert!(
            first.starts_with('/'),
            "absolute input {s:?} normalized to {first:?}",
        );
    }
});
