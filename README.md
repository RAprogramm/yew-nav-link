<a id="top"></a>

# yew-nav-link

Navigation link component for [Yew](https://yew.rs) with automatic active state detection.

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/yew-nav-link)](https://crates.io/crates/yew-nav-link)
[![docs.rs](https://img.shields.io/docsrs/yew-nav-link)](https://docs.rs/yew-nav-link)
[![Downloads](https://img.shields.io/crates/d/yew-nav-link)](https://crates.io/crates/yew-nav-link)
[![MSRV](https://img.shields.io/crates/msrv/yew-nav-link)](https://crates.io/crates/yew-nav-link)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![REUSE](https://api.reuse.software/badge/github.com/RAprogramm/yew-nav-link)](https://api.reuse.software/info/github.com/RAprogramm/yew-nav-link)
[![Codecov](https://codecov.io/gh/RAprogramm/yew-nav-link/branch/main/graph/badge.svg)](https://codecov.io/gh/RAprogramm/yew-nav-link)

</div>

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Requirements](#requirements)
- [Examples](#examples)
- [Usage](#usage)
  - [Component Syntax](#component-syntax)
  - [Function Syntax](#function-syntax)
- [CSS Classes](#css-classes)
  - [Bootstrap Integration](#bootstrap-integration)
  - [Tailwind CSS](#tailwind-css)
- [API Reference](#api-reference)
- [Coverage](#coverage)
- [License](#license)

## Overview

`yew-nav-link` provides a `NavLink` component that wraps Yew Router's `Link` with automatic active state management. When the target route matches the current URL, an `active` CSS class is applied automatically.

<div align="right"><a href="#top">↑ top</a></div>

## Installation

```toml
[dependencies]
yew-nav-link = "0.3"
```

<div align="right"><a href="#top">↑ top</a></div>

## Requirements

| Dependency | Version |
|------------|---------|
| yew | 0.22+ |
| yew-router | 0.19+ |

<div align="right"><a href="#top">↑ top</a></div>

## Examples

Full working examples are available in the [examples/](https://github.com/RAprogramm/yew-nav-link/tree/main/examples) directory:

| Example | Description |
|---------|-------------|
| [basic](https://github.com/RAprogramm/yew-nav-link/tree/main/examples/basic) | Simple navigation with Home, About, Contact pages |
| [bootstrap](https://github.com/RAprogramm/yew-nav-link/tree/main/examples/bootstrap) | Integration with Bootstrap 5 navbar |
| [tailwind](https://github.com/RAprogramm/yew-nav-link/tree/main/examples/tailwind) | Sidebar navigation styled with Tailwind CSS |
| [nested-routes](https://github.com/RAprogramm/yew-nav-link/tree/main/examples/nested-routes) | Multi-level navigation with nested routing |

### Running Examples

```bash
cd examples/basic
trunk serve
```

Open http://localhost:8080 in your browser.

<div align="right"><a href="#top">↑ top</a></div>

## Usage

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
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Navigation />
            <main>
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
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

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::About => html! { <h1>{ "About" }</h1> },
    }
}
```

### Function Syntax

For text-only links, use the `nav_link` helper:

```rust
use yew::prelude::*;
use yew_nav_link::nav_link;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
}

#[component]
fn Menu() -> Html {
    html! {
        <ul class="nav">
            <li>{ nav_link(Route::Home, "Home") }</li>
            <li>{ nav_link(Route::About, "About") }</li>
        </ul>
    }
}
```

<div align="right"><a href="#top">↑ top</a></div>

## CSS Classes

The component applies these classes to the rendered `<a>` element:

| Class | When Applied |
|-------|--------------|
| `nav-link` | Always |
| `active` | Route matches current URL |

### Bootstrap Integration

```html
<ul class="nav nav-pills">
    <li class="nav-item">
        <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
        <!-- Renders: <a class="nav-link active" href="/">Home</a> -->
    </li>
</ul>
```

### Tailwind CSS

```css
.nav-link {
    @apply px-4 py-2 text-gray-600 hover:text-gray-900;
}
.nav-link.active {
    @apply text-blue-600 font-semibold;
}
```

<div align="right"><a href="#top">↑ top</a></div>

## API Reference

### `NavLink<R>` Component

| Prop | Type | Description |
|------|------|-------------|
| `to` | `R: Routable` | Target route |
| `children` | `Children` | Link content |

### `nav_link<R>` Function

```rust
fn nav_link<R: Routable>(to: R, children: &str) -> Html
```

Creates a `NavLink` with text content.

<div align="right"><a href="#top">↑ top</a></div>

<details>
<summary><h2>Coverage</h2></summary>

Code coverage is tracked via [Codecov](https://codecov.io/gh/RAprogramm/yew-nav-link). Target: **95%+** coverage.

### Sunburst

The inner-most circle is the entire project, moving away from the center are folders then, finally, a single file. The size and color of each slice represents the number of statements and the coverage, respectively.

<p align="center">
  <a href="https://codecov.io/gh/RAprogramm/yew-nav-link">
    <img src="https://codecov.io/gh/RAprogramm/yew-nav-link/graphs/sunburst.svg?token=E93AERE3UC" alt="Sunburst"/>
  </a>
</p>

### Grid

Each block represents a single file in the project. The size and color of each block represents the number of statements and the coverage, respectively.

<p align="center">
  <a href="https://codecov.io/gh/RAprogramm/yew-nav-link">
    <img src="https://codecov.io/gh/RAprogramm/yew-nav-link/graphs/tree.svg?token=E93AERE3UC" alt="Grid"/>
  </a>
</p>

### Icicle

The top section represents the entire project. Proceeding with folders and finally individual files. The size and color of each slice represents the number of statements and the coverage, respectively.

<p align="center">
  <a href="https://codecov.io/gh/RAprogramm/yew-nav-link">
    <img src="https://codecov.io/gh/RAprogramm/yew-nav-link/graphs/icicle.svg?token=E93AERE3UC" alt="Icicle"/>
  </a>
</p>

<div align="right"><a href="#top">↑ top</a></div>

</details>

## License

Licensed under the [MIT License](LICENSE-MIT).

<div align="right"><a href="#top">↑ top</a></div>
