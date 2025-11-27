// HTTP utility functions for API operations

use url::Url;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
