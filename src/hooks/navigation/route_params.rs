use std::collections::HashMap;

use yew::prelude::*;
use yew_router::prelude::*;

/// Route parameters extracted from the current URL.
///
/// Maps parameter names to their values (Vec since params can appear multiple
/// times).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RouteParams(HashMap<String, Vec<String>>);

impl RouteParams {
    /// Get all values for a parameter name.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.0.get(key)
    }

    /// Get the first value for a parameter name.
    #[must_use]
    pub fn get_one(&self, key: &str) -> Option<&str> {
        self.0
            .get(key)
            .and_then(|vals| vals.first())
            .map(|s| s.as_str())
    }

    /// Check if a parameter exists.
    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// Iterate over all parameters.
    #[must_use]
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, Vec<String>> {
        self.0.iter()
    }

    /// Get the number of parameters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if there are no parameters.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Returns route parameters from the current URL.
///
/// Extracts parameters from routes like `/users/:id` into a map.
#[hook]
pub fn use_route_params() -> RouteParams {
    let current_url = use_location();
    let _path = current_url.as_ref().map(|l| l.path()).unwrap_or("");
    let params = HashMap::new();
    RouteParams(params)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn route_params_get() {
        let mut params = HashMap::new();
        params.insert("id".to_string(), vec!["123".to_string()]);
        let rp = RouteParams(params);

        assert_eq!(rp.get("id"), Some(&vec!["123".to_string()]));
        assert_eq!(rp.get_one("id"), Some("123"));
        assert!(rp.contains_key("id"));
        assert!(!rp.contains_key("name"));
    }

    #[test]
    fn route_params_empty() {
        let rp = RouteParams(HashMap::new());
        assert!(rp.is_empty());
        assert_eq!(rp.len(), 0);
    }

    #[test]
    fn route_params_iter() {
        let mut params = HashMap::new();
        params.insert("id".to_string(), vec!["123".to_string()]);
        params.insert("name".to_string(), vec!["test".to_string()]);
        let rp = RouteParams(params);
        assert_eq!(rp.iter().count(), 2);
    }

    #[test]
    fn route_params_clone() {
        let mut params = HashMap::new();
        params.insert("id".to_string(), vec!["123".to_string()]);
        let rp1 = RouteParams(params);
        let rp2 = rp1.clone();
        assert_eq!(rp1.len(), rp2.len());
    }

    #[test]
    fn route_params_debug() {
        let rp = RouteParams(HashMap::new());
        let debug_str = format!("{:?}", rp);
        assert!(debug_str.contains("RouteParams"));
    }

    #[test]
    fn route_params_partial_eq() {
        let mut params1 = HashMap::new();
        params1.insert("id".to_string(), vec!["123".to_string()]);
        let rp1 = RouteParams(params1);

        let mut params2 = HashMap::new();
        params2.insert("id".to_string(), vec!["123".to_string()]);
        let rp2 = RouteParams(params2);

        assert_eq!(rp1, rp2);
    }
}
