use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkState {
    pub wifi_enabled: bool,
    pub bluetooth_enabled: bool,
}

#[tauri::command]
pub fn toggle_wifi() -> Result<bool, String> {
    let current_state = get_wifi_state()?;
    let new_state = if current_state { "off" } else { "on" };

    let output = Command::new("networksetup")
        .arg("-setairportpower")
        .arg("en0")
        .arg(new_state)
        .output()
        .map_err(|e| format!("Failed to toggle WiFi: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to toggle WiFi: {}. Note: This may require admin privileges.", error));
    }

    Ok(!current_state)
}

#[tauri::command]
pub fn get_wifi_state() -> Result<bool, String> {
    let output = Command::new("networksetup")
        .arg("-getairportpower")
        .arg("en0")
        .output()
        .map_err(|e| format!("Failed to get WiFi state: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to get WiFi state: {}", error));
    }

    let result = String::from_utf8_lossy(&output.stdout);
    Ok(result.contains("On"))
}

#[tauri::command]
pub fn toggle_bluetooth() -> Result<bool, String> {
    let current_state = get_bluetooth_state()?;
    let new_state = if current_state { "0" } else { "1" };

    // Using blueutil if available, otherwise try system commands
    let output = Command::new("blueutil")
        .arg("-p")
        .arg(new_state)
        .output();

    match output {
        Ok(out) if out.status.success() => Ok(!current_state),
        _ => {
            Err("Bluetooth toggle requires 'blueutil' to be installed. Install it with: brew install blueutil".to_string())
        }
    }
}

#[tauri::command]
pub fn get_bluetooth_state() -> Result<bool, String> {
    // Try blueutil first
    let output = Command::new("blueutil")
        .arg("-p")
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let result = String::from_utf8_lossy(&out.stdout).trim().to_string();
            Ok(result == "1")
        }
        _ => {
            // Fallback: check if Bluetooth is in system_profiler
            let output = Command::new("system_profiler")
                .arg("SPBluetoothDataType")
                .output()
                .map_err(|e| format!("Failed to get Bluetooth state: {}", e))?;

            let result = String::from_utf8_lossy(&output.stdout);
            Ok(result.contains("Bluetooth Power: On"))
        }
    }
}

#[tauri::command]
pub fn get_network_state() -> Result<NetworkState, String> {
    let wifi_enabled = get_wifi_state().unwrap_or(false);
    let bluetooth_enabled = get_bluetooth_state().unwrap_or(false);

    Ok(NetworkState {
        wifi_enabled,
        bluetooth_enabled,
    })
}
