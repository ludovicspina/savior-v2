<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useRouter } from "vue-router";
import { useMagicKeys } from "@vueuse/core";
import { Search } from "lucide-vue-next";
import { MODULES, isModuleAvailable, platformLabel } from "@/modules/registry";
import { resolveModuleIcon } from "@/utils/icons";
import { useUiStore } from "@/stores/ui";

const router = useRouter();
const ui = useUiStore();
const query = ref("");
const highlight = ref(0);

const keys = useMagicKeys();
const cmdK = keys["Meta+K"];
const ctrlK = keys["Ctrl+K"];

watch([cmdK, ctrlK], ([meta, ctrl]) => {
  if (meta || ctrl) ui.openCommandPalette();
});

watch(
  () => ui.commandPaletteOpen,
  (open) => {
    if (open) {
      query.value = "";
      highlight.value = 0;
    }
  },
);

const filtered = computed(() => {
  const q = query.value.trim().toLowerCase();
  return MODULES.filter((m) => {
    if (!q) return true;
    return m.label.toLowerCase().includes(q) || m.id.includes(q);
  });
});

function close() {
  ui.closeCommandPalette();
}

function select(index: number) {
  const mod = filtered.value[index];
  if (!mod) return;
  if (!isModuleAvailable(mod, ui.currentOs)) return;
  router.push(mod.route);
  close();
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === "Escape") close();
  if (e.key === "ArrowDown") {
    e.preventDefault();
    highlight.value = Math.min(highlight.value + 1, filtered.value.length - 1);
  }
  if (e.key === "ArrowUp") {
    e.preventDefault();
    highlight.value = Math.max(highlight.value - 1, 0);
  }
  if (e.key === "Enter") select(highlight.value);
}
</script>

<template>
  <Teleport to="body">
    <div
      v-if="ui.commandPaletteOpen"
      class="fixed inset-0 z-50 flex items-start justify-center bg-black/50 px-4 pt-[12vh]"
      @click.self="close"
    >
      <div
        class="w-full max-w-xl overflow-hidden rounded-xl border border-[var(--border)] bg-[var(--bg-panel)] shadow-2xl"
        @keydown="onKeydown"
      >
        <div class="flex items-center gap-2 border-b border-[var(--border)] px-4 py-3">
          <Search :size="18" class="text-[var(--text-faint)]" />
          <input
            v-model="query"
            type="text"
            autofocus
            placeholder="Aller à un module…"
            class="w-full bg-transparent text-sm text-[var(--text)] outline-none placeholder:text-[var(--text-faint)]"
          />
        </div>

        <ul class="max-h-80 overflow-y-auto py-2">
          <li v-for="(mod, i) in filtered" :key="mod.id">
            <button
              type="button"
              class="flex w-full items-center gap-3 px-4 py-2.5 text-left text-sm transition"
              :class="
                i === highlight
                  ? 'bg-[var(--accent-muted)] text-[var(--accent-strong)]'
                  : 'text-[var(--text-muted)] hover:bg-[var(--surface-2)]'
              "
              :disabled="!isModuleAvailable(mod, ui.currentOs)"
              :title="platformLabel(mod, ui.currentOs) ?? undefined"
              @click="select(i)"
              @mouseenter="highlight = i"
            >
              <component :is="resolveModuleIcon(mod.icon)" :size="16" />
              <span class="flex-1">{{ mod.label }}</span>
              <span
                v-if="mod.status === 'planned'"
                class="text-[10px] uppercase text-[var(--text-faint)]"
              >
                Bientôt
              </span>
            </button>
          </li>
        </ul>
      </div>
    </div>
  </Teleport>
</template>
