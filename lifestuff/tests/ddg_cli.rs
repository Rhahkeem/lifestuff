use std::process::{Command, Output};

fn command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_lifestuff"));
    command
        .env_remove("DDG_BEARER")
        .env_remove("DDG_DEFAULT_SENDER")
        .env("NO_COLOR", "1");
    command
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).unwrap()
}

// Check DDG diagnostics independently of the CLI's error stream policy.
fn diagnostics(output: &Output) -> String {
    format!(
        "{}{}",
        stdout(output),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn custom_conversion_works_without_bearer_or_default_configuration() {
    let output = command()
        .args([
            "ddg",
            "convert",
            "--recipient",
            "alice@gmail.com",
            "--sender",
            "my-alias@duck.com",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        stdout(&output).trim(),
        "Use alice_at_gmail.com_my-alias@duck.com to send to alice@gmail.com from my-alias@duck.com"
    );
}

#[test]
fn default_conversion_reads_configuration_without_bearer() {
    let output = command()
        .env("DDG_DEFAULT_SENDER", "  default@duck.com  ")
        .args([
            "ddg",
            "convert",
            "--recipient",
            "alice@gmail.com",
            "--use-default",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        stdout(&output).trim(),
        "Use alice_at_gmail.com_default@duck.com to send to alice@gmail.com from default@duck.com"
    );
}

#[test]
fn missing_default_configuration_is_reported() {
    let output = command()
        .args([
            "ddg",
            "convert",
            "--recipient",
            "alice@gmail.com",
            "--use-default",
        ])
        .output()
        .unwrap();
    assert!(diagnostics(&output).contains("DDG_DEFAULT_SENDER is not configured"));
    assert!(!stdout(&output).contains("Use "));
}

#[test]
fn cli_rejects_non_duck_sender() {
    let output = command()
        .args([
            "ddg",
            "convert",
            "--recipient",
            "alice@gmail.com",
            "--sender",
            "myalias@gmail.com",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("Sender must be a @duck.com address")
    );
}

#[test]
fn generation_still_requires_bearer() {
    for args in [
        vec!["ddg", "generate"],
        vec![
            "ddg",
            "convert",
            "--recipient",
            "alice@gmail.com",
            "--generate",
        ],
    ] {
        let output = command().args(args).output().unwrap();
        assert!(diagnostics(&output).contains("DDG_BEARER"));
        assert!(!stdout(&output).contains("Use "));
    }
}
