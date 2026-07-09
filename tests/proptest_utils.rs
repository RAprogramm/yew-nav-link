// SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Property-based coverage for the path and URL helpers. Hand-written
//! examples in the unit-test modules pin specific cases; these properties
//! assert invariants over arbitrary UTF-8 / arbitrary path inputs.

#![cfg(not(target_arch = "wasm32"))]

use proptest::prelude::*;
use yew_nav_link::utils::{
    is_absolute, join_paths, normalize_path, urlencoding_decode, urlencoding_encode
};

proptest! {
    /// `normalize_path` always produces the same output when applied twice.
    #[test]
    fn normalize_path_is_idempotent(input in "[/a-zA-Z0-9./]{0,64}") {
        let once = normalize_path(&input);
        let twice = normalize_path(&once);
        prop_assert_eq!(once, twice);
    }

    /// Absolute input keeps its leading `/` after normalization.
    #[test]
    fn normalize_path_preserves_absoluteness(input in "[/a-zA-Z0-9./]{0,64}") {
        if is_absolute(&input) {
            let normalized = normalize_path(&input);
            prop_assert!(
                normalized.is_empty() || normalized.starts_with('/'),
                "absolute input {input:?} normalized to {normalized:?}",
            );
        }
    }

    /// `..` and `.` segments must not appear in the normalized output —
    /// they should have been resolved away.
    #[test]
    fn normalize_path_resolves_dot_segments(input in "[/a-zA-Z0-9./]{0,64}") {
        let normalized = normalize_path(&input);
        for segment in normalized.split('/').filter(|s| !s.is_empty()) {
            prop_assert_ne!(segment, ".");
            prop_assert_ne!(segment, "..");
        }
    }

    /// Encoding then decoding any UTF-8 string round-trips back to the
    /// original. This covers ASCII, multi-byte UTF-8 (Cyrillic, CJK, emoji),
    /// reserved characters, and the empty string.
    #[test]
    fn urlencoding_round_trip(input in ".*") {
        let encoded = urlencoding_encode(&input);
        let decoded = urlencoding_decode(&encoded);
        prop_assert_eq!(decoded, Some(input));
    }

    /// `urlencoding_encode` produces output composed of unreserved bytes
    /// only — every byte is one of `[A-Za-z0-9_.~%+-]`.
    #[test]
    fn urlencoding_encode_uses_only_safe_bytes(input in ".*") {
        let encoded = urlencoding_encode(&input);
        for byte in encoded.bytes() {
            let is_safe = byte.is_ascii_alphanumeric()
                || matches!(byte, b'-' | b'_' | b'.' | b'~' | b'%' | b'+');
            prop_assert!(is_safe, "encoded {input:?} produced byte 0x{byte:02X}");
        }
    }

    /// Joining with an absolute child path discards `base` entirely.
    #[test]
    fn join_paths_absolute_child_replaces_base(
        base in "[/a-zA-Z0-9.]{0,32}",
        child in "/[/a-zA-Z0-9.]{0,32}",
    ) {
        let joined = join_paths(&base, &child);
        let normalized_child = normalize_path(&child);
        prop_assert_eq!(joined, normalized_child);
    }

    /// `join_paths` output is normalised (no `..`/`.` segments left).
    #[test]
    fn join_paths_output_is_normalised(
        base in "[/a-zA-Z0-9.]{0,32}",
        child in "[/a-zA-Z0-9.]{0,32}",
    ) {
        let joined = join_paths(&base, &child);
        for segment in joined.split('/').filter(|s| !s.is_empty()) {
            prop_assert_ne!(segment, ".");
            prop_assert_ne!(segment, "..");
        }
    }
}
