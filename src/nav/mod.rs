//! Core navigation primitives.
//!
//! Provides semantic building blocks for navigation lists: items, lists,
//! and dividers with proper ARIA attributes.

mod divider;
mod item;
mod list;

pub use divider::{NavDivider, NavDividerProps};
pub use item::{NavItem, NavItemProps};
pub use list::{NavList, NavListProps};
