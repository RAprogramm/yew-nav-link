<a id="top"></a>

# yew-nav-link

Enterprise-grade navigation library for [Yew](https://yew.rs) — automatic active state detection and a complete component system.

<div align="center">

[**🌐 Live demo →**](https://raprogramm.github.io/yew-nav-link/) — [**📖 Architectural book →**](https://raprogramm.github.io/yew-nav-link/book/)

[![CI](https://github.com/RAprogramm/yew-nav-link/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/RAprogramm/yew-nav-link/actions/workflows/ci.yml)
[![Pages](https://github.com/RAprogramm/yew-nav-link/actions/workflows/pages.yml/badge.svg?branch=main)](https://github.com/RAprogramm/yew-nav-link/actions/workflows/pages.yml)
[![Mutants](https://github.com/RAprogramm/yew-nav-link/actions/workflows/mutants.yml/badge.svg?branch=main)](https://github.com/RAprogramm/yew-nav-link/actions/workflows/mutants.yml)
[![Crates.io](https://img.shields.io/crates/v/yew-nav-link)](https://crates.io/crates/yew-nav-link)
[![docs.rs](https://img.shields.io/docsrs/yew-nav-link)](https://docs.rs/yew-nav-link)
[![Downloads](https://img.shields.io/crates/d/yew-nav-link)](https://crates.io/crates/yew-nav-link)
[![MSRV](https://img.shields.io/crates/msrv/yew-nav-link)](https://crates.io/crates/yew-nav-link)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![REUSE](https://api.reuse.software/badge/github.com/RAprogramm/yew-nav-link)](https://api.reuse.software/info/github.com/RAprogramm/yew-nav-link)
[![OSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/RAprogramm/yew-nav-link/badge)](https://securityscorecards.dev/viewer/?uri=github.com/RAprogramm/yew-nav-link)
[![Codecov](https://codecov.io/gh/RAprogramm/yew-nav-link/branch/main/graph/badge.svg)](https://codecov.io/gh/RAprogramm/yew-nav-link)

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
| **Hooks** | Reactive hooks for route state, active checking, breadcrumbs, and programmatic navigation |
| **Utilities** | Path manipulation, URL encoding, keyboard navigation, and query string handling |
| **Customization** | Custom CSS classes, programmatic navigation, and extensible breadcrumb providers |

The core `NavLink` component eliminates manual active state tracking. It compares the current route against the target on every render and applies the `active` CSS class automatically — zero configuration required.

<p align="right">(<a href="#top">back to top</a>)</p>

---

## Installation

```toml
[dependencies]
yew-nav-link = "0.9"
```

<p align="right">(<a href="#top">back to top</a>)</p>

## Requirements

| Dependency | Version |
|------------|---------|
| Rust | 1.96+ |
| Edition | 2024 |
| Yew | 0.23+ |
| yew-router | 0.20+ |

<p align="right">(<a href="#top">back to top</a>)</p>

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

### Custom CSS Classes

Customize the default `nav-link` and `active` classes:

```rust
html! {
    <nav>
        // Custom base class
        <NavLink<Route> to={Route::Home} class="menu-item">{ "Home" }</NavLink<Route>>
        
        // Custom active class
        <NavLink<Route> to={Route::About} active_class="is-selected">{ "About" }</NavLink<Route>>
        
        // Both custom
        <NavLink<Route> to={Route::Contact} class="sidebar-link" active_class="highlighted">{ "Contact" }</NavLink<Route>>
    </nav>
}
```

### Programmatic Navigation

`use_navigation::<R>()` returns a [`Navigation<R>`] handle exposing pre-built `Callback`s — no manual `Callback::from(...)` boilerplate.

```rust
use yew::prelude::*;
use yew_nav_link::use_navigation;

#[component]
fn MyComponent() -> Html {
    let nav = use_navigation::<Route>();

    html! {
        <>
            // Push a new entry onto the history stack.
            <button onclick={nav.push_callback(Route::About).reform(|_: MouseEvent| ())}>
                { "Go to About" }
            </button>

            // Replace the current entry without growing history.
            <button onclick={nav.replace_callback(Route::Home).reform(|_: MouseEvent| ())}>
                { "Replace with Home" }
            </button>

            // Browser back / forward.
            <button onclick={nav.go_back.reform(|_: MouseEvent| ())}>{ "Back" }</button>
            <button onclick={nav.go_forward.reform(|_: MouseEvent| ())}>{ "Forward" }</button>
        </>
    }
}
```

### Custom Breadcrumb Providers

Implement [`BreadcrumbLabelProvider`] to control how each path segment is rendered. The provider operates on **paths** (e.g. `/docs/api`), not on `Routable` enum variants — it works the same for static and parameterised routes.

```rust
use std::rc::Rc;
use yew_nav_link::{BreadcrumbLabelProvider, use_breadcrumbs};

struct MyLabels;

impl BreadcrumbLabelProvider for MyLabels {
    fn label_for_path(&self, path: &str) -> String {
        match path {
            "/" => "Home".into(),
            "/about" => "About us".into(),
            p if p.starts_with("/users/") => format!("User {}", &p[7..]),
            other => other.into(),
        }
    }
}

#[component]
fn Crumbs() -> Html {
    // Provide the implementation through context (omitted for brevity);
    // then read the trail.
    let trail = use_breadcrumbs::<Route>();

    html! {
        <nav aria-label="Breadcrumb">
            { for trail.into_iter().map(|item| html! {
                <span aria-current={if item.is_active { "page" } else { "" }}>
                    { item.label }
                </span>
            }) }
        </nav>
    }
}
```

<p align="right">(<a href="#top">back to top</a>)</p>

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
| `use_breadcrumbs()` | `Vec<BreadcrumbItem<R>>` | Auto-generated breadcrumb trail from current route |
| `use_navigation<R>()` | `Navigation<R>` | Programmatic navigation (push, replace, go back/forward) |
| `use_query_params()` | `HashMap<String, String>` | URL query string parameters |

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

<p align="right">(<a href="#top">back to top</a>)</p>

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

<p align="right">(<a href="#top">back to top</a>)</p>

---

## Architecture

```
yew-nav-link
├── active_link       # Core NavLink component + Match enum
├── nav               # Primitives: NavList, NavItem, NavDivider
├── components        # UI: Badge, Dropdown, Icon, Tabs, Pagination
├── hooks             # Reactive and programmatic route/navigation helpers
├── utils             # Path, URL, keyboard navigation utilities
├── attrs             # Type-safe attribute builders
└── errors            # NavError, NavResult types
```

For *why* the crate is shaped this way — the trade-offs picked over each
alternative — see [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for the
overview and [`docs/adr/`](docs/adr/) for individual architecture
decision records.

<p align="right">(<a href="#top">back to top</a>)</p>

---

## Examples

A **live demo** is published at <https://raprogramm.github.io/yew-nav-link/>. It exercises every component, hook, and utility in the public API.

Run the same demo locally:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk

cd example
trunk serve
```

Open <http://127.0.0.1:3000> (port set in [`example/trunk.toml`](example/trunk.toml)).

<p align="right">(<a href="#top">back to top</a>)</p>

---

## API Reference

### `NavLink<R>`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `to` | `R: Routable` | required | Target route |
| `children` | `Children` | required | Link content |
| `partial` | `bool` | `false` | Enable prefix matching |
| `class` | `&str` | `"nav-link"` | Custom CSS class (replaces default) |
| `active_class` | `&str` | `"active"` | Custom active state class |

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

<p align="right">(<a href="#top">back to top</a>)</p>

---

## Project documentation

| File | Purpose |
|------|---------|
| [Architectural book](https://raprogramm.github.io/yew-nav-link/book/) | Rendered mdBook of the documents below — search, syntax-highlighting, navigation. Source lives under `docs/`. |
| [`docs/REQUIREMENTS.md`](docs/REQUIREMENTS.md) | Functional and non-functional requirements (what the crate does, the constraints under which it does it) |
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | Module layout, the active-state algorithm, hook contracts, breadcrumb context flow |
| [`docs/ROADMAP.md`](docs/ROADMAP.md) | Trajectory through 0.10.x (current line) toward 1.0 (API freeze) |
| [`docs/BRANCHING.md`](docs/BRANCHING.md) | Branching, commit, and merge policy enforced on `main` |
| [`SECURITY.md`](SECURITY.md) | Coordinated disclosure policy |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Workflow, commit format, code standards |

<p align="right">(<a href="#top">back to top</a>)</p>

---

## Migration Guides

For a full release-by-release log, see [CHANGELOG.md](CHANGELOG.md).

| From | To | Notes |
|------|-----|-------|
| 0.8.x | 0.9.x | Macro feature removed; component/function APIs unchanged. See [CHANGELOG `[0.9.0]`](CHANGELOG.md). |
| 0.9.0 | 0.9.1 | Single-file SPA demo replaces the multi-page docs site under `example/`; library API unchanged. |
| 0.9.1 | 0.9.2 | `BreadcrumbLabelProvider` now re-exported at the crate root. Drop-in upgrade. |

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

<p align="right">(<a href="#top">back to top</a>)</p>

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the contribution workflow and code standards.

<p align="right">(<a href="#top">back to top</a>)</p>

---

## License

Licensed under the [MIT License](LICENSE).

<p align="right">(<a href="#top">back to top</a>)</p>
