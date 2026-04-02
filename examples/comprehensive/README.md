# No Macros Example

This example demonstrates yew-nav-link using the **full syntax** without any procedural macros.

## Code Comparison

| Approach | Lines for 3 Links | Macros Used |
|----------|------------------|-------------|
| **This example (no macros)** | ~12 lines | 0 |
| **with-macros example** | ~3 lines | `nav_list!` |

## Features Demonstrated

- NavLink component with explicit HTML structure
- NavList, NavItem, NavDivider components
- nav_link function for text-only links
- Route hooks (use_route_info, use_is_active, etc.)
- Match::Exact and Match::Partial modes

## Run

```bash
# Prerequisites (once)
rustup target add wasm32-unknown-unknown
cargo install trunk

# Run
trunk serve
```

Open http://127.0.0.1:8080

## Project Structure

```
no-macros/
├── Cargo.toml
├── index.html
└── src/
    ├── main.rs
    ├── routes.rs
    └── components/
        ├── mod.rs
        ├── home.rs
        ├── links.rs
        ├── lists.rs
        ├── container.rs
        ├── hooks.rs
        └── not_found.rs
```
