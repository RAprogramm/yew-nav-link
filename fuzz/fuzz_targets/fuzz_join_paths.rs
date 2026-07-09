// SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![no_main]

use libfuzzer_sys::fuzz_target;
use yew_nav_link::utils::join_paths;

fuzz_target!(|data: (&str, &str)| {
    let (base, segment) = data;
    let _ = join_paths(base, segment);
});
