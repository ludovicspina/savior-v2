<script setup lang="ts">
import { computed } from "vue";
import { useSensors } from "@/composables/useSensors";

const { store, enableDeepMode, disableDeepMode } = useSensors();

const snapshot = computed(() => store.snapshot);
const ramPct = computed(() => {
  const ram = snapshot.value?.ram;
  if (!ram || ram.totalMb === 0) return 0;
  return Math.round((ram.usedMb / ram.totalMb) * 100);
});

function formatNullable(value: number | null, suffix = ""): string {
  return value === null ? "—" : `${value}${suffix}`;
}
</script>

<template>
  <div class="space-y-6">
    <header class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-slate-100">Savior</h1>
        <p class="text-sm text-slate-400">Hardware monitoring dashboard</p>
      </div>
      <div class="flex items-center gap-3">
        <span
          class="rounded-full px-3 py-1 text-xs font-medium"
          :class="
            snapshot?.mode === 'deep'
              ? 'bg-amber-500/20 text-amber-300'
              : 'bg-emerald-500/20 text-emerald-300'
          "
        >
          {{ snapshot?.mode ?? "…" }} mode
        </span>
        <button
          v-if="snapshot?.mode !== 'deep'"
          class="rounded-lg bg-amber-600 px-4 py-2 text-sm font-medium text-white hover:bg-amber-500 disabled:opacity-50"
          :disabled="store.loading"
          @click="enableDeepMode"
        >
          Enable deep sensors
        </button>
        <button
          v-else
          class="rounded-lg bg-slate-600 px-4 py-2 text-sm font-medium text-white hover:bg-slate-500 disabled:opacity-50"
          :disabled="store.loading"
          @click="disableDeepMode"
        >
          Disable deep sensors
        </button>
      </div>
    </header>

    <p v-if="store.error" class="rounded-lg bg-red-500/20 px-4 py-2 text-sm text-red-300">
      {{ store.error }}
    </p>

    <p v-if="!snapshot" class="text-slate-400">Waiting for sensor data…</p>

    <template v-else>
      <!-- CPU -->
      <section class="rounded-xl border border-slate-700 bg-slate-800/50 p-5">
        <h2 class="mb-3 text-lg font-semibold text-slate-200">CPU</h2>
        <p class="text-sm text-slate-400">{{ snapshot.cpu.name }}</p>
        <div class="mt-3 grid grid-cols-3 gap-4">
          <div>
            <p class="text-xs text-slate-500">Load</p>
            <p class="text-xl font-mono text-slate-100">{{ snapshot.cpu.loadPct.toFixed(1) }}%</p>
          </div>
          <div>
            <p class="text-xs text-slate-500">Temperature</p>
            <p class="text-xl font-mono text-slate-100">
              {{ formatNullable(snapshot.cpu.tempC, "°C") }}
            </p>
          </div>
          <div>
            <p class="text-xs text-slate-500">Clock</p>
            <p class="text-xl font-mono text-slate-100">
              {{ formatNullable(snapshot.cpu.clockMhz, " MHz") }}
            </p>
          </div>
        </div>
      </section>

      <!-- RAM -->
      <section class="rounded-xl border border-slate-700 bg-slate-800/50 p-5">
        <h2 class="mb-3 text-lg font-semibold text-slate-200">RAM</h2>
        <div class="flex items-end gap-4">
          <p class="text-xl font-mono text-slate-100">
            {{ snapshot.ram.usedMb }} / {{ snapshot.ram.totalMb }} MB
          </p>
          <p class="text-sm text-slate-400">({{ ramPct }}%)</p>
        </div>
        <div class="mt-3 h-2 overflow-hidden rounded-full bg-slate-700">
          <div
            class="h-full rounded-full bg-blue-500 transition-all"
            :style="{ width: `${ramPct}%` }"
          />
        </div>
      </section>

      <!-- GPUs -->
      <section
        v-if="snapshot.gpus.length"
        class="rounded-xl border border-slate-700 bg-slate-800/50 p-5"
      >
        <h2 class="mb-3 text-lg font-semibold text-slate-200">GPUs</h2>
        <div
          v-for="(gpu, i) in snapshot.gpus"
          :key="i"
          class="mb-3 last:mb-0"
        >
          <p class="text-sm text-slate-400">{{ gpu.name }}</p>
          <div class="mt-1 grid grid-cols-3 gap-4 text-sm">
            <span>Load: {{ formatNullable(gpu.loadPct, "%") }}</span>
            <span>Temp: {{ formatNullable(gpu.tempC, "°C") }}</span>
            <span>VRAM: {{ formatNullable(gpu.memUsedMb, " MB") }}</span>
          </div>
        </div>
      </section>

      <!-- Disks -->
      <section class="rounded-xl border border-slate-700 bg-slate-800/50 p-5">
        <h2 class="mb-3 text-lg font-semibold text-slate-200">Disks</h2>
        <div
          v-for="(disk, i) in snapshot.disks"
          :key="i"
          class="mb-3 border-b border-slate-700 pb-3 last:mb-0 last:border-0 last:pb-0"
        >
          <p class="text-sm text-slate-400">{{ disk.name }}</p>
          <div class="mt-1 grid grid-cols-4 gap-4 text-sm">
            <span>{{ disk.usedGb }} / {{ disk.totalGb }} GB</span>
            <span>Temp: {{ formatNullable(disk.tempC, "°C") }}</span>
            <span>Health: {{ formatNullable(disk.healthPct, "%") }}</span>
          </div>
        </div>
      </section>

      <p class="text-right text-xs text-slate-600">
        Last update: {{ new Date(snapshot.ts).toLocaleTimeString() }}
      </p>
    </template>
  </div>
</template>
