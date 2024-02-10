//! # Yew Nav Link
//!
//! The `nav_link` module provides a NavLink component and related types for creating
//! navigational links that are aware of their active state based on the current route in the application.
//!
//! # Example
//!
//! ```rust
//! use yew-nav-link::{nav_link, NavLink};
//! ```
//!
//! ```html
//!     <!-- Creating a NavLink for the Home route with the text "Home Page" -->
//!     <li class="nav-item">
//!         { nav_link(HomeRoute::IntroPage, "Home") }
//!     </li>
//!     <!-- or -->
//!     <li class="nav-item">
//!         <NavLink<HomeRoute> to={HomeRoute::IntroPage}>
//!             {"Home"}
//!         </NavLink<HomeRoute>>
//!     </li>
//! ```

mod nav_link;

pub use nav_link::nav_link;
pub use nav_link::NavLink;
pub use nav_link::NavLinkProps;
