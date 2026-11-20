use std::process::Command;

fn command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_lifestuff"));
    command.env("NO_COLOR", "1").env("RUST_BACKTRACE", "0");
    command
}

#[test]
fn runtime_errors_exit_one_and_report_to_stderr() {
    let output = command()
        .args([
            "currency", "--from", "INVALID", "--amt", "100", "--to", "EUR",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("Invalid currency")
    );
}

#[test]
fn configuration_errors_preserve_the_error_chain() {
    let output = command()
        .env_remove("DDG_BEARER")
        .args(["ddg", "generate"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Unable to get `DDG_BEARER` environment variable"));
    assert!(stderr.contains("Caused by:"));
}

#[test]
fn successful_commands_exit_zero_and_keep_output_on_stdout() {
    let output = command()
        .args(["dates", "add", "--date", "01/01/2023", "1", "days"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap().trim(),
        "(2023, January, 2) (00:00:00)"
    );
}

#[test]
fn argument_errors_keep_claps_exit_status() {
    let output = command().arg("unknown-command").output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("unrecognized subcommand")
    );
}

#[test]
fn help_and_version_still_exit_successfully() {
    for (flag, expected) in [("--help", "Usage:"), ("--version", "lifestuff")] {
        let output = command().arg(flag).output().unwrap();
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stderr.is_empty());
        assert!(String::from_utf8(output.stdout).unwrap().contains(expected));
    }
}
