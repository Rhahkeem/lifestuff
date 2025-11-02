use anyhow::{Context, Result};
use lifestuff_types::mortgage::*;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fs;
use std::time::Duration;

use crate::http_utils;

const DEFAULT_MORTGAGE_HOST: &str = "http://localhost:8787";
const ENV_VAR_NAME: &str = "LIFESTUFF_API_ENDPOINT";

/// For non-localhost endpoints, HTTPS-only mode is enforced
fn create_client(base_url: &str) -> Result<reqwest::blocking::Client> {
    let is_localhost = base_url.contains("localhost") || base_url.contains("127.0.0.1");

    let mut builder = reqwest::blocking::Client::builder().timeout(Duration::from_secs(10));

    if !is_localhost {
        builder = builder.https_only(true);
    }

    builder
        .build()
        .context("Unable to create HTTP client for mortgage operations")
}

fn parse_api_result<T>(status: StatusCode, body: &str, context: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    if status == StatusCode::OK {
        let api_response: ApiResponse<T> = serde_json::from_str(body)
            .with_context(|| format!("Failed to parse API response for {}", context))?;

        if api_response.success {
            return api_response
                .data
                .with_context(|| format!("No data in successful {} response", context));
        }

        let error_msg = api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string());
        anyhow::bail!("API Error: {}", error_msg);
    } else {
        let api_response: Result<ApiResponse<Value>, _> = serde_json::from_str(body);
        if let Ok(resp) = api_response {
            if !resp.success {
                let error_msg = resp
                    .error
                    .or(resp.message)
                    .unwrap_or_else(|| "Unknown error".to_string());
                anyhow::bail!("API Error: {}", error_msg);
            }
        }

        anyhow::bail!("Request failed with status {}: {}", status, body);
    }
}

/// Get the base URL for the mortgage API
fn get_base_url(cli_endpoint: Option<String>) -> String {
    // First priority: explicit CLI argument
    if let Some(endpoint) = cli_endpoint {
        return endpoint;
    }

    // Second priority: environment variable
    if let Ok(endpoint) = std::env::var(ENV_VAR_NAME) {
        if !endpoint.is_empty() {
            return endpoint;
        }
    }

    // Final fallback: localhost for development
    DEFAULT_MORTGAGE_HOST.to_string()
}

/// Validate payment amount
fn validate_payment_amount(amount: f64) -> Result<()> {
    anyhow::ensure!(
        amount > 0.0,
        "Payment amount must be positive, got: {}",
        amount
    );
    Ok(())
}

/// Validate payment amounts (if specified, they must be positive)
/// Both can be None, this means use underlying mortgage API's default
/// monthly payment amount with no overpayment value
fn validate_payment_args(args: &PaymentArgs) -> Result<()> {
    // Validate monthly_payment if provided
    if let Some(amount) = args.monthly_payment {
        anyhow::ensure!(
            amount >= 0.0,
            "Monthly payment must be non-negative, got: {}",
            amount
        );
    }

    // Validate overpayment if provided
    if let Some(amount) = args.overpayment {
        anyhow::ensure!(
            amount > 0.0,
            "Overpayment must be positive, got: {}",
            amount
        );
    }

    Ok(())
}

/// Handle payment recording
fn handle_payment(args: PaymentArgs, base_url: &str, verbose: bool) -> Result<()> {
    validate_payment_args(&args)?;

    let client = create_client(base_url)?;
    let url = format!("{}/mortgage/payment", base_url);

    if verbose {
        println!("Recording payment at: {}", url);
        if args.use_default {
            println!("Using default monthly payment");
        } else if let Some(monthly) = args.monthly_payment {
            println!("Monthly payment: {}", monthly);
        } else {
            println!("Monthly payment: 0.0 (pure overpayment)");
        }
        if let Some(overpayment) = args.overpayment {
            println!("Overpayment: {}", overpayment);
        }
        println!("Date: {}", args.date);
    }

    // Determine scheduled_payment based on flags:
    // - use_default: None (API uses mortgage's default monthly payment)
    // - monthly_payment specified: Some(value) (explicit override)
    // - neither: Some(0.0) (pure overpayment, no monthly payment)
    let scheduled_payment = if args.use_default {
        None
    } else if let Some(amount) = args.monthly_payment {
        Some(amount)
    } else {
        Some(0.0)
    };

    let request_body = PaymentRequest {
        payment_date: args.date,
        scheduled_payment,
        additional_payment: args.overpayment,
        notes: args.note,
    };

    let response = client
        .post(&url)
        .json(&request_body)
        .send()
        .context("Failed to send payment request")?;

    let status = response.status();
    let body = response.text()?;

    if verbose {
        println!("Response status: {}", status);
        println!("Response body: {}", body);
    }

    let payment: PaymentRecord = parse_api_result(status, &body, "payment")?;

    println!("✓ Payment recorded successfully");
    println!("  Payment #: {}", payment.payment_number.unwrap_or(0));
    println!("  Total Payment: ${:.2}", payment.total_payment);
    println!("  Principal: ${:.2}", payment.principal_portion);
    println!("  Interest: ${:.2}", payment.interest_portion);
    println!("  Remaining Balance: ${:.2}", payment.remaining_balance);

    Ok(())
}

/// Handle getting mortgage status
fn handle_status(base_url: &str, verbose: bool) -> Result<()> {
    let client = create_client(base_url)?;
    let url = format!("{}/mortgage/status", base_url);

    if verbose {
        println!("Getting mortgage status from: {}", url);
    }

    let response = client
        .get(&url)
        .send()
        .context("Failed to send status request")?;

    let status = response.status();
    let body = response.text()?;

    if verbose {
        println!("Response status: {}", status);
        println!("Response body: {}", body);
    }

    let status_data: MortgageSummary = parse_api_result(status, &body, "status")?;

    println!("\n=== Mortgage Status ===");
    println!("Current Balance: ${:.2}", status_data.current_balance);
    println!("Initial Principal: ${:.2}", status_data.initial_principal);
    println!("Interest Rate: {:.2}%", status_data.interest_rate * 100.0);
    println!("Monthly Payment: ${:.2}", status_data.monthly_payment);
    println!("Term: {} years", status_data.term_years);
    println!("Payments Made: {}", status_data.payments_made);
    println!("Total Paid: ${:.2}", status_data.total_paid);
    println!("Extra Payments: ${:.2}", status_data.extra_payments);
    println!("Start Date: {}", status_data.start_date);
    if !status_data.last_payment_date.is_empty() {
        println!("Last Payment: {}", status_data.last_payment_date);
    }
    println!("Status: {}", status_data.status);
    println!("======================\n");

    Ok(())
}

/// Handle getting payment history
fn handle_history(base_url: &str, verbose: bool) -> Result<()> {
    let client = create_client(base_url)?;
    let url = format!("{}/mortgage/history", base_url);

    if verbose {
        println!("Getting payment history from: {}", url);
    }

    let response = client
        .get(&url)
        .send()
        .context("Failed to send history request")?;

    let status = response.status();
    let body = response.text()?;

    if verbose {
        println!("Response status: {}", status);
        println!("Response body: {}", body);
    }

    let payments: Vec<PaymentRecord> = parse_api_result(status, &body, "payment history")?;

    // Build structured output
    let mut output = String::new();
    output.push_str("\n=== Payment History ===\n");
    for payment in payments.iter() {
        output.push_str(&format!(
            "\nPayment #{}\n",
            payment.payment_number.unwrap_or(0)
        ));
        output.push_str(&format!("  Date: {}\n", payment.payment_date));
        output.push_str(&format!("  Scheduled: ${:.2}\n", payment.scheduled_payment));
        output.push_str(&format!(
            "  Additional: ${:.2}\n",
            payment.additional_payment
        ));
        output.push_str(&format!("  Total: ${:.2}\n", payment.total_payment));
        output.push_str(&format!("  Principal: ${:.2}\n", payment.principal_portion));
        output.push_str(&format!("  Interest: ${:.2}\n", payment.interest_portion));
        output.push_str(&format!(
            "  Remaining Balance: ${:.2}\n",
            payment.remaining_balance
        ));
        if let Some(note) = &payment.notes {
            output.push_str(&format!("  Note: {}\n", note));
        }
    }
    output.push_str("\n======================\n");

    let filename = "payment_history.txt";
    fs::write(filename, &output)
        .with_context(|| format!("Failed to write payment history to {}", filename))?;

    println!("Payment history written to: {}", filename);

    if verbose {
        print!("{}", output);
    }

    Ok(())
}

/// Handle getting interest history
fn handle_interest_history(base_url: &str, verbose: bool) -> Result<()> {
    let client = create_client(base_url)?;
    let url = format!("{}/mortgage/interest-history", base_url);

    if verbose {
        println!("Getting interest history from: {}", url);
    }

    let response = client
        .get(&url)
        .send()
        .context("Failed to send interest history request")?;

    let status = response.status();
    let body = response.text()?;

    if verbose {
        println!("Response status: {}", status);
        println!("Response body: {}", body);
    }

    let postings: Vec<InterestPosting> = parse_api_result(status, &body, "interest history")?;

    // Build structured output
    let mut output = String::new();
    output.push_str("\n=== Interest History ===\n");
    for (idx, posting) in postings.iter().enumerate() {
        output.push_str(&format!("\nPosting #{}\n", idx + 1));
        output.push_str(&format!("  Date: {}\n", posting.posting_date));
        output.push_str(&format!("  Amount: ${:.2}\n", posting.interest_amount));
        output.push_str(&format!(
            "  Remaining Balance: ${:.2}\n",
            posting.remaining_balance
        ));
        if let Some(note) = &posting.notes {
            output.push_str(&format!("  Note: {}\n", note));
        }
    }
    output.push_str("\n======================\n");

    let filename = "interest_history.txt";
    fs::write(filename, &output)
        .with_context(|| format!("Failed to write interest history to {}", filename))?;

    println!("Interest history written to: {}", filename);

    if verbose {
        print!("{}", output);
    }

    Ok(())
}

/// Handle adding an interest posting
fn handle_interest_posting(args: InterestPostingArgs, base_url: &str, verbose: bool) -> Result<()> {
    validate_payment_amount(args.amount)?;

    let client = create_client(base_url)?;
    let url = format!("{}/mortgage/interest", base_url);

    if verbose {
        println!("Adding interest posting at: {}", url);
        println!("Amount: {}, Date: {}", args.amount, args.date);
    }

    let request_body = InterestPostingRequest {
        posting_date: args.date,
        interest_amount: args.amount,
        remaining_balance: None,
        notes: args.note,
    };

    let response = client
        .post(&url)
        .json(&request_body)
        .send()
        .context("Failed to send interest posting request")?;

    let status = response.status();
    let body = response.text()?;

    if verbose {
        println!("Response status: {}", status);
        println!("Response body: {}", body);
    }

    let posting: InterestPosting = parse_api_result(status, &body, "interest posting")?;

    println!("✓ Interest posting added successfully");
    println!("  Date: {}", posting.posting_date);
    println!("  Interest Amount: ${:.2}", posting.interest_amount);
    println!("  Remaining Balance: ${:.2}", posting.remaining_balance);

    Ok(())
}

/// Main handler for mortgage operations
pub fn handle_mortgage_operations(command: MortgageCommand, verbose: bool) -> Result<()> {
    let base_url = get_base_url(command.endpoint);
    let normalized_url = http_utils::normalize_api_url(base_url);

    if verbose {
        println!("Using mortgage API at: {}", normalized_url);
    }

    match command.operation {
        MortgageOperation::Payment(args) => handle_payment(args, &normalized_url, verbose),
        MortgageOperation::Status => handle_status(&normalized_url, verbose),
        MortgageOperation::History => handle_history(&normalized_url, verbose),
        MortgageOperation::InterestHistory => handle_interest_history(&normalized_url, verbose),
        MortgageOperation::Interest(args) => {
            handle_interest_posting(args, &normalized_url, verbose)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    fn test_parse_api_result_successful_response() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct DummyData {
            value: i32,
        }

        let body = r#"{"success":true,"data":{"value":42},"error":null,"message":null}"#;
        let result: DummyData = parse_api_result(StatusCode::OK, body, "dummy")
            .expect("Should parse successful response");

        assert_eq!(result, DummyData { value: 42 });
    }

    #[test]
    fn test_parse_api_result_success_false_returns_error() {
        let body = r#"{"success":false,"data":null,"error":"boom","message":null}"#;
        let err = parse_api_result::<Value>(StatusCode::OK, body, "dummy").unwrap_err();

        let msg = err.to_string();
        assert!(msg.contains("API Error"));
        assert!(msg.contains("boom"));
    }

    #[test]
    fn test_parse_api_result_missing_data_returns_error() {
        let body = r#"{"success":true,"data":null,"error":null,"message":null}"#;
        let err = parse_api_result::<Value>(StatusCode::OK, body, "dummy context").unwrap_err();

        let msg = err.to_string();
        assert!(msg.contains("No data"));
        assert!(msg.contains("dummy context"));
    }

    #[test]
    fn test_parse_api_result_non_ok_with_api_error_message() {
        let body = r#"{"success":false,"data":null,"error":"bad request","message":null}"#;
        let err = parse_api_result::<Value>(StatusCode::BAD_REQUEST, body, "dummy").unwrap_err();

        let msg = err.to_string();
        assert!(msg.contains("API Error"));
        assert!(msg.contains("bad request"));
    }

    #[test]
    fn test_parse_api_result_non_ok_with_non_json_body() {
        let body = "Internal Server Error";
        let err = parse_api_result::<Value>(StatusCode::INTERNAL_SERVER_ERROR, body, "dummy")
            .unwrap_err();

        let msg = err.to_string();
        assert!(msg.contains("status"));
        assert!(msg.contains("500"));
        assert!(msg.contains("Internal Server Error"));
    }

    #[test]
    fn test_validate_payment_amount_positive() {
        let result = validate_payment_amount(1000.0);
        assert!(result.is_ok(), "Should accept positive amounts");
    }

    #[test]
    fn test_validate_payment_amount_negative() {
        let result = validate_payment_amount(-100.0);
        assert!(result.is_err(), "Should reject negative amounts");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("positive"),
            "Error message should mention 'positive'"
        );
        assert!(
            err_msg.contains("-100"),
            "Error message should show the invalid value"
        );
    }

    #[test]
    fn test_validate_payment_amount_zero() {
        let result = validate_payment_amount(0.0);
        assert!(result.is_err(), "Should reject zero amounts");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("positive"),
            "Error message should mention 'positive'"
        );
    }

    #[test]
    fn test_validate_payment_amount_small_positive() {
        let result = validate_payment_amount(0.01);
        assert!(result.is_ok(), "Should accept small positive amounts");
    }

    #[test]
    fn test_validate_payment_amount_large() {
        let result = validate_payment_amount(1_000_000.0);
        assert!(result.is_ok(), "Should accept large amounts");
    }

    #[test]
    fn test_get_base_url_defaults_to_localhost() {
        let url = get_base_url(None);
        assert_eq!(
            url, DEFAULT_MORTGAGE_HOST,
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
            url, DEFAULT_MORTGAGE_HOST,
            "Should use localhost default when env var is empty"
        );

        // Clean up
        std::env::remove_var(ENV_VAR_NAME);
    }

    #[test]
    fn test_constants_are_valid_urls() {
        assert!(
            DEFAULT_MORTGAGE_HOST.starts_with("http://"),
            "Local host should start with http://"
        );
        assert!(
            !DEFAULT_MORTGAGE_HOST.is_empty(),
            "Local host should not be empty"
        );
        assert!(
            !DEFAULT_MORTGAGE_HOST.ends_with('/'),
            "Default host should not have trailing slash"
        );
    }

    #[test]
    fn test_payment_operation_with_negative_amount() {
        let args = PaymentArgs {
            use_default: false,
            monthly_payment: Some(-500.0),
            overpayment: None,
            date: "01/01/2024".to_string(),
            note: None,
        };
        // Validation should catch this before any HTTP call
        let result = validate_payment_args(&args);
        assert!(
            result.is_err(),
            "Should fail validation for negative payment"
        );
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.to_lowercase().contains("non-negative")
                || err_msg.to_lowercase().contains("positive"),
            "Error should mention non-negative requirement, got: {}",
            err_msg
        );
        assert!(
            err_msg.contains("-500"),
            "Error should show the invalid amount, got: {}",
            err_msg
        );
    }

    #[test]
    fn test_payment_operation_with_no_amounts_defaults_to_zero() {
        let args = PaymentArgs {
            use_default: false,
            monthly_payment: None,
            overpayment: None,
            date: "01/01/2024".to_string(),
            note: None,
        };
        let result = validate_payment_args(&args);
        assert!(
            result.is_ok(),
            "Should accept no amounts (defaults to 0 monthly payment)"
        );
    }

    #[test]
    fn test_payment_operation_with_valid_monthly_only() {
        let args = PaymentArgs {
            use_default: false,
            monthly_payment: Some(500.0),
            overpayment: None,
            date: "01/01/2024".to_string(),
            note: None,
        };
        let result = validate_payment_args(&args);
        assert!(result.is_ok(), "Should accept monthly payment only");
    }

    #[test]
    fn test_payment_operation_with_valid_overpayment_only() {
        let args = PaymentArgs {
            use_default: false,
            monthly_payment: None,
            overpayment: Some(1000.0),
            date: "01/01/2024".to_string(),
            note: None,
        };
        let result = validate_payment_args(&args);
        assert!(result.is_ok(), "Should accept overpayment only");
    }

    #[test]
    fn test_payment_operation_with_both_amounts() {
        let args = PaymentArgs {
            use_default: false,
            monthly_payment: Some(500.0),
            overpayment: Some(200.0),
            date: "01/01/2024".to_string(),
            note: None,
        };
        let result = validate_payment_args(&args);
        assert!(result.is_ok(), "Should accept both payment types");
    }

    #[test]
    fn test_payment_operation_with_zero_monthly_and_overpayment() {
        let args = PaymentArgs {
            use_default: false,
            monthly_payment: Some(0.0),
            overpayment: Some(1000.0),
            date: "01/01/2024".to_string(),
            note: None,
        };
        let result = validate_payment_args(&args);
        assert!(
            result.is_ok(),
            "Should accept zero monthly payment with overpayment (pure overpayment case)"
        );
    }

    #[test]
    fn test_payment_operation_with_use_default_flag() {
        let args = PaymentArgs {
            use_default: true,
            monthly_payment: None,
            overpayment: Some(500.0),
            date: "01/01/2024".to_string(),
            note: None,
        };
        let result = validate_payment_args(&args);
        assert!(
            result.is_ok(),
            "Should accept use_default flag with overpayment"
        );
    }

    #[test]
    fn test_interest_posting_with_zero_amount() {
        let args = InterestPostingArgs {
            amount: 0.0,
            date: "01/01/2024".to_string(),
            note: None,
        };
        // Validation should catch this before any HTTP call
        let result = validate_payment_amount(args.amount);
        assert!(result.is_err(), "Should fail validation for zero interest");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("positive"),
            "Error should mention positive requirement"
        );
    }
}
