import type { Component } from "vue";

export type ModuleGroup = "overview" | "diagnostic" | "optimisation" | "securite";
export type Platform = "windows" | "linux";
export type ModuleStatus = "ready" | "planned";

export interface SaviorModule {
  id: string;
  label: string;
  icon: string;
  group: ModuleGroup;
  route: string;
  component: () => Promise<Component>;
  platforms: Platform[];
  status: ModuleStatus;
  description: string;
  pinned?: boolean;
}

export const GROUP_LABELS: Record<ModuleGroup, string> = {
  overview: "Vue d'ensemble",
  diagnostic: "Diagnostic",
  optimisation: "Optimisation",
  securite: "Sécurité",
};

export const GROUP_ORDER: ModuleGroup[] = [
  "overview",
  "diagnostic",
  "optimisation",
  "securite",
];

export const MODULES: SaviorModule[] = [
  {
    id: "dashboard",
    label: "Tableau de bord",
    icon: "gauge",
    group: "overview",
    route: "/dashboard",
    component: () => import("@/views/DashboardView.vue"),
    platforms: ["windows", "linux"],
    status: "ready",
    description: "Vue synthétique des indicateurs machine en temps réel.",
  },
  {
    id: "hardware",
    label: "Matériel & capteurs",
    icon: "cpu",
    group: "diagnostic",
    route: "/hardware",
    component: () => import("@/views/HardwareView.vue"),
    platforms: ["windows", "linux"],
    status: "ready",
    description: "Détail des capteurs CPU, RAM, GPU et disques.",
  },
  {
    id: "network",
    label: "Réseau",
    icon: "network",
    group: "diagnostic",
    route: "/network",
    component: () => import("@/views/NetworkView.vue"),
    platforms: ["windows", "linux"],
    status: "planned",
    description: "Diagnostic réseau, latence, interfaces et connectivité.",
  },
  {
    id: "logs",
    label: "Journaux système",
    icon: "file-text",
    group: "diagnostic",
    route: "/logs",
    component: () => import("@/views/LogsView.vue"),
    platforms: ["windows"],
    status: "planned",
    description: "Consultation et filtrage des journaux Windows Event Viewer.",
  },
  {
    id: "debloat",
    label: "Nettoyage / Debloat",
    icon: "trash-2",
    group: "optimisation",
    route: "/debloat",
    component: () => import("@/views/DebloatView.vue"),
    platforms: ["windows"],
    status: "planned",
    description: "Nettoyage des fichiers temporaires et désinstallation de bloatware.",
  },
  {
    id: "software",
    label: "Logiciels (Winget)",
    icon: "package",
    group: "optimisation",
    route: "/software",
    component: () => import("@/views/SoftwareView.vue"),
    platforms: ["windows"],
    status: "planned",
    description: "Gestion des paquets via Winget.",
  },
  {
    id: "power",
    label: "Plans d'alimentation",
    icon: "battery",
    group: "optimisation",
    route: "/power",
    component: () => import("@/views/PowerView.vue"),
    platforms: ["windows", "linux"],
    status: "planned",
    description: "Visualisation et changement des plans d'alimentation.",
  },
  {
    id: "antivirus",
    label: "Antivirus",
    icon: "shield",
    group: "securite",
    route: "/antivirus",
    component: () => import("@/views/AntivirusView.vue"),
    platforms: ["windows"],
    status: "planned",
    description: "État de la protection et lancement de scans.",
  },
  {
    id: "audit",
    label: "Audit pare-feu",
    icon: "shield-check",
    group: "securite",
    route: "/audit",
    component: () => import("@/views/AuditView.vue"),
    platforms: ["windows", "linux"],
    status: "planned",
    description: "Audit des règles pare-feu et des ports exposés.",
  },
  {
    id: "settings",
    label: "Réglages",
    icon: "settings",
    group: "overview",
    route: "/settings",
    component: () => import("@/views/SettingsView.vue"),
    platforms: ["windows", "linux"],
    status: "ready",
    description: "Préférences d'affichage et comportement de l'application.",
    pinned: true,
  },
];

export function getModuleById(id: string): SaviorModule | undefined {
  return MODULES.find((m) => m.id === id);
}

export function isModuleAvailable(module: SaviorModule, os: Platform): boolean {
  return module.platforms.includes(os);
}

export function platformLabel(module: SaviorModule, os: Platform): string | null {
  if (isModuleAvailable(module, os)) return null;
  if (module.platforms.length === 1 && module.platforms[0] === "windows") {
    return "Windows uniquement";
  }
  if (module.platforms.length === 1 && module.platforms[0] === "linux") {
    return "Linux uniquement";
  }
  return "Non disponible sur cette plateforme";
}

export const sidebarModules = MODULES.filter((m) => !m.pinned);
export const pinnedModules = MODULES.filter((m) => m.pinned);
