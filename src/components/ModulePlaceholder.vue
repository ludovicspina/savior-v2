<script setup lang="ts">
import { computed } from "vue";
import type { SaviorModule } from "@/modules/registry";
import { resolveModuleIcon } from "@/utils/icons";

const props = defineProps<{
  module: SaviorModule;
  showPlatformNotice?: boolean;
}>();

const Icon = computed(() => resolveModuleIcon(props.module.icon));
</script>

<template>
  <div class="mx-auto flex max-w-lg flex-col items-center px-6 py-16 text-center">
    <div
      class="mb-5 flex h-16 w-16 items-center justify-center rounded-2xl border border-[var(--border)] bg-[var(--surface-2)] text-[var(--accent)]"
    >
      <component :is="Icon" :size="28" :stroke-width="1.75" />
    </div>

    <div class="mb-2 flex items-center gap-2">
      <h1 class="text-xl font-semibold text-[var(--text)]">{{ module.label }}</h1>
      <span
        v-if="module.status === 'planned'"
        class="rounded-full bg-[var(--surface-3)] px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide text-[var(--text-muted)]"
      >
        Bientôt
      </span>
    </div>

    <p class="text-sm leading-relaxed text-[var(--text-muted)]">
      {{ module.description }}
    </p>

    <div
      v-if="showPlatformNotice"
      class="mt-6 rounded-lg border border-[var(--border)] bg-[var(--surface-2)] px-4 py-3 text-sm text-[var(--text-muted)]"
    >
      Disponible sur Windows
    </div>
  </div>
</template>
