# Examples

Working examples demonstrating `yew-nav-link` usage.

## Prerequisites

1. Add WebAssembly target:

```bash
rustup target add wasm32-unknown-unknown
```

2. Install [Trunk](https://trunkrs.dev/) - bundler for Yew/WASM apps:

```bash
cargo install trunk
```

## Available Examples

### basic

Simple navigation with Home, About, Contact pages. Shows both component and function syntax.

```bash
cd basic && trunk serve
```

### bootstrap

Integration with Bootstrap 5 navbar. Demonstrates how `nav-link` and `active` classes work seamlessly with Bootstrap.

```bash
cd bootstrap && trunk serve
```

### tailwind

Dashboard layout with sidebar navigation styled using Tailwind CSS. Shows how to customize `nav-link` and `active` classes with `@apply`.

```bash
cd tailwind && trunk serve
```

### nested-routes

Multi-level navigation with nested routing. Main navigation stays active while sub-navigation tabs change based on section.

```bash
cd nested-routes && trunk serve
```

## Running

1. Navigate to example directory
2. Run `trunk serve`
3. Open http://localhost:8080
