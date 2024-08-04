use crate::ddg::generate::generate_ddg_address;
use crate::ddg::is_valid_email_address;
use anyhow::{ensure, Result};
use clap::{ArgGroup, Args};
use colored::Colorize;
use reqwest::blocking::Client;

#[derive(Debug, Args, Clone)]
#[clap(group(
    ArgGroup::new("senderAddress")
        .required(true)
        .multiple(false)
))]
pub struct DDGConvert {
    #[clap(
        help = "Converts a regular email address to be used by Duckduckgo as a recipient",
        short,
        long,
        value_parser = validate_email
    )]
    pub recipient: String,
    #[clap(
        action=clap::ArgAction::SetTrue,
        help = "Use the default DDG address as a sender",
        short = 'd',
        long,
        group = "senderAddress"
    )]
    pub use_default: bool,
    #[clap(
        action=clap::ArgAction::SetTrue,
        help = "Generate a new DDG address to use as sender",
        short,
        long,
        group = "senderAddress"
    )]
    pub generate: bool,
    #[clap(
        action=clap::ArgAction::Set,
        help = "Provide a DDG address to use as sender",
        short,
        long,
        group = "senderAddress",
        value_parser = validate_email
    )]
    pub sender: Option<String>,
}

fn validate_email(email: &str) -> Result<String> {
    ensure!(
        is_valid_email_address(email),
        "Invalid email address provided"
    );
    Ok(email.to_string())
}

pub fn perform_address_conversion(
    convert_args: &DDGConvert,
    verbose: bool,
    target_url: &str,
    reqwest_client: Client,
) -> Result<()> {
    ensure!(
        is_valid_email_address(convert_args.recipient.as_str()),
        format!(
            "Invalid email address: {:?} provided",
            convert_args.recipient
        )
    );

    let sender_address = if let Some(sender_address_arg) = &convert_args.sender {
        sender_address_arg.to_string()
    } else if convert_args.use_default {
        String::from("jonathanc@duck.com")
    } else if convert_args.generate {
        generate_ddg_address(verbose, target_url, reqwest_client)?
    } else {
        return Err(anyhow::anyhow!(
            "At least one sender address option must be specified."
        ));
    };

    let final_address = format!(
        "{}_{}",
        convert_args.recipient.replace("@", "_at_").trim(),
        sender_address
    );
    println!(
        "Use {} to send to {} from {}",
        { final_address.bold().green() },
        { convert_args.recipient.italic().bright_yellow() },
        { sender_address.italic().bold().bright_cyan() }
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::time::Duration;

    #[test]
    fn test_perform_address_conversion_generate() {
        dotenv().ok(); // This loads the .env file

        let mut s = mockito::Server::new();

        s.mock("POST", "/api/email/addresses")
            .with_status(201)
            .with_body(r#"{"address": "generated@duck.com"}"#)
            .expect(1)
            .create();

        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .https_only(false) // Set https_only to false for mocking
            .build()
            .unwrap();

        let args = DDGConvert {
            recipient: String::from("test@example.com"),
            use_default: false,
            generate: true,
            sender: None,
        };
        let target_url = format!("{}/api/email/addresses", &s.url());

        assert!(perform_address_conversion(&args, false, &target_url, client).is_ok());
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("invalid-email").is_err());
    }

    #[test]
    fn test_perform_address_conversion_valid() {
        let args = DDGConvert {
            recipient: String::from("test@example.com"),
            use_default: false,
            generate: false,
            sender: Some(String::from("sender@duck.com")),
        };
        assert!(
            perform_address_conversion(&args, false, "", reqwest::blocking::Client::new()).is_ok()
        );
    }

    #[test]
    fn test_perform_address_conversion_use_default() {
        let args = DDGConvert {
            recipient: String::from("test@example.com"),
            use_default: true,
            generate: false,
            sender: None,
        };
        assert!(
            perform_address_conversion(&args, false, "", reqwest::blocking::Client::new()).is_ok()
        );
    }

    #[test]
    fn test_perform_address_conversion_missing_sender() {
        let args = DDGConvert {
            recipient: String::from("test@example.com"),
            use_default: false,
            generate: false,
            sender: None,
        };
        assert!(
            perform_address_conversion(&args, false, "", reqwest::blocking::Client::new()).is_err()
        );
    }
}
