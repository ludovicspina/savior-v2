import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { SystemInfo } from "@/types/system";

export const useSystemStore = defineStore("system", () => {
  const info = ref<SystemInfo | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchInfo() {
    loading.value = true;
    try {
      info.value = await invoke<SystemInfo>("system_info");
      error.value = null;
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
    } finally {
      loading.value = false;
    }
  }

  return { info, loading, error, fetchInfo };
});
