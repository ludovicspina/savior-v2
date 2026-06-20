<script setup lang="ts">
import ModulePlaceholder from "@/components/ModulePlaceholder.vue";
import { getModuleById } from "@/modules/registry";

const module = getModuleById("network")!;
</script>

<template>
  <ModulePlaceholder :module="module" />
</template>
