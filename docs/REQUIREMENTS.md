<!--
SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Requirements

This document captures what `yew-nav-link` is contractually expected to do
(functional requirements) and the constraints under which it does it
(non-functional requirements). Anything claimed here is exercised by the
test suite, the demo, or the CI pipeline.

## 1. Functional requirements

### 1.1 `NavLink<R>`

**FR-NL-1.** Render an `<a>` element whose `href` reflects the target
route's path (prefixed with the router basename when one is set), with the
given child content. History updates are delegated to yew-router's
`Navigator`.

**FR-NL-2.** Compute an *active* state by comparing the current route to the
target on every render:

| `partial` | Match condition |
|-----------|-----------------|
| `false` (default) | exact equality between `current` and `to` |
| `true` | `to.to_path()` is a path-segment prefix of `current.to_path()`; a root target (`"/"`) matches only the root path |

**FR-NL-3.** Apply two CSS classes to the rendered anchor:

- The base class — defaults to `"nav-link"`, overridable via the `class`
  prop (`&'static str`).
- The active class — defaults to `"active"`, overridable via the
  `active_class` prop (`&'static str`). Only emitted when *active* per
  FR-NL-2.

**FR-NL-4.** A plain left-click is intercepted: the default browser
navigation is prevented and `to` is pushed through the `Navigator`, so
dependent hooks re-render. Modifier-clicks (Cmd/Ctrl/Shift/Alt) fall
through to the browser, preserving "open in new tab" affordances. Active
links additionally emit `aria-current="page"`.

### 1.2 `nav_link()` function

**FR-FN-1.** Return an `Html` value that contains a `NavLink<R>` whose
`partial` flag is derived from a `Match` argument:
`Match::Exact` → `partial = false`, `Match::Partial` → `partial = true`.

**FR-FN-2.** Accept the link text as `&str`; it is rendered as a single
text child.

### 1.3 Reactive hooks

**FR-HK-1.** `use_route_info::<R>() -> Option<R>` returns the currently
matched route, or `None` when no registered route matches.

**FR-HK-2.** `use_is_active(route)` and the `use_is_exact_active(route)`
alias return `true` iff the current route equals `route`.

**FR-HK-3.** `use_is_partial_active(route)` returns `true` iff
`route.to_path()` is a path-segment prefix of the current path; a root
target (`"/"`) matches only the root path.

**FR-HK-4.** `use_navigation::<R>() -> Navigation<R>` returns a value-type
struct. `go_back` and `go_forward` are ready-made `Callback<()>` fields;
`push_callback(route)`, `replace_callback(route)`, and `go_callback(delta:
isize)` are methods that build a `Callback<()>` from an argument. Every
callback routes through the router's `Navigator`, so a configured basename
is honored. Consumers adapt them with `.reform(...)` for `onclick` handlers.

**FR-HK-5.** `use_query_params() -> QueryParams` parses the current URL's
query string into a multi-value map (`utils::QueryParams`, backed by
`Vec<(String, Vec<String>)>` so repeated keys are preserved); reactive on
every URL change.

**FR-HK-6.** `use_breadcrumbs::<R>() -> Vec<BreadcrumbItem<R>>` builds a
breadcrumb trail from the current path. The label of each item comes from a
`BreadcrumbLabelProvider` injected into the tree via
`BreadcrumbLabelProviderContext`. When no provider is present the path
itself is used as the label. The last item has `is_active == true`.
Each item's `route` is resolved from its own path prefix via
`Routable::recognize`; when the prefix does not correspond to any route
in `R`, the item falls back to the current route.

### 1.4 Components

The crate ships UI components that are render-only — they hold no business
logic and accept all required state through props.

| Component | Role |
|-----------|------|
| `NavList` | `<ul>` with sensible ARIA defaults |
| `NavItem` | `<li>` |
| `NavDivider` | `<hr>` |
| `NavHeader` | section heading inside a list |
| `NavText` | inert text inside a list |
| `NavBadge` | inline pill, variant + optional `pill=true` |
| `NavIcon`, `NavLinkWithIcon` | icon container + paired layout helper |
| `NavTabs`, `NavTab`, `NavTabPanel` | tab strip; consumer drives `active` |
| `NavDropdown`, `NavDropdownItem`, `NavDropdownDivider` | self-managed open/close menu |
| `Pagination`, `PageItem`, `PageLink` | full pagination renderer + lower-level building blocks |

### 1.5 Errors

**FR-ER-1.** `NavError` is a `#[non_exhaustive]` `pub enum`. As of 0.10
it has three variants: `RouteNotFound`, `InvalidRoute(String)`,
`NavigationCancelled`. It implements `std::fmt::Display`,
`std::error::Error`, `Clone`, `PartialEq`, `Eq`, and `Debug`. Future
minor releases may add new variants without bumping the major version;
consumer matches must include a `_ =>` arm.

**FR-ER-2.** `NavResult<T>` is a public alias for `Result<T, NavError>`.

### 1.6 Utilities

**FR-UT-1.** `is_absolute(path)` returns `true` iff `path` starts with
`/`.

**FR-UT-2.** `join_paths(a, b)` concatenates two path segments,
collapsing duplicate separators.

**FR-UT-3.** `normalize_path(path)` resolves `.` and `..` segments without
escaping the root.

**FR-UT-4.** `urlencoding_encode` percent-encodes a string;
`urlencoding_decode` returns `Option<String>` (`None` on malformed input).

## 2. Non-functional requirements

### 2.1 Compatibility

| Requirement | Value |
|-------------|-------|
| MSRV | Rust **1.96+**, enforced by CI's MSRV matrix on Linux/macOS/Windows |
| Edition | 2024 |
| Yew | 0.23+ |
| yew-router | 0.20+ |
| `no_std` | **Not supported.** The Yew runtime requires `std` (allocations, `Rc`, threading primitives behind WASM); this is a deliberate non-goal. |
| Browser support | Whatever Yew CSR + `wasm32-unknown-unknown` supports |

### 2.2 Versioning

**NFR-V-1.** The crate adheres to **Semantic Versioning 2.0.0** and to the
**Cargo `0.x` interpretation**: while the major version is `0`, every
breaking change increases the minor (`0.x → 0.(x+1)`); additive changes
increase the patch (`0.x.y → 0.x.(y+1)`).

**NFR-V-2.** Every release is tagged on `main` (`vX.Y.Z`), published to
`crates.io`, and published as a GitHub release whose body reproduces the
matching `CHANGELOG.md` section.

**NFR-V-3.** `CHANGELOG.md` follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and is written
ahead of the merge that tags the release.

### 2.3 Quality gates

**NFR-Q-1.** Every commit on `main` was produced by a PR whose CI passed
the following gates:

- `cargo +nightly fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery`
- `cargo nextest run --all-features --profile ci`
- `cargo test --doc --all-features`
- `cargo llvm-cov` upload to Codecov
- `cargo deny check`
- `cargo audit`
- `reuse lint`
- `actionlint`
- `trunk build --release` against `example/`
- Lighthouse CI thresholds (perf 0.85, a11y 0.9, best-practices 0.9, SEO 0.9)

**NFR-Q-2.** No `unsafe` blocks in `src/` or `tests/`.

**NFR-Q-3.** No `unwrap()` or `expect()` outside `#[cfg(test)]` in the
public crate — error handling uses `?`, `Option::map_or_else`,
`Option::unwrap_or`, etc.

**NFR-Q-4.** Every public item carries a `///` doc comment and a doctest
where it makes sense.

### 2.4 Accessibility

**NFR-A-1.** Library components emit ARIA attributes appropriate to their
role: `NavList` carries `role="navigation"` + `aria-label`, `NavTabs` set
`role="tab"` / `aria-selected` / `aria-controls`, etc.

**NFR-A-2.** The bundled demo (`example/`) honours `prefers-color-scheme:
dark`, `prefers-reduced-motion: reduce`, ships a skip-to-content link, and
maintains a visible `:focus-visible` ring across the whole UI.

### 2.5 Security

**NFR-S-1.** Every supply-chain advisory surfaced by `cargo audit` or
`cargo deny check` blocks the release pipeline. Two `unmaintained` warnings
on transitive `proc-macro-error` (pulled through `yew-macro`) are
acknowledged in CI as informational; they do not have a known exploit path.

**NFR-S-2.** Disclosure policy lives in `SECURITY.md`.

### 2.6 Licensing

**NFR-L-1.** The crate is **MIT-licensed**. Every Rust, SCSS, HTML, YAML,
and Markdown file carries `SPDX-FileCopyrightText` and
`SPDX-License-Identifier` headers, and `reuse lint` passes in CI.

## 3. Out-of-scope

- Server-side rendering — Yew SSR is supported by yew-router, but
  `yew-nav-link`'s active-state computation has only been tested under CSR.
- A custom router — the crate is a *companion* to `yew-router`, not a
  replacement.
- Internationalisation of breadcrumb labels — provided by the consumer via
  `BreadcrumbLabelProvider`.
- Telemetry / analytics integration.
