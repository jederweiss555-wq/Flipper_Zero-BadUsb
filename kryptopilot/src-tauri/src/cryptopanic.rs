use anyhow::{Context, Result};
use serde::Deserialize;
use tracing::info;

use crate::commands::NewsItem;

const BASE_URL: &str = "https://cryptopanic.com/api/v1/posts/";

#[derive(Deserialize)]
struct PanicResponse {
    results: Vec<PanicPost>,
}

#[derive(Deserialize)]
struct PanicPost {
    title: String,
    url: String,
    source: PanicSource,
    votes: Option<PanicVotes>,
}

#[derive(Deserialize)]
struct PanicSource {
    title: String,
}

#[derive(Deserialize)]
struct PanicVotes {
    positive: Option<i64>,
    negative: Option<i64>,
}

pub async fn fetch_news(symbols: &[String], auth_token: Option<&str>) -> Result<Vec<NewsItem>> {
    let token = match auth_token {
        Some(t) if !t.is_empty() => t,
        _ => return Ok(vec![]),
    };

    let currencies = symbols.join(",");
    let url = format!("{BASE_URL}?auth_token={token}&currencies={currencies}&public=true");
    info!("Fetching news for {currencies}");

    let client = reqwest::Client::new();
    let resp: PanicResponse = client
        .get(&url)
        .header("Accept", "application/json")
        .send()
        .await
        .context("CryptoPanic request failed")?
        .json()
        .await
        .context("CryptoPanic JSON parse failed")?;

    let items = resp
        .results
        .into_iter()
        .take(10)
        .map(|p| {
            let sentiment = p.votes.and_then(|v| {
                let pos = v.positive.unwrap_or(0);
                let neg = v.negative.unwrap_or(0);
                if pos > neg + 2 {
                    Some("bullish".to_string())
                } else if neg > pos + 2 {
                    Some("bearish".to_string())
                } else {
                    None
                }
            });
            NewsItem {
                title: p.title,
                url: p.url,
                source: p.source.title,
                sentiment,
            }
        })
        .collect();

    Ok(items)
}
