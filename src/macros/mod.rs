//! Procedural macros for yew-nav-link
//!
//! This module provides both derive and function-like macros for reducing
//! boilerplate and improving compile-time safety.

pub mod function;
pub mod macros;

pub use function::*;
pub use macros::*;
