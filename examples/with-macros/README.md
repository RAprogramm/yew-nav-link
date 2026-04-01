# With Macros Example

This example demonstrates yew-nav-link using **procedural macros** to reduce boilerplate code.

## Code Comparison

| Approach | Lines for 3 Links | Macros Used |
|----------|------------------|-------------|
| **no-macros example** | ~12 lines | 0 |
| **This example (with macros)** | ~3 lines | `nav_list!`, `nav_links!` |

## Macros Available

| Macro | Purpose | Example |
|-------|---------|---------|
| `nav_list!` | Create navigation lists | `{ nav_list! { (Route::Home, "Home", Match::Exact) } }` |
| `nav_links!` | Create multiple links | `{ nav_links! { (Route::Home, "Home", Match::Exact) } }` |
| `nav_menu!` | Create nested menus | `{ nav_menu! { (Route::Home, "Home", Match::Partial, { ... }) } }` |
| `nav_tabs!` | Create tabbed navigation | `{ nav_tabs! { (Route::Tab1, "Tab 1") } }` |
| `breadcrumbs!` | Create breadcrumbs | `{ breadcrumbs! { (Route::Home, "Home") } }` |
| `nav_pagination!` | Create pagination | `{ nav_pagination! { current_page = 1, total_pages = 10 } }` |

## Features Demonstrated

- NavLink component with full HTML structure
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
with-macros/
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

## Benefits of Macros

1. **Less Boilerplate**: No `<ul>`, `<li>`, `<NavLink>` tags needed
2. **Cleaner Code**: Single line per link instead of 3-4
3. **Type Safety**: Macros validate routes at compile time
4. **Consistency**: Ensures proper class names and attributes
