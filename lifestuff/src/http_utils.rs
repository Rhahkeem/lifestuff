// HTTP utility functions for API operations

/// Normalize a base URL to enforce HTTPS for non-localhost endpoints
///
/// For localhost URLs (containing "localhost" or "127.0.0.1"), the URL is returned unchanged.
/// For remote URLs:
/// - URLs already using https:// are returned unchanged
/// - URLs using http:// are upgraded to https://
/// - URLs without a scheme get https:// prepended
pub(crate) fn normalize_api_url(base_url: String) -> String {
    let is_localhost = base_url.contains("localhost") || base_url.contains("127.0.0.1");

    if is_localhost || base_url.starts_with("https://") {
        base_url
    } else if base_url.starts_with("http://") {
        base_url.replace("http://", "https://")
    } else {
        format!("https://{}", base_url)
    }
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
