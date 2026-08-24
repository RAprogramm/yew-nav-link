// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

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
    /// Supports both absolute URLs (`scheme://host[:port]/path?query#fragment`)
    /// and path-only strings (`/path?query#fragment`). A `host` is only set
    /// when an authority is present (i.e. the input has a `scheme://`); a
    /// path-only input leaves `host` as `None`. `query` and `fragment` are
    /// parsed even when the authority carries no path
    /// (e.g. `https://example.com?a=1`).
    ///
    /// A userinfo component (`user:pass@host`) is stripped and not exposed.
    /// IPv6 hosts keep their brackets (e.g. `"[::1]"`), matching the WHATWG
    /// URL representation.
    #[must_use]
    pub fn parse(url: &str) -> Self {
        let mut parts = Self::default();

        let after_authority = if let Some((scheme, rest)) = url.split_once("://") {
            parts.scheme = Some(scheme.to_string());
            let authority_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
            let (authority, remainder) = rest.split_at(authority_end);
            let (host, port) = split_host_port(authority);
            parts.host = Some(host.to_string());
            parts.port = port.map(ToString::to_string);
            remainder
        } else {
            url
        };

        let mut rest = after_authority;
        if let Some((before, fragment)) = rest.split_once('#') {
            parts.fragment = Some(fragment.to_string());
            rest = before;
        }

        let path = if let Some((path, query)) = rest.split_once('?') {
            parts.query = Some(query.to_string());
            path
        } else {
            rest
        };

        parts.path = if path.is_empty() || path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{path}")
        };

        parts
    }

    /// Returns the query string parsed into [`QueryParams`], if present.
    #[must_use]
    pub fn query_params(&self) -> Option<QueryParams> {
        self.query.as_ref().map(|q| QueryParams::parse(q))
    }
}

/// Splits an RFC 3986 authority into host and optional port, discarding any
/// userinfo (`user:pass@`) prefix.
///
/// Bracketed IPv6 hosts (`[::1]`) keep their brackets and only text after the
/// closing bracket is considered a port delimiter; an unclosed bracket yields
/// the whole remainder as the host. Non-bracketed hosts split on the last
/// colon, since a registered name cannot contain `:`.
fn split_host_port(authority: &str) -> (&str, Option<&str>) {
    let host_port = authority.rsplit_once('@').map_or(authority, |(_, hp)| hp);
    if host_port.starts_with('[') {
        host_port.find(']').map_or((host_port, None), |end| {
            let (host, rest) = host_port.split_at(end + 1);
            (host, rest.strip_prefix(':'))
        })
    } else {
        host_port
            .rsplit_once(':')
            .map_or((host_port, None), |(host, port)| (host, Some(port)))
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

    /// A path-only input has no authority, so `host` stays `None`.
    #[test]
    fn url_parts_parse_path_only() {
        let parts = UrlParts::parse("/api/v1/users");
        assert_eq!(parts.path, "/api/v1/users");
        assert!(parts.scheme.is_none());
        assert_eq!(parts.host, None);
    }

    #[test]
    fn url_parts_parse_host_only_with_query() {
        let parts = UrlParts::parse("https://example.com?foo=bar");
        assert_eq!(parts.scheme, Some("https".to_string()));
        assert_eq!(parts.host, Some("example.com".to_string()));
        assert_eq!(parts.path, "");
        assert_eq!(parts.query, Some("foo=bar".to_string()));
        assert_eq!(parts.query_params().unwrap().get("foo"), Some("bar"));
    }

    #[test]
    fn url_parts_parse_host_only_with_fragment() {
        let parts = UrlParts::parse("https://example.com#top");
        assert_eq!(parts.host, Some("example.com".to_string()));
        assert_eq!(parts.path, "");
        assert_eq!(parts.fragment, Some("top".to_string()));
        assert!(parts.query.is_none());
    }

    #[test]
    fn url_parts_parse_host_only_with_query_and_fragment() {
        let parts = UrlParts::parse("https://example.com:8080?a=1#frag");
        assert_eq!(parts.host, Some("example.com".to_string()));
        assert_eq!(parts.port, Some("8080".to_string()));
        assert_eq!(parts.path, "");
        assert_eq!(parts.query, Some("a=1".to_string()));
        assert_eq!(parts.fragment, Some("frag".to_string()));
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
        assert_eq!(parts.path, "");
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

    #[test]
    fn url_parts_parse_relative_path_gains_leading_slash() {
        let parts = UrlParts::parse("docs/api?x=1");
        assert_eq!(parts.path, "/docs/api");
        assert_eq!(parts.query, Some("x=1".to_string()));
        assert!(parts.host.is_none());
    }

    #[test]
    fn url_parts_parse_ipv6_host_with_port() {
        let parts = UrlParts::parse("https://[::1]:8080/api");
        assert_eq!(parts.host, Some("[::1]".to_string()));
        assert_eq!(parts.port, Some("8080".to_string()));
        assert_eq!(parts.path, "/api");
    }

    #[test]
    fn url_parts_parse_ipv6_host_without_port() {
        let parts = UrlParts::parse("https://[2001:db8::1]/api");
        assert_eq!(parts.host, Some("[2001:db8::1]".to_string()));
        assert_eq!(parts.port, None);
        assert_eq!(parts.path, "/api");
    }

    #[test]
    fn url_parts_parse_userinfo_is_stripped() {
        let parts = UrlParts::parse("https://user:pass@example.com/api");
        assert_eq!(parts.host, Some("example.com".to_string()));
        assert_eq!(parts.port, None);
        assert_eq!(parts.path, "/api");
    }

    #[test]
    fn url_parts_parse_userinfo_with_port() {
        let parts = UrlParts::parse("ftp://user@example.com:21/files");
        assert_eq!(parts.scheme, Some("ftp".to_string()));
        assert_eq!(parts.host, Some("example.com".to_string()));
        assert_eq!(parts.port, Some("21".to_string()));
    }

    #[test]
    fn url_parts_parse_userinfo_with_ipv6_host() {
        let parts = UrlParts::parse("https://admin@[::1]:9443");
        assert_eq!(parts.host, Some("[::1]".to_string()));
        assert_eq!(parts.port, Some("9443".to_string()));
        assert_eq!(parts.path, "");
    }

    #[test]
    fn url_parts_parse_unclosed_ipv6_bracket() {
        let parts = UrlParts::parse("https://[::1/api");
        assert_eq!(parts.host, Some("[::1".to_string()));
        assert_eq!(parts.port, None);
    }
}
