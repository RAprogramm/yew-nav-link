#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn query_params_get() {
        let mut params = HashMap::new();
        params.insert("q".to_string(), vec!["rust".to_string()]);
        let qp = QueryParams(params);

        assert_eq!(qp.get("q"), Some(&vec!["rust".to_string()]));
        assert_eq!(qp.get_one("q"), Some("rust"));
        assert!(qp.contains_key("q"));
        assert!(!qp.contains_key("name"));
    }

    #[test]
    fn query_params_empty() {
        let qp = QueryParams(HashMap::new());
        assert!(qp.is_empty());
        assert_eq!(qp.len(), 0);
        assert_eq!(qp.get("id").is_none(), true);
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
            vec!["rust".to_string(), "web".to_string()],
        );
        let qp = QueryParams(params);

        assert_eq!(qp.get_one("tag"), Some("rust"));
        assert_eq!(
            qp.get("tag"),
            Some(&vec!["rust".to_string(), "web".to_string()])
        );
    }

    #[test]
    fn query_params_iter() {
        let mut params = HashMap::new();
        params.insert("q".to_string(), vec!["rust".to_string()]);
        let qp = QueryParams(params);

        let count = qp.iter().count();
        assert_eq!(count, 1);
    }

    #[test]
    fn query_params_clone() {
        let mut params = HashMap::new();
        params.insert("q".to_string(), vec!["rust".to_string()]);
        let qp1 = QueryParams(params);
        let qp2 = qp1.clone();

        assert_eq!(qp1.len(), qp2.len());
        assert_eq!(qp1.get_one("q"), qp2.get_one("q"));
    }

    #[test]
    fn query_params_debug() {
        let qp = QueryParams(HashMap::new());
        let debug_str = format!("{:?}", qp);
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
    fn query_params_get_none() {
        let qp = QueryParams(HashMap::new());
        assert_eq!(qp.get("nonexistent"), None);
        assert_eq!(qp.get_one("nonexistent"), None);
    }

    #[test]
    fn query_params_not_equal_different_values() {
        let mut params1 = HashMap::new();
        params1.insert("q".to_string(), vec!["rust".to_string()]);
        let qp1 = QueryParams(params1);

        let mut params2 = HashMap::new();
        params2.insert("q".to_string(), vec!["go".to_string()]);
        let qp2 = QueryParams(params2);

        assert_ne!(qp1, qp2);
    }

    #[test]
    fn query_params_not_equal_different_keys() {
        let mut params1 = HashMap::new();
        params1.insert("q".to_string(), vec!["rust".to_string()]);
        let qp1 = QueryParams(params1);

        let mut params2 = HashMap::new();
        params2.insert("page".to_string(), vec!["1".to_string()]);
        let qp2 = QueryParams(params2);

        assert_ne!(qp1, qp2);
    }

    #[test]
    fn query_params_not_equal_different_length() {
        let mut params1 = HashMap::new();
        params1.insert("q".to_string(), vec!["rust".to_string()]);
        let qp1 = QueryParams(params1);

        let mut params2 = HashMap::new();
        params2.insert("q".to_string(), vec!["rust".to_string(), "go".to_string()]);
        let qp2 = QueryParams(params2);

        assert_ne!(qp1, qp2);
    }
}
