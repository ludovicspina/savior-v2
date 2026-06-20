mod commands;
mod sensors;
mod system;

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
            system::system_info,
            system::is_elevated,
            system::current_os,
            commands::set_deep_mode,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
