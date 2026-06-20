export type SensorMode = "basic" | "deep";

export interface CpuSnapshot {
  name: string;
  loadPct: number;
  tempC: number | null;
  clockMhz: number | null;
}

export interface RamSnapshot {
  usedMb: number;
  totalMb: number;
}

export interface GpuSnapshot {
  name: string;
  loadPct: number | null;
  tempC: number | null;
  memUsedMb: number | null;
}

export interface DiskSnapshot {
  name: string;
  tempC: number | null;
  healthPct: number | null;
  usedGb: number;
  totalGb: number;
}

export interface SensorSnapshot {
  ts: number;
  mode: SensorMode;
  cpu: CpuSnapshot;
  ram: RamSnapshot;
  gpus: GpuSnapshot[];
  disks: DiskSnapshot[];
}
