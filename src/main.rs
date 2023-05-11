use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let open_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");
    let response = fetch_gpt_api(&open_api_key).await?;
    println!("{:?}", response);
    Ok(())
}

async fn fetch_gpt_api(open_api_key: &str) -> Result<ChatCompletion, anyhow::Error> {
    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    let request_body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": "Say this is a test!"}],
        "temperature": 0.7
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", open_api_key))
        .json(&request_body)
        .send()
        .await?
        .text()
        .await?;
    let parsed_response = serde_json::from_str(&response)?;
    Ok(parsed_response)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletion {
    id: String,     // Id of response
    object: String, // Response object type
    created: u64,
    model: String,
    usage: Usage, // Token usage
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    prompt_tokens: u32,     // Number of tokens required for prompt
    completion_tokens: u32, // Number of tokens used to generate the response
    total_tokens: u32,      // Total number of tokens used for both input prompts and responses
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    message: Message,      // A list of messages describing the conversation so far.
    finish_reason: String, // Reason for response completed
    index: u32,            // Index of choices
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    role: String,    // system, user, or assistant
    content: String, // The contents of the message.
}
