<script lang="ts">
  interface Props {
    data: [number, number][];
    positive?: boolean;
    width?: number;
    height?: number;
  }
  let { data, positive = true, width = 80, height = 32 }: Props = $props();

  const points = $derived(() => {
    if (data.length < 2) return "";
    const prices = data.map((d) => d[1]);
    const min = Math.min(...prices);
    const max = Math.max(...prices);
    const range = max - min || 1;
    return data
      .map((d, i) => {
        const x = (i / (data.length - 1)) * width;
        const y = height - ((d[1] - min) / range) * height;
        return `${x},${y}`;
      })
      .join(" ");
  });
</script>

{#if data.length > 1}
  <svg {width} {height} class="overflow-visible">
    <polyline
      points={points()}
      fill="none"
      stroke={positive ? "#22c55e" : "#ef4444"}
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </svg>
{/if}
