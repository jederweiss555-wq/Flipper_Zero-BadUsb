<script lang="ts">
  // Dashboard / Watchlist – implemented in Phase 4
  let loading = $state(false);
  let watchlist = $state<Array<{ id: string; symbol: string; name: string; price: number; change24h: number }>>([]);
</script>

<div class="p-4">
  <div class="flex items-center justify-between mb-4">
    <h1 class="text-xl font-semibold text-slate-200">Watchlist</h1>
    <button
      class="bg-purple-600 hover:bg-purple-700 text-white text-sm font-medium px-4 py-2 rounded-xl transition-colors min-h-[44px]"
    >
      + Coin
    </button>
  </div>

  {#if watchlist.length === 0}
    <div class="flex flex-col items-center justify-center py-20 text-slate-500 gap-3">
      <span class="text-4xl">◈</span>
      <p class="text-sm">Noch keine Coins. Füge deinen ersten hinzu.</p>
    </div>
  {/if}

  <div class="grid gap-3">
    {#each watchlist as coin}
      <a
        href="/coin/{coin.id}"
        class="bg-[#1e1e2e] rounded-2xl p-4 flex items-center justify-between active:scale-[0.98] transition-transform"
      >
        <div>
          <p class="font-semibold text-slate-200">{coin.name}</p>
          <p class="text-xs text-slate-500 uppercase">{coin.symbol}</p>
        </div>
        <div class="text-right">
          <p class="font-semibold text-slate-200">€{coin.price.toLocaleString("de-DE")}</p>
          <p class="text-xs {coin.change24h >= 0 ? 'text-green-400' : 'text-red-400'}">
            {coin.change24h >= 0 ? "+" : ""}{coin.change24h.toFixed(2)}%
          </p>
        </div>
      </a>
    {/each}
  </div>
</div>
