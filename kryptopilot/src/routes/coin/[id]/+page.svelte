<script lang="ts">
  import { page } from "$app/stores";

  let coinId = $derived($page.params.id);
  let analyzing = $state(false);
  let analysis = $state<string | null>(null);
</script>

<div class="p-4">
  <a href="/" class="text-purple-400 text-sm mb-4 block">← Zurück</a>

  <h1 class="text-2xl font-bold text-slate-200 mb-1 uppercase">{coinId}</h1>
  <p class="text-slate-500 text-sm mb-6">Detailansicht – implementiert in Phase 4</p>

  <button
    onclick={() => { analyzing = true; }}
    disabled={analyzing}
    class="w-full bg-purple-600 hover:bg-purple-700 disabled:opacity-60 text-white font-semibold py-4 rounded-2xl transition-colors min-h-[54px] mb-4"
  >
    {analyzing ? "KI analysiert…" : "KI-Analyse starten"}
  </button>

  {#if analysis}
    <div class="bg-[#1e1e2e] rounded-2xl p-4 prose prose-invert prose-sm max-w-none">
      {@html analysis}
    </div>
  {/if}
</div>
