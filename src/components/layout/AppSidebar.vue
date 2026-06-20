<script setup lang="ts">
import { computed } from "vue";
import { useRoute, RouterLink } from "vue-router";
import { PanelLeftClose, PanelLeftOpen } from "lucide-vue-next";
import {
  GROUP_ORDER,
  GROUP_LABELS,
  sidebarModules,
  pinnedModules,
  isModuleAvailable,
  platformLabel,
  type SaviorModule,
} from "@/modules/registry";
import { resolveModuleIcon } from "@/utils/icons";
import { useUiStore } from "@/stores/ui";

const route = useRoute();
const ui = useUiStore();

const grouped = computed(() =>
  GROUP_ORDER.map((group) => ({
    group,
    label: GROUP_LABELS[group],
    items: sidebarModules.filter((m) => m.group === group),
  })).filter((g) => g.items.length > 0),
);

function itemClasses(module: SaviorModule): string[] {
  const active = route.path === module.route;
  const available = isModuleAvailable(module, ui.currentOs);
  const planned = module.status === "planned";

  return [
    "group relative flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm transition",
    active
      ? "bg-[var(--accent-muted)] text-[var(--accent-strong)]"
      : "text-[var(--text-muted)] hover:bg-[var(--surface-2)] hover:text-[var(--text)]",
    !available ? "cursor-not-allowed opacity-45 hover:bg-transparent hover:text-[var(--text-muted)]" : "",
    planned && available ? "opacity-80" : "",
  ];
}

function renderLink(module: SaviorModule) {
  const available = isModuleAvailable(module, ui.currentOs);
  return available;
}
</script>

<template>
  <aside
    class="flex h-full shrink-0 flex-col border-r border-[var(--border)] bg-[var(--bg-elevated)] transition-[width] duration-200"
    :class="ui.sidebarCollapsed ? 'w-[4.25rem]' : 'w-60'"
  >
    <div class="flex h-14 items-center gap-2 border-b border-[var(--border)] px-3">
      <div
        class="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-[var(--accent)] text-sm font-bold text-[#0c0f14]"
      >
        S
      </div>
      <span
        v-if="!ui.sidebarCollapsed"
        class="truncate text-sm font-semibold tracking-wide text-[var(--text)]"
      >
        Savior
      </span>
    </div>

    <nav class="scroll-thin flex-1 overflow-y-auto px-2 py-3">
      <section v-for="section in grouped" :key="section.group" class="mb-4">
        <p
          v-if="!ui.sidebarCollapsed"
          class="mb-1 px-3 text-[10px] font-bold uppercase tracking-widest text-[var(--text-faint)]"
        >
          {{ section.label }}
        </p>

        <ul class="space-y-0.5">
          <li v-for="module in section.items" :key="module.id">
            <RouterLink
              v-if="renderLink(module)"
              :to="module.route"
              :class="itemClasses(module)"
              :title="ui.sidebarCollapsed ? module.label : undefined"
            >
              <component
                :is="resolveModuleIcon(module.icon)"
                :size="18"
                :stroke-width="1.75"
                class="shrink-0"
              />
              <span v-if="!ui.sidebarCollapsed" class="truncate">{{ module.label }}</span>
              <span
                v-if="!ui.sidebarCollapsed && module.status === 'planned'"
                class="ml-auto rounded bg-[var(--surface-3)] px-1.5 py-0.5 text-[9px] font-semibold uppercase text-[var(--text-faint)]"
              >
                Bientôt
              </span>
            </RouterLink>

            <span
              v-else
              :class="itemClasses(module)"
              :title="platformLabel(module, ui.currentOs) ?? module.label"
            >
              <component
                :is="resolveModuleIcon(module.icon)"
                :size="18"
                :stroke-width="1.75"
                class="shrink-0"
              />
              <span v-if="!ui.sidebarCollapsed" class="truncate">{{ module.label }}</span>
            </span>
          </li>
        </ul>
      </section>
    </nav>

    <div class="border-t border-[var(--border)] px-2 py-3">
      <RouterLink
        v-for="module in pinnedModules"
        :key="module.id"
        :to="module.route"
        :class="itemClasses(module)"
        :title="ui.sidebarCollapsed ? module.label : undefined"
      >
        <component
          :is="resolveModuleIcon(module.icon)"
          :size="18"
          :stroke-width="1.75"
          class="shrink-0"
        />
        <span v-if="!ui.sidebarCollapsed">{{ module.label }}</span>
      </RouterLink>

      <button
        type="button"
        class="mt-2 flex w-full items-center justify-center rounded-lg border border-[var(--border)] p-2 text-[var(--text-muted)] hover:bg-[var(--surface-2)] hover:text-[var(--text)]"
        :title="ui.sidebarCollapsed ? 'Déplier' : 'Replier'"
        @click="ui.toggleSidebar()"
      >
        <PanelLeftClose v-if="!ui.sidebarCollapsed" :size="18" />
        <PanelLeftOpen v-else :size="18" />
      </button>
    </div>
  </aside>
</template>
