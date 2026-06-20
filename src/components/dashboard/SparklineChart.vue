<script setup lang="ts">
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    data: number[];
    color?: string;
    height?: number;
  }>(),
  {
    color: "var(--accent)",
    height: 48,
  },
);

const pathD = computed(() => {
  const pts = props.data;
  if (pts.length < 2) return "";
  const w = 200;
  const h = props.height - 4;
  const max = Math.max(...pts, 1);
  const min = Math.min(...pts, 0);
  const range = Math.max(max - min, 1);

  return pts
    .map((v, i) => {
      const x = (i / (pts.length - 1)) * w;
      const y = h - ((v - min) / range) * h + 2;
      return `${i === 0 ? "M" : "L"}${x.toFixed(1)},${y.toFixed(1)}`;
    })
    .join(" ");
});
</script>

<template>
  <svg
    :viewBox="`0 0 200 ${height}`"
    class="w-full overflow-visible"
    preserveAspectRatio="none"
    role="img"
    aria-hidden="true"
  >
    <path
      v-if="pathD"
      :d="pathD"
      fill="none"
      :stroke="color"
      stroke-width="1.75"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
    <line
      v-else
      x1="0"
      :y1="height / 2"
      x2="200"
      :y2="height / 2"
      stroke="var(--border-muted)"
      stroke-dasharray="4 4"
    />
  </svg>
</template>
