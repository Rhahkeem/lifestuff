use anyhow::{ensure, Context, Result};
mod tests;

use clap::Args;
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

///Convert from one currency to another
#[derive(Debug, Args, Clone)]
pub struct Currency {
    #[clap(short, long, help = "Currency to convert from")]
    from: String,
    #[clap(
        short,
        long,
        allow_negative_numbers = false,
        help = "Amount to convert"
    )]
    amt: f64,
    #[clap(short, long, help = "Currency to convert to")]
    to: Vec<String>,
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
    let json_response_type: ResponseMessage = serde_json::from_str(&response_body)?;

    ensure!(
        matches!(json_response_type, ResponseMessage::success { .. }),
        "Got a bad response from currency API: {:?}",
        json_response_type
    );

    if verbose {
        println!("The response type was {:?}", json_response_type);
    }

    let json_response: Value = serde_json::from_str(&response_body)?;
    let response_message = json_response["success"]["message"].as_str().unwrap();
    let rate = json_response["success"]["rate"].as_f64().unwrap();
    for currency_target in currency_args.to.iter() {
        println!(
            "{:?} at a rate of 1 {} = {} {}",
            response_message, currency_args.from, rate, currency_target
        );
    }

    Ok(())
}
