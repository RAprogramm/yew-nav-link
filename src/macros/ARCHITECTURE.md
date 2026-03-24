# Macro Architecture for yew-nav-link

## Overview

This document describes the architecture and implementation patterns used in the yew-nav-link procedural macros.

## Architecture

```
src/macros/
├── mod.rs                    # Module declarations
├── README.md                 # User documentation
├── IMPLEMENTATION_PATTERNS.md # Implementation guide
├── USAGE_GUIDE.md           # User guide
└── ARCHITECTURE.md          # This file
└── derive/                   # Derive macros directory
    ├── mod.rs                # Re-exports
    ├── routable_ext.rs       # RoutableExt derive
    └── nav_item.rs           # NavItem derive
└── function/                 # Function macros directory
    ├── mod.rs                # Re-exports
    └── nav_link_macro.rs     # nav_link! macro
```

## Design Philosophy

### 1. Simplicity

- Minimal API surface area
- Clear error messages
- Intuitive naming

### 2. Safety

- Compile-time validation where possible
- Type-safe route handling
- Span preservation for errors

### 3. Performance

- Zero runtime overhead
- Efficient code generation
- Minimal dependencies

### 4. Extensibility

- Easy to add new macros
- Consistent patterns
- Well-documented

## Component Breakdown

### Derive Macros

#### RoutableExt

**Purpose**: Extend Routable with validation

**Features**:
- Compile-time validation
- Debug span support
- Extended API

**Usage**:
```rust
#[derive(Routable, RoutableExt)]
enum Route { ... }
```

#### NavItem

**Purpose**: Configure navigation items

**Features**:
- Automatic path generation
- Validation
- Configuration

**Usage**:
```rust
#[derive(NavItem, Routable)]
#[nav_item(root = "/")]
enum Route { ... }
```

### Function-like Macros

#### nav_link

**Purpose**: Create navigation links

**Features**:
- Compile-time validation
- Simplified syntax
- Type safety

**Usage**:
```rust
nav_link!(Route::Home, "Home", Match::Exact)
```

## Error Handling

### 1. Parse Errors

```rust
match parse_macro_input!(input as DeriveInput) {
    Ok(input) => process(input),
    Err(e) => return e.to_compile_error(),
}
```

### 2. Validation Errors

```rust
if !is_valid_route(&route) {
    let error = quote_spanned! { span =>
        compile_error!("invalid route format")
    };
    return TokenStream::from(error);
}
```

### 3. span Preservation

```rust
let span = item.span();
quote_spanned! { span =>
    // Your code here
}
```

## Type Safety

### 1. Trait Bounds

```rust
pub fn macro_with_route<T: Routable>(...) {
    // T must implement Routable
}
```

### 2. Generic Constraints

```rust
pub fn validate_routes<R: Routable>(route: R) {
    // Compile-time check
}
```

## Code Generation

### 1. Token Stream Generation

```rust
let expanded = quote! {
    impl #name {
        // Generated code
    }
};
```

### 2. Span Tracking

```rust
let span = item.span();
quote_spanned! { span =>
    // Code with proper source locations
}
```

## Testing Strategy

### 1. Unit Tests

```rust
#[test]
fn test_parse_input() {
    let input = parse_quote!(Route::Home);
    assert!(input.is_correct());
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

## Performance Considerations

### 1. Compilation Time

- Minimize macro complexity
- Use efficient parsing
- Avoid unnecessary code generation

### 2. Runtime Performance

- Zero overhead abstractions
- Compile-time validation
- Efficient code generation

## Future Enhancements

1. **Route Inheritance**: Support for nested route structures
2. **Wildcard Routes**: Compile-time wildcard validation
3. **Parameter Validation**: Type-safe parameter extraction
4. **Link Generation**: Generate routes from URLs and vice versa
5. **Middleware Support**: Compile-time middleware validation

## References

- [Syn Documentation](https://docs.rs/syn)
- [Quote Documentation](https://docs.rs/quote)
- [proc-macro2 Documentation](https://docs.rs/proc-macro2)
- [Rust Procedural Macros Workshop](https://github.com/dtolnay/proc-macro-workshop)
- [Trybuild](https://github.com/dtolnay/trybuild)
