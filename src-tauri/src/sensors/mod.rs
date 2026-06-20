use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CpuSnapshot {
    pub name: String,
    pub load_pct: f64,
    pub temp_c: Option<f64>,
    pub clock_mhz: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RamSnapshot {
    pub used_mb: u64,
    pub total_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GpuSnapshot {
    pub name: String,
    pub load_pct: Option<f64>,
    pub temp_c: Option<f64>,
    pub mem_used_mb: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DiskSnapshot {
    pub name: String,
    pub temp_c: Option<f64>,
    pub health_pct: Option<f64>,
    pub used_gb: u64,
    pub total_gb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SensorSnapshot {
    pub ts: i64,
    pub mode: String,
    pub cpu: CpuSnapshot,
    pub ram: RamSnapshot,
    pub gpus: Vec<GpuSnapshot>,
    pub disks: Vec<DiskSnapshot>,
}

#[async_trait::async_trait]
pub trait SensorSource: Send + Sync {
    async fn start(&self, interval_ms: u64) -> anyhow::Result<()>;
    async fn snapshot(&self) -> anyhow::Result<SensorSnapshot>;
    async fn set_deep(&self, enabled: bool) -> anyhow::Result<()>;
    async fn stop(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[cfg(windows)]
mod windows_lhm;
#[cfg(target_os = "linux")]
mod linux_sysfs;

#[cfg(windows)]
pub use windows_lhm::WindowsLhmSource;
#[cfg(target_os = "linux")]
pub use linux_sysfs::LinuxSysfsSource;

pub fn create_source(app: tauri::AppHandle) -> Box<dyn SensorSource> {
    #[cfg(windows)]
    {
        Box::new(WindowsLhmSource::new(app))
    }
    #[cfg(target_os = "linux")]
    {
        let _ = app;
        Box::new(LinuxSysfsSource::new())
    }
    #[cfg(not(any(windows, target_os = "linux")))]
    {
        compile_error!("Savior supports Windows and Linux only");
    }
}
