# Examples

## Quick Start

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk

# No macros example (full syntax)
cd no-macros
trunk serve

# With macros example (reduced syntax)
cd with-macros
trunk serve
```

Open http://127.0.0.1:8080

## Examples

| Example | Description |
|---------|-------------|
| [no-macros](./no-macros) | **Full syntax** - No procedural macros, explicit HTML structure |
| [with-macros](./with-macros) | **Macros version** - Uses procedural macros for reduced code |
| [basic](./basic) | Simple navigation with component and function syntax |
| [bootstrap](./bootstrap) | Bootstrap 5 navbar integration |
| [tailwind](./tailwind) | Sidebar with Tailwind CSS styling |
| [nested-routes](./nested-routes) | Multi-level navigation |

Each example has its own README with details.

## Code Comparison

| Approach | Lines for 3 Links | Macros |
|----------|------------------|--------|
| **no-macros** | ~12 lines | 0 |
| **with-macros** | ~3 lines | `nav_list!` |

The macros version reduces code by ~75%!
