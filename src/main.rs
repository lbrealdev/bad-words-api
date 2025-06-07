#![warn(clippy::all)]

use anyhow::{Result, anyhow};
use colored::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{env, process, time::Duration};

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    content: String,
    bad_words_total: i32,
    censored_content: String,
}

static API_URL: &str = "https://api.apilayer.com/bad_words?censor_character=*";

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("API_KEY").map_err(|_| {
        anyhow!("Environment variable API_KEY is required (e.g., export API_KEY=your_token)")
    })?;

    let body = env::var("BODY").map_err(|_| anyhow!("Variable 'BODY' was not found!"))?;

    let client = Client::builder()
        .connect_timeout(Duration::from_secs(2))
        .timeout(Duration::from_secs(5))
        .build()?;

    let response = client
        .post(API_URL)
        .header("apikey", token)
        .body(body)
        .send()
        .await;

    match response {
        Ok(res) => match res.json::<Message>().await {
            Ok(msg) => {
                let json = serde_json::json!({
                    "input_content": msg.content,
                    "output_content": msg.censored_content,
                    "bad_words_total": msg.bad_words_total,
                });

                let json_pretty = serde_json::to_string_pretty(&json)?;

                println!("{}", json_pretty.bright_green());
            }
            Err(e) => {
                eprintln!("JSON convert error: {e}");
            }
        },
        Err(e) if e.is_timeout() => {
            eprintln!("Timeout! The request took too long to complete.");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    }

    Ok(())
}
