import { invoke } from "@tauri-apps/api/core";

export interface WatchlistEntry {
  coin_id: string;
  symbol: string;
  name: string;
  added_at: string;
}

export interface PriceInfo {
  price: number;
  change24h: number;
  volume: number;
}

export interface AlertRule {
  id: number;
  coin_id: string;
  metric: "price" | "change_24h" | "volume_24h";
  operator: "<" | ">" | "=";
  value: number;
  enabled: boolean;
}

export interface CoinSearchResult {
  id: string;
  symbol: string;
  name: string;
}

export const watchlist = $state<WatchlistEntry[]>([]);
export const prices = $state<Record<string, PriceInfo>>({});
export const alertRules = $state<AlertRule[]>([]);
export const loading = $state({ watchlist: false, prices: false, analysis: false, rules: false });

export async function loadWatchlist() {
  loading.watchlist = true;
  try {
    const entries = await invoke<WatchlistEntry[]>("get_watchlist");
    watchlist.splice(0, watchlist.length, ...entries);
    if (entries.length > 0) {
      await refreshPrices();
    }
  } finally {
    loading.watchlist = false;
  }
}

export async function refreshPrices() {
  if (watchlist.length === 0) return;
  loading.prices = true;
  try {
    const ids = watchlist.map((c) => c.coin_id);
    const result = await invoke<Record<string, PriceInfo>>("fetch_prices", { ids });
    Object.assign(prices, result);
  } finally {
    loading.prices = false;
  }
}

export async function addCoin(coinId: string, symbol: string, name: string) {
  await invoke("add_to_watchlist", { coinId, symbol, name });
  await loadWatchlist();
}

export async function removeCoin(coinId: string) {
  await invoke("remove_from_watchlist", { coinId });
  const idx = watchlist.findIndex((c) => c.coin_id === coinId);
  if (idx !== -1) watchlist.splice(idx, 1);
  delete prices[coinId];
}

export async function loadAlertRules() {
  loading.rules = true;
  try {
    const rules = await invoke<AlertRule[]>("get_alert_rules");
    alertRules.splice(0, alertRules.length, ...rules);
  } finally {
    loading.rules = false;
  }
}

export async function saveRule(
  coinId: string,
  metric: string,
  operator: string,
  value: number
) {
  await invoke("save_alert_rule", { coinId, metric, operator, value });
  await loadAlertRules();
}

export async function deleteRule(id: number) {
  await invoke("delete_alert_rule", { id });
  const idx = alertRules.findIndex((r) => r.id === id);
  if (idx !== -1) alertRules.splice(idx, 1);
}

export async function toggleRule(id: number, enabled: boolean) {
  await invoke("toggle_alert_rule", { id, enabled });
  const rule = alertRules.find((r) => r.id === id);
  if (rule) rule.enabled = enabled;
}
