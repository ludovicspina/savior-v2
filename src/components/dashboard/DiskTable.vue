<script setup lang="ts">
import type { DiskSnapshot } from "@/types/sensors";
import { formatNullable, smartHealthLevel } from "@/utils/health";

defineProps<{
  disks: DiskSnapshot[];
}>();

function usagePct(disk: DiskSnapshot): number {
  if (disk.totalGb === 0) return 0;
  return Math.round((disk.usedGb / disk.totalGb) * 100);
}
</script>

<template>
  <div class="card overflow-hidden">
    <div class="card-header">Disques</div>
    <div class="overflow-x-auto">
      <table class="w-full min-w-[640px] text-sm">
        <thead>
          <tr class="border-b border-[var(--border-muted)] text-left text-[var(--text-faint)]">
            <th class="px-4 py-2.5 font-medium">Nom</th>
            <th class="px-4 py-2.5 font-medium">Santé SMART</th>
            <th class="px-4 py-2.5 font-medium">Temp.</th>
            <th class="px-4 py-2.5 font-medium">Capacité</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="(disk, i) in disks"
            :key="i"
            class="border-b border-[var(--border-muted)] last:border-0"
          >
            <td class="px-4 py-3 text-[var(--text)]">{{ disk.name }}</td>
            <td class="px-4 py-3">
              <span class="inline-flex items-center gap-2">
                <span
                  class="h-2 w-2 rounded-full"
                  :class="{
                    'bg-[var(--status-ok)]': smartHealthLevel(disk.healthPct) === 'healthy',
                    'bg-[var(--status-warn)]': smartHealthLevel(disk.healthPct) === 'warning',
                    'bg-[var(--status-crit)]': smartHealthLevel(disk.healthPct) === 'critical',
                    'bg-[var(--text-faint)]': smartHealthLevel(disk.healthPct) === 'unknown',
                  }"
                />
                <span class="font-tabular">{{ formatNullable(disk.healthPct, "%") }}</span>
              </span>
            </td>
            <td class="px-4 py-3 font-tabular">{{ formatNullable(disk.tempC, "°C") }}</td>
            <td class="px-4 py-3 font-tabular">
              {{ disk.usedGb }} / {{ disk.totalGb }} Go
              <span class="text-[var(--text-muted)]">({{ usagePct(disk) }}%)</span>
            </td>
          </tr>
          <tr v-if="!disks.length">
            <td colspan="4" class="px-4 py-6 text-center text-[var(--text-muted)]">
              Aucun disque détecté
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
