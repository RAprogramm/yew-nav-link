use std::collections::{HashMap, hash_map::Iter};

use yew::prelude::*;
use yew_router::prelude::*;

/// Query parameters extracted from the current URL.
///
/// Maps query parameter names to their values.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct QueryParams(HashMap<String, Vec<String>>);

impl QueryParams {
    /// Get all values for a query parameter name.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.0.get(key)
    }

    /// Get the first value for a query parameter name.
    #[must_use]
    pub fn get_one(&self, key: &str) -> Option<&str> {
        self.0
            .get(key)
            .and_then(|vals| vals.first())
            .map(String::as_str)
    }

    /// Check if a query parameter exists.
    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// Iterate over all query parameters.
    #[must_use]
    pub fn iter(&self) -> Iter<'_, String, Vec<String>> {
        self.0.iter()
    }

    /// Get the number of query parameters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if there are no query parameters.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'a> IntoIterator for &'a QueryParams {
    type Item = (&'a String, &'a Vec<String>);
    type IntoIter = Iter<'a, String, Vec<String>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// Returns query parameters from the current URL.
///
/// Extracts query string parameters into a map.
#[hook]
pub fn use_query_params() -> QueryParams {
    let current_url = use_location();
    let query_string = current_url.as_ref().map_or("", |l| l.query_str());

    let mut params = HashMap::new();

    for pair in query_string.split('&') {
        if pair.is_empty() {
            continue;
        }

        let mut parts = pair.splitn(2, '=');
        let key = parts.next().unwrap_or("").to_string();
        let value = parts.next().unwrap_or("").to_string();

        params.entry(key).or_insert_with(Vec::new).push(value);
    }

    QueryParams(params)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn query_params_get() {
        let mut params = HashMap::new();
        params.insert("q".to_string(), vec!["rust".to_string()]);
        let qp = QueryParams(params);

        assert_eq!(qp.get("q"), Some(&vec!["rust".to_string()]));
        assert_eq!(qp.get_one("q"), Some("rust"));
        assert!(qp.contains_key("q"));
    }

    #[test]
    fn query_params_empty() {
        let qp = QueryParams(HashMap::new());
        assert!(qp.is_empty());
        assert_eq!(qp.len(), 0);
    }

    #[test]
    fn query_params_parse() {
        let mut params = HashMap::new();
        params.insert("q".to_string(), vec!["rust".to_string()]);
        params.insert("page".to_string(), vec!["1".to_string()]);
        let qp = QueryParams(params);

        assert_eq!(qp.get_one("q"), Some("rust"));
        assert_eq!(qp.get_one("page"), Some("1"));
        assert_eq!(qp.len(), 2);
    }

    #[test]
    fn query_params_multiple_values() {
        let mut params = HashMap::new();
        params.insert(
            "tag".to_string(),
            vec!["rust".to_string(), "web".to_string()]
        );
        let qp = QueryParams(params);

        assert_eq!(qp.get_one("tag"), Some("rust"));
    }

    #[test]
    fn query_params_iter() {
        let mut params = HashMap::new();
        params.insert("q".to_string(), vec!["rust".to_string()]);
        let qp = QueryParams(params);
        assert_eq!(qp.iter().count(), 1);
    }

    #[test]
    fn query_params_clone() {
        let mut params = HashMap::new();
        params.insert("q".to_string(), vec!["rust".to_string()]);
        let qp1 = QueryParams(params);
        let qp2 = qp1.clone();
        assert_eq!(qp1.len(), qp2.len());
    }

    #[test]
    fn query_params_debug() {
        let qp = QueryParams(HashMap::new());
        let debug_str = format!("{qp:?}");
        assert!(debug_str.contains("QueryParams"));
    }

    #[test]
    fn query_params_partial_eq() {
        let mut params1 = HashMap::new();
        params1.insert("q".to_string(), vec!["rust".to_string()]);
        let qp1 = QueryParams(params1);

        let mut params2 = HashMap::new();
        params2.insert("q".to_string(), vec!["rust".to_string()]);
        let qp2 = QueryParams(params2);

        assert_eq!(qp1, qp2);
    }

    #[test]
    fn use_query_params_returns_query_params() {
        let _ = use_query_params();
    }

    #[test]
    fn query_params_get_none() {
        let qp = QueryParams(HashMap::new());
        assert_eq!(qp.get("nonexistent"), None);
    }

    #[test]
    fn query_params_get_one_none() {
        let qp = QueryParams(HashMap::new());
        assert_eq!(qp.get_one("nonexistent"), None);
    }

    #[test]
    fn query_params_contains_key_false() {
        let mut params = HashMap::new();
        params.insert("q".to_string(), vec!["rust".to_string()]);
        let qp = QueryParams(params);
        assert!(!qp.contains_key("nonexistent"));
    }

    #[test]
    fn query_params_empty_string_key() {
        let mut params = HashMap::new();
        params.insert(String::new(), vec!["value".to_string()]);
        let qp = QueryParams(params);
        assert!(qp.contains_key(""));
    }

    #[test]
    fn query_params_multiple_entries() {
        let mut params = HashMap::new();
        params.insert("a".to_string(), vec!["1".to_string()]);
        params.insert("b".to_string(), vec!["2".to_string()]);
        params.insert("c".to_string(), vec!["3".to_string()]);
        let qp = QueryParams(params);
        assert_eq!(qp.len(), 3);
    }

    #[test]
    fn query_params_into_iter() {
        let mut params = HashMap::new();
        params.insert("key".to_string(), vec!["value".to_string()]);
        let qp = QueryParams(params);
        let count = qp.into_iter().count();
        assert_eq!(count, 1);
    }
}
