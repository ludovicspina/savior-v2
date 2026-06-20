import type { SensorSnapshot } from "@/types/sensors";

export type HealthLevel = "healthy" | "warning" | "critical" | "unknown";

export interface HealthStatus {
  level: HealthLevel;
  label: string;
}

const CPU_WARN = 70;
const CPU_CRIT = 85;
const GPU_WARN = 75;
const GPU_CRIT = 90;
const SMART_WARN = 80;
const SMART_CRIT = 50;

function tempLevel(temp: number | null, warn: number, crit: number): HealthLevel {
  if (temp === null) return "unknown";
  if (temp >= crit) return "critical";
  if (temp >= warn) return "warning";
  return "healthy";
}

function smartLevel(health: number | null): HealthLevel {
  if (health === null) return "unknown";
  if (health < SMART_CRIT) return "critical";
  if (health < SMART_WARN) return "warning";
  return "healthy";
}

function mergeLevels(...levels: HealthLevel[]): HealthLevel {
  if (levels.includes("critical")) return "critical";
  if (levels.includes("warning")) return "warning";
  if (levels.every((l) => l === "unknown")) return "unknown";
  return "healthy";
}

export function computeHealth(snapshot: SensorSnapshot | null): HealthStatus {
  if (!snapshot) {
    return { level: "unknown", label: "Indisponible" };
  }

  const levels: HealthLevel[] = [
    tempLevel(snapshot.cpu.tempC, CPU_WARN, CPU_CRIT),
    ...snapshot.gpus.map((g) => tempLevel(g.tempC, GPU_WARN, GPU_CRIT)),
    ...snapshot.disks.map((d) => smartLevel(d.healthPct)),
    ...snapshot.disks.map((d) => tempLevel(d.tempC, 50, 60)),
  ];

  const level = mergeLevels(...levels);
  const labels: Record<HealthLevel, string> = {
    healthy: "Sain",
    warning: "Attention",
    critical: "Critique",
    unknown: "Indisponible",
  };
  return { level, label: labels[level] };
}

export function metricBorderClass(level: HealthLevel | "neutral"): string {
  switch (level) {
    case "healthy":
      return "border-[var(--status-ok)]";
    case "warning":
      return "border-[var(--status-warn)]";
    case "critical":
      return "border-[var(--status-crit)]";
    default:
      return "border-[var(--border-muted)]";
  }
}

export function metricTextClass(level: HealthLevel | "neutral"): string {
  switch (level) {
    case "healthy":
      return "text-[var(--status-ok)]";
    case "warning":
      return "text-[var(--status-warn)]";
    case "critical":
      return "text-[var(--status-crit)]";
    default:
      return "text-[var(--text-muted)]";
  }
}

export function healthPillClass(level: HealthLevel): string {
  switch (level) {
    case "healthy":
      return "bg-[var(--status-ok)]/15 text-[var(--status-ok)] ring-[var(--status-ok)]/30";
    case "warning":
      return "bg-[var(--status-warn)]/15 text-[var(--status-warn)] ring-[var(--status-warn)]/30";
    case "critical":
      return "bg-[var(--status-crit)]/15 text-[var(--status-crit)] ring-[var(--status-crit)]/30";
    default:
      return "bg-[var(--surface-3)] text-[var(--text-muted)] ring-[var(--border-muted)]";
  }
}

export function tempHealthLevel(temp: number | null): HealthLevel {
  return tempLevel(temp, CPU_WARN, CPU_CRIT);
}

export function smartHealthLevel(health: number | null): HealthLevel {
  return smartLevel(health);
}

export function formatNullable(value: number | null, suffix = ""): string {
  return value === null ? "—" : `${Math.round(value * 10) / 10}${suffix}`;
}

export function formatUptime(secs: number | null): string {
  if (secs === null) return "—";
  const d = Math.floor(secs / 86400);
  const h = Math.floor((secs % 86400) / 3600);
  const m = Math.floor((secs % 3600) / 60);
  if (d > 0) return `${d}j ${h}h`;
  if (h > 0) return `${h}h ${m}m`;
  return `${m}m`;
}
