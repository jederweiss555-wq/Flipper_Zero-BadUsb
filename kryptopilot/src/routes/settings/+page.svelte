<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  interface KeyField {
    provider: string;
    label: string;
    placeholder: string;
    hint: string;
    value: string;
    exists: boolean;
  }

  let fields = $state<KeyField[]>([
    {
      provider: "gemini",
      label: "Google Gemini API Key",
      placeholder: "AIza...",
      hint: "aistudio.google.com/app/apikey → Free: 1500 req/day",
      value: "",
      exists: false,
    },
    {
      provider: "groq",
      label: "Groq API Key (Fallback)",
      placeholder: "gsk_...",
      hint: "console.groq.com/keys → Free: 14400 req/day",
      value: "",
      exists: false,
    },
    {
      provider: "cryptopanic",
      label: "CryptoPanic API Token",
      placeholder: "Token...",
      hint: "cryptopanic.com/developers/api → Free Tier",
      value: "",
      exists: false,
    },
  ]);

  let saving = $state(false);
  let savedProvider = $state<string | null>(null);
  let error = $state("");

  onMount(async () => {
    for (const field of fields) {
      try {
        field.exists = await invoke<boolean>("get_api_key_exists", {
          provider: field.provider,
        });
      } catch {}
    }
  });

  async function saveKey(field: KeyField) {
    if (!field.value.trim()) return;
    saving = true;
    error = "";
    try {
      await invoke("save_api_key", {
        provider: field.provider,
        key: field.value.trim(),
      });
      field.exists = true;
      field.value = "";
      savedProvider = field.provider;
      setTimeout(() => (savedProvider = null), 2000);
    } catch (e) {
      error = String(e);
    } finally {
      saving = false;
    }
  }
</script>

<div class="p-4">
  <h1 class="text-xl font-semibold text-slate-200 mb-6">Settings</h1>

  <div class="space-y-4">
    {#each fields as field}
      <div class="bg-[#1e1e2e] rounded-2xl p-4">
        <div class="flex items-center justify-between mb-1">
          <span class="text-sm font-medium text-slate-300">{field.label}</span>
          {#if field.exists}
            <span class="text-xs bg-green-500/20 text-green-400 px-2 py-0.5 rounded-full">✓ gesetzt</span>
          {:else}
            <span class="text-xs bg-yellow-500/20 text-yellow-400 px-2 py-0.5 rounded-full">nicht gesetzt</span>
          {/if}
        </div>

        <p class="text-xs text-slate-600 mb-3">{field.hint}</p>

        <div class="flex gap-2">
          <input
            type="password"
            bind:value={field.value}
            placeholder={field.exists ? "••••••••••••• (neu eingeben zum Ändern)" : field.placeholder}
            class="flex-1 bg-[#0f0f14] border border-white/10 rounded-xl px-3 py-3 text-sm text-slate-200
                   placeholder-slate-700 focus:outline-none focus:border-purple-500 min-h-[44px]"
          />
          <button
            onclick={() => saveKey(field)}
            disabled={saving || !field.value.trim()}
            class="bg-purple-600 hover:bg-purple-700 disabled:opacity-40 text-white text-sm font-medium
                   px-4 rounded-xl transition-colors min-h-[44px] shrink-0"
          >
            {savedProvider === field.provider ? "✓" : "Speichern"}
          </button>
        </div>
      </div>
    {/each}

    {#if error}
      <div class="bg-red-500/10 border border-red-500/20 rounded-xl p-3 text-sm text-red-400">
        {error}
      </div>
    {/if}

    <div class="bg-[#1e1e2e] rounded-2xl p-4">
      <h2 class="text-sm font-medium text-slate-300 mb-3">KI-Provider Info</h2>
      <div class="space-y-2 text-xs text-slate-500">
        <p>• <span class="text-slate-400">Primary:</span> Gemini 2.5 Flash (Gemini Key erforderlich)</p>
        <p>• <span class="text-slate-400">Fallback:</span> Groq Llama 3.3 70B (bei Gemini-Fehler/Limit)</p>
        <p>• Keys werden lokal verschlüsselt gespeichert und nie an Dritte gesendet.</p>
      </div>
    </div>
  </div>
</div>
