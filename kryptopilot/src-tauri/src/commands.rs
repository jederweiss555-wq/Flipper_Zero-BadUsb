use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{coingecko, cryptopanic, db};

#[derive(Serialize, Deserialize, Clone)]
pub struct WatchlistEntry {
    pub coin_id: String,
    pub symbol: String,
    pub name: String,
    pub added_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PriceInfo {
    pub price: f64,
    pub change24h: f64,
    pub volume: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewsItem {
    pub title: String,
    pub url: String,
    pub source: String,
    pub sentiment: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AlertRule {
    pub id: i64,
    pub coin_id: String,
    pub metric: String,
    pub operator: String,
    pub value: f64,
    pub enabled: bool,
}

#[tauri::command]
pub async fn get_watchlist(
    app: tauri::AppHandle,
) -> Result<Vec<WatchlistEntry>, String> {
    db::get_watchlist(&app).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_to_watchlist(
    app: tauri::AppHandle,
    coin_id: String,
    symbol: String,
    name: String,
) -> Result<(), String> {
    db::add_to_watchlist(&app, &coin_id, &symbol, &name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_from_watchlist(
    app: tauri::AppHandle,
    coin_id: String,
) -> Result<(), String> {
    db::remove_from_watchlist(&app, &coin_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_prices(
    ids: Vec<String>,
) -> Result<std::collections::HashMap<String, PriceInfo>, String> {
    coingecko::fetch_prices(&ids).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_chart(
    coin_id: String,
    days: u32,
) -> Result<Vec<(i64, f64)>, String> {
    coingecko::fetch_chart(&coin_id, days)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_news(
    app: tauri::AppHandle,
    symbols: Vec<String>,
) -> Result<Vec<NewsItem>, String> {
    let token = db::get_api_key(&app, "cryptopanic")
        .await
        .map_err(|e| e.to_string())?;
    cryptopanic::fetch_news(&symbols, token.as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ai_analyze_coin(
    app: tauri::AppHandle,
    coin_id: String,
) -> Result<String, String> {
    use crate::ai::router::LlmRouter;

    let prices = coingecko::fetch_prices(&[coin_id.clone()])
        .await
        .map_err(|e| e.to_string())?;
    let chart = coingecko::fetch_chart(&coin_id, 30)
        .await
        .map_err(|e| e.to_string())?;

    let price_info = prices.get(&coin_id).cloned().unwrap_or(PriceInfo {
        price: 0.0,
        change24h: 0.0,
        volume: 0.0,
    });

    let chart_summary = if chart.len() > 1 {
        let first = chart.first().map(|p| p.1).unwrap_or(0.0);
        let last = chart.last().map(|p| p.1).unwrap_or(0.0);
        format!(
            "30-Tage-Chart: Start €{:.2}, Ende €{:.2}, {} Datenpunkte",
            first,
            last,
            chart.len()
        )
    } else {
        "Keine Chart-Daten verfügbar".to_string()
    };

    let user_prompt = format!(
        "Analysiere {coin_id}:\n\
        - Aktueller Preis: €{:.2}\n\
        - 24h-Änderung: {:.2}%\n\
        - 24h-Volumen: €{:.0}\n\
        - {chart_summary}",
        price_info.price, price_info.change24h, price_info.volume
    );

    let gemini_key = db::get_api_key(&app, "gemini").await.map_err(|e| e.to_string())?;
    let groq_key = db::get_api_key(&app, "groq").await.map_err(|e| e.to_string())?;
    let router = LlmRouter::new(gemini_key, groq_key);

    let result = router.complete(&user_prompt).await.map_err(|e| e.to_string())?;

    db::cache_analysis(&app, &coin_id, &result)
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
pub async fn save_api_key(
    app: tauri::AppHandle,
    provider: String,
    key: String,
) -> Result<(), String> {
    db::save_api_key(&app, &provider, &key)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_alert_rules(
    app: tauri::AppHandle,
) -> Result<Vec<AlertRule>, String> {
    db::get_alert_rules(&app).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_alert_rule(
    app: tauri::AppHandle,
    coin_id: String,
    metric: String,
    operator: String,
    value: f64,
) -> Result<i64, String> {
    db::save_alert_rule(&app, &coin_id, &metric, &operator, value)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_alert_rule(
    app: tauri::AppHandle,
    id: i64,
) -> Result<(), String> {
    db::delete_alert_rule(&app, id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_alert_rule(
    app: tauri::AppHandle,
    id: i64,
    enabled: bool,
) -> Result<(), String> {
    db::toggle_alert_rule(&app, id, enabled)
        .await
        .map_err(|e| e.to_string())
}
