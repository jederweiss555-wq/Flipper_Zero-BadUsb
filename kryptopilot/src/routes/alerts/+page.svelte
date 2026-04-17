<script lang="ts">
  // Alerts – implemented in Phase 4
  let rules = $state<Array<{ id: number; coin_id: string; metric: string; operator: string; value: number; enabled: boolean }>>([]);
</script>

<div class="p-4">
  <div class="flex items-center justify-between mb-4">
    <h1 class="text-xl font-semibold text-slate-200">Smart Alerts</h1>
    <button
      class="bg-purple-600 hover:bg-purple-700 text-white text-sm font-medium px-4 py-2 rounded-xl transition-colors min-h-[44px]"
    >
      + Regel
    </button>
  </div>

  {#if rules.length === 0}
    <div class="flex flex-col items-center justify-center py-20 text-slate-500 gap-3">
      <span class="text-4xl">◉</span>
      <p class="text-sm">Keine Alert-Regeln definiert.</p>
    </div>
  {/if}
</div>
