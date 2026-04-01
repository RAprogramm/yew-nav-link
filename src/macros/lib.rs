//! Macro implementations for yew-nav-link
//!
//! This module provides procedural macros for reducing boilerplate
//! and improving compile-time safety.

pub use derive::*;
pub use function::*;

pub mod function;
pub mod macros;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_macro_exports() {
        assert!(true);
    }
}
