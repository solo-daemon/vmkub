use dotenv::dotenv;
use reqwest::Client;
use serde_json::{from_str, Value};

pub async fn verify_payment(transaction_id: &str) -> Result<bool, reqwest::Error> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set.");
    let url = format!(
        "https://api-sepolia.etherscan.io/api?module=transaction&action=gettxreceiptstatus&txhash={}&apikey={}",
        transaction_id, api_key
    );

    let response = Client::new().get(&url).send().await?;
    let body = response.text().await?;
    let json_body: Value = match serde_json::from_str(&body) {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            return Ok(false); // Return false in case of parsing error
        }
    };

    if let Some(status) = json_body["result"]["status"].as_str() {
        Ok(status == "1")
    } else {
        Ok(false)
    }
}
