use anyhow::ensure;
use clap::{ArgGroup, Args};
use regex::Regex;
use std::sync::LazyLock;

static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i-u)\A[\w.!#$%&'*+/=?^`{|}~-]+@[a-z\d](?:[a-z\d-]{0,61}[a-z\d])?(?:\.[a-z\d](?:[a-z\d-]{0,61}[a-z\d])?)*\z")
        .expect("valid email regex")
});

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
    /// Original recipient email address
    pub recipient: String,
    #[clap(
        action=clap::ArgAction::SetTrue,
        help = "Use DDG_DEFAULT_SENDER as the sender (@duck.com address)",
        short = 'd',
        long,
        group = "senderAddress"
    )]
    /// Use the default DDG address as a sender     
    pub use_default: bool,
    #[clap(
        action=clap::ArgAction::SetTrue,
        help = "Generate a new DDG address to use as sender",
        short,
        long,
        group = "senderAddress"
    )]
    /// Generate a new DDG address to use as sender
    pub generate: bool,
    #[clap(
        action=clap::ArgAction::Set,
        help = "Provide your @duck.com address to use as sender",
        short,
        long,
        group = "senderAddress",
        value_parser = validate_sender
    )]
    /// Optional DDG address to use as sender
    pub sender: Option<String>,
}

pub fn is_valid_email_address(email: &str) -> bool {
    EMAIL_REGEX.is_match(email.trim())
}

pub fn validate_email(email: &str) -> anyhow::Result<String> {
    ensure!(
        is_valid_email_address(email),
        format!("Invalid email address: {:?} provided", email)
    );
    Ok(email.trim().to_string())
}

pub fn validate_sender(email: &str) -> anyhow::Result<String> {
    let email = validate_email(email)?;
    ensure!(
        email.to_ascii_lowercase().ends_with("@duck.com"),
        "Sender must be a @duck.com address"
    );
    Ok(email)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cli, Commands, ddg::DDGOption};
    use clap::Parser;

    #[test]
    fn validates_complete_email_addresses_with_worker_compatible_rules() {
        for email in [
            "test@example.com",
            "123user@domain.co.uk",
            "user_name@domain.com",
            "user+tag@domain.org",
            "first-last@example.com",
            "billing-team@example.technology",
            "customer's.invoice@example.com",
        ] {
            assert!(is_valid_email_address(email), "should accept {email}");
        }
        for email in [
            "",
            "invalid-email",
            "@domain.com",
            "user@",
            "user.domain.com",
            "user@example.com<script>",
            "user@example.com extra",
            "user@example.com@duck.com",
            "user@-example.com",
            "user@example-.com",
            "user@example..com",
        ] {
            assert!(!is_valid_email_address(email), "should reject {email}");
        }
    }

    #[test]
    fn sender_must_use_exact_duck_domain() {
        assert_eq!(
            validate_sender("  my-alias@DUCK.COM  ").unwrap(),
            "my-alias@DUCK.COM"
        );
        for sender in [
            "user@gmail.com",
            "user@sub.duck.com",
            "user@duck.com.evil",
            "user@duck.com extra",
            "@duck.com",
        ] {
            assert!(validate_sender(sender).is_err(), "should reject {sender}");
        }
    }

    #[test]
    fn cli_trims_addresses_and_accepts_hyphens_and_long_domains() {
        let cli = Cli::try_parse_from([
            "lifestuff",
            "ddg",
            "convert",
            "--recipient",
            " billing-team@example.technology ",
            "--sender",
            " my-alias@duck.com ",
        ])
        .unwrap();
        let Commands::DDG(operations) = cli.command else {
            panic!("expected DDG")
        };
        let DDGOption::Convert(args) = operations.operation_type else {
            panic!("expected conversion")
        };
        assert_eq!(args.recipient, "billing-team@example.technology");
        assert_eq!(args.sender.as_deref(), Some("my-alias@duck.com"));
    }

    #[test]
    fn cli_rejects_invalid_senders_and_requires_one_sender_mode() {
        for options in [
            vec![],
            vec!["--sender", "user@gmail.com"],
            vec!["--sender", "user@duck.com extra"],
            vec!["--use-default", "--generate"],
            vec!["--use-default", "--sender", "user@duck.com"],
            vec!["--generate", "--sender", "user@duck.com"],
        ] {
            let mut args = vec![
                "lifestuff",
                "ddg",
                "convert",
                "--recipient",
                "alice@gmail.com",
            ];
            args.extend(options);
            assert!(Cli::try_parse_from(args).is_err());
        }
    }
}
