<script setup lang="ts">
import { computed } from "vue";
import { useSensorsStore } from "@/stores/sensors";
import { formatNullable } from "@/utils/health";

const sensors = useSensorsStore();
const snapshot = computed(() => sensors.snapshot);
</script>

<template>
  <div class="mx-auto max-w-6xl space-y-5">
    <div>
      <h1 class="text-xl font-semibold">Matériel & capteurs</h1>
      <p class="mt-1 text-sm text-[var(--text-muted)]">Relevés détaillés du pipeline capteurs</p>
    </div>

    <p v-if="!snapshot" class="text-sm text-[var(--text-muted)]">En attente des capteurs…</p>

    <template v-else>
      <div class="card overflow-hidden">
        <div class="card-header">Processeur</div>
        <div class="grid gap-4 p-4 sm:grid-cols-4">
          <div>
            <p class="text-xs text-[var(--text-faint)]">Modèle</p>
            <p class="mt-1 text-sm">{{ snapshot.cpu.name }}</p>
          </div>
          <div>
            <p class="text-xs text-[var(--text-faint)]">Charge</p>
            <p class="font-tabular mt-1 text-lg">{{ snapshot.cpu.loadPct.toFixed(1) }}%</p>
          </div>
          <div>
            <p class="text-xs text-[var(--text-faint)]">Température</p>
            <p class="font-tabular mt-1 text-lg">{{ formatNullable(snapshot.cpu.tempC, "°C") }}</p>
          </div>
          <div>
            <p class="text-xs text-[var(--text-faint)]">Horloge</p>
            <p class="font-tabular mt-1 text-lg">{{ formatNullable(snapshot.cpu.clockMhz, " MHz") }}</p>
          </div>
        </div>
      </div>

      <div class="card overflow-hidden">
        <div class="card-header">Mémoire</div>
        <div class="p-4">
          <p class="font-tabular text-lg">
            {{ snapshot.ram.usedMb }} / {{ snapshot.ram.totalMb }} Mo
          </p>
        </div>
      </div>

      <div v-if="snapshot.gpus.length" class="card overflow-hidden">
        <div class="card-header">GPU</div>
        <div
          v-for="(gpu, i) in snapshot.gpus"
          :key="i"
          class="grid gap-4 border-b border-[var(--border-muted)] p-4 last:border-0 sm:grid-cols-4"
        >
          <div class="sm:col-span-2">
            <p class="text-xs text-[var(--text-faint)]">Nom</p>
            <p class="mt-1 text-sm">{{ gpu.name }}</p>
          </div>
          <div>
            <p class="text-xs text-[var(--text-faint)]">Charge</p>
            <p class="font-tabular mt-1">{{ formatNullable(gpu.loadPct, "%") }}</p>
          </div>
          <div>
            <p class="text-xs text-[var(--text-faint)]">Température</p>
            <p class="font-tabular mt-1">{{ formatNullable(gpu.tempC, "°C") }}</p>
          </div>
        </div>
      </div>

      <div class="card overflow-hidden">
        <div class="card-header">Mode capteurs</div>
        <div class="p-4 text-sm text-[var(--text-muted)]">
          Mode actuel :
          <span class="font-semibold text-[var(--accent-strong)]">{{ snapshot.mode }}</span>
        </div>
      </div>
    </template>
  </div>
</template>
