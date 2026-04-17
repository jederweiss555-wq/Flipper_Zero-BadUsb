use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use tracing::info;

use crate::commands::PriceInfo;

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

pub async fn fetch_prices(ids: &[String]) -> Result<HashMap<String, PriceInfo>> {
    if ids.is_empty() {
        return Ok(HashMap::new());
    }
    let ids_str = ids.join(",");
    let url = format!(
        "{BASE_URL}/coins/markets?vs_currency=eur&ids={ids_str}&order=market_cap_desc&per_page=50&page=1"
    );
    info!("Fetching prices for: {ids_str}");

    let client = reqwest::Client::new();
    let resp: Vec<CoinMarketResponse> = client
        .get(&url)
        .header("Accept", "application/json")
        .send()
        .await
        .context("CoinGecko request failed")?
        .json()
        .await
        .context("CoinGecko JSON parse failed")?;

    let mut map = HashMap::new();
    for coin in resp {
        map.insert(
            coin.id,
            PriceInfo {
                price: coin.current_price.unwrap_or(0.0),
                change24h: coin.price_change_percentage_24h.unwrap_or(0.0),
                volume: coin.total_volume.unwrap_or(0.0),
            },
        );
    }
    Ok(map)
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
