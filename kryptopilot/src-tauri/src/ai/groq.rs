use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use tracing::info;

use super::LlmProvider;

const ENDPOINT: &str = "https://api.groq.com/openai/v1/chat/completions";
const MODEL: &str = "llama-3.3-70b-versatile";

#[derive(Serialize)]
struct GroqRequest {
    model: String,
    messages: Vec<GroqMessage>,
    max_tokens: u32,
}

#[derive(Serialize)]
struct GroqMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct GroqResponse {
    choices: Option<Vec<GroqChoice>>,
    error: Option<GroqError>,
}

#[derive(Deserialize)]
struct GroqChoice {
    message: GroqResponseMessage,
}

#[derive(Deserialize)]
struct GroqResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct GroqError {
    message: String,
}

pub struct GroqProvider {
    api_key: String,
}

impl GroqProvider {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait::async_trait]
impl LlmProvider for GroqProvider {
    async fn complete(&self, system: &str, user: &str) -> Result<String> {
        info!("Calling Groq Llama 3.3 70B");
        let body = GroqRequest {
            model: MODEL.to_string(),
            messages: vec![
                GroqMessage { role: "system".to_string(), content: system.to_string() },
                GroqMessage { role: "user".to_string(), content: user.to_string() },
            ],
            max_tokens: 2048,
        };

        let client = reqwest::Client::new();
        let resp = client
            .post(ENDPOINT)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .context("Groq request failed")?;

        let status = resp.status();
        if status == 429 || status.is_server_error() {
            bail!("Groq HTTP {status}");
        }

        let parsed: GroqResponse = resp.json().await.context("Groq JSON parse failed")?;

        if let Some(err) = parsed.error {
            bail!("Groq error: {}", err.message);
        }

        parsed
            .choices
            .and_then(|c| c.into_iter().next())
            .map(|c| c.message.content)
            .context("Groq returned empty response")
    }
}
