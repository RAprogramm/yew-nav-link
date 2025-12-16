# Tailwind CSS Example

Dashboard with sidebar navigation styled using Tailwind CSS.

## What it demonstrates

- Custom styling for `nav-link` and `active` classes
- Using Tailwind's `@apply` directive

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
tailwind/
├── Cargo.toml
├── index.html    # Tailwind CDN + custom styles
└── src/
    └── main.rs
```

## Key code

Styling in `index.html`:

```css
.nav-link {
    @apply px-4 py-2 rounded-lg text-gray-300
           hover:text-white hover:bg-gray-700 transition-colors;
}

.nav-link.active {
    @apply bg-indigo-600 text-white font-medium;
}
```

Usage in Rust:

```rust
<nav class="space-y-2">
    <NavLink<Route> to={Route::Dashboard}>{ "Dashboard" }</NavLink<Route>>
    <NavLink<Route> to={Route::Settings}>{ "Settings" }</NavLink<Route>>
</nav>
```

## Result

- Inactive links: gray text, hover effect
- Active link: indigo background, white text, bold
