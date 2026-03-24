# Macro Usage Guide for yew-nav-link

## Overview

The `yew-nav-link` crate provides two categories of procedural macros:

1. **Derive Macros** - Extend Rust's derive system
2. **Function-like Macros** - Reduce boilerplate in function calls

## Derive Macros

### RoutableExt

Extends yew-router's `Routable` with additional functionality.

#### Example

```rust
use yew_nav_link_macros::RoutableExt;
use yew_router::prelude::Routable;

#[derive(Routable, RoutableExt)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/docs")]
    Docs,
}

// Use the extended functionality
let _ = Route::validate_routes();
Route::debug_routes();
```

#### Features

- Compile-time route validation
- Debug span support for better error messages
- Extended validation methods

### NavItem

Provides navigation item configuration with compile-time checks.

#### Example

```rust
use yew_nav_link_macros::NavItem;
use yew_router::prelude::Routable;

#[derive(NavItem, Routable, Clone, PartialEq)]
#[nav_item(root = "/")]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
}

// Get navigation path for a route
let path = Route::Home.nav_path();
```

## Function-like Macros

### nav_link

Creates navigation links with compile-time route validation.

#### Example

```rust
use yew_nav_link_macros::nav_link;
use yew_router::prelude::*;

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
}

// Compile-time validated navigation link
nav_link!(Route::Home, "Home", Match::Exact)
```

## Syntax Reference

### RoutableExt

```rust
#[derive(Routable, RoutableExt)]
enum YourRoute {
    #[at("/")]
    Variant,
}
```

### NavItem

```rust
#[derive(NavItem, Routable)]
#[nav_item(root = "/")]
enum YourRoute {
    #[at("/")]
    Variant,
}
```

### nav_link

```rust
nav_link!(Route::Variant, "Label", Match::Mode)
```

## Error Messages

### Invalid Route Format

```
error: route must start with '/'
  --> src/main.rs:10:5
   |
10 |     #[at("home")]
   |     ^^^^^^^^^^^^^
```

### Duplicate Routes

```
error: duplicate route: /about
  --> src/main.rs:15:5
   |
15 |     #[at("/about")]
   |     ^^^^^^^^^^^^^^^
```

### Missing Attribute

```
error: missing required attribute: #[at]
  --> src/main.rs:8:10
   |
8  |     Home,
   |          ^^^^^
```

## Best Practices

### 1. Always Use routable_ext

```rust
// Good
#[derive(Routable, RoutableExt)]

// Bad
#[derive(Routable)]
```

### 2. Validate Routes

```rust
// Good
#[derive(NavItem, Routable)]
#[nav_item(validate = true)]

// Bad
#[derive(NavItem, Routable)]
```

### 3. Use Function Macros for Simplified Syntax

```rust
// Good
let link = nav_link!(Route::Home, "Home", Match::Exact);

// Better for complex cases
let link = NavLink::new()
    .to(Route::Home)
    .children("Home")
    .build();
```

## Performance Considerations

1. **Compile Time**: Macros add to compilation time but improve runtime
2. **Code Size**: Generated code is optimized for size
3. **Memory**: Compile-time validation reduces runtime checks

## Troubleshooting

### Macro Expansion Issues

Use `cargo expand` to see the expanded code:

```bash
cargo install cargo-expand
cargo expand
```

### Compilation Errors

1. Check route format matches yew-router requirements
2. Ensure all variants have `#[at]` attributes
3. Verify route uniqueness

## Examples

See the `examples/` directory for complete working examples:

- `examples/basic/` - Simple navigation
- `examples/bootstrap/` - Bootstrap integration
- `examples/tailwind/` - Tailwind CSS integration

## Migration Guide

### From Manual Implementation

```rust
// Before
let link = NavLink::new()
    .to(Route::Home)
    .children("Home")
    .build();

// After
let link = nav_link!(Route::Home, "Home", Match::Exact);
```

### From Route Validation

```rust
// Before
let valid = route.validate();

// After
Route::validate_routes();
```

## FAQs

### Q: Do I need to use all macros?

A: No, use what fits your needs. The core `NavLink` component works without macros.

### Q: Do macros affect runtime performance?

A: No, macros generate code at compile time with no runtime overhead.

### Q: Can I customize the generated code?

A: Yes, use attributes to customize behavior.

## Future Enhancements

- [ ] Route inheritance
- [ ] Wildcard route support
- [ ] Parameter validation
- [ ] Link generation from URLs
- [ ] Middleware validation

## Support

For issues and questions:

- Check the documentation
- Review the examples
- Open an issue on GitHub
