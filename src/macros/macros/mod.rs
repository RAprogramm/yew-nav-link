//! Core macros for yew-nav-link
//!
//! This module provides production-ready procedural macros for reducing
//! boilerplate and improving developer experience.

pub mod breadcrumbs;
pub mod link;
pub mod list;
pub mod menu;
pub mod pagination;
pub mod tabs;

pub use breadcrumbs::breadcrumbs;
pub use link::nav_links;
pub use list::nav_list;
pub use menu::nav_menu;
pub use pagination::nav_pagination;
pub use tabs::nav_tabs;
