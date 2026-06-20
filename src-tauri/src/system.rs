use serde::Serialize;
use sysinfo::System;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    pub hostname: Option<String>,
    pub os_name: Option<String>,
    pub os_version: Option<String>,
    pub uptime_secs: Option<u64>,
}

#[tauri::command]
pub fn current_os() -> String {
    #[cfg(target_os = "linux")]
    {
        return "linux".into();
    }
    #[cfg(windows)]
    {
        return "windows".into();
    }
    #[cfg(not(any(target_os = "linux", windows)))]
    {
        "unknown".into()
    }
}

#[tauri::command]
pub fn is_elevated() -> bool {
    #[cfg(unix)]
    {
        return unsafe { libc::geteuid() == 0 };
    }
    #[cfg(windows)]
    {
        // TODO: Windows elevation check via token
        return false;
    }
    #[cfg(not(any(unix, windows)))]
    {
        false
    }
}

#[tauri::command]
pub fn system_info() -> SystemInfo {
    let hostname = System::host_name();
    let uptime_secs = System::uptime();

    #[cfg(target_os = "linux")]
    let (os_name, os_version) = linux_os_release();

    #[cfg(windows)]
    let (os_name, os_version) = (
        Some("Windows".to_string()),
        std::env::var("OS").ok(),
    );

    #[cfg(not(any(target_os = "linux", windows)))]
    let (os_name, os_version) = (None, None);

    SystemInfo {
        hostname,
        os_name,
        os_version,
        uptime_secs: Some(uptime_secs),
    }
}

#[cfg(target_os = "linux")]
fn linux_os_release() -> (Option<String>, Option<String>) {
    let content = std::fs::read_to_string("/etc/os-release").ok();
    let Some(content) = content else {
        return (Some("Linux".into()), None);
    };

    let mut name = None;
    let mut version = None;
    for line in content.lines() {
        if let Some(v) = line.strip_prefix("PRETTY_NAME=") {
            return (Some(trim_quotes(v)), None);
        }
        if let Some(v) = line.strip_prefix("NAME=") {
            name = Some(trim_quotes(v));
        }
        if let Some(v) = line.strip_prefix("VERSION=") {
            version = Some(trim_quotes(v));
        }
    }
    (name.or(Some("Linux".into())), version)
}

fn trim_quotes(s: &str) -> String {
    s.trim_matches('"').to_string()
}
