<script setup lang="ts">
import type { HealthLevel } from "@/utils/health";
import { metricBorderClass } from "@/utils/health";

defineProps<{
  label: string;
  value: string;
  sub?: string;
  status?: HealthLevel | "neutral";
}>();
</script>

<template>
  <div
    class="card border-l-4 p-4"
    :class="metricBorderClass(status ?? 'neutral')"
  >
    <p class="text-[11px] font-semibold uppercase tracking-wider text-[var(--text-faint)]">
      {{ label }}
    </p>
    <p class="font-tabular mt-2 text-2xl font-semibold text-[var(--text)]">
      {{ value }}
    </p>
    <p v-if="sub" class="mt-1 text-xs text-[var(--text-muted)]">{{ sub }}</p>
  </div>
</template>
