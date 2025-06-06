#![warn(clippy::all)]

use colored::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{env, time::Duration};

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    content: String,
    bad_words_total: i32,
    censored_content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("API_KEY")
        .expect("Environment variable API_KEY is required (e.g., export API_KEY=your_token)");

    let body = env::var("BODY").expect("Variable 'BODY' was not found!");

    let client = Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .build()?;

    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", &token)
        .body(body)
        .send()
        .await?
        .json::<Message>()
        .await?;

    let json = serde_json::json!({
        "input_content": res.content,
        "output_content": res.censored_content,
        "bad_words_total": res.bad_words_total,
    });

    let json_pretty = serde_json::to_string_pretty(&json)?;

    println!("{}", json_pretty.bright_green());

    Ok(())
}
