use tauri::AppHandle;
use tokio::time::{interval, Duration};
use tracing::{error, info, warn};

use crate::alerts;

pub async fn start(app: AppHandle) {
    info!("Scheduler starting");
    let app_clone = app.clone();
    tokio::spawn(alert_loop(app_clone));
    tokio::spawn(briefing_loop(app));
}

async fn alert_loop(app: AppHandle) {
    let mut ticker = interval(Duration::from_secs(15 * 60));
    loop {
        ticker.tick().await;
        info!("Checking alert rules");
        match alerts::evaluate_rules().await {
            Ok(triggered) => {
                for (rule, value) in triggered {
                    send_notification(
                        &app,
                        &format!("KryptoPilot Alert: {}", rule.coin_id.to_uppercase()),
                        &format!(
                            "{} {} {} {} (aktuell: {:.2})",
                            rule.coin_id, rule.metric, rule.operator, rule.value, value
                        ),
                    );
                }
            }
            Err(e) => warn!("Alert evaluation failed: {e:#}"),
        }
    }
}

async fn briefing_loop(app: AppHandle) {
    let mut ticker = interval(Duration::from_secs(60));
    let mut last_briefing_day: Option<u32> = None;

    loop {
        ticker.tick().await;
        let now = chrono::Local::now();

        if now.format("%H:%M").to_string() == "08:00" {
            let today = now.day();
            if last_briefing_day != Some(today) {
                last_briefing_day = Some(today);
                info!("Sending morning briefing");
                send_notification(
                    &app,
                    "KryptoPilot Tages-Briefing",
                    "Dein Krypto-Marktüberblick für heute ist bereit. Öffne die App für die KI-Analyse.",
                );
            }
        }
    }
}

fn send_notification(app: &AppHandle, title: &str, body: &str) {
    use tauri_plugin_notification::NotificationExt;
    if let Err(e) = app
        .notification()
        .builder()
        .title(title)
        .body(body)
        .show()
    {
        warn!("Notification failed: {e:#}");
    }
}
