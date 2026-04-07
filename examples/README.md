# Examples

## Comprehensive Demo

A single comprehensive interactive demo showcases every component, hook, and pattern in the library.

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk

cd comprehensive
trunk serve
```

Open http://127.0.0.1:8080

## What's Included

The comprehensive demo includes documentation and live examples for:

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
- **Basic** — Simple 3-page app tutorial
- **Bootstrap 5** — Navbar, pills, tabs, vertical sidebar
- **Tailwind CSS** — Dashboard sidebar with @apply
- **Nested Routes** — Multi-level routing with partial matching
- **With Macros** — Declarative navigation with compile-time code generation
