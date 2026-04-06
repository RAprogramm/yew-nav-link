<a id="top"></a>

# yew-nav-link

Enterprise-grade navigation library for [Yew](https://yew.rs) — automatic active state detection, declarative macros, and a complete component system.

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/yew-nav-link)](https://crates.io/crates/yew-nav-link)
[![docs.rs](https://img.shields.io/docsrs/yew-nav-link)](https://docs.rs/yew-nav-link)
[![Downloads](https://img.shields.io/crates/d/yew-nav-link)](https://crates.io/crates/yew-nav-link)
[![MSRV](https://img.shields.io/crates/msrv/yew-nav-link)](https://crates.io/crates/yew-nav-link)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![REUSE](https://api.reuse.software/badge/github.com/RAprogramm/yew-nav-link)](https://api.reuse.software/info/github.com/RAprogramm/yew-nav-link)
[![Codecov](https://codecov.io/gh/RAprogramm/yew-nav-link/branch/main/graph/badge.svg)](https://codecov.io/gh/RAprogramm/yew-nav-link)
[![Hits-of-Code](https://hitsofcode.com/github/RAprogramm/yew-nav-link?branch=main&label=HoC)](https://hitsofcode.com/github/RAprogramm/yew-nav-link/view?branch=main)

</div>

---

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Requirements](#requirements)
- [Quick Start](#quick-start)
  - [Component Syntax](#component-syntax)
  - [Function Syntax](#function-syntax)
  - [Partial Matching](#partial-matching)
  - [Macros](#macros)
- [Components](#components)
  - [Core Navigation](#core-navigation)
  - [UI Components](#ui-components)
  - [Hooks](#hooks)
  - [Utilities](#utilities)
- [CSS Integration](#css-integration)
  - [Bootstrap 5](#bootstrap-5)
  - [Tailwind CSS](#tailwind-css)
- [Architecture](#architecture)
- [Examples](#examples)
- [API Reference](#api-reference)
- [Migration Guides](#migration-guides)
- [Coverage](#coverage)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

`yew-nav-link` is a comprehensive navigation library for the Yew web framework. It provides:

| Feature | Description |
|---------|-------------|
| **NavLink** | Drop-in replacement for Yew Router's `<Link>` with automatic `active` class detection |
| **Component System** | 15+ ready-to-use UI components (tabs, dropdowns, pagination, badges, icons) |
| **Hooks** | Reactive hooks for route state, active checking, and breadcrumbs |
| **Macros** | Declarative macros for boilerplate-free navigation (`nav_link!`, `nav_list!`, `nav_tabs!`) |
| **Utilities** | Path manipulation, URL encoding, keyboard navigation, and query string handling |

The core `NavLink` component eliminates manual active state tracking. It compares the current route against the target on every render and applies the `active` CSS class automatically — zero configuration required.

---

## Installation

```toml
[dependencies]
yew-nav-link = "0.7"
```

With macros:

```toml
[dependencies]
yew-nav-link = { version = "0.7", features = ["macros"] }
```

---

## Requirements

| Dependency | Version |
|------------|---------|
| Rust | 1.85+ |
| Edition | 2024 |
| Yew | 0.23+ |
| yew-router | 0.20+ |

---

## Quick Start

### Component Syntax

```rust
use yew::prelude::*;
use yew_nav_link::NavLink;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
}

#[component]
fn Navigation() -> Html {
    html! {
        <nav>
            <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
            <NavLink<Route> to={Route::About}>{ "About" }</NavLink<Route>>
        </nav>
    }
}
```

When the user visits `/about`, the second link automatically receives `class="nav-link active"`.

### Function Syntax

```rust
use yew::prelude::*;
use yew_nav_link::{nav_link, Match};
use yew_router::prelude::*;

#[component]
fn Menu() -> Html {
    html! {
        <nav>
            { nav_link(Route::Home, "Home", Match::Exact) }
            { nav_link(Route::Docs, "Docs", Match::Partial) }
        </nav>
    }
}
```

### Partial Matching

Keep parent links highlighted on nested routes:

```rust
html! {
    <nav>
        // Active on /docs, /docs/api, /docs/anything
        <NavLink<Route> to={Route::Docs} partial=true>{ "Docs" }</NavLink<Route>>
    </nav>
}
```

Partial matching is segment-aware: `/docs` matches `/docs/api` but **not** `/documentation`.

### Macros

```rust
use yew_nav_link::{nav_link, Match};

html! {
    <nav>
        { nav_link!(Route::Home, "Home", Match::Exact) }
        { nav_link!(Route::About, "About", Match::Exact) }
    </nav>
}
```

---

## Components

### Core Navigation

| Component | Purpose |
|-----------|---------|
| [`NavLink<R>`](#navlinkr) | Navigation link with automatic active state |
| [`NavList`] | Accessible navigation list container (`<ul>` with ARIA) |
| [`NavItem`] | Navigation list item (`<li>`) |
| [`NavDivider`] | Visual separator between navigation groups |

### UI Components

| Component | Purpose |
|-----------|---------|
| [`NavBadge`] | Badge/counter for navigation items |
| [`NavHeader`] | Section header for navigation groups |
| [`NavText`] | Plain text element within navigation |
| [`NavIcon`] | Icon with configurable size |
| [`NavLinkWithIcon`] | Link with integrated icon |
| [`NavDropdown`] | Dropdown menu with items and dividers |
| [`NavTabs`] | Tabbed navigation container |
| [`NavTab`] | Individual tab with active state |
| [`NavTabPanel`] | Content panel for tabs |
| [`Pagination`] | Page navigation controls |
| [`PageItem`] | Individual page indicator |
| [`PageLink`] | Clickable page link |

### Hooks

| Hook | Returns | Description |
|------|---------|-------------|
| `use_route_info()` | `RouteInfo<R>` | Current route, path, and query parameters |
| `use_is_active(route)` | `bool` | Whether the given route is currently active |
| `use_is_exact_active(route)` | `bool` | Whether the route matches exactly |
| `use_is_partial_active(route)` | `bool` | Whether the route is a prefix of the current path |
| `use_breadcrumbs()` | `Vec<BreadcrumbItem>` | Auto-generated breadcrumb trail from current route |

### Utilities

| Function | Description |
|----------|-------------|
| `is_absolute(path)` | Check if a path starts with `/` |
| `join_paths(a, b)` | Join two path segments safely |
| `normalize_path(path)` | Remove duplicate slashes and trailing slashes |
| `urlencoding_encode(s)` | Percent-encode a string for URLs |
| `urlencoding_decode(s)` | Decode a percent-encoded string |
| `handle_arrow_key(config, key)` | Keyboard navigation handler |
| `handle_home_end(config, key)` | Home/End key handler for navigation |

---

## CSS Integration

### Bootstrap 5

Works out of the box — `nav-link` and `active` are native Bootstrap classes.

```html
<ul class="nav nav-pills">
    <li class="nav-item">
        <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
        <!-- Renders: <a class="nav-link active" href="/">Home</a> -->
    </li>
</ul>
```

### Tailwind CSS

Define your own `nav-link` and `active` styles:

```css
.nav-link {
    @apply px-4 py-2 text-gray-600 hover:text-gray-900 transition-colors;
}
.nav-link.active {
    @apply text-blue-600 font-semibold border-b-2 border-blue-600;
}
```

---

## Architecture

```
yew-nav-link
├── nav_link          # Core NavLink component + Match enum
├── nav               # Primitives: NavList, NavItem, NavDivider
├── components        # UI: Badge, Dropdown, Icon, Tabs, Pagination
├── hooks             # Reactive: use_route_info, use_is_active, breadcrumbs
├── utils             # Path, URL, keyboard navigation utilities
├── attrs             # Type-safe attribute builders
├── errors            # NavError, NavResult types
└── macros            # Declarative: nav_link!, nav_list!, nav_tabs!
```

See the source code and inline documentation for detailed design documentation.

---

## Examples

Run the comprehensive interactive demo:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk

cd examples/comprehensive
trunk serve
```

Open http://127.0.0.1:8080 for live demos, code snippets, and architecture diagrams for every component.

---

## API Reference

### `NavLink<R>`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `to` | `R: Routable` | required | Target route |
| `children` | `Children` | required | Link content |
| `partial` | `bool` | `false` | Enable prefix matching |

### `Match`

| Variant | Behavior |
|---------|----------|
| `Exact` | Active only on exact path match |
| `Partial` | Active when current path starts with target (segment-wise) |

### `nav_link<R>` Function

```rust
fn nav_link<R: Routable + PartialEq + Clone + 'static>(
    to: R,
    children: &str,
    match_mode: Match,
) -> Html
```

### `BreadcrumbItem`

```rust
pub struct BreadcrumbItem {
    pub label: String,
    pub route: Option<String>,
    pub is_current: bool,
}
```

---

## Migration Guides

| From | To | Guide |
|------|-----|-------|
| 0.5.x | 0.6.x | See [CHANGELOG.md](CHANGELOG.md) |
| 0.6.x | 0.7.x | See [CHANGELOG.md](CHANGELOG.md) |

---

<details>
<summary><h2>Coverage</h2></summary>

Target: **95%+** coverage, tracked via [Codecov](https://codecov.io/gh/RAprogramm/yew-nav-link).

### Sunburst

The inner-most circle is the entire project, moving outward are folders then individual files. Size and color represent statement count and coverage.

<p align="center">
  <a href="https://codecov.io/gh/RAprogramm/yew-nav-link">
    <img src="https://codecov.io/gh/RAprogramm/yew-nav-link/graphs/sunburst.svg" alt="Sunburst"/>
  </a>
</p>

### Grid

Each block represents a single file. Size and color represent statement count and coverage.

<p align="center">
  <a href="https://codecov.io/gh/RAprogramm/yew-nav-link">
    <img src="https://codecov.io/gh/RAprogramm/yew-nav-link/graphs/tree.svg" alt="Grid"/>
  </a>
</p>

### Icicle

Top section is the entire project, proceeding through folders to individual files.

<p align="center">
  <a href="https://codecov.io/gh/RAprogramm/yew-nav-link">
    <img src="https://codecov.io/gh/RAprogramm/yew-nav-link/graphs/icicle.svg" alt="Icicle"/>
  </a>
</p>

</details>

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the contribution workflow and code standards.

---

## License

Licensed under the [MIT License](LICENSE).
