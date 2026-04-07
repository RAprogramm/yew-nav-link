//! URL parsing utilities.
//!
//! Provides helpers for parsing URLs and query parameters.

mod codec;
mod parts;
mod query;

pub use codec::{urlencoding_decode, urlencoding_encode};
pub use parts::UrlParts;
pub use query::QueryParams;
