use anyhow::{bail, Result};
use tracing::{info, warn};

use super::{gemini::GeminiProvider, groq::GroqProvider, LlmProvider, SYSTEM_PROMPT};

pub struct LlmRouter {
    gemini: Option<GeminiProvider>,
    groq: Option<GroqProvider>,
}

impl LlmRouter {
    pub fn new(gemini_key: Option<String>, groq_key: Option<String>) -> Self {
        Self {
            gemini: gemini_key.filter(|k| !k.is_empty()).map(GeminiProvider::new),
            groq: groq_key.filter(|k| !k.is_empty()).map(GroqProvider::new),
        }
    }

    pub async fn complete(&self, user: &str) -> Result<String> {
        if let Some(gemini) = &self.gemini {
            info!("Trying Gemini first");
            match gemini.complete(SYSTEM_PROMPT, user).await {
                Ok(result) => return Ok(result),
                Err(e) => warn!("Gemini failed: {e:#}, trying Groq fallback"),
            }
        }

        if let Some(groq) = &self.groq {
            info!("Trying Groq fallback");
            return groq.complete(SYSTEM_PROMPT, user).await;
        }

        bail!("Keine KI-Provider konfiguriert. Bitte API-Keys in Settings eingeben.")
    }
}
