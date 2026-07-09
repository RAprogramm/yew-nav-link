<!--
SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Browser tests

Real-browser coverage for the public API. Each file holds `#[wasm_bindgen_test]`
cases that render a small Yew app, wait for the scheduler to flush, and
assert directly against the resulting DOM.

## Layout

```
tests/
  wasm.rs              entry, declares the modules below
  wasm/
    common.rs          shared helpers (route enum, fresh root, render flush)
    nav_link.rs        NavLink rendering and active-state behaviour
```

Each file at top level of `tests/` is its own integration test crate, so
`tests/wasm.rs` and its submodules compile and run as the `wasm` test
target. Other files (`tests/*_test.rs`) are independent native test
crates and are not pulled into wasm builds.

## Running locally

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked wasm-pack

wasm-pack test --headless --chrome   --test wasm
wasm-pack test --headless --firefox  --test wasm
```

`--test wasm` restricts compilation to this target so native-only tests
under `tests/*_test.rs` are skipped.

## CI

`.github/workflows/ci.yml` runs the `WASM Tests` job on every PR. It
installs `chromedriver` and `geckodriver` and runs the suite against
both browsers in headless mode.

## Adding a new case

1. Pick or extend a module under `tests/wasm/`.
2. Annotate the function with `#[wasm_bindgen_test]`. Async is allowed
   and is usually necessary to wait for Yew to flush — call
   `wait_for_render().await` after `Renderer::render()`.
3. Use `navigate()` from `common.rs` to set the URL before mounting so
   the router resolves the right `current_route`.
4. Clean up: `root.remove()` and restore the URL with `navigate("/")` if
   the test changed it, so subsequent tests start from a known state.
