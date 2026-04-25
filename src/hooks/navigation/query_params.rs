//! Query parameter hook.
//!
//! Provides a reactive hook for extracting and managing query parameters
//! from the current URL.

use yew::prelude::*;
use yew_router::prelude::*;

/// Query parameters extracted from the current URL.
///
/// This is a type alias for
/// [`utils::QueryParams`][`crate::utils::QueryParams`].
pub type QueryParams = crate::utils::QueryParams;

/// Returns query parameters from the current URL.
///
/// Extracts query string parameters into a [`QueryParams`] map supporting
/// multiple values per key.
///
/// # Examples
///
/// ```rust,ignore
/// use yew_nav_link::hooks::use_query_params;
///
/// let params = use_query_params();
/// if let Some(page) = params.get("page") {
///     // page is "1", "2", etc.
/// }
///
/// // Multiple values
/// let tags = params.get_all("tag");
/// ```
#[hook]
pub fn use_query_params() -> QueryParams {
    let current_url = use_location();
    let query_string = current_url.as_ref().map_or("", |l| l.query_str());

    QueryParams::parse(query_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_query_params_returns_query_params() {
        // Cannot test actual hook behavior in non-WASM
    }

    #[test]
    fn query_params_type_alias() {
        // Verify type alias resolves correctly
        let _params: crate::utils::QueryParams = QueryParams::parse("a=1");
        let _params: QueryParams = QueryParams::parse("a=1");
    }

    #[test]
    fn query_params_parse_via_alias() {
        let params: QueryParams = QueryParams::parse("page=1&tag=rust&tag=web");
        assert_eq!(params.get("page"), Some("1"));
        assert_eq!(params.get("tag"), Some("rust"));

        let all_tags = params.get_all("tag").unwrap();
        assert_eq!(all_tags.len(), 2);
    }

    #[test]
    fn query_params_empty_via_alias() {
        let params: QueryParams = QueryParams::parse("");
        assert!(params.is_empty());
    }

    #[test]
    fn query_params_multiple_values_via_alias() {
        let params: QueryParams = QueryParams::parse("a=1&b=2&a=3");
        assert_eq!(params.get("a"), Some("1"));
        let all_a = params.get_all("a").unwrap();
        assert_eq!(all_a, &vec!["1".to_string(), "3".to_string()]);
        assert_eq!(params.get("b"), Some("2"));
    }

    #[test]
    fn query_params_to_query_string_via_alias() {
        let mut params: QueryParams = QueryParams::parse("page=1");
        params.set("sort", "name");
        let qs = params.to_query_string();
        assert!(qs.starts_with('?'));
        assert!(qs.contains("page=1"));
    }
}
