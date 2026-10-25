use anyhow::{Context, Result};
use lifestuff_types::ddg::{DDGOperations, DDGOption};
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
        DDGOption::Convert(convert_args) => {
            convert::perform_address_conversion(convert_args, || {
                generate::generate_ddg_address(
                    verbose,
                    "https://quack.duckduckgo.com/api/email/addresses",
                    create_client()?,
                )
            })
        }
    }
}

fn create_client() -> Result<Client> {
    let bearer =
        env::var("DDG_BEARER").context("Unable to get `DDG_BEARER` environment variable")?;
    anyhow::ensure!(!bearer.trim().is_empty(), "DDG_BEARER is not configured");
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

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

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
