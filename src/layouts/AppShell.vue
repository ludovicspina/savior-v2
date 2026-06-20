<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { RouterView } from "vue-router";
import AppSidebar from "@/components/layout/AppSidebar.vue";
import StatusBar from "@/components/layout/StatusBar.vue";
import CommandPalette from "@/components/layout/CommandPalette.vue";
import { useSensors } from "@/composables/useSensors";
import { useSystemStore } from "@/stores/system";
import { useUiStore } from "@/stores/ui";
import type { Platform } from "@/types/system";

useSensors();

const system = useSystemStore();
const ui = useUiStore();

onMounted(async () => {
  try {
    const os = await invoke<string>("current_os");
    if (os === "windows" || os === "linux") {
      ui.currentOs = os as Platform;
    }
    ui.elevated = await invoke<boolean>("is_elevated");
  } catch {
    /* stubs ok */
  }
  system.fetchInfo();
});

const refreshTimer = setInterval(() => system.fetchInfo(), 60_000);
onUnmounted(() => clearInterval(refreshTimer));
</script>

<template>
  <div class="flex h-full min-h-screen bg-[var(--bg-base)] text-[var(--text)]">
    <AppSidebar />
    <div class="flex min-w-0 flex-1 flex-col">
      <StatusBar />
      <main class="scroll-thin flex-1 overflow-y-auto p-5">
        <RouterView />
      </main>
    </div>
    <CommandPalette />
  </div>
</template>
