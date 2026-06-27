use std::process::Command;

#[tauri::command]
pub fn toggle_microphone() -> Result<bool, String> {
    // Get current volume to determine if we should mute or unmute
    let current_volume = get_microphone_volume()?;
    let is_currently_muted = current_volume == 0.0;

    let new_volume = if is_currently_muted { 50.0 } else { 0.0 };

    let script = format!("set volume input volume {}", new_volume);

    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| format!("Failed to execute osascript: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to toggle microphone: {}", error));
    }

    Ok(!is_currently_muted)
}

#[tauri::command]
pub fn get_microphone_mute_state() -> Result<bool, String> {
    // Microphone is considered muted if volume is 0
    let volume = get_microphone_volume()?;
    Ok(volume == 0.0)
}

#[tauri::command]
pub fn get_microphone_volume() -> Result<f32, String> {
    let script = "input volume of (get volume settings)";

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to execute osascript: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to get microphone volume: {}", error));
    }

    let volume_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let volume = volume_str
        .parse::<f32>()
        .map_err(|e| format!("Failed to parse volume: {}", e))?;

    Ok(volume)
}
