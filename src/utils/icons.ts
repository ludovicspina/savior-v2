import type { Component } from "vue";
import {
  Battery,
  Cpu,
  FileText,
  Gauge,
  HelpCircle,
  Network,
  Package,
  Settings,
  Shield,
  ShieldCheck,
  Trash2,
} from "lucide-vue-next";

const ICON_MAP: Record<string, Component> = {
  gauge: Gauge,
  cpu: Cpu,
  network: Network,
  "file-text": FileText,
  "trash-2": Trash2,
  package: Package,
  battery: Battery,
  shield: Shield,
  "shield-check": ShieldCheck,
  settings: Settings,
};

export function resolveModuleIcon(name: string): Component {
  return ICON_MAP[name] ?? HelpCircle;
}
