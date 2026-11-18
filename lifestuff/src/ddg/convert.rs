use anyhow::{Context, Result, ensure};
use colored::Colorize;
use lifestuff_types::ddg::convert::{DDGConvert, validate_email, validate_sender};

#[derive(Debug)]
struct ConvertedAddress {
    recipient: String,
    sender: String,
    address: String,
}

fn convert_address(
    args: &DDGConvert,
    default_sender: Option<&str>,
    generate_sender: impl FnOnce() -> Result<String>,
) -> Result<ConvertedAddress> {
    let recipient = validate_email(&args.recipient)?;
    let mode_count = usize::from(args.sender.is_some())
        + usize::from(args.use_default)
        + usize::from(args.generate);
    ensure!(
        mode_count == 1,
        "Exactly one sender address option must be specified."
    );

    let sender = if let Some(sender) = &args.sender {
        validate_sender(sender)?
    } else if args.use_default {
        let configured = default_sender
            .filter(|sender| !sender.trim().is_empty())
            .context("DDG_DEFAULT_SENDER is not configured")?;
        validate_sender(configured)
            .map_err(|_| anyhow::anyhow!("DDG_DEFAULT_SENDER is not a valid @duck.com address"))?
    } else {
        validate_sender(&generate_sender()?)?
    };

    let sender_local = sender.split_once('@').expect("validated sender").0;
    let address = format!(
        "{}_{}@duck.com",
        recipient.replace('@', "_at_"),
        sender_local
    );
    Ok(ConvertedAddress {
        recipient,
        sender,
        address,
    })
}

pub fn perform_address_conversion(
    args: &DDGConvert,
    generate_sender: impl FnOnce() -> Result<String>,
) -> Result<()> {
    let default_sender = std::env::var("DDG_DEFAULT_SENDER").ok();
    let converted = convert_address(args, default_sender.as_deref(), generate_sender)?;
    println!(
        "Use {} to send to {} from {}",
        converted.address.bold().green(),
        converted.recipient.italic().bright_yellow(),
        converted.sender.italic().bold().bright_cyan()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args() -> DDGConvert {
        DDGConvert {
            recipient: "test@example.com".into(),
            use_default: false,
            generate: false,
            sender: None,
        }
    }

    fn no_generation() -> Result<String> {
        panic!("local conversion must not generate an alias")
    }

    #[test]
    fn custom_sender_is_trimmed_and_domain_is_formatted_consistently() {
        let args = DDGConvert {
            recipient: "  billing-team@example.technology  ".into(),
            sender: Some("  My-Alias@DUCK.COM  ".into()),
            ..args()
        };
        let converted = convert_address(&args, None, no_generation).unwrap();
        assert_eq!(converted.recipient, "billing-team@example.technology");
        assert_eq!(converted.sender, "My-Alias@DUCK.COM");
        assert_eq!(
            converted.address,
            "billing-team_at_example.technology_My-Alias@duck.com"
        );
    }

    #[test]
    fn rejects_non_duck_custom_sender() {
        let args = DDGConvert {
            sender: Some("sender@example.org".into()),
            ..args()
        };
        let error = convert_address(&args, None, no_generation).unwrap_err();
        assert!(error.to_string().contains("@duck.com"));
    }

    #[test]
    fn uses_configured_default_sender_without_generation() {
        let args = DDGConvert {
            use_default: true,
            ..args()
        };
        let converted =
            convert_address(&args, Some("  default@duck.com  "), no_generation).unwrap();
        assert_eq!(converted.sender, "default@duck.com");
        assert_eq!(converted.address, "test_at_example.com_default@duck.com");
    }

    #[test]
    fn rejects_missing_or_invalid_default_sender() {
        let args = DDGConvert {
            use_default: true,
            ..args()
        };
        for configured in [None, Some(""), Some("  ")] {
            let error = convert_address(&args, configured, no_generation).unwrap_err();
            assert_eq!(error.to_string(), "DDG_DEFAULT_SENDER is not configured");
        }
        for configured in ["sender@example.com", "bad@duck.com extra", "@duck.com"] {
            let error = convert_address(&args, Some(configured), no_generation).unwrap_err();
            assert_eq!(
                error.to_string(),
                "DDG_DEFAULT_SENDER is not a valid @duck.com address"
            );
        }
    }

    #[test]
    fn generated_sender_is_included_in_final_address() {
        let args = DDGConvert {
            generate: true,
            ..args()
        };
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/api/email/addresses")
            .with_status(201)
            .with_body(r#"{"address":"generated-alias"}"#)
            .expect(1)
            .create();
        let url = format!("{}/api/email/addresses", server.url());
        let converted = convert_address(&args, None, || {
            crate::ddg::generate::generate_ddg_address(
                false,
                &url,
                reqwest::blocking::Client::new(),
            )
        })
        .unwrap();
        assert_eq!(converted.sender, "generated-alias@duck.com");
        assert_eq!(
            converted.address,
            "test_at_example.com_generated-alias@duck.com"
        );
        mock.assert();
    }

    #[test]
    fn propagates_generation_errors() {
        let args = DDGConvert {
            generate: true,
            ..args()
        };
        let error =
            convert_address(&args, None, || anyhow::bail!("generation failed")).unwrap_err();
        assert_eq!(error.to_string(), "generation failed");
    }

    #[test]
    fn rejects_invalid_recipient_before_generation() {
        let args = DDGConvert {
            recipient: "test@example.com extra".into(),
            generate: true,
            ..args()
        };
        assert!(convert_address(&args, None, no_generation).is_err());
    }

    #[test]
    fn requires_exactly_one_sender_mode() {
        for args in [
            args(),
            DDGConvert {
                generate: true,
                use_default: true,
                ..args()
            },
        ] {
            let error = convert_address(&args, None, no_generation).unwrap_err();
            assert_eq!(
                error.to_string(),
                "Exactly one sender address option must be specified."
            );
        }
    }
}
