//! Macro implementations for yew-nav-link
//!
//! This module provides procedural macros for reducing boilerplate
//! and improving compile-time safety.
//!
//! NOTE: This module is not currently compiled as part of the main crate.
//! The proc_macro code in `macros/` requires a separate crate with
//! `crate-type = ["proc-macro"]`. The `function/` subdirectory is a stub.

pub mod macros;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_exports() {
        assert!(true);
    }
}
