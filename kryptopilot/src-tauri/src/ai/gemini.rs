use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use super::LlmProvider;

const ENDPOINT: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent";

#[derive(Serialize)]
struct GeminiRequest {
    system_instruction: GeminiContent,
    contents: Vec<GeminiContent>,
}

#[derive(Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
struct GeminiPart {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
    error: Option<GeminiError>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: Option<GeminiResponseContent>,
}

#[derive(Deserialize)]
struct GeminiResponseContent {
    parts: Vec<GeminiResponsePart>,
}

#[derive(Deserialize)]
struct GeminiResponsePart {
    text: String,
}

#[derive(Deserialize)]
struct GeminiError {
    message: String,
}

pub struct GeminiProvider {
    api_key: String,
}

impl GeminiProvider {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait::async_trait]
impl LlmProvider for GeminiProvider {
    async fn complete(&self, system: &str, user: &str) -> Result<String> {
        info!("Calling Gemini 2.5 Flash");
        let url = format!("{ENDPOINT}?key={}", self.api_key);
        let body = GeminiRequest {
            system_instruction: GeminiContent {
                parts: vec![GeminiPart { text: system.to_string() }],
            },
            contents: vec![GeminiContent {
                parts: vec![GeminiPart { text: user.to_string() }],
            }],
        };

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("Gemini request failed")?;

        let status = resp.status();
        if status == 429 || status.is_server_error() {
            bail!("Gemini HTTP {status}");
        }

        let parsed: GeminiResponse = resp.json().await.context("Gemini JSON parse failed")?;

        if let Some(err) = parsed.error {
            bail!("Gemini error: {}", err.message);
        }

        parsed
            .candidates
            .and_then(|c| c.into_iter().next())
            .and_then(|c| c.content)
            .and_then(|c| c.parts.into_iter().next())
            .map(|p| p.text)
            .context("Gemini returned empty response")
    }
}
