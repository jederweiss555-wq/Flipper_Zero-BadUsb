<script lang="ts">
  import "../app.css";
  import { page } from "$app/stores";

  const navItems = [
    { href: "/", label: "Dashboard", icon: "◈" },
    { href: "/alerts", label: "Alerts", icon: "◉" },
    { href: "/settings", label: "Settings", icon: "⚙" },
  ];
</script>

<div class="flex flex-col min-h-screen bg-[#0f0f14]">
  <header class="px-4 py-3 bg-[#1a1a24] border-b border-white/10 flex items-center justify-between">
    <span class="text-purple-400 font-bold text-lg tracking-wide">KryptoPilot</span>
    <span class="text-xs text-slate-500">v0.1</span>
  </header>

  <main class="flex-1 overflow-y-auto pb-20">
    <slot />
  </main>

  <nav class="fixed bottom-0 left-0 right-0 bg-[#1a1a24] border-t border-white/10 flex">
    {#each navItems as item}
      <a
        href={item.href}
        class="flex-1 flex flex-col items-center py-3 gap-1 text-xs transition-colors
          {$page.url.pathname === item.href
            ? 'text-purple-400'
            : 'text-slate-500 hover:text-slate-300'}"
      >
        <span class="text-xl">{item.icon}</span>
        <span>{item.label}</span>
      </a>
    {/each}
  </nav>
</div>
