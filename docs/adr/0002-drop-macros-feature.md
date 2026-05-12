<!--
SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# 0002 — Drop the `macros` feature in 0.9.0

- **Status:** accepted
- **Date:** 2026-04-16
- **Deciders:** RAprogramm

## Context

Versions 0.1 – 0.8 shipped a proc-macro crate (`yew-nav-link-macros`)
behind a `macros` feature flag. It offered a `nav_link!()` declarative
macro that expanded to the same `NavLink` invocation that the function
syntax (`nav_link(...)`) already produced.

The macro existed for one reason: callers who wanted to spell the link
inline (`html! { nav_link!(Route::Home, "Home") }`) without an extra
`{ }` around the function call. That ergonomic gain came with a
disproportionate maintenance tail:

- A whole proc-macro crate (build-script, separate `Cargo.toml`,
  separate publish step) for what compiled down to one line of code.
- Trybuild tests for the macro's diagnostic messages — flaky across
  toolchain versions, and yet another CI matrix to maintain.
- A second documented API surface every contributor had to keep in
  sync with the function and component syntaxes.
- A feature flag that almost no public consumer enabled (visible from
  reverse-deps on crates.io at the time of the decision).

## Decision

Remove the `macros` feature and the `yew-nav-link-macros` sub-crate in
0.9.0. Keep both the component syntax (`<NavLink<R> ...>`) and the
function syntax (`nav_link(Route::Home, "Home", Match::Exact)`).

`nav_link()` is the de facto replacement for the macro: it composes
into `html! { }` without extra braces (`html! { { nav_link(...) } }`
becomes `html! { nav_link(...) }` because `Html` implements the
necessary conversions inline).

## Consequences

**Positive**

- One less crate to publish, one less feature matrix to test, one
  less proc-macro to debug across compiler upgrades.
- The two remaining APIs (component, function) are clearly differentiated:
  component when you need props, function when you need an inline
  expression.
- MSRV bumps and Yew bumps land faster because there is no proc-macro
  layer to chase.

**Negative**

- A breaking change for consumers who had enabled `macros`. Migration
  is mechanical: `nav_link!(R, "x")` → `nav_link(R, "x", Match::Exact)`.
- One historical convenience is gone.

**Cost to reverse**

High. The proc-macro crate would have to be re-published from scratch
with its own version history, and the feature flag re-introduced. We do
not expect to reverse.

## References

- CHANGELOG `[0.9.0]` — "Macros feature removed".
- ROADMAP `0.10.0 — breaking-change pass`.
