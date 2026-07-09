// SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
// SPDX-License-Identifier: MIT

//! # NavError
//!
//! Error types returned by navigation operations. Use [`NavResult<T>`] as a
//! convenience alias for `Result<T, NavError>`.
//!
//! # Example
//!
//! ```rust
//! use yew_nav_link::NavError;
//!
//! fn handle_error(err: &NavError) {
//!     match err {
//!         NavError::RouteNotFound => { /* handle */ }
//!         NavError::InvalidRoute(msg) => { /* handle */ }
//!         NavError::NavigationCancelled => { /* handle */ }
//!         // Required because NavError is non_exhaustive.
//!         _ => { /* handle future variants */ }
//!     }
//! }
//! ```
//!
//! # Variants
//!
//! | Variant | Description |
//! |---------|-------------|
//! | `RouteNotFound` | Target route does not match any registered route |
//! | `InvalidRoute(String)` | Route string could not be parsed |
//! | `NavigationCancelled` | Navigation was cancelled |

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult}
};

/// Errors that can occur during navigation operations.
///
/// Returned by navigation hooks and helper functions when something
/// goes wrong while resolving or activating a route.
///
/// # Stability
///
/// This enum is `#[non_exhaustive]`: future minor releases may add new
/// variants without bumping the major version. Consumers matching on
/// `NavError` must include a wildcard arm (`_ =>`) so the match remains
/// exhaustive across upgrades.
///
/// ```rust
/// use yew_nav_link::NavError;
///
/// fn describe(err: &NavError) -> &'static str {
///     match err {
///         NavError::RouteNotFound => "missing route",
///         NavError::InvalidRoute(_) => "bad route",
///         NavError::NavigationCancelled => "cancelled",
///         // Required because NavError is non_exhaustive.
///         _ => "unknown navigation error"
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
#[non_exhaustive]
pub enum NavError {
    /// The target route does not match any registered route.
    RouteNotFound,
    /// A route string could not be parsed. Contains a diagnostic message.
    InvalidRoute(String),
    /// Navigation was cancelled before completion.
    NavigationCancelled
}

impl Display for NavError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::RouteNotFound => write!(f, "route not found"),
            Self::InvalidRoute(msg) => write!(f, "invalid route: {msg}"),
            Self::NavigationCancelled => write!(f, "navigation cancelled")
        }
    }
}

impl Error for NavError {}

impl NavError {
    /// Creates a [`NavError::RouteNotFound`] error.
    pub const fn route_not_found() -> Self {
        Self::RouteNotFound
    }

    /// Creates a [`NavError::InvalidRoute`] error with the given message.
    pub fn invalid_route<S: Into<String>>(msg: S) -> Self {
        Self::InvalidRoute(msg.into())
    }

    /// Creates a [`NavError::NavigationCancelled`] error.
    pub const fn navigation_cancelled() -> Self {
        Self::NavigationCancelled
    }
}

/// A convenience alias for `Result<T, NavError>`.
pub type NavResult<T> = Result<T, NavError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_not_found() {
        let err = NavError::route_not_found();
        assert_eq!(err, NavError::RouteNotFound);
    }

    #[test]
    fn invalid_route() {
        let err = NavError::invalid_route("bad route");
        assert_eq!(err, NavError::InvalidRoute("bad route".to_string()));
    }

    #[test]
    fn navigation_cancelled() {
        let err = NavError::navigation_cancelled();
        assert_eq!(err, NavError::NavigationCancelled);
    }

    #[test]
    fn nav_error_display() {
        assert_eq!(
            format!("{}", NavError::route_not_found()),
            "route not found"
        );
        assert_eq!(
            format!("{}", NavError::invalid_route("foo")),
            "invalid route: foo"
        );
        assert_eq!(
            format!("{}", NavError::navigation_cancelled()),
            "navigation cancelled"
        );
    }

    #[test]
    fn nav_error_debug() {
        let err = NavError::route_not_found();
        let debug_str = format!("{err:?}");
        assert!(debug_str.contains("RouteNotFound"));
    }

    #[test]
    fn nav_error_clone() {
        let err1 = NavError::route_not_found();
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn nav_error_partial_eq() {
        assert_eq!(NavError::route_not_found(), NavError::route_not_found());
        assert_eq!(
            NavError::navigation_cancelled(),
            NavError::navigation_cancelled()
        );
    }

    #[test]
    fn nav_result_alias() {
        let ok: NavResult<i32> = Ok(42);
        let err: NavResult<i32> = Err(NavError::route_not_found());
        assert!(ok.is_ok());
        assert!(err.is_err());
    }
}
