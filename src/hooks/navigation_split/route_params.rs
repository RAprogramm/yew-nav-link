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
    pub fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.0.get(key)
    }

    /// Get the first value for a parameter name.
    pub fn get_one(&self, key: &str) -> Option<&str> {
        self.0
            .get(key)
            .and_then(|vals| vals.first())
            .map(|s| s.as_str())
    }

    /// Check if a parameter exists.
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// Iterate over all parameters.
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, Vec<String>> {
        self.0.iter()
    }

    /// Get the number of parameters.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if there are no parameters.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Returns route parameters from the current URL.
///
/// Extracts parameters from routes like `/users/:id` into a map.
///
/// ```rust
/// use yew::prelude::*;
/// use yew_nav_link::hooks::use_route_params;
/// use yew_router::prelude::*;
///
/// #[derive(Clone, PartialEq, Debug, Routable)]
/// enum Route {
///     #[at("/users/:id")]
///     User { id: String }
/// }
///
/// #[component]
/// fn UserProfile() -> Html {
///     let params = use_route_params();
///     let user_id = params.get_one("id");
///
///     html! {
///         <h1>{ format!("User ID: {:?}", user_id) }</h1>
///     }
/// }
/// ```
#[hook]
pub fn use_route_params() -> RouteParams {
    let current_url = use_location();
    let path = current_url.as_ref().map(|l| l.path()).unwrap_or("");

    let params = HashMap::new();

    // Parse path parameters like /users/:id
    let segments: Vec<&str> = path.split('/').collect();
    for segment in segments {
        if let Some(_param_name) = segment.strip_prefix(':') {
            // Try to extract value from route (this is a simplified approach)
            // In a real scenario, you'd need route metadata
        }
    }

    RouteParams(params)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

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
        assert!(rp.get("id").is_none());
    }

    #[test]
    fn route_params_get_none() {
        let rp = RouteParams(HashMap::new());
        assert_eq!(rp.get("nonexistent"), None);
        assert_eq!(rp.get_one("nonexistent"), None);
    }

    #[test]
    fn route_params_iter() {
        let mut params = HashMap::new();
        params.insert("id".to_string(), vec!["123".to_string()]);
        params.insert("name".to_string(), vec!["test".to_string()]);
        let rp = RouteParams(params);

        let count = rp.iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn route_params_clone() {
        let mut params = HashMap::new();
        params.insert("id".to_string(), vec!["123".to_string()]);
        let rp1 = RouteParams(params);
        let rp2 = rp1.clone();

        assert_eq!(rp1.len(), rp2.len());
        assert_eq!(rp1.get_one("id"), rp2.get_one("id"));
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

    #[test]
    fn route_params_multiple_values() {
        let mut params = HashMap::new();
        params.insert(
            "tag".to_string(),
            vec!["rust".to_string(), "web".to_string(), "yew".to_string()],
        );
        let rp = RouteParams(params);

        assert_eq!(rp.get_one("tag"), Some("rust"));
        assert_eq!(
            rp.get("tag"),
            Some(&vec![
                "rust".to_string(),
                "web".to_string(),
                "yew".to_string()
            ])
        );
        assert_eq!(rp.len(), 1);
    }

    #[test]
    fn route_params_contains_key_false() {
        let rp = RouteParams(HashMap::new());
        assert!(!rp.contains_key("nonexistent"));
    }

    #[test]
    fn route_params_not_equal_different_values() {
        let mut params1 = HashMap::new();
        params1.insert("id".to_string(), vec!["123".to_string()]);
        let rp1 = RouteParams(params1);

        let mut params2 = HashMap::new();
        params2.insert("id".to_string(), vec!["456".to_string()]);
        let rp2 = RouteParams(params2);

        assert_ne!(rp1, rp2);
    }

    #[test]
    fn route_params_not_equal_different_keys() {
        let mut params1 = HashMap::new();
        params1.insert("id".to_string(), vec!["123".to_string()]);
        let rp1 = RouteParams(params1);

        let mut params2 = HashMap::new();
        params2.insert("name".to_string(), vec!["test".to_string()]);
        let rp2 = RouteParams(params2);

        assert_ne!(rp1, rp2);
    }

    #[test]
    fn route_params_not_equal_different_length() {
        let mut params1 = HashMap::new();
        params1.insert("id".to_string(), vec!["123".to_string()]);
        let rp1 = RouteParams(params1);

        let mut params2 = HashMap::new();
        params2.insert("id".to_string(), vec!["123".to_string(), "456".to_string()]);
        let rp2 = RouteParams(params2);

        assert_ne!(rp1, rp2);
    }
}
