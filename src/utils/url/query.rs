// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Query parameters utilities.
//!
//! Provides a [`QueryParams`] type that supports:
//! - Parsing from query strings (with or without leading `?`)
//! - Setting, getting, and removing parameters
//! - Serialization back to query strings
//! - Automatic percent-encoding/decoding
//!
//! Keys map to **multiple values** to support query strings like
//! `tag=rust&tag=web` properly.

use std::fmt::{Display, Formatter, Result as FmtResult};

use super::codec::{urlencoding_decode, urlencoding_encode};

/// A collection of URL query parameters with get, set, and serialization
/// support.
///
/// Keys and values are automatically percent-decoded on parse and
/// percent-encoded on serialization. Multiple values per key are supported
/// for query strings such as `tag=rust&tag=web`.
///
/// Keys keep their insertion order, so
/// [`to_query_string`](Self::to_query_string) is deterministic and `parse` →
/// serialize round-trips preserve the original parameter order. Equality
/// remains key-order-insensitive.
///
/// # Examples
///
/// ```rust
/// use yew_nav_link::utils::QueryParams;
///
/// let mut params = QueryParams::parse("page=1&limit=10");
/// assert_eq!(params.get("page"), Some("1"));
/// assert!(params.has("limit"));
///
/// params.set("sort", "name");
/// let qs = params.to_query_string();
/// assert!(qs.starts_with('?'));
/// assert!(qs.contains("page=1"));
/// assert!(qs.contains("limit=10"));
/// assert!(qs.contains("sort=name"));
/// ```
#[derive(Clone, Debug, Default)]
pub struct QueryParams {
    params: Vec<(String, Vec<String>)>
}

impl PartialEq for QueryParams {
    fn eq(&self, other: &Self) -> bool {
        self.params.len() == other.params.len()
            && self
                .params
                .iter()
                .all(|(key, values)| other.get_all(key) == Some(values))
    }
}

impl Eq for QueryParams {}

impl QueryParams {
    fn entry_mut(&mut self, key: &str) -> Option<&mut Vec<String>> {
        self.params
            .iter_mut()
            .find_map(|(k, values)| (k == key).then_some(values))
    }
    /// Creates an empty [`QueryParams`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses a query string (with or without a leading `?`) into parameters.
    ///
    /// Supports multiple values per key: `tag=rust&tag=web` stores both values
    /// under the key `"tag"`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew_nav_link::utils::QueryParams;
    ///
    /// let params = QueryParams::parse("page=1&limit=10");
    /// assert_eq!(params.get("page"), Some("1"));
    ///
    /// let params2 = QueryParams::parse("tag=rust&tag=web");
    /// let tag_all = params2
    ///     .get_all("tag")
    ///     .expect("`tag` is set twice by the input above");
    /// assert_eq!(tag_all, &vec!["rust".to_string(), "web".to_string()]);
    /// ```
    #[must_use]
    pub fn parse(query: &str) -> Self {
        let query = query.trim_start_matches('?');
        let mut result = Self::default();

        for pair in query.split('&') {
            let (key, value) = pair
                .split_once('=')
                .map_or((pair, None), |(k, v)| (k, Some(v)));
            let decoded_key = urlencoding_decode(key).unwrap_or_else(|| key.to_string());
            if !decoded_key.is_empty() {
                let value = value
                    .map(|v| urlencoding_decode(v).unwrap_or_else(|| v.to_string()))
                    .unwrap_or_default();
                match result.entry_mut(&decoded_key) {
                    Some(values) => values.push(value),
                    None => result.params.push((decoded_key, vec![value]))
                }
            }
        }

        result
    }

    /// Returns the first value for the given key, if present.
    ///
    /// For query strings with multiple values per key (e.g.
    /// `tag=rust&tag=web`), this returns the first value. Use
    /// [`get_all`](Self::get_all) to retrieve all values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew_nav_link::utils::QueryParams;
    ///
    /// let params = QueryParams::parse("page=1");
    /// assert_eq!(params.get("page"), Some("1"));
    /// assert_eq!(params.get("missing"), None);
    /// ```
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.get_all(key)
            .and_then(|values| values.first())
            .map(String::as_str)
    }

    /// Returns the first value for the given key (alias for [`Self::get`]).
    ///
    /// Equivalent to [`Self::get`]. Included for API compatibility.
    #[must_use]
    pub fn get_one(&self, key: &str) -> Option<&str> {
        self.get(key)
    }

    /// Returns **all** values for the given key, if present.
    ///
    /// For single-value parameters this returns a slice with one element.
    /// For multi-value parameters (e.g. `tag=rust&tag=web`) this returns
    /// all values in order.
    #[must_use]
    pub fn get_all(&self, key: &str) -> Option<&Vec<String>> {
        self.params
            .iter()
            .find_map(|(k, values)| (k == key).then_some(values))
    }

    /// Returns `true` if the given key exists in the parameters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew_nav_link::utils::QueryParams;
    ///
    /// let params = QueryParams::parse("key=value");
    /// assert!(params.has("key"));
    /// assert!(!params.has("missing"));
    /// ```
    #[must_use]
    pub fn has(&self, key: &str) -> bool {
        self.params.iter().any(|(k, _)| k == key)
    }

    /// Checks if the given key exists (alias for [`Self::has`]).
    ///
    /// Equivalent to [`Self::has`]. Included for API compatibility.
    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.has(key)
    }

    /// Inserts a value for the given key, appending to any existing values.
    ///
    /// If the key already exists, the value is **appended** to the existing
    /// list rather than replacing it. This supports query strings with
    /// multiple values per key.
    ///
    /// To replace all existing values for a key, use
    /// [`set_value`](Self::set_value).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew_nav_link::utils::QueryParams;
    ///
    /// let mut params = QueryParams::parse("tag=rust");
    /// params.set("tag", "web");
    ///
    /// let all = params
    ///     .get_all("tag")
    ///     .expect("`tag` was set immediately above");
    /// assert_eq!(all, &["rust".to_string(), "web".to_string()]);
    /// ```
    pub fn set(&mut self, key: &str, value: &str) {
        match self.entry_mut(key) {
            Some(values) => values.push(value.to_string()),
            None => self.params.push((key.to_string(), vec![value.to_string()]))
        }
    }

    /// Sets a value for the given key, **replacing** any existing values.
    ///
    /// Use [`set`](Self::set) to append values instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew_nav_link::utils::QueryParams;
    ///
    /// let mut params = QueryParams::parse("key=old1&key=old2");
    /// params.set_value("key", "new");
    ///
    /// let all = params
    ///     .get_all("key")
    ///     .expect("`key` was set immediately above");
    /// assert_eq!(all, &["new".to_string()]);
    /// ```
    pub fn set_value(&mut self, key: &str, value: &str) {
        match self.entry_mut(key) {
            Some(values) => *values = vec![value.to_string()],
            None => self.params.push((key.to_string(), vec![value.to_string()]))
        }
    }

    /// Removes a key and all its values, if present.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew_nav_link::utils::QueryParams;
    ///
    /// let mut params = QueryParams::parse("key=value");
    /// params.remove("key");
    /// assert!(!params.has("key"));
    /// ```
    pub fn remove(&mut self, key: &str) {
        self.params.retain(|(k, _)| k != key);
    }

    /// Serializes the parameters to a query string starting with `?`.
    ///
    /// Returns an empty string if there are no parameters.
    /// Multiple values for the same key are serialized as repeated
    /// `key=value` pairs. Keys appear in insertion order, so the output
    /// is deterministic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew_nav_link::utils::QueryParams;
    ///
    /// let mut params = QueryParams::parse("page=1");
    /// params.set("tag", "rust");
    /// let qs = params.to_query_string();
    /// assert!(qs.starts_with('?'));
    /// assert!(qs.contains("page=1"));
    /// ```
    #[must_use]
    pub fn to_query_string(&self) -> String {
        if self.params.is_empty() {
            return String::new();
        }

        let mut pairs: Vec<String> = Vec::with_capacity(self.params.len());
        for (key, values) in &self.params {
            for value in values {
                pairs.push(format!(
                    "{}={}",
                    urlencoding_encode(key),
                    urlencoding_encode(value)
                ));
            }
        }

        format!("?{}", pairs.join("&"))
    }

    /// Returns the number of unique parameter keys.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.params.len()
    }

    /// Returns `true` if there are no parameters.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    /// Returns an iterator over parameter keys.
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.params.iter().map(|(key, _)| key.as_str())
    }

    /// Returns an iterator over single values (first value per key).
    ///
    /// For multi-value keys, only the first value is yielded.
    /// Use [`iter_all`](Self::iter_all) to iterate over all values.
    pub fn values(&self) -> impl Iterator<Item = &str> {
        self.params
            .iter()
            .map(|(_, values)| values.first().map_or("", String::as_str))
    }

    /// Returns an iterator over all parameter key-value pairs (first value per
    /// key).
    ///
    /// For multi-value keys, only the first value is yielded.
    /// Use [`iter_all`](Self::iter_all) for full iteration.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.params
            .iter()
            .map(|(key, values)| (key.as_str(), values.first().map_or("", String::as_str)))
    }

    /// Returns an iterator over all parameter key-value pairs including
    /// multiple values per key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew_nav_link::utils::QueryParams;
    ///
    /// let params = QueryParams::parse("tag=rust&tag=web");
    /// let count = params.iter_all().count();
    /// assert_eq!(count, 2);
    /// ```
    pub fn iter_all(&self) -> impl Iterator<Item = (&str, &str)> {
        self.params.iter().flat_map(|(key, values)| {
            values
                .iter()
                .map(move |value| (key.as_str(), value.as_str()))
        })
    }
}

impl Display for QueryParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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
    fn query_params_parse_multiple_values_same_key() {
        let params = QueryParams::parse("tag=rust&tag=web&tag=wasm");
        assert_eq!(params.get("tag"), Some("rust"));
        assert_eq!(params.get_one("tag"), Some("rust"));

        let all = params.get_all("tag").unwrap();
        assert_eq!(all.len(), 3);
        assert_eq!(all[0], "rust");
        assert_eq!(all[1], "web");
        assert_eq!(all[2], "wasm");
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
    fn query_params_set_appends() {
        let mut params = QueryParams::new();
        params.set("key", "first");
        params.set("key", "second");
        let all = params.get_all("key").unwrap();
        assert_eq!(all, &["first".to_string(), "second".to_string()]);
    }

    #[test]
    fn query_params_set_value_replaces() {
        let mut params = QueryParams::new();
        params.set("key", "first");
        params.set("key", "second");
        params.set_value("key", "replaced");
        let all = params.get_all("key").unwrap();
        assert_eq!(all, &["replaced".to_string()]);
    }

    #[test]
    fn query_params_remove() {
        let mut params = QueryParams::new();
        params.set("key", "value");
        params.remove("key");
        assert!(!params.has("key"));
    }

    #[test]
    fn query_params_keys() {
        let params = QueryParams::parse("a=1&b=2");
        assert_eq!(params.keys().count(), 2);
    }

    #[test]
    fn query_params_values() {
        let params = QueryParams::parse("a=1&b=2");
        assert_eq!(params.values().count(), 2);
    }

    #[test]
    fn query_params_default() {
        let params = QueryParams::default();
        assert!(params.is_empty());
    }

    #[test]
    fn query_params_len() {
        let params = QueryParams::parse("a=1&b=2");
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn query_params_parse_with_leading_question_mark() {
        let params = QueryParams::parse("?page=1&limit=10");
        assert_eq!(params.get("page"), Some("1"));
        assert_eq!(params.get("limit"), Some("10"));
    }

    #[test]
    fn query_params_parse_key_without_value() {
        let params = QueryParams::parse("key&other=value");
        assert_eq!(params.get("key"), Some(""));
        assert_eq!(params.get("other"), Some("value"));
    }

    #[test]
    fn query_params_parse_special_chars() {
        let params = QueryParams::parse("search=hello%20world");
        assert_eq!(params.get("search"), Some("hello world"));
    }

    #[test]
    fn query_params_to_string_multiple_values() {
        let mut params = QueryParams::new();
        params.set("tag", "rust");
        params.set("tag", "web");
        let qs = params.to_query_string();
        assert!(qs.contains("tag=rust"));
        assert!(qs.contains("tag=web"));
    }

    #[test]
    fn query_params_to_string_empty() {
        let params = QueryParams::new();
        assert_eq!(params.to_query_string(), "");
    }

    #[test]
    fn query_params_has() {
        let params = QueryParams::parse("key=value");
        assert!(params.has("key"));
        assert!(!params.has("nonexistent"));
    }

    #[test]
    fn query_params_contains_key_alias() {
        let params = QueryParams::parse("key=value");
        assert!(params.contains_key("key"));
        assert!(!params.contains_key("missing"));
    }

    #[test]
    fn query_params_iter() {
        let params = QueryParams::parse("a=1&b=2");
        assert_eq!(params.iter().count(), 2);
    }

    #[test]
    fn query_params_iter_all() {
        let params = QueryParams::parse("tag=rust&tag=web&page=1");
        let count = params.iter_all().count();
        assert_eq!(count, 3);
    }

    #[test]
    fn query_params_display() {
        let mut params = QueryParams::new();
        params.set("key", "value");
        let display_str = params.to_string();
        assert!(display_str.starts_with('?'));
        assert!(display_str.contains("key=value"));
    }

    #[test]
    fn query_params_clone() {
        let mut params = QueryParams::parse("a=1");
        params.set("b", "2");
        let cloned = params.clone();
        assert_eq!(cloned.get("a"), Some("1"));
        assert_eq!(cloned.get("b"), Some("2"));
    }

    #[test]
    fn query_params_eq() {
        let p1 = QueryParams::parse("a=1");
        let p2 = QueryParams::parse("a=1");
        let p3 = QueryParams::parse("a=2");

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn query_params_parse_empty_value() {
        let params = QueryParams::parse("key=");
        assert!(params.has("key"));
        assert_eq!(params.get("key"), Some(""));
    }

    #[test]
    fn query_params_parse_multiple_empty() {
        let params = QueryParams::parse("=&=&=");
        // empty and empty keys should be filtered out
        assert!(params.is_empty());
    }

    #[test]
    fn query_params_get_all_none() {
        let params = QueryParams::parse("a=1");
        assert_eq!(params.get_all("missing"), None);
    }

    #[test]
    fn query_params_to_string_preserves_insertion_order() {
        let params = QueryParams::parse("z=1&a=2&m=3&z=4");
        assert_eq!(params.to_query_string(), "?z=1&z=4&a=2&m=3");

        let mut built = QueryParams::new();
        built.set("beta", "1");
        built.set("alpha", "2");
        assert_eq!(built.to_query_string(), "?beta=1&alpha=2");
    }

    #[test]
    fn query_params_eq_ignores_key_order() {
        let p1 = QueryParams::parse("a=1&b=2");
        let p2 = QueryParams::parse("b=2&a=1");
        assert_eq!(p1, p2);
    }

    #[test]
    fn query_params_to_query_string_reversible() {
        let mut params = QueryParams::new();
        params.set("name", "hello");
        params.set("name", "world");
        params.set("page", "1");

        let qs = params.to_query_string();
        let parsed = QueryParams::parse(&qs[1..]); // skip '?'

        assert_eq!(parsed.get("page"), Some("1"));

        // First value should match
        assert_eq!(parsed.get("name"), Some("hello"));
        // All values should be present
        let all = parsed.get_all("name").unwrap();
        assert_eq!(all, &["hello".to_string(), "world".to_string()]);
    }
}
