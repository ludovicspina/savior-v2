export interface SystemInfo {
  hostname: string | null;
  osName: string | null;
  osVersion: string | null;
  uptimeSecs: number | null;
}

export type Platform = "windows" | "linux";
