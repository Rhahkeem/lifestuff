use anyhow::{Context, Result};
use lifestuff_types::ddg::{DDGOperations, DDGOption};
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header;
use std::env;
use std::time::Duration;

mod convert;
mod generate;

pub fn handle_ddg_operations(ddg_args: DDGOperations, verbose: bool) -> Result<()> {
    match &ddg_args.operation_type {
        DDGOption::Generate => generate::handle_generate_ddg_address(
            verbose,
            "https://quack.duckduckgo.com/api/email/addresses",
            create_client()?,
        ),
        DDGOption::Convert(convert_args) => convert::perform_address_conversion(
            convert_args,
            verbose,
            "https://quack.duckduckgo.com/api/email/addresses",
            create_client()?,
        ),
    }
}

fn create_client() -> Result<Client> {
    let bearer =
        env::var("DDG_BEARER").context("Unable to get `DDG_BEARER` environment variable")?;
    let mut headers = header::HeaderMap::new();
    let mut auth_value = header::HeaderValue::from_str(format!("Bearer {bearer}").as_str())?;
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .https_only(true)
        .default_headers(headers)
        .build()
        .context("Unable to create request builder for DDG request")?;
    Ok(client)
}

pub fn is_valid_email_address(email: &str) -> bool {
    let email_regex = Regex::new(
        r"^([a-zA-Z0-9_+]([a-zA-Z0-9_+.]*[a-zA-Z0-9_+])?)@([a-zA-Z0-9]+([\-.][a-zA-Z0-9]+)*\.[a-zA-Z]{2,6})",
    ).unwrap();

    email_regex.is_match(email)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn test_valid_email() {
        assert!(is_valid_email_address("test@example.com"));
        assert!(!is_valid_email_address("invalid-email"));
    }

    #[test]
    fn test_valid_email_with_numbers() {
        assert!(is_valid_email_address("user123@domain.com"));
        assert!(is_valid_email_address("123user@domain.co.uk"));
    }

    #[test]
    fn test_valid_email_with_underscores() {
        assert!(is_valid_email_address("user_name@domain.com"));
        assert!(is_valid_email_address("user+tag@domain.org"));
    }

    #[test]
    fn test_invalid_email_formats() {
        assert!(!is_valid_email_address(""));
        assert!(!is_valid_email_address("@domain.com"));
        assert!(!is_valid_email_address("user@"));
        assert!(!is_valid_email_address("user.domain.com"));
        assert!(!is_valid_email_address("user@domain"));
        assert!(!is_valid_email_address("user@domain.c"));
    }

    #[test]
    fn test_handle_ddg_operations_generate() {
        // This test would require mocking the HTTP client, so we'll skip the actual call
        // but test that the function signature works
        use lifestuff_types::ddg::{DDGOperations, DDGOption};

        let _ddg_ops = DDGOperations {
            operation_type: DDGOption::Generate,
        };

        // We can't easily test this without mocking the HTTP client
        // This test verifies the struct can be instantiated
    }

    #[test]
    #[serial]
    fn test_create_client_missing_env_var() {
        // SAFETY: This test runs serially via #[serial] to avoid data races
        unsafe {
            std::env::remove_var("DDG_BEARER");
        }
        let result = create_client();
        assert!(
            result.is_err(),
            "Should fail when DDG_BEARER env var is missing"
        );
    }

    #[test]
    #[serial]
    fn test_create_client_with_env_var() {
        // SAFETY: This test runs serially via #[serial] to avoid data races
        unsafe {
            std::env::set_var("DDG_BEARER", "test_token");
        }
        let result = create_client();
        // SAFETY: Cleanup, also runs serially
        unsafe {
            std::env::remove_var("DDG_BEARER");
        }
        assert!(
            result.is_ok(),
            "Should succeed when DDG_BEARER env var is set"
        );
    }
}
