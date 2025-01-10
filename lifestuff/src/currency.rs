use anyhow::{ensure, Context, Result};
mod tests;

use lifestuff_types::currency::Currency;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize, Debug)]
enum ResponseMessage {
    success { message: String, rate: f32 },
    error { message: String },
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

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .https_only(true)
        .build()
        .context("Unable to create request buiilder for Currency request")?;

    let target_url = "https://lifestuff.thejcbfamily.workers.dev/currency";

    if verbose {
        println!("target url = {target_url}");
    }

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
    let json_response: Value = serde_json::from_str(&response_body)?;

    // Check if the response contains the expected fields
    let response_message = json_response["success"]["message"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing 'message' in response"))?;
    let rate = json_response["success"]["rate"]
        .as_f64()
        .ok_or_else(|| anyhow::anyhow!("Missing 'rate' in response"))? as f32;

    for currency_target in currency_args.to.iter() {
        println!(
            "{:?} at a rate of 1 {} = {} {}",
            response_message, currency_args.from, rate, currency_target
        );
    }

    Ok(())
}
