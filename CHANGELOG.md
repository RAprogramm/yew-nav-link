# Changelog

## [Unreleased]

### Bug Fixes

- ![RAprogramm](https://github.com/RAprogramm.png?size=20) **deps:** update yew to 0.23 for yew-router 0.20 compatibility


### CI

- ![dependabot](https://github.com/dependabot.png?size=20) **deps:** bump actions/upload-artifact from 4 to 6 in the actions group ([#15](https://github.com/RAprogramm/yew-nav-link/issues/15))

- ![dependabot](https://github.com/dependabot.png?size=20) **deps:** bump actions/upload-artifact from 6 to 7 in the actions group ([#18](https://github.com/RAprogramm/yew-nav-link/issues/18))


### Dependencies

- ![dependabot](https://github.com/dependabot.png?size=20) **cargo:** bump wasm-bindgen-test in the yew-ecosystem group ([#16](https://github.com/RAprogramm/yew-nav-link/issues/16))

- ![dependabot](https://github.com/dependabot.png?size=20) **cargo:** bump wasm-bindgen-test in the yew-ecosystem group ([#17](https://github.com/RAprogramm/yew-nav-link/issues/17))

- ![dependabot](https://github.com/dependabot.png?size=20) **cargo:** bump the yew-ecosystem group with 2 updates ([#19](https://github.com/RAprogramm/yew-nav-link/issues/19))

- ![dependabot](https://github.com/dependabot.png?size=20) **cargo:** bump yew-router from 0.19.0 to 0.20.0 ([#20](https://github.com/RAprogramm/yew-nav-link/issues/20))


### Features

- add `macros` feature to expose `nav_link!`, `routable_ext!`, and `nav_item!` directly from `yew-nav-link`
- remove separate macro crate manifests in favor of a single-crate feature flow
- add navigation components, hooks and integration tests

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

[Unreleased]: https://github.com/RAprogramm/yew-nav-link/compare/v0.6.0...HEAD
[0.5.0]: https://github.com/RAprogramm/yew-nav-link/releases/tag/v0.5.0
[0.4.0]: https://github.com/RAprogramm/yew-nav-link/releases/tag/v0.4.0
[0.3.0]: https://github.com/RAprogramm/yew-nav-link/releases/tag/v0.3.0
[0.2.1]: https://github.com/RAprogramm/yew-nav-link/releases/tag/v0.2.1

