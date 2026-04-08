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
