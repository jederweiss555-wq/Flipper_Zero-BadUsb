mod commands;
mod db;
mod coingecko;
mod cryptopanic;
mod alerts;
mod scheduler;
mod crypto;
pub mod ai;

use tauri::Manager;
use tracing::info;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            info!("KryptoPilot starting up");
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = db::init(&app_handle).await {
                    tracing::error!("DB init failed: {e:#}");
                }
                scheduler::start(app_handle).await;
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
            commands::get_alert_rules,
            commands::save_alert_rule,
            commands::delete_alert_rule,
            commands::toggle_alert_rule,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
