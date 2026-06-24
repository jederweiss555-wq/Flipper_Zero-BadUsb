<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    open: boolean;
    title: string;
    onclose: () => void;
    children: Snippet;
  }
  let { open, title, onclose, children }: Props = $props();
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-end justify-center"
    role="dialog"
    aria-modal="true"
  >
    <button
      class="absolute inset-0 bg-black/60"
      onclick={onclose}
      aria-label="Schließen"
    ></button>
    <div class="relative w-full max-w-lg bg-[#1a1a24] rounded-t-3xl p-6 pb-10 z-10">
      <div class="flex items-center justify-between mb-5">
        <h2 class="text-lg font-semibold text-slate-200">{title}</h2>
        <button
          onclick={onclose}
          class="text-slate-500 hover:text-slate-300 text-2xl leading-none min-w-[44px] min-h-[44px] flex items-center justify-center"
        >×</button>
      </div>
      {@render children()}
    </div>
  </div>
{/if}
