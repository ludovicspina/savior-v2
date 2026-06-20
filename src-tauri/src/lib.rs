mod commands;
mod sensors;

use commands::SensorState;
use sensors::create_source;
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let source = create_source(app.handle().clone());
            app.manage(SensorState::new(Arc::from(source)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_sensors,
            commands::stop_sensors,
            commands::set_deep_sensors,
            commands::get_sensor_snapshot,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
