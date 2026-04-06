# Procedural Macros for yew-nav-link

## Overview

This directory contains the implementation of procedural macros for the `yew-nav-link` crate.

**Note**: The procedural macros in this directory require a separate proc-macro crate with `crate-type = ["proc-macro"]`. They are not currently compiled as part of the main library.

## Current Status

The following macros are available in the main crate via the `macros` feature:

- `nav_link!` - Convenience wrapper for the `nav_link()` function

The following macros are stubs (not yet implemented):

- `routable_ext!` - Would generate route validation methods
- `nav_item!` - Would generate navigation path methods

## Actual Structure

```
src/macros/
├── lib.rs                      # Module root
├── README.md                   # This file
├── USAGE_GUIDE.md             # Usage documentation
├── IMPLEMENTATION_PATTERNS.md # Implementation details (stale)
├── IMPLEMENTATION_SUMMARY.md  # Summary (stale)
├── ARCHITECTURE.md            # Architecture doc (stale)
├── function/                   # Function-like macro implementations
│   └── mod.rs
└── macros/                     # Procedural macros (require proc-macro crate)
    ├── mod.rs
    ├── breadcrumbs.rs
    ├── link.rs
    ├── list.rs
    ├── menu.rs
    ├── pagination.rs
    └── tabs.rs
```

## Usage

Enable the `macros` feature in your `Cargo.toml`:

```toml
[dependencies]
yew-nav-link = { version = "0.6", features = ["macros"] }
```

Then use the declarative macro:

```rust
use yew_nav_link::nav_link;

html! {
    { nav_link(Route::Home, "Home", Match::Exact) }
}
```