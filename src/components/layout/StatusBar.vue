<script setup lang="ts">
import { computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Search } from "lucide-vue-next";
import { useSensorsStore } from "@/stores/sensors";
import { useSystemStore } from "@/stores/system";
import { useUiStore } from "@/stores/ui";
import { computeHealth, formatUptime, healthPillClass } from "@/utils/health";

const sensors = useSensorsStore();
const system = useSystemStore();
const ui = useUiStore();

const health = computed(() => computeHealth(sensors.snapshot));

const machineLine = computed(() => {
  const info = system.info;
  if (!info) return "Chargement…";
  const host = info.hostname ?? "—";
  const os = [info.osName, info.osVersion].filter(Boolean).join(" ") || "—";
  const up = formatUptime(info.uptimeSecs);
  return `${host} · ${os} · uptime ${up}`;
});

const elevationLabel = computed(() => (ui.elevated ? "Élevé" : "Non élevé"));

const showDeepToggle = computed(() => ui.currentOs === "windows");

async function toggleDeepMode() {
  if (!showDeepToggle.value || sensors.loading) return;
  sensors.setLoading(true);
  try {
    await invoke("set_deep_mode", { enabled: !sensors.deepMode });
  } catch (err) {
    sensors.setError(err instanceof Error ? err.message : String(err));
  } finally {
    sensors.setLoading(false);
  }
}
</script>

<template>
  <header
    class="flex h-14 shrink-0 items-center gap-4 border-b border-[var(--border)] bg-[var(--bg-elevated)] px-4"
  >
    <div class="min-w-0 flex-1">
      <p class="truncate text-sm text-[var(--text-muted)]">{{ machineLine }}</p>
    </div>

    <div class="flex items-center gap-3">
      <span
        class="inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-xs font-medium ring-1 ring-inset"
        :class="healthPillClass(health.level)"
      >
        <span
          class="h-1.5 w-1.5 rounded-full"
          :class="{
            'bg-[var(--status-ok)]': health.level === 'healthy',
            'bg-[var(--status-warn)]': health.level === 'warning',
            'bg-[var(--status-crit)]': health.level === 'critical',
            'bg-[var(--text-faint)]': health.level === 'unknown',
          }"
        />
        {{ health.label }}
      </span>

      <span
        class="rounded-md border border-[var(--border)] px-2 py-1 text-xs text-[var(--text-muted)]"
      >
        {{ elevationLabel }}
      </span>

      <label
        v-if="showDeepToggle"
        class="inline-flex cursor-pointer items-center gap-2 text-xs text-[var(--text-muted)]"
      >
        <span>Mode profond</span>
        <button
          type="button"
          role="switch"
          :aria-checked="sensors.deepMode"
          :disabled="sensors.loading"
          class="relative h-5 w-9 rounded-full transition"
          :class="sensors.deepMode ? 'bg-[var(--accent)]' : 'bg-[var(--surface-3)]'"
          @click="toggleDeepMode"
        >
          <span
            class="absolute top-0.5 h-4 w-4 rounded-full bg-white transition"
            :class="sensors.deepMode ? 'left-4' : 'left-0.5'"
          />
        </button>
      </label>

      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-lg border border-[var(--border)] bg-[var(--surface-2)] px-3 py-1.5 text-xs text-[var(--text-muted)] hover:border-[var(--accent)] hover:text-[var(--text)]"
        @click="ui.openCommandPalette()"
      >
        <Search :size="14" />
        <span class="hidden sm:inline">Commandes</span>
        <kbd class="rounded bg-[var(--bg-base)] px-1.5 py-0.5 font-tabular text-[10px]">⌘K</kbd>
      </button>
    </div>
  </header>
</template>
