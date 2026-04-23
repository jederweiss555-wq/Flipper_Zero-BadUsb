<script lang="ts">
  import { onMount } from "svelte";
  import Modal from "$lib/components/Modal.svelte";
  import {
    alertRules,
    watchlist,
    loading,
    loadAlertRules,
    saveRule,
    deleteRule,
    toggleRule,
  } from "$lib/stores.svelte";

  let showModal = $state(false);
  let error = $state("");

  let form = $state({
    coin_id: "",
    metric: "price" as "price" | "change_24h" | "volume_24h",
    operator: ">" as "<" | ">" | "=",
    value: 0,
  });

  const metrics = [
    { value: "price", label: "Preis (€)" },
    { value: "change_24h", label: "24h-Änderung (%)" },
    { value: "volume_24h", label: "24h-Volumen (€)" },
  ];

  onMount(async () => {
    await loadAlertRules();
    if (watchlist.length > 0 && !form.coin_id) {
      form.coin_id = watchlist[0].coin_id;
    }
  });

  async function handleSave() {
    if (!form.coin_id) {
      error = "Bitte einen Coin auswählen.";
      return;
    }
    error = "";
    try {
      await saveRule(form.coin_id, form.metric, form.operator, form.value);
      showModal = false;
      form = { coin_id: watchlist[0]?.coin_id ?? "", metric: "price", operator: ">", value: 0 };
    } catch (e) {
      error = String(e);
    }
  }

  function metricLabel(metric: string) {
    return metrics.find((m) => m.value === metric)?.label ?? metric;
  }

  function coinName(coinId: string) {
    return watchlist.find((c) => c.coin_id === coinId)?.symbol.toUpperCase() ?? coinId.toUpperCase();
  }
</script>

<div class="p-4">
  <div class="flex items-center justify-between mb-4">
    <h1 class="text-xl font-semibold text-slate-200">Smart Alerts</h1>
    <button
      onclick={() => (showModal = true)}
      class="bg-purple-600 hover:bg-purple-700 text-white text-sm font-medium px-4 py-2 rounded-xl transition-colors min-h-[44px]"
    >
      + Regel
    </button>
  </div>

  {#if loading.rules}
    <div class="space-y-3">
      {#each [1, 2] as _}
        <div class="bg-[#1e1e2e] rounded-2xl h-16 animate-pulse"></div>
      {/each}
    </div>
  {:else if alertRules.length === 0}
    <div class="flex flex-col items-center justify-center py-20 text-slate-500 gap-3">
      <span class="text-5xl">◉</span>
      <p class="text-sm text-center">
        Keine Alert-Regeln.<br />Erstelle deine erste Regel.
      </p>
    </div>
  {:else}
    <div class="space-y-3">
      {#each alertRules as rule (rule.id)}
        <div class="bg-[#1e1e2e] rounded-2xl p-4 flex items-center gap-3">
          <button
            onclick={() => toggleRule(rule.id, !rule.enabled)}
            class="w-12 h-6 rounded-full transition-colors shrink-0 relative {rule.enabled
              ? 'bg-purple-600'
              : 'bg-white/10'}"
            aria-label={rule.enabled ? "Deaktivieren" : "Aktivieren"}
          >
            <span
              class="absolute top-1 w-4 h-4 bg-white rounded-full shadow transition-transform {rule.enabled
                ? 'translate-x-7'
                : 'translate-x-1'}"
            ></span>
          </button>

          <div class="flex-1 min-w-0">
            <p class="text-sm font-semibold text-slate-200">
              {coinName(rule.coin_id)}
            </p>
            <p class="text-xs text-slate-500">
              {metricLabel(rule.metric)} {rule.operator} {rule.value.toLocaleString("de-DE")}
            </p>
          </div>

          <button
            onclick={() => deleteRule(rule.id)}
            class="text-slate-600 hover:text-red-400 transition-colors p-2 min-h-[44px] min-w-[44px] flex items-center justify-center"
            aria-label="Löschen"
          >✕</button>
        </div>
      {/each}
    </div>
  {/if}
</div>

<Modal
  open={showModal}
  title="Alert-Regel erstellen"
  onclose={() => (showModal = false)}
>
  <div class="space-y-4">
    {#if watchlist.length === 0}
      <p class="text-sm text-slate-500 text-center py-4">
        Keine Coins in der Watchlist. Füge zuerst Coins hinzu.
      </p>
    {:else}
      <label class="block">
        <span class="text-xs text-slate-500 mb-1 block">Coin</span>
        <select
          bind:value={form.coin_id}
          class="w-full bg-[#0f0f14] border border-white/10 rounded-xl px-3 py-3 text-sm text-slate-200
                 focus:outline-none focus:border-purple-500 min-h-[44px]"
        >
          {#each watchlist as coin}
            <option value={coin.coin_id}>{coin.name} ({coin.symbol})</option>
          {/each}
        </select>
      </label>

      <label class="block">
        <span class="text-xs text-slate-500 mb-1 block">Metrik</span>
        <select
          bind:value={form.metric}
          class="w-full bg-[#0f0f14] border border-white/10 rounded-xl px-3 py-3 text-sm text-slate-200
                 focus:outline-none focus:border-purple-500 min-h-[44px]"
        >
          {#each metrics as m}
            <option value={m.value}>{m.label}</option>
          {/each}
        </select>
      </label>

      <div class="grid grid-cols-2 gap-3">
        <label class="block">
          <span class="text-xs text-slate-500 mb-1 block">Operator</span>
          <select
            bind:value={form.operator}
            class="w-full bg-[#0f0f14] border border-white/10 rounded-xl px-3 py-3 text-sm text-slate-200
                   focus:outline-none focus:border-purple-500 min-h-[44px]"
          >
            <option value=">">&gt; größer als</option>
            <option value="<">&lt; kleiner als</option>
            <option value="=">=  gleich</option>
          </select>
        </label>

        <label class="block">
          <span class="text-xs text-slate-500 mb-1 block">Wert</span>
          <input
            type="number"
            bind:value={form.value}
            step="0.01"
            class="w-full bg-[#0f0f14] border border-white/10 rounded-xl px-3 py-3 text-sm text-slate-200
                   focus:outline-none focus:border-purple-500 min-h-[44px]"
          />
        </label>
      </div>

      {#if error}
        <p class="text-xs text-red-400">{error}</p>
      {/if}

      <button
        onclick={handleSave}
        class="w-full bg-purple-600 hover:bg-purple-700 text-white font-semibold py-4 rounded-2xl
               transition-colors min-h-[54px]"
      >
        Regel speichern
      </button>
    {/if}
  </div>
</Modal>
