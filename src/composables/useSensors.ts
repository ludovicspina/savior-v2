import { onMounted, onUnmounted } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import type { SensorSnapshot } from "@/types/sensors";
import { useSensorsStore } from "@/stores/sensors";

export function useSensors() {
  const store = useSensorsStore();
  let unlisten: UnlistenFn | null = null;

  onMounted(async () => {
    try {
      unlisten = await listen<SensorSnapshot>("sensors:update", (event) => {
        store.setSnapshot(event.payload);
      });
      await invoke("start_sensors", { intervalMs: 1000 });
    } catch (err) {
      store.setError(err instanceof Error ? err.message : String(err));
    }
  });

  onUnmounted(() => {
    unlisten?.();
    invoke("stop_sensors").catch(() => {});
  });

  async function enableDeepMode() {
    store.setLoading(true);
    try {
      await invoke("set_deep_sensors", { enabled: true });
    } catch (err) {
      store.setError(err instanceof Error ? err.message : String(err));
    } finally {
      store.setLoading(false);
    }
  }

  async function disableDeepMode() {
    store.setLoading(true);
    try {
      await invoke("set_deep_sensors", { enabled: false });
    } catch (err) {
      store.setError(err instanceof Error ? err.message : String(err));
    } finally {
      store.setLoading(false);
    }
  }

  return {
    store,
    enableDeepMode,
    disableDeepMode,
  };
}
