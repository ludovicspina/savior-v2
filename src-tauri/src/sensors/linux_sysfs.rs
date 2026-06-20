use crate::sensors::{DiskSnapshot, GpuSnapshot, RamSnapshot, SensorSnapshot, SensorSource};
use parking_lot::Mutex;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use sysinfo::{Disks, System};

pub struct LinuxSysfsSource {
    system: Arc<Mutex<System>>,
    disks: Arc<Mutex<Disks>>,
    mode: Arc<Mutex<String>>,
}

impl LinuxSysfsSource {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self {
            system: Arc::new(Mutex::new(system)),
            disks: Arc::new(Mutex::new(Disks::new_with_refreshed_list())),
            mode: Arc::new(Mutex::new("basic".to_string())),
        }
    }

    fn read_hwmon_temps() -> Vec<(String, f64)> {
        let mut temps = Vec::new();
        let hwmon_root = PathBuf::from("/sys/class/hwmon");
        let Ok(entries) = fs::read_dir(&hwmon_root) else {
            return temps;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let name = fs::read_to_string(path.join("name"))
                .unwrap_or_else(|_| entry.file_name().to_string_lossy().into_owned())
                .trim()
                .to_string();

            for i in 1..=10 {
                let temp_path = path.join(format!("temp{i}_input"));
                if let Ok(raw) = fs::read_to_string(&temp_path) {
                    if let Ok(milli) = raw.trim().parse::<f64>() {
                        temps.push((name.clone(), milli / 1000.0));
                    }
                }
            }
        }
        temps
    }

    fn pick_cpu_temp(temps: &[(String, f64)]) -> Option<f64> {
        temps
            .iter()
            .find(|(name, _)| {
                let n = name.to_lowercase();
                n.contains("k10") || n.contains("coretemp") || n.contains("cpu") || n.contains("zen")
            })
            .map(|(_, t)| *t)
            .or_else(|| temps.first().map(|(_, t)| *t))
    }
}

#[async_trait::async_trait]
impl SensorSource for LinuxSysfsSource {
    async fn start(&self, _interval_ms: u64) -> anyhow::Result<()> {
        Ok(())
    }

    async fn snapshot(&self) -> anyhow::Result<SensorSnapshot> {
        let mut system = self.system.lock();
        system.refresh_cpu_all();
        system.refresh_memory();

        let cpu_name = system
            .cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "CPU".to_string());

        let load_pct = system.global_cpu_usage().into();
        let temps = Self::read_hwmon_temps();
        let cpu_temp = Self::pick_cpu_temp(&temps);

        let used_mb = system.used_memory() / 1_048_576;
        let total_mb = system.total_memory() / 1_048_576;

        let mut gpus = Vec::new();
        // TODO phase 2: parse /sys/class/drm and vendor-specific GPU metrics
        for (name, temp) in &temps {
            let n = name.to_lowercase();
            if n.contains("amdgpu") || n.contains("nouveau") || n.contains("nvidia") {
                gpus.push(GpuSnapshot {
                    name: name.clone(),
                    load_pct: None,
                    temp_c: Some(*temp),
                    mem_used_mb: None,
                });
            }
        }

        let mut disks_guard = self.disks.lock();
        disks_guard.refresh(true);
        let disks: Vec<DiskSnapshot> = disks_guard
            .iter()
            .map(|d| {
                let total = d.total_space();
                let available = d.available_space();
                let used = total.saturating_sub(available);
                DiskSnapshot {
                    name: d.name().to_string_lossy().into_owned(),
                    temp_c: None, // TODO phase 2: nvme/hdd temps via hwmon or smartctl
                    health_pct: None,
                    used_gb: used / 1_073_741_824,
                    total_gb: total / 1_073_741_824,
                }
            })
            .collect();

        let mode = self.mode.lock().clone();

        Ok(SensorSnapshot {
            ts: chrono_like_epoch_ms(),
            mode,
            cpu: crate::sensors::CpuSnapshot {
                name: cpu_name,
                load_pct,
                temp_c: cpu_temp,
                clock_mhz: None, // TODO phase 2: read from /sys/devices/system/cpu
            },
            ram: RamSnapshot { used_mb, total_mb },
            gpus,
            disks,
        })
    }

    async fn set_deep(&self, _enabled: bool) -> anyhow::Result<()> {
        // TODO phase 2: enable deeper sensor reads (SMART, GPU load, etc.)
        let mut mode = self.mode.lock();
        *mode = "basic".to_string();
        Ok(())
    }
}

fn chrono_like_epoch_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}
