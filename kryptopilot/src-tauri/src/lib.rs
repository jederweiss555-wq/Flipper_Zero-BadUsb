mod alerts;
mod coingecko;
mod commands;
mod crypto;
mod cryptopanic;
mod db;
mod scheduler;
pub mod ai;

use tauri::Manager;
use tracing::info;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            info!("KryptoPilot starting");
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = db::init(&handle).await {
                    tracing::error!("DB init failed: {e:#}");
                    return;
                }
                scheduler::start(handle).await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_watchlist,
            commands::add_to_watchlist,
            commands::remove_from_watchlist,
            commands::fetch_prices,
            commands::fetch_chart,
            commands::fetch_news,
            commands::ai_analyze_coin,
            commands::save_api_key,
            commands::get_api_key_exists,
            commands::get_alert_rules,
            commands::save_alert_rule,
            commands::delete_alert_rule,
            commands::toggle_alert_rule,
            commands::search_coins,
        ])
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}
