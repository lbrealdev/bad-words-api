use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let token = env::var("API_KEY").expect("Variable 'API_KEY' was not found!");
    let body = env::var("BODY").expect("Variable 'BODY' was not found!");

    let client = reqwest::Client::new();
    
    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", &token)
        .body(body)
        .send()
        .await?
        .text()
        .await?;
    
    println!("{}", res);
    
    Ok(())
}
