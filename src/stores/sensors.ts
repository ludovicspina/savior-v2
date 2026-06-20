import { defineStore } from "pinia";
import { ref } from "vue";
import type { SensorSnapshot } from "@/types/sensors";

export const useSensorsStore = defineStore("sensors", () => {
  const snapshot = ref<SensorSnapshot | null>(null);
  const deepMode = ref(false);
  const loading = ref(false);
  const error = ref<string | null>(null);

  function setSnapshot(data: SensorSnapshot) {
    snapshot.value = data;
    deepMode.value = data.mode === "deep";
    error.value = null;
  }

  function setError(message: string) {
    error.value = message;
  }

  function setLoading(value: boolean) {
    loading.value = value;
  }

  return {
    snapshot,
    deepMode,
    loading,
    error,
    setSnapshot,
    setError,
    setLoading,
  };
});
