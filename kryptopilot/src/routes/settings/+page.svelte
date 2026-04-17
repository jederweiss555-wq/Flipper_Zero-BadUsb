<script lang="ts">
  // Settings – implemented in Phase 4
  let geminiKey = $state("");
  let groqKey = $state("");
  let cryptoPanicKey = $state("");
  let saving = $state(false);

  async function save() {
    saving = true;
    // invoke("save_api_key", ...) in Phase 4
    await new Promise(r => setTimeout(r, 500));
    saving = false;
  }
</script>

<div class="p-4">
  <h1 class="text-xl font-semibold text-slate-200 mb-6">Settings</h1>

  <div class="space-y-5">
    <div class="bg-[#1e1e2e] rounded-2xl p-4 space-y-4">
      <h2 class="text-sm font-medium text-slate-400 uppercase tracking-wide">KI-Provider API-Keys</h2>

      <label class="block">
        <span class="text-xs text-slate-500 mb-1 block">Google Gemini API Key</span>
        <input
          type="password"
          bind:value={geminiKey}
          placeholder="AIza..."
          class="w-full bg-[#0f0f14] border border-white/10 rounded-xl px-3 py-3 text-sm text-slate-200 placeholder-slate-600 focus:outline-none focus:border-purple-500 min-h-[44px]"
        />
      </label>

      <label class="block">
        <span class="text-xs text-slate-500 mb-1 block">Groq API Key</span>
        <input
          type="password"
          bind:value={groqKey}
          placeholder="gsk_..."
          class="w-full bg-[#0f0f14] border border-white/10 rounded-xl px-3 py-3 text-sm text-slate-200 placeholder-slate-600 focus:outline-none focus:border-purple-500 min-h-[44px]"
        />
      </label>

      <label class="block">
        <span class="text-xs text-slate-500 mb-1 block">CryptoPanic API Key</span>
        <input
          type="password"
          bind:value={cryptoPanicKey}
          placeholder="Token..."
          class="w-full bg-[#0f0f14] border border-white/10 rounded-xl px-3 py-3 text-sm text-slate-200 placeholder-slate-600 focus:outline-none focus:border-purple-500 min-h-[44px]"
        />
      </label>
    </div>

    <button
      onclick={save}
      disabled={saving}
      class="w-full bg-purple-600 hover:bg-purple-700 disabled:opacity-50 text-white font-semibold py-4 rounded-2xl transition-colors min-h-[54px]"
    >
      {saving ? "Speichern…" : "API-Keys speichern"}
    </button>
  </div>
</div>
