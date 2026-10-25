use anyhow::{Context, Result, ensure};
use lifestuff_types::ddg::convert::validate_sender;
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
    println!("New address to use is \"{ddg_address}\"");

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
    normalize_generated_address(&ddg_email.address)
        .context("DDG API returned an invalid generated sender")
}

fn normalize_generated_address(address: &str) -> Result<String> {
    let address = address.trim();
    if address.contains('@') {
        validate_sender(address)
    } else {
        validate_sender(&format!("{address}@duck.com"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_response_is_normalized_to_one_full_sender_address() {
        let mut server = mockito::Server::new();
        for address in ["generated-alias", "generated-alias@duck.com"] {
            let mock = server
                .mock("POST", "/api/email/addresses")
                .with_status(201)
                .with_body(serde_json::json!({ "address": address }).to_string())
                .expect(1)
                .create();
            let result = generate_ddg_address(
                false,
                &format!("{}/api/email/addresses", server.url()),
                Client::new(),
            )
            .unwrap();
            assert_eq!(result, "generated-alias@duck.com");
            mock.assert();
        }
    }

    #[test]
    fn rejects_malformed_generated_addresses() {
        for address in [
            "",
            "  ",
            "bad alias",
            "alias@gmail.com",
            "alias@duck.com extra",
            "alias@duck.com@duck.com",
        ] {
            assert!(
                normalize_generated_address(address).is_err(),
                "should reject {address}"
            );
        }
    }

    #[test]
    fn rejects_failed_or_malformed_api_responses() {
        let mut server = mockito::Server::new();
        for (status, body) in [
            (429, "{}"),
            (201, "not json"),
            (201, "{}"),
            (201, r#"{"address":"bad@gmail.com"}"#),
        ] {
            let mock = server
                .mock("POST", "/api/email/addresses")
                .with_status(status)
                .with_body(body)
                .expect(1)
                .create();
            assert!(
                generate_ddg_address(
                    false,
                    &format!("{}/api/email/addresses", server.url()),
                    Client::new()
                )
                .is_err()
            );
            mock.assert();
        }
    }

    #[test]
    fn test_ddg_address_display() {
        let ddg_address = DDGAddress {
            address: String::from("test@duck.com"),
        };
        assert_eq!(format!("{}", ddg_address), "test@duck.com");
    }
}
