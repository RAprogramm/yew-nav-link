# Basic Example

Simple navigation demo with Home, About, Contact pages.

## What it demonstrates

- `NavLink<Route>` component syntax
- `nav_link()` function syntax
- Automatic `active` class on current route

## Run

```bash
# Prerequisites (once)
rustup target add wasm32-unknown-unknown
cargo install trunk

# Run
trunk serve
```

Open http://127.0.0.1:8080

## Project structure

```
basic/
├── Cargo.toml    # Dependencies
├── index.html    # HTML template with styles
└── src/
    └── main.rs   # App code
```

## Key code

```rust
use yew_nav_link::{NavLink, nav_link};

// Component syntax
<NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>

// Function syntax
{ nav_link(Route::About, "About") }
```

## CSS

NavLink renders `<a class="nav-link">` (or `<a class="nav-link active">` when route matches).

Styles in `index.html`:

```css
.nav-link { color: #ecf0f1; }
.nav-link.active { background: #3498db; }
```
