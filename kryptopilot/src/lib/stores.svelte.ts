export interface CoinMarket {
  id: string;
  symbol: string;
  name: string;
  current_price: number;
  price_change_percentage_24h: number;
  market_cap: number;
  total_volume: number;
  image: string;
}

export interface AlertRule {
  id: number;
  coin_id: string;
  metric: "price" | "rsi" | "volume_24h" | "change_24h";
  operator: "<" | ">" | "=";
  value: number;
  enabled: boolean;
}

export interface WatchlistEntry {
  coin_id: string;
  symbol: string;
  name: string;
  added_at: string;
}

export const watchlist = $state<WatchlistEntry[]>([]);
export const prices = $state<Record<string, CoinMarket>>({});
export const alertRules = $state<AlertRule[]>([]);
export const isLoading = $state({ prices: false, analysis: false });
