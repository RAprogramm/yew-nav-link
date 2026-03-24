# Procedural Macros for yew-nav-link

## Overview

This directory contains the implementation of procedural macros for the `yew-nav-link` crate.

## Recommended Macro Structure

```
src/macros/
├── mod.rs                      # Module declarations
├── README.md                   # User-facing documentation
├── IMPLEMENTATION_PATTERNS.md  # Implementation details
└── derive/                     # Derive macros
    ├── mod.rs
    ├── routable_ext.rs         # Extended Routable derive
    └── nav_item.rs             # Navigation item derive
└── function/                   # Function-like macros
    ├── mod.rs
    └── nav_link_macro.rs       # Navigation link macro
```

## Implementation Patterns

### 1. Derive Macros

Derive macros extend Rust's `#[derive()]` system. They're ideal for:

- Adding trait implementations
- Compile-time validation
- Code generation based on type structure

### 2. Function-like Macros

Function-like macros provide convenient syntax for common patterns:

- Route generation
- Component configuration
- Link creation

## Best Practices

### 1. Error Messages

Always use `quote_spanned!` to preserve source locations:

```rust
let span = ident.span();
let error = quote_spanned! { span =>
    compile_error!("invalid route: expected format '/path'")
};
```

### 2. Debug Spans

Preserve spans for better debugging:

```rust
# [proc_macro_derive(MyDerive)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let span = input.ident.span();
    
    let expanded = quote_spanned! { span =>
        // Your code here
    };
    
    TokenStream::from(expanded)
}
```

### 3. Type Safety

Leverage Rust's type system:

```rust
// Use generics with trait bounds
pub fn my_macro<T: Routable>(...) {
    // T must implement Routable
}
```

## File Structure

### Derive Macros (`derive/`)

- `routable_ext.rs`: Extended Routable derive with validation
- `nav_item.rs`: Navigation item configuration derive

### Function-like Macros (`function/`)

- `nav_link_macro.rs`: Navigation link creation macro

## Testing

Use `trybuild` for UI tests:

```toml
[dev-dependencies]
trybuild = "1.0"
```

```rust
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/compile_fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
```

## Dependencies

```toml
[dependencies]
proc-macro2 = "1.0"
quote = "1.0"
syn = { version = "2.0", features = ["full"] }
```

## Usage Examples

### RoutableExt

```rust
use yew_nav_link_macros::RoutableExt;
use yew_router::prelude::Routable;

#[derive(Routable, RoutableExt)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
}
```

### NavItem

```rust
use yew_nav_link_macros::NavItem;
use yew_router::prelude::Routable;

#[derive(NavItem, Routable)]
enum Route {
    #[at("/")]
    Home,
}
```

### nav_link

```rust
use yew_nav_link_macros::nav_link;

nav_link!(Route::Home, "Home", Match::Exact)
```

## Error Messages

The macros provide helpful error messages with proper spans:

```
error: route must start with '/'
  --> src/main.rs:10:5
   |
10 |     #[at("home")]
   |     ^^

error: duplicate route: /about
  --> src/main.rs:15:5
   |
15 |     #[at("/about")]
   |     ^^
```

## API Reference

### Derive Macros

- `RoutableExt`: Extended Routable with compile-time validation
- `NavItem`: Navigation item configuration

### Function-like Macros

- `nav_link!`: Create navigation links with validation

## Contributing

1. Follow the file structure
2. Use `quote_spanned!` for all code generation
3. Add comprehensive error messages
4. Test with trybuild
5. Document attributes clearly

## References

- [Syn Documentation](https://docs.rs/syn)
- [Quote Documentation](https://docs.rs/quote)
- [proc-macro2 Documentation](https://docs.rs/proc-macro2)
- [Trybuild](https://github.com/dtolnay/trybuild)
- [Rust Procedural Macros Workshop](https://github.com/dtolnay/proc-macro-workshop)
