import { defineStore } from "pinia";
import { ref } from "vue";
import type { SensorSnapshot } from "@/types/sensors";

const HISTORY_SIZE = 60;

export const useSensorsStore = defineStore("sensors", () => {
  const snapshot = ref<SensorSnapshot | null>(null);
  const deepMode = ref(false);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const cpuHistory = ref<number[]>([]);
  const ramHistory = ref<number[]>([]);

  function setSnapshot(data: SensorSnapshot) {
    snapshot.value = data;
    deepMode.value = data.mode === "deep";
    error.value = null;

    cpuHistory.value = [...cpuHistory.value, data.cpu.loadPct].slice(-HISTORY_SIZE);
    const ramPct =
      data.ram.totalMb > 0 ? (data.ram.usedMb / data.ram.totalMb) * 100 : 0;
    ramHistory.value = [...ramHistory.value, ramPct].slice(-HISTORY_SIZE);
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
    cpuHistory,
    ramHistory,
    setSnapshot,
    setError,
    setLoading,
  };
});
