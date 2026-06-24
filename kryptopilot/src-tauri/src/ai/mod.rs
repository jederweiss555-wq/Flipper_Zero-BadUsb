pub mod gemini;
pub mod groq;
pub mod router;

use anyhow::Result;

pub const SYSTEM_PROMPT: &str = "Du bist ein sachlicher Krypto-Analyst. Antworte strukturiert \
in Markdown mit Sektionen: Marktlage, Technische Indikatoren, Unterstützung/Widerstand, \
News-Sentiment, Fazit. Keine Finanzberatung, keine Kauf-Empfehlungen.";

#[async_trait::async_trait]
pub trait LlmProvider: Send + Sync {
    async fn complete(&self, system: &str, user: &str) -> Result<String>;
}
