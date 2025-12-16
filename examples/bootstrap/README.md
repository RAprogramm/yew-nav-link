# Bootstrap Example

NavLink integration with Bootstrap 5 navbar.

## What it demonstrates

- NavLink works with Bootstrap out of the box
- `nav-link` and `active` classes are Bootstrap-compatible

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
bootstrap/
├── Cargo.toml
├── index.html    # Includes Bootstrap CDN
└── src/
    └── main.rs
```

## Key code

```rust
// Bootstrap navbar structure
<nav class="navbar navbar-expand-lg navbar-dark bg-primary">
    <ul class="navbar-nav">
        <li class="nav-item">
            // NavLink outputs: <a class="nav-link active" href="/">
            <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
        </li>
    </ul>
</nav>
```

## Why it works

NavLink always adds `nav-link` class, and `active` when route matches - exactly what Bootstrap expects.

No extra configuration needed.
