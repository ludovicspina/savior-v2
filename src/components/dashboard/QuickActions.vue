<script setup lang="ts">
import { useRouter } from "vue-router";
import { Shield, Trash2, Battery } from "lucide-vue-next";
import { getModuleById } from "@/modules/registry";
import { useUiStore } from "@/stores/ui";
import { isModuleAvailable } from "@/modules/registry";

const router = useRouter();
const ui = useUiStore();

const actions = [
  { id: "antivirus", label: "Lancer un scan AV", icon: Shield },
  { id: "power", label: "Changer plan d'alim.", icon: Battery },
  { id: "debloat", label: "Ouvrir le nettoyage", icon: Trash2 },
] as const;

function go(moduleId: string) {
  const mod = getModuleById(moduleId);
  if (!mod) return;
  if (!isModuleAvailable(mod, ui.currentOs)) return;
  router.push(mod.route);
}
</script>

<template>
  <div class="card p-4">
    <p class="mb-3 text-[11px] font-semibold uppercase tracking-wider text-[var(--text-faint)]">
      Actions rapides
    </p>
    <div class="flex flex-wrap gap-2">
      <button
        v-for="action in actions"
        :key="action.id"
        type="button"
        class="inline-flex items-center gap-2 rounded-lg border border-[var(--border)] bg-[var(--surface-2)] px-3 py-2 text-sm text-[var(--text)] transition hover:border-[var(--accent)] hover:text-[var(--accent-strong)]"
        @click="go(action.id)"
      >
        <component :is="action.icon" :size="16" :stroke-width="1.75" />
        {{ action.label }}
      </button>
    </div>
  </div>
</template>
