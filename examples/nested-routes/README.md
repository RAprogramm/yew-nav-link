# Nested Routes Example

Multi-level navigation with main nav and sub-navigation tabs.

## What it demonstrates

- Multiple route enums (top-level + nested)
- NavLink at different routing levels
- Main nav stays active while sub-nav changes

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
nested-routes/
├── Cargo.toml
├── index.html
└── src/
    └── main.rs
```

## Key code

Multiple route enums:

```rust
// Top-level routes
enum Route {
    #[at("/")]
    Home,
    #[at("/docs/*")]
    Docs,
}

// Nested docs routes
enum DocsRoute {
    #[at("/docs")]
    Overview,
    #[at("/docs/getting-started")]
    GettingStarted,
}
```

Navigation at each level:

```rust
// Main nav - uses Route
<NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
<NavLink<Route> to={Route::DocsRoot}>{ "Docs" }</NavLink<Route>>

// Sub nav - uses DocsRoute
<NavLink<DocsRoute> to={DocsRoute::Overview}>{ "Overview" }</NavLink<DocsRoute>>
<NavLink<DocsRoute> to={DocsRoute::GettingStarted}>{ "Getting Started" }</NavLink<DocsRoute>>
```

## Result

- Click "Docs" → main nav shows "Docs" active
- Click sub-tabs → "Docs" stays active, sub-tab changes
