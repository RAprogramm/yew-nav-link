# WASM Tests for yew-nav-link

These tests verify the library works correctly in WebAssembly environment.

## Running WASM Tests

```bash
# Install wasm-pack if not already
cargo install wasm-pack

# Run WASM tests
wasm-pack test --node
```

## Test Categories

### Navigation Link Tests
Tests for NavLink component active state detection in WASM context.

```rust
#[wasm_bindgen_test]
fn test_navlink_renders_in_wasm() {
    // Verify NavLink renders without panics in WASM
}
```

### Component Rendering Tests
Verify all components render correctly in browser environment.

### Router Integration Tests
Test navigation works with yew-router in WASM context.