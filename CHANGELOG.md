# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
