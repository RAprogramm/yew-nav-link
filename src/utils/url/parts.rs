//! URL parts parsing.

use super::query::QueryParams;

/// Parsed components of a URL.
#[derive(Clone, Debug, Default)]
pub struct UrlParts {
    /// URL scheme (e.g. `"https"`).
    pub scheme:   Option<String>,
    /// Host name (e.g. `"example.com"`).
    pub host:     Option<String>,
    /// Port number as a string (e.g. `"8080"`).
    pub port:     Option<String>,
    /// Path component (e.g. `"/api/v1"`).
    pub path:     String,
    /// Raw query string without the leading `?`.
    pub query:    Option<String>,
    /// Fragment identifier without the leading `#`.
    pub fragment: Option<String>
}

impl UrlParts {
    /// Parses a URL string into its component parts.
    ///
    /// Supports both absolute URLs (`scheme://host/path`) and
    /// path-only strings (`/path?query#fragment`).
    #[must_use]
    pub fn parse(url: &str) -> Self {
        let mut parts = Self::default();

        let mut remaining = url;

        if let Some((scheme, rest)) = remaining.split_once("://") {
            parts.scheme = Some(scheme.to_string());
            remaining = rest;
        }

        if let Some((host_port, rest)) = remaining.split_once('/') {
            if let Some((host, port)) = host_port.split_once(':') {
                parts.host = Some(host.to_string());
                parts.port = Some(port.to_string());
            } else {
                parts.host = Some(host_port.to_string());
            }
            remaining = rest;
        } else {
            if let Some((host, port)) = remaining.split_once(':') {
                parts.host = Some(host.to_string());
                parts.port = Some(port.to_string());
            } else {
                parts.host = Some(remaining.to_string());
            }
            return parts;
        }

        if let Some((path_query, fragment)) = remaining.split_once('#') {
            parts.fragment = Some(fragment.to_string());
            remaining = path_query;
        }

        if let Some((path, query)) = remaining.split_once('?') {
            parts.path = format!("/{path}");
            parts.query = Some(query.to_string());
        } else {
            parts.path = format!("/{remaining}");
        }

        parts
    }

    /// Returns the query string parsed into [`QueryParams`], if present.
    #[must_use]
    pub fn query_params(&self) -> Option<QueryParams> {
        self.query.as_ref().map(|q| QueryParams::parse(q))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_parts_parse_simple() {
        let parts = UrlParts::parse("/path/to/page");
        assert!(parts.path.contains("path"));
    }

    #[test]
    fn url_parts_parse_with_query() {
        let parts = UrlParts::parse("/api/users?page=1&limit=10");
        assert_eq!(parts.path, "/api/users");
        assert!(parts.query.is_some());
        let params = parts.query_params().unwrap();
        assert_eq!(params.get("page"), Some("1"));
    }

    #[test]
    fn url_parts_parse_with_fragment() {
        let parts = UrlParts::parse("/docs#section-1");
        assert_eq!(parts.path, "/docs");
        assert_eq!(parts.fragment, Some("section-1".to_string()));
    }

    #[test]
    fn url_parts_parse_full() {
        let parts = UrlParts::parse("https://example.com:8080/api/v1?key=value#top");
        assert_eq!(parts.scheme, Some("https".to_string()));
        assert_eq!(parts.host, Some("example.com".to_string()));
        assert_eq!(parts.port, Some("8080".to_string()));
        assert_eq!(parts.path, "/api/v1");
        assert_eq!(parts.fragment, Some("top".to_string()));
    }

    #[test]
    fn url_parts_parse_host_only() {
        let parts = UrlParts::parse("https://example.com");
        assert_eq!(parts.scheme, Some("https".to_string()));
        assert_eq!(parts.host, Some("example.com".to_string()));
        assert_eq!(parts.path, "");
    }

    #[test]
    fn url_parts_parse_scheme_host() {
        let parts = UrlParts::parse("https://example.com/path");
        assert_eq!(parts.scheme, Some("https".to_string()));
        assert_eq!(parts.host, Some("example.com".to_string()));
        assert_eq!(parts.path, "/path");
    }

    #[test]
    fn url_parts_parse_scheme_host_port_only() {
        let parts = UrlParts::parse("https://example.com:8080");
        assert_eq!(parts.scheme, Some("https".to_string()));
        assert_eq!(parts.host, Some("example.com".to_string()));
        assert_eq!(parts.port, Some("8080".to_string()));
        assert_eq!(parts.path, "");
    }

    #[test]
    fn url_parts_parse_path_only() {
        // When URL starts with '/' and has more '/', the part before first '/' becomes
        // host
        let parts = UrlParts::parse("/api/v1/users");
        assert_eq!(parts.path, "/api/v1/users");
        assert!(parts.scheme.is_none());
        assert_eq!(parts.host, Some(String::new()));
    }

    #[test]
    fn url_parts_parse_simple_path() {
        let parts = UrlParts::parse("/path");
        assert_eq!(parts.path, "/path");
    }

    #[test]
    fn url_parts_query_params_with_values() {
        let parts = UrlParts::parse("/api?foo=bar&baz=qux");
        let params = parts.query_params().unwrap();
        assert_eq!(params.get("foo"), Some("bar"));
        assert_eq!(params.get("baz"), Some("qux"));
    }

    #[test]
    fn url_parts_default() {
        let parts = UrlParts::default();
        assert!(parts.path.is_empty());
        assert!(parts.scheme.is_none());
        assert!(parts.host.is_none());
    }

    #[test]
    fn url_parts_parse_no_fragment() {
        let parts = UrlParts::parse("/docs?query=1");
        assert_eq!(parts.path, "/docs");
        assert!(parts.fragment.is_none());
    }

    #[test]
    fn url_parts_parse_no_query() {
        let parts = UrlParts::parse("/docs#section");
        assert_eq!(parts.path, "/docs");
        assert_eq!(parts.fragment, Some("section".to_string()));
    }
}
