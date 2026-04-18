//! Query parameters utilities.

use std::{collections::HashMap, fmt};

use super::codec::{urlencoding_decode, urlencoding_encode};

/// A collection of URL query parameters with get, set, and serialization
/// support.
///
/// Keys and values are automatically percent-decoded on parse and
/// percent-encoded on serialization.
#[derive(Clone, Debug, Default)]
pub struct QueryParams {
    params: HashMap<String, String>
}

impl QueryParams {
    /// Creates an empty [`QueryParams`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses a query string (with or without a leading `?`) into parameters.
    #[must_use]
    pub fn parse(query: &str) -> Self {
        let query = query.trim_start_matches('?');
        let mut params = HashMap::new();

        for pair in query.split('&') {
            let parts: Vec<&str> = pair.splitn(2, '=').collect();
            if let Some(key) = parts.first() {
                let value = parts
                    .get(1)
                    .map(|v| urlencoding_decode(v).unwrap_or_else(|| v.to_string()));
                if !key.is_empty() {
                    params.insert(
                        urlencoding_decode(key).unwrap_or_else(|| key.to_string()),
                        value.unwrap_or_default()
                    );
                }
            }
        }

        Self {
            params
        }
    }

    /// Returns the value for the given key, if present.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(String::as_str)
    }

    /// Returns `true` if the given key exists in the parameters.
    #[must_use]
    pub fn has(&self, key: &str) -> bool {
        self.params.contains_key(key)
    }

    /// Inserts or replaces a key-value pair.
    pub fn set(&mut self, key: &str, value: &str) {
        self.params.insert(key.to_string(), value.to_string());
    }

    /// Removes a key and its value, if present.
    pub fn remove(&mut self, key: &str) {
        self.params.remove(key);
    }

    /// Serializes the parameters to a query string starting with `?`.
    ///
    /// Returns an empty string if there are no parameters.
    #[must_use]
    pub fn to_query_string(&self) -> String {
        if self.params.is_empty() {
            return String::new();
        }

        let pairs: Vec<String> = self
            .params
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding_encode(k), urlencoding_encode(v)))
            .collect();

        format!("?{}", pairs.join("&"))
    }

    /// Returns the number of parameters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.params.len()
    }

    /// Returns `true` if there are no parameters.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    /// Returns an iterator over parameter keys.
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.params.keys().map(String::as_str)
    }

    /// Returns an iterator over parameter values.
    pub fn values(&self) -> impl Iterator<Item = &str> {
        self.params.values().map(String::as_str)
    }

    /// Returns an iterator over all parameter key-value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.params.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}

impl fmt::Display for QueryParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_query_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_params_parse_empty() {
        let params = QueryParams::parse("");
        assert!(params.is_empty());
    }

    #[test]
    fn query_params_parse_single() {
        let params = QueryParams::parse("page=1");
        assert_eq!(params.get("page"), Some("1"));
    }

    #[test]
    fn query_params_parse_multiple() {
        let params = QueryParams::parse("page=1&limit=10&sort=name");
        assert_eq!(params.get("page"), Some("1"));
        assert_eq!(params.get("limit"), Some("10"));
        assert_eq!(params.get("sort"), Some("name"));
    }

    #[test]
    fn query_params_parse_with_plus() {
        let params = QueryParams::parse("search=hello+world");
        assert_eq!(params.get("search"), Some("hello world"));
    }

    #[test]
    fn query_params_to_string() {
        let mut params = QueryParams::new();
        params.set("page", "1");
        params.set("sort", "name");
        let result = params.to_string();
        assert!(result.contains("page=1"));
        assert!(result.contains("sort=name"));
    }

    #[test]
    fn query_params_set_get() {
        let mut params = QueryParams::new();
        params.set("key", "value");
        assert_eq!(params.get("key"), Some("value"));
    }

    #[test]
    fn query_params_remove() {
        let mut params = QueryParams::new();
        params.set("key", "value");
        params.remove("key");
        assert!(!params.has("key"));
    }
}
