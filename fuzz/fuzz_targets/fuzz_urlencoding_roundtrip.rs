// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![no_main]

use libfuzzer_sys::fuzz_target;
use yew_nav_link::utils::{urlencoding_decode, urlencoding_encode};

fuzz_target!(|data: &[u8]| {
    let Ok(s) = std::str::from_utf8(data) else {
        return;
    };

    let encoded = urlencoding_encode(s);
    let decoded = urlencoding_decode(&encoded).expect("encoded output must decode back");
    assert_eq!(decoded, s, "encode/decode roundtrip must preserve input");
});
