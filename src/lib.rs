//! # yew-nav-link
//!
//! Navigation link component for [Yew](https://yew.rs) with automatic active
//! state detection.
//!
//! [![Crates.io](https://img.shields.io/crates/v/yew-nav-link)](https://crates.io/crates/yew-nav-link)
//! [![Documentation](https://docs.rs/yew-nav-link/badge.svg)](https://docs.rs/yew-nav-link)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
//!
//! ## Overview
//!
//! `yew-nav-link` provides a [`NavLink`] component that wraps Yew Router's
//! `Link` with automatic active state management. When the target route matches
//! the current URL, an `active` CSS class is applied.
//!
//! ## Quick Start
//!
//! ```toml
//! [dependencies]
//! yew-nav-link = "0.4"
//! ```
//!
//! ## Component Syntax
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
//! fn Navigation() -> Html {
//!     html! {
//!         <nav>
//!             <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
//!             <NavLink<Route> to={Route::About}>{ "About" }</NavLink<Route>>
//!         </nav>
//!     }
//! }
//! ```
//!
//! ## Function Syntax
//!
//! For text-only links, use [`nav_link`] with explicit [`Match`] mode:
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::{Match, nav_link};
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Debug, Routable)]
//! # enum Route {
//! #     #[at("/")]
//! #     Home,
//! #     #[at("/docs")]
//! #     Docs,
//! # }
//! #[component]
//! fn Menu() -> Html {
//!     html! {
//!         <nav>
//!             { nav_link(Route::Home, "Home", Match::Exact) }
//!             { nav_link(Route::Docs, "Docs", Match::Partial) }
//!         </nav>
//!     }
//! }
//! ```
//!
//! ## Partial Matching
//!
//! Use `partial` prop to keep parent links active on nested routes:
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::NavLink;
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Debug, Routable)]
//! # enum Route {
//! #     #[at("/docs")]
//! #     Docs,
//! #     #[at("/docs/api")]
//! #     DocsApi,
//! # }
//! #[component]
//! fn Navigation() -> Html {
//!     html! {
//!         <nav>
//!             // Active on /docs, /docs/api, /docs/*
//!             <NavLink<Route> to={Route::Docs} partial=true>{ "Docs" }</NavLink<Route>>
//!         </nav>
//!     }
//! }
//! ```
//!
//! ## CSS Classes
//!
//! | Class | Condition |
//! |-------|-----------|
//! | `nav-link` | Always |
//! | `active` | Route matches |
//!
//! Compatible with Bootstrap, Tailwind, and other CSS frameworks.
//!
//! ## Requirements
//!
//! - Yew 0.22+
//! - yew-router 0.19+

mod nav_link;

pub use nav_link::{Match, NavLink, NavLinkProps, nav_link};
