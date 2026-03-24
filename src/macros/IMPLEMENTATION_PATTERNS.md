# Macro Implementation Patterns for yew-nav-link

## Table of Contents

1. [File Structure](#file-structure)
2. [Implementations Patterns](#implementation-patterns)
3. [Error Handling](#error-handling)
4. [Debug Spans](#debug-spans)
5. [Compile-time Safety](#compile-time-safety)
6. [Examples](#examples)

## File Structure

```
src/macros/
├── mod.rs                      # Module declarations
├── README.md                   # User-facing documentation
├── IMPLEMENTATION_PATTERNS.md  # This file - implementation details
└── derive/                     # Derive macros
    ├── mod.rs
    ├── routable_ext.rs         # Extended Routable derive
    └── nav_item.rs             # Navigation item derive
└── function/                   # Function-like macros
    ├── mod.rs
    └── nav_link_macro.rs       # Navigation link macro
```

## Implementation Patterns

### 1. Macro Entry Points

All macros follow a consistent pattern:

```rust
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(MyMacro)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // Process input ...
    let expanded = quote! {
        // Generated code
    };
    TokenStream::from(expanded)
}
```

### 2. Error Handling

Use `compile_error!` with span information for the best user experience:

```rust
let span = ident.span();
let error = quote_spanned! { span =>
    compile_error! {
        "invalid route: expected format '/path', but got 'route'"
    }
};
```

### 3. Debug Spans

Preserve source locations with `quote_spanned!`:

```rust
let span = item.span();
let expanded = quote_spanned! { span =>
    // Your code here - errors will point to correct location
};
```

### 4. Pattern Matching on Token Streams

Use Syn's parsing combinators:

```rust
use syn::{parse::Parse, parse::ParseStream, Ident};

struct MyStruct {
    name: Ident,
    value: Expr,
}

impl Parse for MyStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<syn::Token![=]>()?;
        let value = input.parse::<Expr>()?;
        Ok(MyStruct { name, value })
    }
}
```

## Error Handling Patterns

### 1. Route Validation Errors

```rust
fn validate_route_format(route: &str, span: Span) -> Result<(), TokenStream> {
    if !route.starts_with('/') {
        let error = quote_spanned! { span =>
            compile_error!("route must start with '/'");
        };
        return Err(TokenStream::from(error));
    }
    Ok(())
}
```

### 2. Duplicate Route Detection

```rust
fn check_duplicates(routes: &[String], span: Span) -> Result<(), TokenStream> {
    let mut seen = std::collections::HashSet::new();
    for route in routes {
        if !seen.insert(route) {
            let error = quote_spanned! { span =>
                compile_error!("duplicate route: {}", route);
            };
            return Err(TokenStream::from(error));
        }
    }
    Ok(())
}
```

### 3. Missing Attribute Errors

```rust
fn get_attribute(attr_name: &str, attrs: &[Attribute], span: Span) -> Result<(), TokenStream> {
    if !attrs.iter().any(|a| a.path().is_ident(attr_name)) {
        let error = quote_spanned! { span =>
            compile_error!("missing required attribute: #[{}]", attr_name);
        };
        return Err(TokenStream::from(error));
    }
    Ok(())
}
```

## Compile-time Safety Patterns

### 1. Generic Constraints

```rust
pub fn macro_with_constraints<T: Routable + PartialEq + Clone>(...) {
    // Compile-time check: T must implement Routable
}
```

### 2. Type-level Programming

```rust
trait RouteValidator {
    fn validate_routes();
}

impl<T: Routable> RouteValidator for T {
    fn validate_routes() {
        // Compile-time validation
    }
}
```

### 3. Const fn Validation

```rust
const fn validate_format(route: &str) -> bool {
    route.starts_with('/') && !route.ends_with('/') && route != "/"
}
```

## Debug Spans

### 1. Proper Spans for All Tokens

```rust
// Good: preserves original span
let span = item.span();
quote_spanned! { span =>
    #user_code
}

// Bad: loses span information
quote! {
    #user_code
}
```

### 2. Multi-span Error Messages

```rust
let span1 = route.span();
let span2 = label.span();

let error = quote_spanned! { span1 =>
    compile_error!("invalid route at #1", span2)
};
```

## Examples

### Route Validation Macro

```rust
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, DeriveInput, Data};

#[proc_macro_derive(ValidatedRoute)]
pub fn validated_route(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let span = name.span();
    
    // Verify it's an enum
    let data = match &input.data {
        Data::Enum(data) => data,
        _ => {
            let error = quote_spanned! { span =>
                compile_error!("ValidatedRoute can only be derived for enums")
            };
            return TokenStream::from(error);
        }
    };
    
    let expanded = quote_spanned! { span =>
        impl#name {
            pub fn validate(&self) -> Result<(), String> {
                match self {
                    #(
                        #name::#variant => {
                            // Validate each variant
                        }
                    )*
                }
                Ok(())
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

### Link Generation Macro

```rust
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, Expr, ExprPath};

#[proc_macro]
pub fn generate_link(input: TokenStream) -> TokenStream {
    let route = parse_macro_input!(input as ExprPath);
    let span = route.span();
    
    let expanded = quote_spanned! { span =>
        NavLink::<#route>::new()
    };
    
    TokenStream::from(expanded)
}
```

## Testing Strategies

### 1. Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;
    
    #[test]
    fn test_parse_valid_route() {
        let input: ExprPath = parse_quote! { Route::Home };
        assert_eq!(input.span().line(), 10);
    }
}
```

### 2. Integration Tests

```rust
#[test]
fn test_compile_success() {
    let code = r#"
        use yew_router::prelude::*;
        use yew_nav_link_macros::RoutableExt;
        
        #[derive(Routable, RoutableExt)]
        enum Route {
            #[at("/")]
            Home,
        }
    "#;
    
    compile_success(code);
}
```

### 3. UI Tests (trybuild)

```rust
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/compile_fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
```

## Best Practices Summary

1. **Use `quote_spanned!`** for all code generation to preserve error locations
2. **Validate input early** to provide helpful error messages
3. **Use Syn's parsing combinators** for robust parsing
4. **Test comprehensively** with trybuild for compile-fail scenarios
5. **Document attributes** clearly for users
6. **Handle errors gracefully** with compile_error! and helpful messages
7. **Preserve hygiene** by using proper spans
8. **Optimize for compilation speed** by minimizing macro complexity

## References

- [Syn Documentation](https://docs.rs/syn)
- [Quote Documentation](https://docs.rs/quote)
- [Proc-Macro2 Documentation](https://docs.rs/proc-macro2)
- [Trybuild Tests](https://github.com/dtolnay/trybuild)
- [Rust Procedural Macros Workshop](https://github.com/dtolnay/proc-macro-workshop)
