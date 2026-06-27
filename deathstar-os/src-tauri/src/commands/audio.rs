use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioState {
    pub muted: bool,
    pub volume: f32,
}

#[tauri::command]
pub fn toggle_audio_mute() -> Result<bool, String> {
    // Get current mute state
    let is_muted = get_audio_mute_state()?;

    // Toggle mute using AppleScript
    let new_state = !is_muted;
    let script = format!(
        "set volume output muted {}",
        if new_state { "true" } else { "false" }
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| format!("Failed to execute osascript: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to toggle audio mute: {}", error));
    }

    Ok(new_state)
}

#[tauri::command]
pub fn get_audio_mute_state() -> Result<bool, String> {
    let script = "output muted of (get volume settings)";

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to execute osascript: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to get audio mute state: {}", error));
    }

    let result = String::from_utf8_lossy(&output.stdout).trim().to_lowercase();
    Ok(result == "true")
}

#[tauri::command]
pub fn get_audio_volume() -> Result<f32, String> {
    let script = "output volume of (get volume settings)";

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to execute osascript: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to get audio volume: {}", error));
    }

    let volume_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let volume = volume_str
        .parse::<f32>()
        .map_err(|e| format!("Failed to parse volume: {}", e))?;

    Ok(volume)
}

#[tauri::command]
pub fn set_audio_volume(volume: f32) -> Result<(), String> {
    // Clamp volume between 0 and 100
    let clamped_volume = volume.max(0.0).min(100.0);

    let script = format!("set volume output volume {}", clamped_volume);

    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| format!("Failed to execute osascript: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to set audio volume: {}", error));
    }

    Ok(())
}

#[tauri::command]
pub fn get_audio_state() -> Result<AudioState, String> {
    let muted = get_audio_mute_state()?;
    let volume = get_audio_volume()?;

    Ok(AudioState { muted, volume })
}
