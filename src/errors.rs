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

/// Errors that can occur during navigation operations.
///
/// Returned by navigation hooks and helper functions when something
/// goes wrong while resolving or activating a route.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub enum NavError {
    /// The target route does not match any registered route.
    RouteNotFound,
    /// A route string could not be parsed. Contains a diagnostic message.
    InvalidRoute(String),
    /// Navigation was cancelled before completion.
    NavigationCancelled
}

impl std::fmt::Display for NavError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RouteNotFound => write!(f, "route not found"),
            Self::InvalidRoute(msg) => write!(f, "invalid route: {msg}"),
            Self::NavigationCancelled => write!(f, "navigation cancelled")
        }
    }
}

impl std::error::Error for NavError {}

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
