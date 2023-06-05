use anyhow::{ensure, Result};
use clap::Args;
use reqwest::StatusCode;
use serde_json::Value;
use std::time::Duration;

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
    // TODO add date for historical exchange info <-do this after datetimekeeper revamp
}

pub fn handle_currency_opertions(currency_args: &Currency, verbose: bool) -> Result<()> {
    ensure!(
        currency_args.from.len() == 3,
        "Invalid currency passed you Jabroni!"
    );

    ensure!(
        currency_args.to.iter().all(|currency| currency.len() == 3),
        "Invalid destination currency passed.... you Jabroni!!"
    );

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .https_only(true)
        .build()?;

    let target_url = format!(
        "https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest/currencies/{}.json",
        currency_args.from.to_lowercase()
    );

    if verbose {
        println!("target url = {target_url}");
    }
    let response = client.get(target_url).send()?;

    ensure!(
        response.status() == StatusCode::OK,
        "Got a bad response code from currency API: {}",
        response.status()
    );

    let response_body = response.text()?;
    let json_response: Value = serde_json::from_str(&response_body)?;
    let currency_map = json_response[currency_args.from.to_lowercase()]
        .as_object()
        .unwrap();

    if verbose {
        println!("{:#?}", currency_map);
    }

    let mut bad_currencies: Vec<&str> = Vec::new();
    for currency in &currency_args.to {
        let currency_data = currency_map.get(&currency.to_lowercase());
        if currency_data.is_none() {
            bad_currencies.push(currency);
            continue;
        }

        let multiplier = currency_data.unwrap().as_f64().unwrap();

        println!(
            "{}{} = {:.2}{}",
            currency_args.amt,
            currency_args.from.to_uppercase(),
            (currency_args.amt * multiplier),
            currency.to_uppercase()
        );
    }

    if !bad_currencies.is_empty() {
        println!(
            "Invalid currencies: {:#?}",
            bad_currencies
                .iter()
                .map(|currency| currency.to_uppercase())
                .collect::<Vec<_>>()
                .join(",")
        );
    }
    Ok(())
}
