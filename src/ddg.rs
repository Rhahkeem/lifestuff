use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header;
use std::env;
use std::time::Duration;

mod convert;
mod generate;

#[derive(Args, Debug)]
pub struct DDGOperations {
    #[command(subcommand)]
    operation_type: DDGOption,
}

#[derive(Subcommand, Debug)]
enum DDGOption {
    ///Generates a Duckduckgo email alias address
    Generate,
    ///Converts a regular email address to be used by Duckduckgo as a recipient
    Convert(convert::DDGConvert),
}

pub fn handle_ddg_operations(ddg_args: DDGOperations, verbose: bool) -> Result<()> {
    match &ddg_args.operation_type {
        DDGOption::Generate => generate::handle_generate_ddg_address(verbose,  "https://quack.duckduckgo.com/api/email/addresses", create_client()?),
        DDGOption::Convert(convert_args) => {
            convert::perform_address_conversion(convert_args, verbose, "https://quack.duckduckgo.com/api/email/addresses", create_client()?)
        }
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

    #[test]
    fn test_valid_email() {
        assert!(is_valid_email_address("test@example.com"));
        assert!(!is_valid_email_address("invalid-email"));
    }
}
