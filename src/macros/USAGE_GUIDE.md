# Macro Usage Guide for yew-nav-link

## Overview

The `yew-nav-link` crate provides declarative macros (via the `macros` feature) to reduce boilerplate.

## Available Macros

### nav_link!

A convenience wrapper around the `nav_link()` function for declarative usage.

```rust
use yew_nav_link::{nav_link, Match};
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Debug, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/docs")]
    Docs,
}

// Usage in html!
#[component]
fn Navigation() -> Html {
    html! {
        <nav>
            { nav_link!(Route::Home, "Home", Match::Exact) }
            { nav_link!(Route::About, "About", Match::Exact) }
            { nav_link!(Route::Docs, "Docs", Match::Partial) }
        </nav>
    }
}
```

This is equivalent to:

```rust
html! {
    <nav>
        { nav_link(Route::Home, "Home", Match::Exact) }
        { nav_link(Route::About, "About", Match::Exact) }
        { nav_link(Route::Docs, "Docs", Match::Partial) }
    </nav>
}
```

## Stub Macros (Not Yet Implemented)

The following macros are documented but not yet implemented:

- `routable_ext!` - Would generate route validation methods
- `nav_item!` - Would generate navigation path methods

Using these macros will produce a compile-time error indicating they are not yet implemented.

## Procedural Macros (Requires Separate Crate)

The `src/macros/macros/` directory contains procedural macro implementations. These require a separate proc-macro crate with `crate-type = ["proc-macro"]` and are not currently compiled as part of the main library.

Available proc-macros:
- `nav_list!` - Generate navigation lists
- `nav_links!` - Generate multiple links
- `nav_menu!` - Generate sidebar menus
- `nav_tabs!` - Generate tab navigation
- `breadcrumbs!` - Generate breadcrumbs
- `nav_pagination!` - Generate pagination controls