import { defineStore } from "pinia";
import { ref, watch } from "vue";
import type { Platform } from "@/types/system";

export type Theme = "dark" | "light";

const SIDEBAR_KEY = "savior.sidebarCollapsed";
const THEME_KEY = "savior.theme";

function readBool(key: string, fallback: boolean): boolean {
  try {
    const v = localStorage.getItem(key);
    if (v === null) return fallback;
    return v === "true";
  } catch {
    return fallback;
  }
}

function readTheme(): Theme {
  try {
    const v = localStorage.getItem(THEME_KEY);
    return v === "light" ? "light" : "dark";
  } catch {
    return "dark";
  }
}

export const useUiStore = defineStore("ui", () => {
  const currentOs = ref<Platform>("linux");
  const elevated = ref(false);
  const sidebarCollapsed = ref(readBool(SIDEBAR_KEY, false));
  const theme = ref<Theme>(readTheme());
  const commandPaletteOpen = ref(false);

  watch(sidebarCollapsed, (v) => {
    try {
      localStorage.setItem(SIDEBAR_KEY, String(v));
    } catch {
      /* ignore */
    }
  });

  watch(theme, (v) => {
    document.documentElement.dataset.theme = v;
    try {
      localStorage.setItem(THEME_KEY, v);
    } catch {
      /* ignore */
    }
  }, { immediate: true });

  function toggleSidebar() {
    sidebarCollapsed.value = !sidebarCollapsed.value;
  }

  function setTheme(value: Theme) {
    theme.value = value;
  }

  function toggleTheme() {
    theme.value = theme.value === "dark" ? "light" : "dark";
  }

  function openCommandPalette() {
    commandPaletteOpen.value = true;
  }

  function closeCommandPalette() {
    commandPaletteOpen.value = false;
  }

  return {
    currentOs,
    elevated,
    sidebarCollapsed,
    theme,
    commandPaletteOpen,
    toggleSidebar,
    setTheme,
    toggleTheme,
    openCommandPalette,
    closeCommandPalette,
  };
});
