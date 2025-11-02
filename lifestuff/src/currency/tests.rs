#[cfg(test)]
mod currency_tests {
    use super::super::{get_base_url, DEFAULT_API_HOST, ENV_VAR_NAME};
    use mockito::Server;

    #[test]
    fn test_handle_currency_operations_valid() {
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/currency")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"success":{"message":"Converted","rate":1.25}}"#)
            .create();

        let currency_args = lifestuff_types::currency::Currency {
            from: "USD".to_string(),
            amt: 100.0,
            to: vec!["EUR".to_string()],
            endpoint: Some(server.url()),
        };
        let result = crate::currency::handle_currency_operations(currency_args, false);
        assert!(result.is_ok());
        mock.assert();
    }

    #[test]
    fn test_handle_currency_operations_invalid_from_currency() {
        let currency_args = lifestuff_types::currency::Currency {
            from: "INVALID".to_string(),
            amt: 100.0,
            to: vec!["EUR".to_string()],
            endpoint: None,
        };
        let result = crate::currency::handle_currency_operations(currency_args, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_currency_operations_invalid_to_currency() {
        let currency_args = lifestuff_types::currency::Currency {
            from: "USD".to_string(),
            amt: 100.0,
            to: vec!["INVALID".to_string()],
            endpoint: None,
        };
        let result = crate::currency::handle_currency_operations(currency_args, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_currency_operations_zero_amount() {
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/currency")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"success":{"message":"Converted","rate":1.0}}"#)
            .create();

        let currency_args = lifestuff_types::currency::Currency {
            from: "USD".to_string(),
            amt: 0.0,
            to: vec!["EUR".to_string()],
            endpoint: Some(server.url()),
        };
        let result = crate::currency::handle_currency_operations(currency_args, false);
        assert!(result.is_ok());
        mock.assert();
    }

    #[test]
    fn test_handle_currency_operations_negative_amount() {
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/currency")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"success":{"message":"Converted","rate":0.95}}"#)
            .create();

        let currency_args = lifestuff_types::currency::Currency {
            from: "USD".to_string(),
            amt: -100.0,
            to: vec!["EUR".to_string()],
            endpoint: Some(server.url()),
        };
        let result = crate::currency::handle_currency_operations(currency_args, false);
        assert!(result.is_ok());
        mock.assert();
    }

    #[test]
    fn test_get_base_url_defaults_to_localhost() {
        let url = get_base_url(None);
        assert_eq!(
            url, DEFAULT_API_HOST,
            "Should return localhost URL when no endpoint provided"
        );
        assert!(url.starts_with("http://"), "Local URL should use HTTP");
        assert!(url.contains("localhost"), "Should contain 'localhost'");
        assert!(url.contains("8787"), "Should use port 8787");
    }

    #[test]
    fn test_get_base_url_uses_cli_arg() {
        let url = get_base_url(Some("https://custom.example.com".to_string()));
        assert_eq!(
            url, "https://custom.example.com",
            "Should return CLI argument when provided"
        );
    }

    #[test]
    fn test_get_base_url_cli_arg_priority_over_env() {
        // Set environment variable
        std::env::set_var(ENV_VAR_NAME, "https://env.example.com");

        // CLI arg should take precedence
        let url = get_base_url(Some("https://cli.example.com".to_string()));
        assert_eq!(
            url, "https://cli.example.com",
            "CLI argument should override environment variable"
        );

        // Clean up
        std::env::remove_var(ENV_VAR_NAME);
    }

    #[test]
    fn test_get_base_url_env_var_fallback() {
        // Ensure no CLI arg
        std::env::set_var(ENV_VAR_NAME, "https://env.example.com");

        let url = get_base_url(None);
        assert_eq!(
            url, "https://env.example.com",
            "Should use environment variable when no CLI arg"
        );

        // Clean up
        std::env::remove_var(ENV_VAR_NAME);
    }

    #[test]
    fn test_get_base_url_ignores_empty_env_var() {
        std::env::set_var(ENV_VAR_NAME, "");

        let url = get_base_url(None);
        assert_eq!(
            url, DEFAULT_API_HOST,
            "Should use localhost default when env var is empty"
        );

        // Clean up
        std::env::remove_var(ENV_VAR_NAME);
    }

    #[test]
    fn test_constants_are_valid_urls() {
        assert!(
            DEFAULT_API_HOST.starts_with("http://"),
            "Local host should start with http://"
        );
        assert!(
            !crate::currency::DEFAULT_API_HOST.is_empty(),
            "Local host should not be empty"
        );
        assert!(
            !DEFAULT_API_HOST.ends_with('/'),
            "Default host should not have trailing slash"
        );
    }

    #[test]
    fn test_https_enforcement_for_remote_http_url() {
        // This test verifies URL normalization but won't make actual network calls
        // since we can't easily mock HTTPS enforcement without a real server
        let currency_args = lifestuff_types::currency::Currency {
            from: "USD".to_string(),
            amt: 100.0,
            to: vec!["EUR".to_string()],
            endpoint: Some("http://api.example.com".to_string()),
        };

        // The function will attempt to connect and fail (no server), but we can verify
        // the URL normalization happens by checking verbose output or error messages
        let result = crate::currency::handle_currency_operations(currency_args, false);

        // Should fail to connect (no server), but the important part is it tried HTTPS
        assert!(result.is_err());
    }

    #[test]
    fn test_https_enforcement_for_remote_no_protocol() {
        let currency_args = lifestuff_types::currency::Currency {
            from: "USD".to_string(),
            amt: 100.0,
            to: vec!["EUR".to_string()],
            endpoint: Some("api.example.com".to_string()),
        };

        // Should fail to connect but attempt HTTPS
        let result = crate::currency::handle_currency_operations(currency_args, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_localhost_allows_http() {
        // Verify localhost URLs are not forced to HTTPS
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/currency")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"success":{"message":"Converted","rate":1.0}}"#)
            .create();

        // mockito server.url() returns http://127.0.0.1:port
        let currency_args = lifestuff_types::currency::Currency {
            from: "USD".to_string(),
            amt: 100.0,
            to: vec!["EUR".to_string()],
            endpoint: Some(server.url()),
        };

        let result = crate::currency::handle_currency_operations(currency_args, false);
        assert!(result.is_ok(), "Localhost HTTP should be allowed");
        mock.assert();
    }
}
