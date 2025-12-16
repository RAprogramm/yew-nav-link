//! # yew-nav-link
//!
//! A navigation link component for [Yew](https://yew.rs) applications with
//! automatic active state detection based on the current route.
//!
//! [![Crates.io](https://img.shields.io/crates/v/yew-nav-link)](https://crates.io/crates/yew-nav-link)
//! [![Documentation](https://docs.rs/yew-nav-link/badge.svg)](https://docs.rs/yew-nav-link)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
//!
//! ## Overview
//!
//! `yew-nav-link` provides a [`NavLink`] component that wraps Yew Router's
//! `Link` component with automatic active state management. When the link's
//! target route matches the current URL, an `active` CSS class is automatically
//! applied.
//!
//! ## Quick Start
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! yew-nav-link = "0.3"
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::NavLink;
//! use yew_router::prelude::*;
//!
//! #[derive(Clone, PartialEq, Debug, Routable)]
//! enum Route {
//!     #[at("/")]
//!     Home,
//!     #[at("/about")]
//!     About
//! }
//!
//! #[component]
//! fn App() -> Html {
//!     html! {
//!         <BrowserRouter>
//!             <nav>
//!                 <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
//!                 <NavLink<Route> to={Route::About}>{ "About" }</NavLink<Route>>
//!             </nav>
//!             <Switch<Route> render={switch} />
//!         </BrowserRouter>
//!     }
//! }
//!
//! fn switch(route: Route) -> Html {
//!     match route {
//!         Route::Home => html! { <h1>{ "Home" }</h1> },
//!         Route::About => html! { <h1>{ "About" }</h1> }
//!     }
//! }
//! ```
//!
//! ## Helper Function
//!
//! For text-only links, use the [`nav_link`] helper function:
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::nav_link;
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Debug, Routable)]
//! # enum Route {
//! #     #[at("/")]
//! #     Home,
//! # }
//! #[component]
//! fn Menu() -> Html {
//!     html! {
//!         <ul class="nav">
//!             <li>{ nav_link(Route::Home, "Home") }</li>
//!         </ul>
//!     }
//! }
//! ```
//!
//! ## CSS Classes
//!
//! The rendered `<a>` element receives:
//! - `nav-link` - always applied
//! - `active` - applied when route matches current URL
//!
//! Compatible with Bootstrap, Tailwind, and other CSS frameworks.
//!
//! ## Requirements
//!
//! - Yew 0.22+
//! - yew-router 0.19+
//!
//! ## License
//!
//! Licensed under the MIT License. See [LICENSE-MIT](LICENSE-MIT) for details.

mod nav_link;

pub use nav_link::{NavLink, NavLinkProps, nav_link};
