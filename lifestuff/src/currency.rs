use crate::http_utils;
use anyhow::{ensure, Context, Result};
mod tests;

use lifestuff_types::currency::Currency;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

pub(super) const DEFAULT_API_HOST: &str = "http://localhost:8787";
pub(super) const ENV_VAR_NAME: &str = "LIFESTUFF_API_ENDPOINT";

#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize, Debug)]
enum ResponseMessage {
    success { message: String, rate: f32 },
    error { message: String },
}

/// Get the base URL for the currency API
pub(super) fn get_base_url(cli_endpoint: Option<String>) -> String {
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
    DEFAULT_API_HOST.to_string()
}

pub fn handle_currency_operations(currency_args: Currency, verbose: bool) -> Result<()> {
    ensure!(
        currency_args.from.len() == 3,
        "Invalid currency \"{}\" passed you Jabroni!",
        currency_args.from
    );

    ensure!(
        currency_args.to.iter().all(|currency| currency.len() == 3),
        "Invalid destination currency passed.... you Jabroni!!"
    );

    let base_url = get_base_url(currency_args.endpoint);
    let normalized_url = http_utils::normalize_api_url(base_url);

    let target_url = format!("{}/currency", normalized_url);

    if verbose {
        println!("Using currency API at: {}", normalized_url);
        println!("target url = {target_url}");
    }

    let is_localhost = normalized_url.contains("localhost") || normalized_url.contains("127.0.0.1");
    let mut client_builder = reqwest::blocking::Client::builder().timeout(Duration::from_secs(5));
    if !is_localhost {
        client_builder = client_builder.https_only(true);
    }

    let client = client_builder
        .build()
        .context("Unable to create request buiilder for Currency request")?;

    let mut json_body_map = HashMap::new();
    json_body_map.insert("target", currency_args.to[0].to_uppercase());
    json_body_map.insert("source", currency_args.from.to_uppercase());
    json_body_map.insert("amount", currency_args.amt.abs().to_string());

    let response = client
        .post(target_url)
        .json(&json_body_map)
        .send()
        .context("Unable to send request to get currency exchange rate")?;

    ensure!(
        response.status() == StatusCode::OK,
        "Got a bad response code from currency API: {}",
        response.status()
    );

    let response_body = response.text()?;
    if verbose {
        println!("So the response from the backend was {response_body}");
    }

    let api_response: ResponseMessage =
        serde_json::from_str(&response_body).context("Unable to parse currency API response")?;

    match api_response {
        ResponseMessage::success { message, rate } => {
            for currency_target in currency_args.to.iter() {
                println!(
                    "{:?} at a rate of 1 {} = {} {}",
                    message, currency_args.from, rate, currency_target
                );
            }
        }
        ResponseMessage::error { message } => {
            anyhow::bail!("Currency API error: {}", message);
        }
    }

    Ok(())
}
