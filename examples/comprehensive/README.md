# yew-nav-link — Comprehensive Demo

Interactive documentation and showcase of every component, hook, and pattern in the library.

## What's Included

### Components
- **NavLink** — Core navigation link with automatic active state
- **NavList & NavItem** — Accessible navigation list container
- **NavDivider** — Visual separator for navigation groups
- **Badge / Header / Text** — Navigation metadata components
- **NavDropdown** — Dropdown menu component
- **NavIcon** — Icon integration
- **NavTabs / NavTab** — Tabbed navigation
- **Pagination** — Page navigation controls

### Hooks & Utilities
- **Route Hooks** — use_route_info, use_is_active, use_is_exact_active, use_is_partial_active
- **Breadcrumbs** — Breadcrumb trail generation
- **Path Utilities** — Path manipulation helpers

### Macros
- **nav_list!** — Generate navigation lists
- **nav_links!** — Generate multiple links
- **nav_menu!** — Generate sidebar menus
- **nav_tabs!** — Generate tab navigation
- **breadcrumbs!** — Generate breadcrumbs
- **nav_pagination!** — Generate pagination controls

### Integration Patterns
- **Basic** — Step-by-step tutorial from `cargo new` to working app
- **Bootstrap 5** — Navbar, pills, tabs, vertical sidebar
- **Tailwind CSS** — Dashboard sidebar with @apply directives
- **Nested Routes** — Multi-level routing with partial matching
- **With Macros** — Declarative navigation with compile-time code generation

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
comprehensive/
├── Cargo.toml
├── index.html
├── styles/
│   └── main.css
└── src/
    ├── main.rs           # App entry point, sidebar, routing
    ├── routes.rs         # Route enum for the demo
    ├── code_utils.rs     # Copy-to-clipboard utilities
    ├── demo_popup.rs     # Tooltip overlay for live demos
    ├── doc_page.rs       # Documentation page template
    ├── doc_parser.rs     # Doc comment parser & renderer
    ├── file_tree.rs      # File tree component
    └── pages/            # Individual documentation pages
        ├── home/
        ├── basic_example.rs
        ├── bootstrap_example.rs
        ├── tailwind_example.rs
        ├── nested_routes_example.rs
        ├── with_macros_example.rs
        ├── examples_home.rs
        └── ... (component docs)
```
