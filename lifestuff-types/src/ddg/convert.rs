use anyhow::ensure;
use clap::{ArgGroup, Args};
use regex::Regex;

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
        help = "Use the default DDG address as a sender",
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
        help = "Provide a DDG address to use as sender",
        short,
        long,
        group = "senderAddress",
        value_parser = validate_email
    )]
    /// Optional DDG address to use as sender 
    pub sender: Option<String>,
}

pub fn is_valid_email_address(email: &str) -> bool {
    let email_regex = Regex::new(
        r"^([a-zA-Z0-9_+]([a-zA-Z0-9_+.]*[a-zA-Z0-9_+])?)@([a-zA-Z0-9]+([\-.][a-zA-Z0-9]+)*\.[a-zA-Z]{2,6})",
    ).unwrap();

    email_regex.is_match(email)
}

fn validate_email(email: &str) -> anyhow::Result<String> {
    ensure!(
        is_valid_email_address(email),
        format!("Invalid email address: {:?} provided", email)
    );
    Ok(email.to_string())
}
