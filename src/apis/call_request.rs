use crate::models::general::llm::{ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::{header::AUTHORIZATION, Client, RequestBuilder};
use std::{env, fmt::format};

use reqwest::header::{HeaderMap, HeaderValue};

// Call Large Language Model (i.e. GTP-4)
pub async fn call_gpt(messages: Vec<Message>) {
    dotenv().ok();

    // Extract API Key information
    let api_key: String =
        env::var("GROQ_AI_KEY").expect("AI_KEY not found in environment variables");
    let content_type: &str = "application/json";
    // Confirm endpoint
    let url: &str = "https://api.groq.com/openai/v1/chat/completions";

    let mut headers: HeaderMap = HeaderMap::new();

    // Create headers
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );

    headers.insert("Content-Type", HeaderValue::from_str(content_type).unwrap());

    // Create DeepSeek AI Client

    let client = Client::builder().default_headers(headers).build().unwrap();

    // Create chat completions

    let chat_completion: ChatCompletion = ChatCompletion {
        model: "deepseek-r1-distill-llama-70b".to_string(),
        messages,
        temperature: 0.1,
    };

    // Troubleshooting
    let res_raw: reqwest::Response = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .unwrap();

    dbg!(res_raw.text().await.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_deepseek() {
        let message: Message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Give me a short response".to_string(),
        };

        let messages: Vec<Message> = vec![message];

        call_gpt(messages).await;
    }
}
