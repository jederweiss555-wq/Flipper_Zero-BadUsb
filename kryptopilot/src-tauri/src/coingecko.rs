use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use tracing::info;

use crate::commands::{CoinSearchResult, PriceInfo};

const BASE_URL: &str = "https://api.coingecko.com/api/v3";

#[derive(Deserialize)]
struct CoinMarketResponse {
    id: String,
    current_price: Option<f64>,
    price_change_percentage_24h: Option<f64>,
    total_volume: Option<f64>,
}

#[derive(Deserialize)]
struct ChartResponse {
    prices: Vec<(i64, f64)>,
}

#[derive(Deserialize)]
struct SearchResponse {
    coins: Vec<SearchCoin>,
}

#[derive(Deserialize)]
struct SearchCoin {
    id: String,
    symbol: String,
    name: String,
}

pub async fn fetch_prices(ids: &[impl AsRef<str>]) -> Result<HashMap<String, PriceInfo>> {
    if ids.is_empty() {
        return Ok(HashMap::new());
    }
    let ids_str = ids.iter().map(|s| s.as_ref()).collect::<Vec<_>>().join(",");
    let url = format!(
        "{BASE_URL}/coins/markets?vs_currency=eur&ids={ids_str}\
        &order=market_cap_desc&per_page=50&page=1&sparkline=false"
    );
    info!("Fetching prices for: {ids_str}");

    let client = reqwest::Client::new();
    let resp: Vec<CoinMarketResponse> = client
        .get(&url)
        .header("Accept", "application/json")
        .send()
        .await
        .context("CoinGecko markets request failed")?
        .json()
        .await
        .context("CoinGecko markets JSON parse failed")?;

    Ok(resp
        .into_iter()
        .map(|c| {
            (
                c.id,
                PriceInfo {
                    price: c.current_price.unwrap_or(0.0),
                    change24h: c.price_change_percentage_24h.unwrap_or(0.0),
                    volume: c.total_volume.unwrap_or(0.0),
                },
            )
        })
        .collect())
}

pub async fn fetch_chart(coin_id: &str, days: u32) -> Result<Vec<(i64, f64)>> {
    let url = format!(
        "{BASE_URL}/coins/{coin_id}/market_chart?vs_currency=eur&days={days}"
    );
    info!("Fetching chart for {coin_id}");

    let client = reqwest::Client::new();
    let resp: ChartResponse = client
        .get(&url)
        .header("Accept", "application/json")
        .send()
        .await
        .context("CoinGecko chart request failed")?
        .json()
        .await
        .context("CoinGecko chart JSON parse failed")?;

    Ok(resp.prices)
}

pub async fn search_coins(query: &str) -> Result<Vec<CoinSearchResult>> {
    if query.trim().is_empty() {
        return Ok(vec![]);
    }
    let url = format!("{BASE_URL}/search?query={}", urlencoding::encode(query));
    info!("Searching coins: {query}");

    let client = reqwest::Client::new();
    let resp: SearchResponse = client
        .get(&url)
        .header("Accept", "application/json")
        .send()
        .await
        .context("CoinGecko search request failed")?
        .json()
        .await
        .context("CoinGecko search JSON parse failed")?;

    Ok(resp
        .coins
        .into_iter()
        .take(10)
        .map(|c| CoinSearchResult {
            id: c.id,
            symbol: c.symbol.to_uppercase(),
            name: c.name,
        })
        .collect())
}
