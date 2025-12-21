// HTTP utility functions for API operations

use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use url::Url;

pub(crate) const API_KEY_ENV_VAR: &str = "LIFESTUFF_API_KEY";
const API_KEY_HEADER: &str = "X-API-Key";

/// Check if a parsed URL host is localhost (localhost or 127.0.0.1)
fn is_localhost_host(url: &Url) -> bool {
    url.host_str()
        .map(|h| h == "localhost" || h == "127.0.0.1")
        .unwrap_or(false)
}

/// Check if a URL string points to localhost (localhost or 127.0.0.1)
pub(crate) fn is_localhost_url(base_url: &str) -> bool {
    Url::parse(base_url)
        .map(|u| is_localhost_host(&u))
        .unwrap_or(false)
}

/// Normalize a base URL to enforce HTTPS for non-localhost endpoints
///
/// For localhost URLs (host is "localhost" or "127.0.0.1"), the URL is returned unchanged.
/// For remote URLs:
/// - URLs already using https:// are returned unchanged
/// - URLs using http:// are upgraded to https://
/// - URLs without a scheme get https:// prepended
pub(crate) fn normalize_api_url(base_url: String) -> String {
    // First, try to parse as-is
    if let Ok(parsed) = Url::parse(&base_url) {
        if is_localhost_host(&parsed) || parsed.scheme() == "https" {
            return base_url;
        }
        // Remote URL with http:// - upgrade to https://
        if parsed.scheme() == "http" {
            let mut upgraded = parsed;
            upgraded.set_scheme("https").ok();
            return upgraded.to_string().trim_end_matches('/').to_string();
        }
    }

    // URL without scheme - try adding https://
    let with_scheme = format!("https://{}", base_url);
    if let Ok(parsed) = Url::parse(&with_scheme) {
        if is_localhost_host(&parsed) {
            // Localhost without scheme - use http://
            return format!("http://{}", base_url);
        }
        return with_scheme;
    }

    // Fallback: return original (will likely fail at HTTP client level)
    base_url
}

/// Get the API key from environment variable
pub(crate) fn get_api_key() -> Option<String> {
    std::env::var(API_KEY_ENV_VAR)
        .ok()
        .filter(|k| !k.is_empty())
}

/// Build default headers for API requests, including API key if available.
/// For non-localhost endpoints, warns if API key is missing.
pub(crate) fn build_request_headers(base_url: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    if let Some(api_key) = get_api_key() {
        if let Ok(mut value) = HeaderValue::from_str(&api_key) {
            value.set_sensitive(true);
            headers.insert(API_KEY_HEADER, value);
        }
    } else if !is_localhost_url(base_url) {
        eprintln!(
            "Warning: {} not set. Remote API requests may fail authentication.",
            API_KEY_ENV_VAR
        );
    }

    headers
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn test_normalize_localhost_unchanged() {
        assert_eq!(
            normalize_api_url("http://localhost:8787".to_string()),
            "http://localhost:8787"
        );
        assert_eq!(
            normalize_api_url("http://127.0.0.1:8787".to_string()),
            "http://127.0.0.1:8787"
        );
    }

    #[test]
    fn test_normalize_https_unchanged() {
        assert_eq!(
            normalize_api_url("https://api.example.com".to_string()),
            "https://api.example.com"
        );
    }

    #[test]
    fn test_normalize_http_to_https() {
        assert_eq!(
            normalize_api_url("http://api.example.com".to_string()),
            "https://api.example.com"
        );
    }

    #[test]
    fn test_normalize_no_scheme_gets_https() {
        assert_eq!(
            normalize_api_url("api.example.com".to_string()),
            "https://api.example.com"
        );
    }

    #[test]
    #[serial]
    fn test_get_api_key_returns_value_when_set() {
        // SAFETY: This test runs serially via #[serial] to avoid data races
        unsafe {
            std::env::set_var(API_KEY_ENV_VAR, "test-api-key-12345");
        }

        let key = get_api_key();
        assert_eq!(key, Some("test-api-key-12345".to_string()));

        // SAFETY: Cleanup, also runs serially
        unsafe {
            std::env::remove_var(API_KEY_ENV_VAR);
        }
    }

    #[test]
    #[serial]
    fn test_get_api_key_returns_none_when_empty() {
        // SAFETY: This test runs serially via #[serial] to avoid data races
        unsafe {
            std::env::set_var(API_KEY_ENV_VAR, "");
        }

        let key = get_api_key();
        assert!(key.is_none(), "Empty API key should return None");

        // SAFETY: Cleanup, also runs serially
        unsafe {
            std::env::remove_var(API_KEY_ENV_VAR);
        }
    }

    #[test]
    #[serial]
    fn test_get_api_key_returns_none_when_unset() {
        // SAFETY: This test runs serially via #[serial] to avoid data races
        unsafe {
            std::env::remove_var(API_KEY_ENV_VAR);
        }

        let key = get_api_key();
        assert!(key.is_none(), "Unset API key should return None");
    }

    #[test]
    #[serial]
    fn test_build_request_headers_includes_api_key() {
        // SAFETY: This test runs serially via #[serial] to avoid data races
        unsafe {
            std::env::set_var(API_KEY_ENV_VAR, "my-secret-key");
        }

        let headers = build_request_headers("https://api.example.com");

        assert_eq!(
            headers.get("Content-Type").map(|v| v.to_str().unwrap()),
            Some("application/json")
        );
        assert_eq!(
            headers.get("X-API-Key").map(|v| v.to_str().unwrap()),
            Some("my-secret-key")
        );

        // SAFETY: Cleanup, also runs serially
        unsafe {
            std::env::remove_var(API_KEY_ENV_VAR);
        }
    }

    #[test]
    #[serial]
    fn test_build_request_headers_no_api_key_for_localhost() {
        // SAFETY: This test runs serially via #[serial] to avoid data races
        unsafe {
            std::env::remove_var(API_KEY_ENV_VAR);
        }

        let headers = build_request_headers("http://localhost:8787");

        assert_eq!(
            headers.get("Content-Type").map(|v| v.to_str().unwrap()),
            Some("application/json")
        );
        assert!(
            headers.get("X-API-Key").is_none(),
            "Localhost requests without API key should not have X-API-Key header"
        );
    }
}
