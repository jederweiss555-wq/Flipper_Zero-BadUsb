use anyhow::Result;
use tracing::{info, warn};

use crate::{coingecko, commands::AlertRule, db};

pub async fn evaluate_rules() -> Result<Vec<(AlertRule, f64)>> {
    let rules = db::get_enabled_rules().await?;
    if rules.is_empty() {
        return Ok(vec![]);
    }

    let coin_ids: Vec<String> = rules.iter().map(|r| r.coin_id.clone()).collect();
    let mut unique_ids = coin_ids.clone();
    unique_ids.dedup();

    let prices = coingecko::fetch_prices(&unique_ids).await?;
    let mut triggered = vec![];

    for rule in rules {
        let recently = db::rule_triggered_recently(rule.id, 2).await.unwrap_or(false);
        if recently {
            continue;
        }

        let price_info = match prices.get(&rule.coin_id) {
            Some(p) => p,
            None => continue,
        };

        let current_value = match rule.metric.as_str() {
            "price" => price_info.price,
            "change_24h" => price_info.change24h,
            "volume_24h" => price_info.volume,
            other => {
                warn!("Unknown metric: {other}");
                continue;
            }
        };

        let matched = match rule.operator.as_str() {
            "<" => current_value < rule.value,
            ">" => current_value > rule.value,
            "=" => (current_value - rule.value).abs() < rule.value * 0.01,
            _ => false,
        };

        if matched {
            info!(
                "Alert triggered: {} {} {} {} (current: {})",
                rule.coin_id, rule.metric, rule.operator, rule.value, current_value
            );
            if let Err(e) = db::update_rule_triggered(rule.id).await {
                warn!("Failed to update rule triggered time: {e:#}");
            }
            triggered.push((rule, current_value));
        }
    }

    Ok(triggered)
}
