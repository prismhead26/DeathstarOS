mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Audio commands
            commands::toggle_audio_mute,
            commands::get_audio_mute_state,
            commands::get_audio_volume,
            commands::set_audio_volume,
            commands::get_audio_state,
            // Microphone commands
            commands::toggle_microphone,
            commands::get_microphone_mute_state,
            commands::get_microphone_volume,
            // Camera commands
            commands::get_camera_status,
            commands::is_camera_in_use,
            commands::toggle_camera,
            commands::get_camera_state,
            commands::open_camera_settings,
            // Network commands
            commands::toggle_wifi,
            commands::get_wifi_state,
            commands::toggle_bluetooth,
            commands::get_bluetooth_state,
            commands::get_network_state,
            // Brightness commands
            commands::get_brightness,
            commands::set_brightness,
            // Focus/DND commands
            commands::toggle_do_not_disturb,
            commands::get_do_not_disturb_state,
            commands::open_focus_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
