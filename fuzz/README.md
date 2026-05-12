<!--
SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Fuzz targets

`cargo-fuzz` drives libFuzzer against the path and URL helpers exposed
by `yew-nav-link`. The `tests/proptest_utils.rs` property suite bounds
its inputs by a regex grammar; these fuzz targets drive the same
functions with unbounded byte sequences from libFuzzer's coverage-guided
input generator.

## Targets

| Target | What it asserts |
|---|---|
| `fuzz_normalize_path` | `normalize_path` is idempotent and preserves absoluteness for absolute inputs. |
| `fuzz_join_paths` | `join_paths(base, segment)` does not panic on arbitrary UTF-8 inputs. |
| `fuzz_urlencoding_roundtrip` | `urlencoding_decode(urlencoding_encode(s)) == s` for any UTF-8 input. |

## Running locally

```bash
cargo install --locked cargo-fuzz
cd fuzz
cargo +nightly fuzz run fuzz_normalize_path -- -max_total_time=60
cargo +nightly fuzz run fuzz_join_paths -- -max_total_time=60
cargo +nightly fuzz run fuzz_urlencoding_roundtrip -- -max_total_time=60
```

`cargo fuzz` requires a nightly toolchain — the macro it expands to
needs unstable `#[no_main]` instrumentation hooks.

## Layout

`fuzz/` is a self-contained workspace (the empty `[workspace]` table in
`fuzz/Cargo.toml`). It is not built by the parent crate's `cargo build`
and does not show up under `cargo public-api`. The corpora live under
`fuzz/corpus/<target>/`; libFuzzer writes new inputs there as it
discovers new code paths.

## CI

`.github/workflows/fuzz.yml` runs each target for five minutes on a
weekly schedule and on demand (`workflow_dispatch`). The job is out of
the PR critical path — the right cadence for fuzzing is "let it run"
not "block every change."
