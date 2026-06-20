use crate::sensors::{SensorSnapshot, SensorSource};
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};

pub struct SensorState {
    source: Arc<dyn SensorSource>,
    running: Arc<AtomicBool>,
    interval_ms: Arc<Mutex<u64>>,
    task_handle: Mutex<Option<tauri::async_runtime::JoinHandle<()>>>,
}

impl SensorState {
    pub fn new(source: Arc<dyn SensorSource>) -> Self {
        Self {
            source,
            running: Arc::new(AtomicBool::new(false)),
            interval_ms: Arc::new(Mutex::new(1000)),
            task_handle: Mutex::new(None),
        }
    }
}

#[tauri::command]
pub async fn start_sensors(
    app: AppHandle,
    state: State<'_, SensorState>,
    interval_ms: Option<u64>,
) -> Result<(), String> {
    if state.running.swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    let ms = interval_ms.unwrap_or(1000).max(250);
    *state.interval_ms.lock() = ms;

    state
        .source
        .start(ms)
        .await
        .map_err(|e| e.to_string())?;

    let source = Arc::clone(&state.source);
    let running = Arc::clone(&state.running);
    let interval_ms = Arc::clone(&state.interval_ms);

    let handle = tauri::async_runtime::spawn(async move {
        loop {
            if !running.load(Ordering::SeqCst) {
                break;
            }

            let interval = *interval_ms.lock();

            match source.snapshot().await {
                Ok(snapshot) => {
                    let _ = app.emit("sensors:update", &snapshot);
                }
                Err(err) => {
                    eprintln!("sensor snapshot error: {err}");
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(interval)).await;
        }
    });

    *state.task_handle.lock() = Some(handle);
    Ok(())
}

#[tauri::command]
pub async fn stop_sensors(state: State<'_, SensorState>) -> Result<(), String> {
    state.running.store(false, Ordering::SeqCst);
    if let Some(handle) = state.task_handle.lock().take() {
        handle.abort();
    }
    state.source.stop().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn set_deep_sensors(
    state: State<'_, SensorState>,
    enabled: bool,
) -> Result<(), String> {
    state
        .source
        .set_deep(enabled)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_sensor_snapshot(
    state: State<'_, SensorState>,
) -> Result<SensorSnapshot, String> {
    state
        .source
        .snapshot()
        .await
        .map_err(|e| e.to_string())
}
