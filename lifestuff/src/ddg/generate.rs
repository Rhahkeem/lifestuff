use anyhow::{Context, Result, ensure};
use reqwest::StatusCode;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DDGAddress {
    address: String,
}

impl std::fmt::Display for DDGAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.address)
    }
}

pub fn handle_generate_ddg_address(verbose: bool, url: &str, client: Client) -> Result<()> {
    let ddg_address = generate_ddg_address(verbose, url, client)?;
    println!("New address to use is \"{ddg_address}@duck.com\"");

    Ok(())
}

pub fn generate_ddg_address(verbose: bool, url: &str, client: Client) -> Result<String> {
    let response = client
        .post(url)
        .send()
        .context("Unable to send request to get new DDG Address")?;

    ensure!(
        response.status() == StatusCode::CREATED,
        "Got a bad response code from DDG Endpoint: {}",
        response.status()
    );

    let response_body = response
        .text()
        .context("Unable to parse JSON response from DDG")?;
    if verbose {
        println!("So the response from the backend was {response_body}");
    }

    let ddg_email: DDGAddress = serde_json::from_str(response_body.as_str())?;
    Ok(ddg_email.address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ddg_address_display() {
        let ddg_address = DDGAddress {
            address: String::from("test@duck.com"),
        };
        assert_eq!(format!("{}", ddg_address), "test@duck.com");
    }
}
