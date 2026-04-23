<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { page } from "$app/stores";
  import SparkLine from "$lib/components/SparkLine.svelte";
  import { prices, watchlist } from "$lib/stores.svelte";
  import { marked } from "marked";
  import DOMPurify from "dompurify";

  let coinId = $derived($page.params.id);
  let coin = $derived(watchlist.find((c) => c.coin_id === coinId));
  let priceInfo = $derived(prices[coinId]);

  let chart = $state<[number, number][]>([]);
  let chartLoading = $state(false);
  let analyzing = $state(false);
  let analysisHtml = $state("");
  let analysisError = $state("");
  let chartDays = $state(30);

  onMount(async () => {
    await loadChart();
  });

  async function loadChart() {
    chartLoading = true;
    try {
      chart = await invoke<[number, number][]>("fetch_chart", {
        coinId,
        days: chartDays,
      });
    } catch (e) {
      console.error(e);
    } finally {
      chartLoading = false;
    }
  }

  async function runAnalysis() {
    analyzing = true;
    analysisError = "";
    analysisHtml = "";
    try {
      const md = await invoke<string>("ai_analyze_coin", { coinId });
      const raw = await marked.parse(md, { breaks: true });
      analysisHtml = DOMPurify.sanitize(raw);
    } catch (e) {
      analysisError = String(e);
    } finally {
      analyzing = false;
    }
  }

  function formatPrice(price: number) {
    if (price >= 1000) return price.toLocaleString("de-DE", { maximumFractionDigits: 0 });
    if (price >= 1) return price.toLocaleString("de-DE", { minimumFractionDigits: 2, maximumFractionDigits: 2 });
    return price.toLocaleString("de-DE", { minimumFractionDigits: 4, maximumFractionDigits: 6 });
  }

  const positive = $derived((priceInfo?.change24h ?? 0) >= 0);
  const chartWidth = $derived(typeof window !== "undefined" ? window.innerWidth - 32 : 350);
</script>

<div class="p-4 pb-6">
  <a href="/" class="text-purple-400 text-sm mb-5 flex items-center gap-1">
    ← Zurück
  </a>

  <!-- Header -->
  <div class="flex items-start justify-between mb-4">
    <div>
      <h1 class="text-2xl font-bold text-slate-200">
        {coin?.name ?? coinId.toUpperCase()}
      </h1>
      <p class="text-slate-500 text-sm uppercase">{coin?.symbol ?? coinId}</p>
    </div>
    {#if priceInfo}
      <div class="text-right">
        <p class="text-2xl font-bold text-slate-200">€{formatPrice(priceInfo.price)}</p>
        <p class="text-sm font-medium {positive ? 'text-green-400' : 'text-red-400'}">
          {positive ? "+" : ""}{priceInfo.change24h.toFixed(2)}%
        </p>
      </div>
    {/if}
  </div>

  <!-- Chart -->
  <div class="bg-[#1e1e2e] rounded-2xl p-4 mb-4">
    <div class="flex gap-2 mb-3">
      {#each [7, 30, 90] as d}
        <button
          onclick={() => { chartDays = d; loadChart(); }}
          class="text-xs px-3 py-1.5 rounded-lg transition-colors min-h-[32px]
            {chartDays === d
              ? 'bg-purple-600 text-white'
              : 'bg-white/5 text-slate-400 hover:bg-white/10'}"
        >{d}T</button>
      {/each}
    </div>

    {#if chartLoading}
      <div class="h-24 flex items-center justify-center">
        <div class="w-5 h-5 border-2 border-purple-500 border-t-transparent rounded-full animate-spin"></div>
      </div>
    {:else if chart.length > 1}
      <SparkLine data={chart} {positive} width={chartWidth} height={96} />
    {:else}
      <p class="text-slate-600 text-sm text-center py-8">Keine Chart-Daten</p>
    {/if}
  </div>

  <!-- Stats -->
  {#if priceInfo}
    <div class="grid grid-cols-2 gap-3 mb-4">
      <div class="bg-[#1e1e2e] rounded-xl p-3">
        <p class="text-xs text-slate-500 mb-1">24h Volumen</p>
        <p class="text-sm font-semibold text-slate-200">
          €{(priceInfo.volume / 1_000_000).toFixed(1)}M
        </p>
      </div>
      <div class="bg-[#1e1e2e] rounded-xl p-3">
        <p class="text-xs text-slate-500 mb-1">24h Änderung</p>
        <p class="text-sm font-semibold {positive ? 'text-green-400' : 'text-red-400'}">
          {positive ? "+" : ""}{priceInfo.change24h.toFixed(2)}%
        </p>
      </div>
    </div>
  {/if}

  <!-- KI-Analyse -->
  <button
    onclick={runAnalysis}
    disabled={analyzing}
    class="w-full bg-purple-600 hover:bg-purple-700 disabled:opacity-60 text-white
           font-semibold py-4 rounded-2xl transition-colors min-h-[54px] mb-4 flex items-center justify-center gap-2"
  >
    {#if analyzing}
      <div class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
      KI analysiert…
    {:else}
      ✦ KI-Analyse starten
    {/if}
  </button>

  {#if analysisError}
    <div class="bg-red-500/10 border border-red-500/20 rounded-xl p-4 mb-4">
      <p class="text-sm text-red-400">{analysisError}</p>
      <p class="text-xs text-slate-500 mt-1">Stelle sicher, dass API-Keys in den Settings eingetragen sind.</p>
    </div>
  {/if}

  {#if analysisHtml}
    <div class="bg-[#1e1e2e] rounded-2xl p-4 prose-custom">
      {@html analysisHtml}
    </div>
  {/if}
</div>

<style>
  :global(.prose-custom h2) {
    color: #a78bfa;
    font-size: 0.9rem;
    font-weight: 700;
    margin: 1rem 0 0.3rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  :global(.prose-custom p) {
    color: #cbd5e1;
    font-size: 0.875rem;
    line-height: 1.6;
    margin-bottom: 0.5rem;
  }
  :global(.prose-custom ul) {
    color: #cbd5e1;
    font-size: 0.875rem;
    padding-left: 1.25rem;
    margin-bottom: 0.5rem;
  }
  :global(.prose-custom li) {
    margin-bottom: 0.2rem;
  }
  :global(.prose-custom strong) {
    color: #e2e8f0;
  }
</style>
