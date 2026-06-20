<script setup lang="ts">
import { computed } from "vue";
import ModulePlaceholder from "@/components/ModulePlaceholder.vue";
import { getModuleById } from "@/modules/registry";
import { useUiStore } from "@/stores/ui";

const module = getModuleById("logs")!;
const ui = useUiStore();
const showPlatformNotice = computed(
  () => ui.currentOs === "linux" && module.platforms.length === 1 && module.platforms[0] === "windows",
);
</script>

<template>
  <ModulePlaceholder :module="module" :show-platform-notice="showPlatformNotice" />
</template>
