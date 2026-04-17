use anyhow::{Context, Result};
use tauri::AppHandle;
use tracing::info;

use crate::commands::{AlertRule, WatchlistEntry};
use crate::crypto;

static DB: once_cell::sync::OnceCell<sqlx::SqlitePool> = once_cell::sync::OnceCell::new();

pub async fn init(app: &AppHandle) -> Result<()> {
    let app_dir = app
        .path()
        .app_data_dir()
        .context("failed to get app data dir")?;
    std::fs::create_dir_all(&app_dir)?;
    let db_path = app_dir.join("kryptopilot.db");
    let url = format!("sqlite://{}?mode=rwc", db_path.display());

    info!("Opening DB at {}", db_path.display());
    let pool = sqlx::SqlitePool::connect(&url).await?;
    run_migrations(&pool).await?;
    DB.set(pool).ok();
    Ok(())
}

fn pool() -> &'static sqlx::SqlitePool {
    DB.get().expect("DB not initialized")
}

async fn run_migrations(pool: &sqlx::SqlitePool) -> Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS watchlist (
            coin_id TEXT PRIMARY KEY,
            symbol TEXT NOT NULL,
            name TEXT NOT NULL,
            added_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS alert_rules (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            coin_id TEXT NOT NULL,
            metric TEXT NOT NULL,
            operator TEXT NOT NULL,
            value REAL NOT NULL,
            enabled INTEGER NOT NULL DEFAULT 1,
            last_triggered TEXT
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value_encrypted TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS analysis_cache (
            coin_id TEXT PRIMARY KEY,
            analysis TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_watchlist(_app: &AppHandle) -> Result<Vec<WatchlistEntry>> {
    let rows = sqlx::query_as!(
        WatchlistEntry,
        "SELECT coin_id, symbol, name, added_at FROM watchlist ORDER BY added_at DESC"
    )
    .fetch_all(pool())
    .await?;
    Ok(rows)
}

pub async fn add_to_watchlist(
    _app: &AppHandle,
    coin_id: &str,
    symbol: &str,
    name: &str,
) -> Result<()> {
    sqlx::query(
        "INSERT OR IGNORE INTO watchlist (coin_id, symbol, name) VALUES (?, ?, ?)",
    )
    .bind(coin_id)
    .bind(symbol)
    .bind(name)
    .execute(pool())
    .await?;
    Ok(())
}

pub async fn remove_from_watchlist(_app: &AppHandle, coin_id: &str) -> Result<()> {
    sqlx::query("DELETE FROM watchlist WHERE coin_id = ?")
        .bind(coin_id)
        .execute(pool())
        .await?;
    Ok(())
}

pub async fn save_api_key(_app: &AppHandle, provider: &str, key: &str) -> Result<()> {
    let encrypted = crypto::encrypt(key);
    sqlx::query("INSERT OR REPLACE INTO settings (key, value_encrypted) VALUES (?, ?)")
        .bind(format!("api_key_{provider}"))
        .bind(encrypted)
        .execute(pool())
        .await?;
    Ok(())
}

pub async fn get_api_key(_app: &AppHandle, provider: &str) -> Result<Option<String>> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT value_encrypted FROM settings WHERE key = ?",
    )
    .bind(format!("api_key_{provider}"))
    .fetch_optional(pool())
    .await?;

    Ok(row.map(|(enc,)| crypto::decrypt(&enc)))
}

pub async fn cache_analysis(_app: &AppHandle, coin_id: &str, analysis: &str) -> Result<()> {
    sqlx::query(
        "INSERT OR REPLACE INTO analysis_cache (coin_id, analysis, created_at)
         VALUES (?, ?, datetime('now'))",
    )
    .bind(coin_id)
    .bind(analysis)
    .execute(pool())
    .await?;
    Ok(())
}

pub async fn get_alert_rules(_app: &AppHandle) -> Result<Vec<AlertRule>> {
    let rows = sqlx::query!(
        "SELECT id, coin_id, metric, operator, value, enabled FROM alert_rules ORDER BY id"
    )
    .fetch_all(pool())
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| AlertRule {
            id: r.id,
            coin_id: r.coin_id,
            metric: r.metric,
            operator: r.operator,
            value: r.value,
            enabled: r.enabled != 0,
        })
        .collect())
}

pub async fn save_alert_rule(
    _app: &AppHandle,
    coin_id: &str,
    metric: &str,
    operator: &str,
    value: f64,
) -> Result<i64> {
    let result = sqlx::query(
        "INSERT INTO alert_rules (coin_id, metric, operator, value) VALUES (?, ?, ?, ?)",
    )
    .bind(coin_id)
    .bind(metric)
    .bind(operator)
    .bind(value)
    .execute(pool())
    .await?;
    Ok(result.last_insert_rowid())
}

pub async fn delete_alert_rule(_app: &AppHandle, id: i64) -> Result<()> {
    sqlx::query("DELETE FROM alert_rules WHERE id = ?")
        .bind(id)
        .execute(pool())
        .await?;
    Ok(())
}

pub async fn toggle_alert_rule(_app: &AppHandle, id: i64, enabled: bool) -> Result<()> {
    sqlx::query("UPDATE alert_rules SET enabled = ? WHERE id = ?")
        .bind(enabled as i64)
        .bind(id)
        .execute(pool())
        .await?;
    Ok(())
}

pub async fn get_enabled_rules() -> Result<Vec<AlertRule>> {
    let rows = sqlx::query!(
        "SELECT id, coin_id, metric, operator, value, enabled FROM alert_rules WHERE enabled = 1"
    )
    .fetch_all(pool())
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| AlertRule {
            id: r.id,
            coin_id: r.coin_id,
            metric: r.metric,
            operator: r.operator,
            value: r.value,
            enabled: r.enabled != 0,
        })
        .collect())
}

pub async fn update_rule_triggered(id: i64) -> Result<()> {
    sqlx::query("UPDATE alert_rules SET last_triggered = datetime('now') WHERE id = ?")
        .bind(id)
        .execute(pool())
        .await?;
    Ok(())
}

pub async fn rule_triggered_recently(id: i64, hours: i64) -> Result<bool> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM alert_rules WHERE id = ? AND last_triggered > datetime('now', ?)",
    )
    .bind(id)
    .bind(format!("-{hours} hours"))
    .fetch_optional(pool())
    .await?;

    Ok(row.map(|(c,)| c > 0).unwrap_or(false))
}
