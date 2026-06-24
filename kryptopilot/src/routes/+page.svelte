<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import CoinCard from "$lib/components/CoinCard.svelte";
  import Modal from "$lib/components/Modal.svelte";
  import {
    watchlist,
    prices,
    loading,
    loadWatchlist,
    addCoin,
    removeCoin,
    type CoinSearchResult,
  } from "$lib/stores.svelte";

  let showAddModal = $state(false);
  let searchQuery = $state("");
  let searchResults = $state<CoinSearchResult[]>([]);
  let searchLoading = $state(false);
  let searchError = $state("");
  let error = $state("");

  let debounceTimer: ReturnType<typeof setTimeout>;

  onMount(async () => {
    try {
      await loadWatchlist();
    } catch (e) {
      error = String(e);
    }

    // Refresh prices every 5 minutes
    const interval = setInterval(async () => {
      if (watchlist.length > 0) {
        const ids = watchlist.map((c) => c.coin_id);
        try {
          const result = await invoke<Record<string, { price: number; change24h: number; volume: number }>>(
            "fetch_prices",
            { ids }
          );
          Object.assign(prices, result);
        } catch {}
      }
    }, 5 * 60 * 1000);

    return () => clearInterval(interval);
  });

  function onSearchInput() {
    clearTimeout(debounceTimer);
    searchError = "";
    if (!searchQuery.trim()) {
      searchResults = [];
      return;
    }
    debounceTimer = setTimeout(async () => {
      searchLoading = true;
      try {
        searchResults = await invoke<CoinSearchResult[]>("search_coins", {
          query: searchQuery,
        });
      } catch (e) {
        searchError = String(e);
      } finally {
        searchLoading = false;
      }
    }, 400);
  }

  async function handleAdd(coin: CoinSearchResult) {
    try {
      await addCoin(coin.id, coin.symbol, coin.name);
      showAddModal = false;
      searchQuery = "";
      searchResults = [];
    } catch (e) {
      searchError = String(e);
    }
  }

  async function handleRemove(coinId: string) {
    try {
      await removeCoin(coinId);
    } catch (e) {
      error = String(e);
    }
  }
</script>

<div class="p-4">
  <div class="flex items-center justify-between mb-4">
    <h1 class="text-xl font-semibold text-slate-200">Watchlist</h1>
    <button
      onclick={() => (showAddModal = true)}
      class="bg-purple-600 hover:bg-purple-700 text-white text-sm font-medium px-4 py-2 rounded-xl transition-colors min-h-[44px]"
    >
      + Coin
    </button>
  </div>

  {#if error}
    <div class="bg-red-500/10 border border-red-500/20 rounded-xl p-3 mb-4 text-sm text-red-400">
      {error}
    </div>
  {/if}

  {#if loading.watchlist}
    <div class="space-y-3">
      {#each [1, 2, 3] as _}
        <div class="bg-[#1e1e2e] rounded-2xl p-4 h-[72px] animate-pulse"></div>
      {/each}
    </div>
  {:else if watchlist.length === 0}
    <div class="flex flex-col items-center justify-center py-20 text-slate-500 gap-3">
      <span class="text-5xl">◈</span>
      <p class="text-sm text-center">
        Keine Coins in der Watchlist.<br />Füge deinen ersten Coin hinzu.
      </p>
    </div>
  {:else}
    <div class="space-y-3">
      {#each watchlist as coin (coin.coin_id)}
        <CoinCard
          {coin}
          priceInfo={prices[coin.coin_id]}
          onRemove={handleRemove}
        />
      {/each}
    </div>

    {#if loading.prices}
      <p class="text-xs text-slate-600 text-center mt-4">Preise werden aktualisiert…</p>
    {/if}
  {/if}
</div>

<Modal
  open={showAddModal}
  title="Coin hinzufügen"
  onclose={() => {
    showAddModal = false;
    searchQuery = "";
    searchResults = [];
    searchError = "";
  }}
>
  <div class="space-y-4">
    <input
      type="text"
      bind:value={searchQuery}
      oninput={onSearchInput}
      placeholder="Bitcoin, Ethereum, Solana…"
      autofocus
      class="w-full bg-[#0f0f14] border border-white/10 rounded-xl px-4 py-3 text-sm text-slate-200
             placeholder-slate-600 focus:outline-none focus:border-purple-500 min-h-[44px]"
    />

    {#if searchError}
      <p class="text-xs text-red-400">{searchError}</p>
    {/if}

    {#if searchLoading}
      <div class="space-y-2">
        {#each [1, 2, 3] as _}
          <div class="h-12 bg-white/5 rounded-xl animate-pulse"></div>
        {/each}
      </div>
    {:else if searchResults.length > 0}
      <div class="space-y-2 max-h-64 overflow-y-auto">
        {#each searchResults as coin}
          <button
            onclick={() => handleAdd(coin)}
            class="w-full flex items-center gap-3 p-3 rounded-xl hover:bg-white/5 transition-colors text-left min-h-[48px]"
          >
            <div class="w-8 h-8 rounded-full bg-purple-600/20 flex items-center justify-center shrink-0">
              <span class="text-xs font-bold text-purple-300">{coin.symbol.slice(0, 3)}</span>
            </div>
            <div>
              <p class="text-sm font-medium text-slate-200">{coin.name}</p>
              <p class="text-xs text-slate-500">{coin.symbol}</p>
            </div>
          </button>
        {/each}
      </div>
    {:else if searchQuery.trim() && !searchLoading}
      <p class="text-sm text-slate-500 text-center py-4">Keine Ergebnisse für "{searchQuery}"</p>
    {/if}
  </div>
</Modal>
