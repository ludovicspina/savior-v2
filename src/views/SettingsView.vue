<script setup lang="ts">
import { useUiStore } from "@/stores/ui";
import { useSystemStore } from "@/stores/system";
import { formatUptime } from "@/utils/health";

const ui = useUiStore();
const system = useSystemStore();
</script>

<template>
  <div class="mx-auto max-w-2xl space-y-5">
    <div>
      <h1 class="text-xl font-semibold">Réglages</h1>
      <p class="mt-1 text-sm text-[var(--text-muted)]">Préférences de l'interface</p>
    </div>

    <div class="card divide-y divide-[var(--border-muted)]">
      <div class="flex items-center justify-between p-4">
        <div>
          <p class="text-sm font-medium">Thème</p>
          <p class="text-xs text-[var(--text-muted)]">Sombre par défaut</p>
        </div>
        <div class="flex gap-2">
          <button
            type="button"
            class="rounded-lg px-3 py-1.5 text-sm"
            :class="
              ui.theme === 'dark'
                ? 'bg-[var(--accent-muted)] text-[var(--accent-strong)]'
                : 'border border-[var(--border)] text-[var(--text-muted)]'
            "
            @click="ui.setTheme('dark')"
          >
            Sombre
          </button>
          <button
            type="button"
            class="rounded-lg px-3 py-1.5 text-sm"
            :class="
              ui.theme === 'light'
                ? 'bg-[var(--accent-muted)] text-[var(--accent-strong)]'
                : 'border border-[var(--border)] text-[var(--text-muted)]'
            "
            @click="ui.setTheme('light')"
          >
            Clair
          </button>
        </div>
      </div>

      <div class="flex items-center justify-between p-4">
        <div>
          <p class="text-sm font-medium">Barre latérale</p>
          <p class="text-xs text-[var(--text-muted)]">État mémorisé localement</p>
        </div>
        <button
          type="button"
          class="rounded-lg border border-[var(--border)] px-3 py-1.5 text-sm text-[var(--text-muted)] hover:text-[var(--text)]"
          @click="ui.toggleSidebar()"
        >
          {{ ui.sidebarCollapsed ? "Déplier" : "Replier" }}
        </button>
      </div>

      <div class="p-4 text-sm text-[var(--text-muted)]">
        <p>Plateforme : <span class="text-[var(--text)]">{{ ui.currentOs }}</span></p>
        <p class="mt-1">
          Machine :
          <span class="text-[var(--text)]">
            {{ system.info?.hostname ?? "—" }}
          </span>
          · uptime {{ formatUptime(system.info?.uptimeSecs ?? null) }}
        </p>
      </div>
    </div>
  </div>
</template>
