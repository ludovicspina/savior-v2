<script setup lang="ts">
import { computed } from "vue";
import { useSensorsStore } from "@/stores/sensors";
import { useUiStore } from "@/stores/ui";
import KpiTile from "@/components/dashboard/KpiTile.vue";
import SparklineChart from "@/components/dashboard/SparklineChart.vue";
import DiskTable from "@/components/dashboard/DiskTable.vue";
import QuickActions from "@/components/dashboard/QuickActions.vue";
import {
  formatNullable,
  tempHealthLevel,
  smartHealthLevel,
} from "@/utils/health";

const sensors = useSensorsStore();
const ui = useUiStore();

const snapshot = computed(() => sensors.snapshot);

const ramPct = computed(() => {
  const ram = snapshot.value?.ram;
  if (!ram || ram.totalMb === 0) return 0;
  return Math.round((ram.usedMb / ram.totalMb) * 100);
});

const storageKpi = computed(() => {
  const disks = snapshot.value?.disks ?? [];
  if (!disks.length) return { health: null as number | null, temp: null as number | null };

  let worstHealth: number | null = null;
  let hottest: number | null = null;

  for (const d of disks) {
    if (d.healthPct !== null) {
      worstHealth =
        worstHealth === null ? d.healthPct : Math.min(worstHealth, d.healthPct);
    }
    if (d.tempC !== null) {
      hottest = hottest === null ? d.tempC : Math.max(hottest, d.tempC);
    }
  }
  return { health: worstHealth, temp: hottest };
});

const gpuLoad = computed(() => {
  const gpus = snapshot.value?.gpus ?? [];
  const loads = gpus.map((g) => g.loadPct).filter((v): v is number => v !== null);
  if (!loads.length) return null;
  return loads.reduce((a, b) => a + b, 0) / loads.length;
});

const gpuTemp = computed(() => {
  const gpus = snapshot.value?.gpus ?? [];
  const temps = gpus.map((g) => g.tempC).filter((v): v is number => v !== null);
  if (!temps.length) return null;
  return Math.max(...temps);
});

const showBasicBanner = computed(
  () => snapshot.value?.mode === "basic" && ui.currentOs === "windows",
);
</script>

<template>
  <div class="mx-auto max-w-6xl space-y-5">
    <div class="flex items-end justify-between gap-4">
      <div>
        <h1 class="text-xl font-semibold text-[var(--text)]">Tableau de bord</h1>
        <p class="mt-1 text-sm text-[var(--text-muted)]">
          Vue synthétique en temps réel
        </p>
      </div>
      <p v-if="!snapshot" class="text-sm text-[var(--text-muted)]">En attente des capteurs…</p>
    </div>

    <p
      v-if="showBasicBanner"
      class="rounded-lg border border-[var(--border)] bg-[var(--surface-2)] px-4 py-2 text-xs text-[var(--text-muted)]"
    >
      Capteurs avancés désactivés —
      <span class="text-[var(--accent-strong)]">activez le Mode profond</span>
      dans la barre supérieure pour les températures et SMART détaillés.
    </p>

    <p
      v-else-if="snapshot?.mode === 'basic' && ui.currentOs === 'linux'"
      class="rounded-lg border border-[var(--border)] bg-[var(--surface-2)] px-4 py-2 text-xs text-[var(--text-muted)]"
    >
      Capteurs avancés limités sous Linux (phase 2).
    </p>

    <div v-if="snapshot" class="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
      <KpiTile
        label="CPU"
        :value="`${snapshot.cpu.loadPct.toFixed(1)}%`"
        :sub="`Temp ${formatNullable(snapshot.cpu.tempC, '°C')}`"
        :status="tempHealthLevel(snapshot.cpu.tempC)"
      />
      <KpiTile
        label="RAM"
        :value="`${ramPct}%`"
        :sub="`${snapshot.ram.usedMb} / ${snapshot.ram.totalMb} Mo`"
        :status="ramPct > 90 ? 'critical' : ramPct > 75 ? 'warning' : 'healthy'"
      />
      <KpiTile
        label="GPU"
        :value="formatNullable(gpuLoad, '%')"
        :sub="`Temp ${formatNullable(gpuTemp, '°C')}`"
        :status="tempHealthLevel(gpuTemp)"
      />
      <KpiTile
        label="Stockage"
        :value="formatNullable(storageKpi.health, '% SMART')"
        :sub="`Disque le plus chaud ${formatNullable(storageKpi.temp, '°C')}`"
        :status="smartHealthLevel(storageKpi.health)"
      />
    </div>

    <div v-if="snapshot" class="grid gap-4 lg:grid-cols-2">
      <div class="card p-4">
        <p class="mb-2 text-[11px] font-semibold uppercase tracking-wider text-[var(--text-faint)]">
          CPU — 60 s
        </p>
        <SparklineChart :data="sensors.cpuHistory" color="var(--accent)" />
      </div>
      <div class="card p-4">
        <p class="mb-2 text-[11px] font-semibold uppercase tracking-wider text-[var(--text-faint)]">
          RAM — 60 s
        </p>
        <SparklineChart :data="sensors.ramHistory" color="var(--status-ok)" />
      </div>
    </div>

    <DiskTable v-if="snapshot" :disks="snapshot.disks" />
    <QuickActions />
  </div>
</template>
