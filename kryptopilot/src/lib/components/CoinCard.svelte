<script lang="ts">
  import type { PriceInfo, WatchlistEntry } from "$lib/stores.svelte";
  import SparkLine from "./SparkLine.svelte";

  interface Props {
    coin: WatchlistEntry;
    priceInfo?: PriceInfo;
    onRemove: (coinId: string) => void;
  }
  let { coin, priceInfo, onRemove }: Props = $props();

  const positive = $derived((priceInfo?.change24h ?? 0) >= 0);

  function formatPrice(price: number) {
    if (price >= 1000) return price.toLocaleString("de-DE", { maximumFractionDigits: 0 });
    if (price >= 1) return price.toLocaleString("de-DE", { minimumFractionDigits: 2, maximumFractionDigits: 2 });
    return price.toLocaleString("de-DE", { minimumFractionDigits: 4, maximumFractionDigits: 6 });
  }
</script>

<a
  href="/coin/{coin.coin_id}"
  class="bg-[#1e1e2e] rounded-2xl p-4 flex items-center gap-3 active:scale-[0.98] transition-transform relative"
>
  <div class="w-10 h-10 rounded-full bg-purple-600/20 flex items-center justify-center shrink-0">
    <span class="text-xs font-bold text-purple-300">{coin.symbol.slice(0, 3)}</span>
  </div>

  <div class="flex-1 min-w-0">
    <p class="font-semibold text-slate-200 truncate">{coin.name}</p>
    <p class="text-xs text-slate-500 uppercase">{coin.symbol}</p>
  </div>

  <div class="flex flex-col items-end gap-1">
    {#if priceInfo}
      <p class="font-semibold text-slate-200 text-sm">€{formatPrice(priceInfo.price)}</p>
      <p class="text-xs font-medium {positive ? 'text-green-400' : 'text-red-400'}">
        {positive ? "+" : ""}{priceInfo.change24h.toFixed(2)}%
      </p>
    {:else}
      <div class="w-16 h-4 bg-white/10 rounded animate-pulse"></div>
      <div class="w-10 h-3 bg-white/10 rounded animate-pulse mt-1"></div>
    {/if}
  </div>

  <button
    onclick={(e) => { e.preventDefault(); onRemove(coin.coin_id); }}
    class="absolute top-2 right-2 text-slate-700 hover:text-red-400 transition-colors p-1 text-xs"
    aria-label="Entfernen"
  >✕</button>
</a>
