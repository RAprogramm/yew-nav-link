//! Error types for navigation components.
//!
//! Provides error types used across the crate.
//!
//! # Example
//!
//! ```rust
//! use yew_nav_link::NavError;
//!
//! fn handle_error(err: &NavError) {
//!     match err {
//!         NavError::RouteNotFound => {
//!             // Handle not found
//!         }
//!         NavError::InvalidRoute(msg) => {
//!             // Handle invalid route
//!         }
//!         NavError::NavigationCancelled => {
//!             // Handle cancelled navigation
//!         }
//!     }
//! }
//! ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavError {
    RouteNotFound,
    InvalidRoute(String),
    NavigationCancelled
}

impl NavError {
    pub fn route_not_found() -> Self {
        NavError::RouteNotFound
    }

    pub fn invalid_route<S: Into<String>>(msg: S) -> Self {
        NavError::InvalidRoute(msg.into())
    }

    pub fn navigation_cancelled() -> Self {
        NavError::NavigationCancelled
    }
}

pub type NavResult<T> = Result<T, NavError>;
