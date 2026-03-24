# yew-nav-link Macro Implementation Summary

## Overview

This document summarizes the complete macro implementation for the yew-nav-link crate.

## File Structure

```
src/macros/
├── mod.rs                                  # Module declarations
├── README.md                               # User documentation
├── IMPLEMENTATION_PATTERNS.md              # Implementation guide
├── USAGE_GUIDE.md                         # User guide
├── ARCHITECTURE.md                        # Architecture details
├── IMPLEMENTATION_SUMMARY.md              # This file
└── derive/                                 # Derive macros
    ├── mod.rs
    ├── routable_ext.rs
    └── nav_item.rs
└── function/                               # Function macros
    ├── mod.rs
    └── nav_link_macro.rs
```

## Implementation Patterns

### 1. File Organization

- **Top-level**: Module declarations
- **derive/**: All derive macros
- **function/**: All function-like macros

### 2. Derive Macros

**Pattern**:
```rust
#[proc_macro_derive(Name, attributes(attr))]
pub fn name_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let span = input.ident.span();
    let expanded = quote_spanned! { span => ... };
    TokenStream::from(expanded)
}
```

**Key Points**:
- Use `quote_spanned!` for all code generation
- Validate input early with helpful errors
- Preserve source locations

### 3. Function-like Macros

**Pattern**:
```rust
#[proc_macro]
pub fn name(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as Args);
    let span = args.route.span();
    let expanded = quote_spanned! { span => ... };
    TokenStream::from(expanded)
}
```

**Key Points**:
- Parse arguments with custom types
- Validate input format
- Preserve spans in error messages

### 4. Error Messages

**Pattern**:
```rust
let error = quote_spanned! { span =>
    compile_error!("error: expected '/path', got '{path}'")
};
```

**Key Points**:
- Use `compile_error!` for errors
- Include helpful context
- Preserve original spans

## Safety Features

### 1. Compile-time Validation

```rust
// Check route format
if !route.starts_with('/') {
    compile_error!("route must start with '/'")
}

// Check route uniqueness
// (would require tracking all routes)
```

### 2. Type Safety

```rust
// Use generics with trait bounds
pub fn macro_with_route<T: Routable>(...) {
    // T must implement Routable
}
```

### 3. Input Validation

```rust
// Parse and validate input
let route = input.parse::<ExprPath>()?;
if !is_valid_path(&route) {
    compile_error!("invalid route format")
}
```

## Debug Spans

### 1. Span Preservation

```rust
let span = item.span();
quote_spanned! { span =>
    // Code with proper source locations
}
```

### 2. Multi-span Errors

```rust
let span1 = route.span();
let span2 = label.span();
quote_spanned! { span1 =>
    compile_error!("route at #1, label at #2", span2)
}
```

## Testing Strategy

### 1. Unit Tests

```rust
#[test]
fn test_parse_valid() {
    let input: ExprPath = parse_quote!(Route::Home);
    assert_eq!(input.span().line(), 10);
}
```

### 2. Integration Tests

```rust
#[test]
fn test_macro_expansion() {
    let code = r#"
        use yew_nav_link_macros::RoutableExt;
        use yew_router::prelude::Routable;
        
        #[derive(Routable, RoutableExt)]
        enum Route { ... }
    "#;
    
    expand_and_assert(code);
}
```

### 3. UI Tests

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

[dev-dependencies]
trybuild = "1.0"
```

## Error Messages Examples

### 1. Invalid Route Format

```
error: route must start with '/'
  --> src/main.rs:10:5
   |
10 |     #[at("home")]
   |     ^^
```

### 2. Duplicate Routes

```
error: duplicate route: /about
  --> src/main.rs:15:5
   |
15 |     #[at("/about")]
   |     ^^
```

### 3. Missing Attribute

```
error: missing required attribute: #[at]
  --> src/main.rs:8:10
   |
8  |     Home,
   |          ^
```

## Best Practices

1. **Use `quote_spanned!`** for all code generation
2. **Validate input early** to provide helpful errors
3. **Preserve spans** for accurate error locations
4. **Test comprehensively** with trybuild
5. **Document attributes** clearly for users
6. **Optimize for compilation speed** by minimizing complexity

## API Reference

### Derive Macros

- `RoutableExt`: Extended Routable with validation
- `NavItem`: Navigation item configuration

### Function-like Macros

- `nav_link!`: Create navigation links with validation

## Usage Examples

### RoutableExt

```rust
use yew_nav_link_macros::RoutableExt;
use yew_router::prelude::Routable;

#[derive(Routable, RoutableExt)]
enum Route {
    #[at("/")]
    Home,
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

## Comparison with Manual Implementation

### Before (Manual)

```rust
let link = NavLink::new()
    .to(Route::Home)
    .children("Home")
    .build();
```

### After (Macro)

```rust
nav_link!(Route::Home, "Home", Match::Exact)
```

## Performance Characteristics

1. **Compile Time**: Macros add to compilation time but improve runtime
2. **Code Size**: Generated code is optimized for size
3. **Runtime**: No overhead - all validation happens at compile time

##未来 Enhancements

1. Route inheritance
2. Wildcard route support
3. Parameter validation
4. Link generation from URLs
5. Middleware validation

## References

- [Syn Documentation](https://docs.rs/syn)
- [Quote Documentation](https://docs.rs/quote)
- [proc-macro2 Documentation](https://docs.rs/proc-macro2)
- [Rust Procedural Macros Workshop](https://github.com/dtolnay/proc-macro-workshop)
- [Trybuild](https://github.com/dtolnay/trybuild)
