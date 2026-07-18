# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.12.4](https://github.com/RAprogramm/yew-nav-link/compare/v0.12.3...v0.12.4) - 2026-07-18

### Documentation

- add KaiCode 2026 winner badge to README ([#249](https://github.com/RAprogramm/yew-nav-link/issues/249))

## [0.12.3](https://github.com/RAprogramm/yew-nav-link/compare/v0.12.2...v0.12.3) - 2026-07-15

### CI

- normalize concurrency settings ([#245](https://github.com/RAprogramm/yew-nav-link/issues/245))

## [0.12.2](https://github.com/RAprogramm/yew-nav-link/compare/v0.12.1...v0.12.2) - 2026-07-13

### CI

- *(deps)* bump the actions group with 5 updates ([#243](https://github.com/RAprogramm/yew-nav-link/issues/243))

## [0.12.1](https://github.com/RAprogramm/yew-nav-link/compare/v0.12.0...v0.12.1) - 2026-07-11

### CI

- publish releases only from release-PR merges ([#242](https://github.com/RAprogramm/yew-nav-link/issues/242))

### Documentation

- add the curated 0.12.0 changelog section ([#240](https://github.com/RAprogramm/yew-nav-link/issues/240))

## [0.12.0](https://github.com/RAprogramm/yew-nav-link/compare/v0.11.2...v0.12.0) - 2026-07-11

### Fixed

- Restore canonical MIT license text so GitHub detects the license again ([#235](https://github.com/RAprogramm/yew-nav-link/pull/235))
- Breadcrumbs no longer link intermediate crumbs to the `#[not_found]` route; labels and provider paths are percent-decoded ([#238](https://github.com/RAprogramm/yew-nav-link/pull/238))
- Partial matching treats a root route as matching only the root path, so a Home link is not active everywhere ([#238](https://github.com/RAprogramm/yew-nav-link/pull/238))
- `NavLink` without a `Navigator` degrades to a plain anchor instead of a dead click ([#238](https://github.com/RAprogramm/yew-nav-link/pull/238))
- `UrlParts::parse` handles IPv6 and userinfo authorities; `normalize_path` preserves unresolvable leading `..` in relative paths ([#238](https://github.com/RAprogramm/yew-nav-link/pull/238))
- `KeyboardNavConfig::default()` now equals `new()`; `handle_home_end` returns `None` for empty lists ([#238](https://github.com/RAprogramm/yew-nav-link/pull/238))
- Pagination first/last controls render as `«`/`»` jump buttons instead of duplicating page numbers; the active page stays focusable with `aria-current="page"` and ellipsis gaps are non-interactive ([#238](https://github.com/RAprogramm/yew-nav-link/pull/238))

### Changed

- **Breaking:** all string props take `AttrValue` (`Option<AttrValue>` when optional); see ADR 0006 ([#238](https://github.com/RAprogramm/yew-nav-link/pull/238))
- **Breaking:** removed the unused `attrs` module and the unused `PageItemProps::page` prop; `NavTab::onclick` gained a default ([#238](https://github.com/RAprogramm/yew-nav-link/pull/238))
- `NavDropdown` follows the WAI-ARIA disclosure-navigation pattern (no `menu`/`menuitem` roles; `aria-controls` wired to the `id` prop, which is now rendered) ([#238](https://github.com/RAprogramm/yew-nav-link/pull/238))

### Added

- `NavTabs`/`NavTab` implement the ARIA tabs keyboard pattern: roving tabindex, wrapping arrow keys that skip disabled tabs, `Home`/`End`, optional `vertical` orientation ([#238](https://github.com/RAprogramm/yew-nav-link/pull/238))
- `utils::percent_decode` for path components (no `+`-to-space rule) ([#238](https://github.com/RAprogramm/yew-nav-link/pull/238))
- Browser test suite grew from 9 to 29 tests; e2e from 12 to 22 scenarios; two new fuzz targets for the query and URL parsers ([#238](https://github.com/RAprogramm/yew-nav-link/pull/238))

## [0.11.2](https://github.com/RAprogramm/yew-nav-link/compare/v0.11.1...v0.11.2) - 2026-07-10

### CI

- self-sync sub-crate lockfiles in release PRs and add yank workflow ([#232](https://github.com/RAprogramm/yew-nav-link/issues/232))

## [0.11.1](https://github.com/RAprogramm/yew-nav-link/compare/v0.11.0...v0.11.1) - 2026-07-10

### Bug Fixes

- give pagination prev/next buttons an accessible name ([#226](https://github.com/RAprogramm/yew-nav-link/issues/226))
- parse host-only and path-only URLs correctly ([#224](https://github.com/RAprogramm/yew-nav-link/issues/224))

### CI

- gate clippy pedantic/nursery and compile README examples ([#228](https://github.com/RAprogramm/yew-nav-link/issues/228))

### Documentation

- sync README and docs with the actual public API ([#225](https://github.com/RAprogramm/yew-nav-link/issues/225))

### Features

- keyboard navigation for NavDropdown ([#227](https://github.com/RAprogramm/yew-nav-link/issues/227))

## [0.11.0](https://github.com/RAprogramm/yew-nav-link/compare/v0.10.10...v0.11.0) - 2026-07-10

### Bug Fixes

- [**breaking**] route programmatic navigation through the router Navigator ([#223](https://github.com/RAprogramm/yew-nav-link/issues/223))
- render class attribute instead of classes shorthand ([#220](https://github.com/RAprogramm/yew-nav-link/issues/220))
- give pagination prev/next buttons an accessible name ([#226](https://github.com/RAprogramm/yew-nav-link/issues/226))
- parse host-only and path-only URLs correctly ([#224](https://github.com/RAprogramm/yew-nav-link/issues/224))

### CI

- gate clippy pedantic/nursery and compile README examples ([#228](https://github.com/RAprogramm/yew-nav-link/issues/228))

### Documentation

- sync README and docs with the actual public API ([#225](https://github.com/RAprogramm/yew-nav-link/issues/225))

### Features

- keyboard navigation for NavDropdown ([#227](https://github.com/RAprogramm/yew-nav-link/issues/227))

## [0.10.10](https://github.com/RAprogramm/yew-nav-link/compare/v0.10.9...v0.10.10) - 2026-07-09

### Documentation

- drop REQUIREMENTS reference to removed no_std CI job ([#207](https://github.com/RAprogramm/yew-nav-link/issues/207))
- expand PR template into author and reviewer checklists ([#205](https://github.com/RAprogramm/yew-nav-link/issues/205))
- add layered module architecture diagram ([#203](https://github.com/RAprogramm/yew-nav-link/issues/203))
- add scannable quality-gates matrix to README ([#201](https://github.com/RAprogramm/yew-nav-link/issues/201))
- update ARCHITECTURE.md layout after mod.rs rename ([#197](https://github.com/RAprogramm/yew-nav-link/issues/197))

### Refactoring

- replace mod.rs files with module_name.rs per RustManifest ([#194](https://github.com/RAprogramm/yew-nav-link/issues/194))

## [0.10.9](https://github.com/RAprogramm/yew-nav-link/compare/v0.10.8...v0.10.9) - 2026-07-06

### CI

- *(deps)* bump taiki-e/install-action in the actions group ([#191](https://github.com/RAprogramm/yew-nav-link/issues/191))

## [0.10.8](https://github.com/RAprogramm/yew-nav-link/compare/v0.10.7...v0.10.8) - 2026-07-03

### Documentation

- add animated NavLink demo GIF to README ([#190](https://github.com/RAprogramm/yew-nav-link/issues/190))

### Documentation

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) add animated NavLink demo GIF to README ([#190](https://github.com/RAprogramm/yew-nav-link/issues/190))

## [0.10.7](https://github.com/RAprogramm/yew-nav-link/compare/v0.10.5...v0.10.7) - 2026-07-03

### Bug Fixes

- wrap backward arrow navigation to last item ([#176](https://github.com/RAprogramm/yew-nav-link/issues/176))
- resolve breadcrumb routes from their own path prefixes ([#176](https://github.com/RAprogramm/yew-nav-link/issues/176))

### Refactoring

- preserve insertion order in QueryParams serialization ([#176](https://github.com/RAprogramm/yew-nav-link/issues/176))
- decode percent triplets without per-triplet allocation ([#176](https://github.com/RAprogramm/yew-nav-link/issues/176))

### Documentation

- add back-to-top links to README sections ([#176](https://github.com/RAprogramm/yew-nav-link/issues/176))
- update install snippets to 0.10 and MSRV references to 1.96 ([#176](https://github.com/RAprogramm/yew-nav-link/issues/176))
- align REQUIREMENTS NavLink and is_absolute contracts with implementation ([#176](https://github.com/RAprogramm/yew-nav-link/issues/176))
- refresh public-api snapshot for QueryParams changes and cargo-public-api 0.52 ([#176](https://github.com/RAprogramm/yew-nav-link/issues/176))

### CI

- sync doc version references from Cargo.toml single source of truth ([#176](https://github.com/RAprogramm/yew-nav-link/issues/176))
- strip git-cliff header when refreshing Unreleased changelog ([#185](https://github.com/RAprogramm/yew-nav-link/issues/185))
- revert split_commits, dependabot squash bodies flood the changelog ([#183](https://github.com/RAprogramm/yew-nav-link/issues/183))
- parse squash-merge commit bodies line by line in git-cliff ([#181](https://github.com/RAprogramm/yew-nav-link/issues/181))
- *(deps)* bump the actions group across 1 directory with 14 updates ([#174](https://github.com/RAprogramm/yew-nav-link/issues/174))

## [0.10.5](https://github.com/RAprogramm/yew-nav-link/compare/v0.10.4...v0.10.5) - 2026-05-13

### CI

- pin cargo-fuzz target to x86_64-unknown-linux-gnu ([#169](https://github.com/RAprogramm/yew-nav-link/issues/169))
- pass GITHUB_TOKEN to git-cliff in changelog-refresh workflow ([#165](https://github.com/RAprogramm/yew-nav-link/issues/165))

## [0.10.4](https://github.com/RAprogramm/yew-nav-link/compare/v0.10.3...v0.10.4) - 2026-05-12

### CI

- mirror cliff.toml commit preprocessor into release-plz.toml ([#163](https://github.com/RAprogramm/yew-nav-link/issues/163))
- normalize project commit format in cliff.toml so CHANGELOG body populates ([#161](https://github.com/RAprogramm/yew-nav-link/issues/161))

### Documentation

- refresh ROADMAP after 0.10.x release ([#158](https://github.com/RAprogramm/yew-nav-link/issues/158))

## [0.10.3](https://github.com/RAprogramm/yew-nav-link/compare/v0.10.2...v0.10.3) - 2026-05-12

## [0.10.2](https://github.com/RAprogramm/yew-nav-link/compare/v0.10.1...v0.10.2) - 2026-05-12

## [0.10.1](https://github.com/RAprogramm/yew-nav-link/compare/v0.10.0...v0.10.1) - 2026-05-12

### CI

- *(deps)* bump the actions group with 8 updates

## [0.10.0] - 2026-05-10

The breaking-change pass before 1.0. Four small, targeted breakages consumers
upgrade through in one hop. No new dependencies, MSRV unchanged at 1.95.

### Breaking changes

- `NavError` is now `#[non_exhaustive]`. Foreign-crate exhaustive matches must
  add a wildcard arm. Internal matches inside `yew-nav-link` are unaffected.
  This unlocks adding new error variants (timeouts, redirect cancellation,
  permission denied) under future semver-minor releases without breaking
  consumers.
- `BreadcrumbLabelProviderContext`'s tuple field is now private. Construct
  with `::new(Rc<dyn BreadcrumbLabelProvider>)` and read with `.provider()`.
  Hiding the field lets the type evolve (provider chain, internal cache,
  swap `Rc` for `Arc`) without breaking consumers in 1.x.
- The orphan `src/hooks/navigation/route_params.rs` module is removed. It was
  declared in CHANGELOG 0.8.0 and the README but never wired into `pub use`,
  so no consumer could actually reach it. Migration: use yew-router's
  `use_route::<R>()` with a `Routable` enum whose variants capture the
  parameters as struct fields.
- Active `NavLink` emits `aria-current="page"` on its rendered `<a>`. The
  rendered DOM changes; CSS that targets the active state via `aria-current`
  will start matching for the first time, while CSS that only targets
  `.active` continues to work.

### Added

- New `BreadcrumbLabelProviderContext::provider()` accessor returning a clone
  of the inner `Rc<dyn BreadcrumbLabelProvider>`.
- `NavLink` now reads the active `Navigator`'s basename to build the rendered
  `href` so consumers deploying under a project subpath (e.g. GitHub Pages)
  get the correct URLs without the demo's runtime detection workaround.

### Changed

- `NavLink` is rendered as a manual `<a>` element instead of wrapping
  `yew_router::Link`. Behaviour matches `Link` for navigation, modifier
  clicks (Cmd/Ctrl/Shift/Alt) still fall through to the browser, and we now
  control the rendered attribute set.

### Migration guide

```rust
// Was — exhaustive match on NavError
match err {
    NavError::RouteNotFound       => /* ... */,
    NavError::InvalidRoute(msg)   => /* ... */,
    NavError::NavigationCancelled => /* ... */,
}

// Now — wildcard required because NavError is non_exhaustive
match err {
    NavError::RouteNotFound       => /* ... */,
    NavError::InvalidRoute(msg)   => /* ... */,
    NavError::NavigationCancelled => /* ... */,
    _ => /* future variants */,
}
```

```rust
// Was — tuple-construction or .0 field access
let ctx = BreadcrumbLabelProviderContext(Rc::new(my_provider));
let inner = ctx.0;

// Now
let ctx = BreadcrumbLabelProviderContext::new(Rc::new(my_provider));
let inner = ctx.provider();
```

`use_route_params()` was advertised but never reachable; its replacement is
the standard yew-router pattern:

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[at("/users/:id")]
    User { id: String },
}

if let Some(Route::User { id }) = use_route::<Route>() {
    // use id
}
```

## [0.9.4] - 2026-05-10

### Fixed

- `normalize_path` now actually resolves `.` and `..` segments without escaping the root, matching the contract documented in `docs/REQUIREMENTS.md` FR-UT-3, the README, and the live demo. The previous implementation only collapsed duplicate slashes and trimmed a trailing slash; passing `"/foo/bar/../baz/"` now returns `"/foo/baz/"` as advertised.
- `urlencoding_decode` is now UTF-8 aware. Previously each `%XX` byte was cast to `char` (Latin-1), so multi-byte sequences came back garbled — `%E2%9C%93` returned three junk chars instead of `"✓"`. Bytes are now accumulated and decoded via `String::from_utf8`, and the function correctly returns `None` for input that does not decode to valid UTF-8.
- `pagination_page::generate_pages` no longer panics on adversarial inputs. `current` outside `[1, total]` is clamped, `total = 0` returns an empty list, and `siblings = u32::MAX` no longer overflows on wasm32 (32-bit `usize`); all internal arithmetic uses saturating helpers on `u32`.

### Changed

- `normalize_path` preserves a single trailing `/` when the input ended with one (e.g. `"/docs/"` returns `"/docs/"`). Previously it always stripped trailing slashes, contradicting the documented contract and the demo's example output.

### Performance

- `NavBadge` no longer allocates a fresh `String` per render. The `format!("nav-badge-{}", variant)` call is replaced by a `const fn` returning a precomputed `&'static str` for each documented variant.

### Documentation

- `QueryParams` rustdoc examples now use `.expect("<reason>")` instead of bare `.unwrap()` so users do not copy a panic-on-missing-key idiom into their own code.
- `nav_link()` ships a runnable doctest demonstrating both `Match::Exact` and `Match::Partial` usage. Previously the public top-level export had no `///` example.

### Tests

- Eight new `normalize_path` cases covering `.`/`..` resolution, root-escape attempts, trailing-slash preservation, and empty input.
- Three new `urlencoding_decode` cases for ✓, Cyrillic, and invalid-UTF-8 input, plus a UTF-8 round-trip property covering ASCII, Cyrillic, Japanese, and reserved characters.
- Four new `generate_pages` boundary cases (`total = 0`, out-of-range `current`, `siblings = u32::MAX`).

## [0.9.3] - 2026-05-10

### Added

- Make `BreadcrumbLabelProviderContext` part of the public API and add a `::new(Rc<dyn BreadcrumbLabelProvider>)` constructor. Consumers can now actually inject a custom `BreadcrumbLabelProvider` via Yew's `ContextProvider`; previously the trait was exported but unusable because the context wrapper was crate-private. Re-exported at the crate root and from `hooks::*` for parity with the existing breadcrumb types.

## [0.9.2] - 2026-05-10

### Added

- Re-export `BreadcrumbLabelProvider` at the crate root so consumers can write `use yew_nav_link::BreadcrumbLabelProvider;` instead of reaching into the `hooks::` submodule. Purely additive; no behaviour change.

## [0.9.1] - 2026-05-10

### Fixed

- `use_breadcrumbs` segment loop no longer reuses the `is_last` binding name for both the loop bound and the per-iteration boolean; rename the loop bound to `total` so the intent is unambiguous.

### Changed

- Replace the build-script-driven multi-page documentation site under `example/` with a self-contained single-file `cdylib` SPA demo that exercises every public component, hook, and utility against trunk-served routes.
- Switch the demo crate (`yew-nav-link-demo`) to a `cdylib` library target served by trunk; drop `wasm-bindgen-futures` and `web-sys` feature surface that the rewrite no longer needs.
- Apply `SPDX-FileCopyrightText` / `SPDX-License-Identifier` headers to every Rust source file and the demo entry HTML.
- Tighten `.rustfmt.toml` compliance: drop trailing commas inside `use` statements to match `trailing_comma = "Never"`.

### Internal

- Replace ad-hoc `.gitignore` paths with workspace-wide rules; trunk-generated `example/dist/` is no longer tracked.

## [0.9.0] - 2026-04-16

### Removed

- Removed all macro functionality from the main crate, including declarative macro modules, procedural macro integration, and macro-specific tests.
- Removed the standalone `yew-nav-link-macros` crate from the repository and from dependency wiring.
- Removed macro-focused demo pages and routes from the comprehensive example app.

### Changed

- Updated public docs/examples to reflect component and function APIs only (no macro feature).
- Bumped crate version to `0.9.0` due the macro API removal.

## [0.8.1] - 2026-04-16

### Changed

- Align crate `rust-version` to `1.92` in both main and macros crates to match the validated CI MSRV lane.
- Harden CI linting by adding a dedicated nightly `rustfmt` check and fixing Clippy argument forwarding so pedantic and nursery lints are applied correctly.
- Upgrade security gates to fail on real audit/deny findings instead of masking failures, and run `cargo deny check` as a single strict gate.

### Security

- Security workflows now fail when `cargo audit` or `cargo deny` detects issues, preventing false-green pipelines.

## [0.8.0] - 2026-04-07

### Added

- **Custom CSS Classes for NavLink**: Added `class` and `active_class` props to `NavLink` component, allowing customization of the default `nav-link` and `active` CSS classes
- **Navigation Hooks**: Added three new hooks for programmatic navigation:
  - `use_navigation<R>()` — Returns `Navigation<R>` for push, replace, back, forward navigation
  - `use_route_params()` — Returns `RouteParams` for accessing URL route parameters (`/users/:id`)
  - `use_query_params()` — Returns `QueryParams` for accessing URL query string parameters
- **Custom Breadcrumb Provider**: Added `BreadcrumbLabelProvider` trait allowing custom breadcrumb label generation for routes
- **Documentation Pages**: Added comprehensive documentation pages for all new features in the interactive demo

### Changed

- Updated interactive demo with new feature showcase pages
- Enhanced sidebar navigation with "New in v0.8" section

### Fixed

- None

### Security

- None

## [0.5.0] - 2026-02-15

### Features

- Add `macros` feature to expose `nav_link!` macro directly from `yew-nav-link`
- Add navigation components and hooks

### Dependencies

- Update yew-router from 0.19.0 to 0.20.0

## [0.6.0] - 2026-04-06

### Features

- Add comprehensive interactive demo showcasing all components
- Add NavLink component with automatic active state detection
- Add NavList, NavItem, NavDivider components
- Add NavBadge, NavHeader, NavText components
- Add NavDropdown, NavIcon, NavTabs, Pagination components
- Add route hooks: use_route_info, use_is_active, use_is_exact_active, use_is_partial_active
- Add breadcrumbs generation
- Add path utilities: is_absolute, join_paths, normalize_path

### Bug Fixes

- Update yew to 0.23 for yew-router 0.20 compatibility

### Documentation

- Add comprehensive documentation with live demos
- Add architecture diagrams and flow explanations

### Maintenance

- Remove broken E2E tests referencing deleted examples
- Simplify examples to single comprehensive demo
- Update MSRV to 1.85 for Rust 2024 edition

## [0.4.0] - 2025-12-17

### Documentation

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) add crypto donation link via etherscan

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) add BTC and Solana donation links

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) change BTC explorer to blockchain.com

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) add issue and PR templates

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) rewrite CONTRIBUTING with clear workflow

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) fix commit format in CONTRIBUTING


### Features

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) add working examples ([#6](https://github.com/RAprogramm/yew-nav-link/issues/6))

## [0.3.0] - 2025-12-16

### CI

- ![dependabot](https://github.com/dependabot.png?size=20) **deps:** bump actions/checkout from 4 to 6 in the actions group ([#2](https://github.com/RAprogramm/yew-nav-link/issues/2))

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) optimize caching and use stable toolchain ([#3](https://github.com/RAprogramm/yew-nav-link/issues/3))

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) use CRATES_IO_TOKEN for publishing

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) add job summaries, artifacts and nextest

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) add security-events permission for SARIF upload

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) fix llvm-cov report command options

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) fix package job dirty file error

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) add cargo-info composite action ([#4](https://github.com/RAprogramm/yew-nav-link/issues/4))

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) add pull-requests permission to release workflow


### Documentation

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) add table of contents and back-to-top links


### Features

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) upgrade to Yew 0.22 ([#1](https://github.com/RAprogramm/yew-nav-link/issues/1))

## [0.2.1] - 2025-08-14

### Bug Fixes

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) version

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) doctests

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) version


### Documentation

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) update README

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) update README


### Other

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) nav_link

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) LISENCE

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) bootstrap example

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) nav_link function

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) docs shield

[Unreleased]: https://github.com/RAprogramm/yew-nav-link/compare/v0.8.0...HEAD
[0.8.0]: https://github.com/RAprogramm/yew-nav-link/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/RAprogramm/yew-nav-link/releases/tag/v0.7.0
[0.6.0]: https://github.com/RAprogramm/yew-nav-link/releases/tag/v0.6.0
[0.5.0]: https://github.com/RAprogramm/yew-nav-link/releases/tag/v0.5.0
[0.4.0]: https://github.com/RAprogramm/yew-nav-link/releases/tag/v0.4.0
[0.3.0]: https://github.com/RAprogramm/yew-nav-link/releases/tag/v0.3.0
[0.2.1]: https://github.com/RAprogramm/yew-nav-link/releases/tag/v0.2.1
