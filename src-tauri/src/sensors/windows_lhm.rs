use crate::sensors::{SensorSnapshot, SensorSource};
use anyhow::Context;
use parking_lot::Mutex;
use std::io::Write;
use std::process::Stdio;
use std::sync::Arc;
use tauri::AppHandle;
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;

pub struct WindowsLhmSource {
    app: AppHandle,
    child: Arc<Mutex<Option<CommandChild>>>,
    latest: Arc<Mutex<Option<SensorSnapshot>>>,
    deep: Arc<Mutex<bool>>,
}

impl WindowsLhmSource {
    pub fn new(app: AppHandle) -> Self {
        Self {
            app,
            child: Arc::new(Mutex::new(None)),
            latest: Arc::new(Mutex::new(None)),
            deep: Arc::new(Mutex::new(false)),
        }
    }

    fn spawn_sidecar(&self) -> anyhow::Result<()> {
        let sidecar = self
            .app
            .shell()
            .sidecar("savior-sensord")
            .context("failed to resolve savior-sensord sidecar")?
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());

        let (mut rx, child) = sidecar.spawn().context("failed to spawn sidecar")?;

        let latest = Arc::clone(&self.latest);
        tauri::async_runtime::spawn(async move {
            while let Some(event) = rx.recv().await {
                if let CommandEvent::Stdout(line) = event {
                    if let Ok(text) = String::from_utf8(line) {
                        for raw in text.lines() {
                            if let Ok(snapshot) = serde_json::from_str::<SensorSnapshot>(raw) {
                                *latest.lock() = Some(snapshot);
                            }
                        }
                    }
                }
            }
        });

        *self.child.lock() = Some(child);
        Ok(())
    }

    fn write_stdin(&self, cmd: &str) -> anyhow::Result<()> {
        let mut guard = self.child.lock();
        let child = guard.as_mut().context("sidecar not running")?;
        child
            .write(cmd.as_bytes())
            .context("failed to write to sidecar stdin")?;
        child
            .write(b"\n")
            .context("failed to write newline to sidecar stdin")?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl SensorSource for WindowsLhmSource {
    async fn start(&self, interval_ms: u64) -> anyhow::Result<()> {
        self.spawn_sidecar()?;
        self.write_stdin(&format!("set-interval {interval_ms}"))?;
        Ok(())
    }

    async fn snapshot(&self) -> anyhow::Result<SensorSnapshot> {
        self.latest
            .lock()
            .clone()
            .context("no sensor snapshot available yet")
    }

    async fn set_deep(&self, enabled: bool) -> anyhow::Result<()> {
        if enabled {
            self.write_stdin("enable-deep")?;
            *self.deep.lock() = true;
        } else {
            self.write_stdin("disable-deep")?;
            *self.deep.lock() = false;
        }
        Ok(())
    }

    async fn stop(&self) -> anyhow::Result<()> {
        let _ = self.write_stdin("quit");
        *self.child.lock() = None;
        Ok(())
    }
}
