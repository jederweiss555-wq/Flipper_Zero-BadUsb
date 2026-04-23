use chrono::Timelike;
use tauri::AppHandle;
use tokio::time::{interval, Duration};
use tracing::{info, warn};

use crate::alerts;

pub async fn start(app: AppHandle) {
    info!("Scheduler starting");
    let app_alert = app.clone();
    let app_brief = app;
    tokio::spawn(alert_loop(app_alert));
    tokio::spawn(briefing_loop(app_brief));
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
                        &format!("Alert: {}", rule.coin_id.to_uppercase()),
                        &format!(
                            "{} {} {} {:.2} (aktuell: {:.2})",
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
        let hour = now.hour();
        let minute = now.minute();

        if hour == 8 && minute == 0 {
            let today = now.day0();
            if last_briefing_day != Some(today) {
                last_briefing_day = Some(today);
                info!("Sending morning briefing");
                send_notification(
                    &app,
                    "KryptoPilot Tages-Briefing",
                    "Dein Krypto-Marktüberblick ist bereit. Öffne die App.",
                );
            }
        }
    }
}

fn send_notification(app: &AppHandle, title: &str, body: &str) {
    use tauri_plugin_notification::NotificationExt;
    if let Err(e) = app.notification().builder().title(title).body(body).show() {
        warn!("Notification failed: {e:#}");
    }
}
