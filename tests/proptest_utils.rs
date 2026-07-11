// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Property-based coverage for the path and URL helpers. Hand-written
//! examples in the unit-test modules pin specific cases; these properties
//! assert invariants over arbitrary UTF-8 / arbitrary path inputs.

#![cfg(not(target_arch = "wasm32"))]

use std::fmt::Write as _;

use proptest::prelude::*;
use yew_nav_link::utils::{
    QueryParams, UrlParts, is_absolute, join_paths, normalize_path, urlencoding_decode,
    urlencoding_encode
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

    /// `.` segments never survive normalization. `..` segments are fully
    /// resolved in absolute paths; in relative paths only an unresolvable
    /// leading `..` run may remain, never one after a named segment.
    #[test]
    fn normalize_path_resolves_dot_segments(input in "[/a-zA-Z0-9./]{0,64}") {
        let normalized = normalize_path(&input);
        let mut named_seen = false;
        for segment in normalized.split('/').filter(|s| !s.is_empty()) {
            prop_assert_ne!(segment, ".");
            if segment == ".." {
                prop_assert!(
                    !is_absolute(&normalized) && !named_seen,
                    "unexpected `..` in {normalized:?} from {input:?}",
                );
            } else {
                named_seen = true;
            }
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

    /// `join_paths` with an absolute base yields fully resolved output —
    /// no `.` or `..` segments survive.
    #[test]
    fn join_paths_output_is_normalised(
        base in "/[/a-zA-Z0-9.]{0,32}",
        child in "[/a-zA-Z0-9.]{0,32}",
    ) {
        let joined = join_paths(&base, &child);
        for segment in joined.split('/').filter(|s| !s.is_empty()) {
            prop_assert_ne!(segment, ".");
            prop_assert_ne!(segment, "..");
        }
    }

    /// A query string survives a `parse` → `to_query_string` → `parse`
    /// round-trip: every key still maps to the same value list.
    #[test]
    fn query_params_round_trip(
        pairs in prop::collection::vec(("[a-zA-Z0-9_]{1,8}", "[a-zA-Z0-9_.~ +/&=%№✓-]{0,12}"), 0..6),
    ) {
        let mut params = QueryParams::new();
        for (key, value) in &pairs {
            params.set(key, value);
        }
        let reparsed = QueryParams::parse(&params.to_query_string());
        for (key, _) in &pairs {
            prop_assert_eq!(
                reparsed.get_all(key),
                params.get_all(key),
                "values for {} must survive the round-trip",
                key
            );
        }
        prop_assert_eq!(reparsed.keys().count(), params.keys().count());
    }

    /// `set_value` collapses a key to exactly one value and leaves other
    /// keys untouched.
    #[test]
    fn query_params_set_value_is_idempotent_per_key(
        key in "[a-z]{1,6}",
        values in prop::collection::vec("[a-zA-Z0-9]{0,8}", 1..4),
        other in "[A-Z]{1,6}",
    ) {
        let mut params = QueryParams::new();
        params.set(&other, "kept");
        for value in &values {
            params.set_value(&key, value);
        }
        let last = values.last().expect("range starts at 1").clone();
        prop_assert_eq!(params.get_all(&key), Some(&vec![last]));
        prop_assert_eq!(params.get(&other), Some("kept"));
    }

    /// `UrlParts::parse` decomposes a structured URL into exactly the pieces
    /// it was built from.
    #[test]
    fn url_parts_round_trip(
        scheme in "[a-z]{2,5}",
        host in "[a-z0-9.-]{1,12}",
        port in prop::option::of(0u16..=65535),
        segments in prop::collection::vec("[a-zA-Z0-9._~-]{1,8}", 0..4),
        query in prop::option::of("[a-z]{1,4}=[a-z0-9]{0,6}"),
        fragment in prop::option::of("[a-zA-Z0-9-]{1,8}"),
    ) {
        let path = if segments.is_empty() {
            String::new()
        } else {
            format!("/{}", segments.join("/"))
        };
        let mut url = format!("{scheme}://{host}");
        if let Some(port) = port {
            let _ = write!(url, ":{port}");
        }
        url.push_str(&path);
        if let Some(query) = &query {
            let _ = write!(url, "?{query}");
        }
        if let Some(fragment) = &fragment {
            let _ = write!(url, "#{fragment}");
        }

        let parts = UrlParts::parse(&url);
        prop_assert_eq!(parts.scheme.as_deref(), Some(scheme.as_str()));
        prop_assert_eq!(parts.host.as_deref(), Some(host.as_str()));
        prop_assert_eq!(parts.port, port.map(|p| p.to_string()));
        prop_assert_eq!(parts.path, path);
        prop_assert_eq!(parts.query, query);
        prop_assert_eq!(parts.fragment, fragment);
    }
}
