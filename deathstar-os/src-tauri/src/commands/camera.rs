use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CameraState {
    pub enabled: bool,
    pub in_use: bool,
    pub used_by: Option<String>,
}

#[tauri::command]
pub fn get_camera_status() -> Result<bool, String> {
    // Check if camera process (VDCAssistant) is running
    let output = Command::new("pgrep")
        .arg("VDCAssistant")
        .output()
        .map_err(|e| format!("Failed to check camera status: {}", e))?;

    // If pgrep finds the process, camera is active
    Ok(output.status.success())
}

#[tauri::command]
pub fn is_camera_in_use() -> Result<bool, String> {
    // Check if any app is actively using the camera
    let output = Command::new("lsof")
        .output()
        .map_err(|e| format!("Failed to check camera usage: {}", e))?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        // Check if any process is using camera-related libraries
        let camera_in_use = result.lines().any(|line| {
            line.contains("AppleCamera") ||
            line.contains("VDCAssistant") ||
            line.contains("CMIOKit") ||
            line.contains("AVFoundation")
        });
        Ok(camera_in_use)
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub fn get_camera_app() -> Result<Option<String>, String> {
    // Check which app is using the camera
    let output = Command::new("lsof")
        .output()
        .map_err(|e| format!("Failed to check camera usage: {}", e))?;

    if !output.status.success() {
        return Ok(None);
    }

    let result = String::from_utf8_lossy(&output.stdout);

    // Look for processes using camera-related frameworks
    for line in result.lines() {
        if line.contains("AppleCamera") || line.contains("CMIOKit") {
            // Parse lsof output to get process name
            // lsof format: COMMAND  PID  USER  FD  TYPE  DEVICE  SIZE/OFF  NODE  NAME
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 0 {
                let process_name = parts[0];
                // Filter out system processes
                if process_name != "VDCAssista" &&
                   process_name != "kernel_task" &&
                   process_name != "launchd" {
                    return Ok(Some(process_name.to_string()));
                }
            }
        }
    }

    // Alternative: check processes with camera access
    let ps_output = Command::new("ps")
        .arg("-ax")
        .arg("-o")
        .arg("comm")
        .output();

    if let Ok(_ps) = ps_output {

        // Cross-reference with lsof to find apps with camera frameworks loaded
        for line in result.lines() {
            if line.contains("AppleCamera") || line.contains("AVFoundation") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 0 {
                    let cmd = parts[0];
                    if !cmd.contains("VDC") && !cmd.contains("kernel") {
                        return Ok(Some(cmd.to_string()));
                    }
                }
            }
        }
    }

    Ok(None)
}

#[tauri::command]
pub fn toggle_camera() -> Result<bool, String> {
    let is_enabled = get_camera_status()?;

    if is_enabled {
        // Try to unload the camera daemon using launchctl
        let output = Command::new("launchctl")
            .arg("unload")
            .arg("-w")
            .arg("/System/Library/LaunchAgents/com.apple.camera.assistant.plist")
            .output();

        // If that fails, try killing VDCAssistant
        if output.is_err() || !output.as_ref().unwrap().status.success() {
            let kill_output = Command::new("killall")
                .arg("VDCAssistant")
                .output()
                .map_err(|e| format!("Failed to disable camera: {}", e))?;

            if !kill_output.status.success() {
                let stderr = String::from_utf8_lossy(&kill_output.stderr);
                if stderr.contains("No matching processes") {
                    return Ok(false); // Already disabled
                }
            }
        }

        Ok(false) // Camera disabled
    } else {
        // Try to reload the camera daemon
        let _ = Command::new("launchctl")
            .arg("load")
            .arg("-w")
            .arg("/System/Library/LaunchAgents/com.apple.camera.assistant.plist")
            .output();

        // Camera will auto-enable when an app tries to use it anyway
        Ok(true) // Will be enabled on next camera access
    }
}

#[tauri::command]
pub fn get_camera_state() -> Result<CameraState, String> {
    let enabled = get_camera_status()?;
    let in_use = is_camera_in_use()?;
    let used_by = if in_use {
        get_camera_app()?
    } else {
        None
    };

    Ok(CameraState { enabled, in_use, used_by })
}

#[tauri::command]
pub fn open_camera_settings() -> Result<(), String> {
    // Open System Settings to Privacy & Security > Camera
    let output = Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Camera")
        .output()
        .map_err(|e| format!("Failed to open camera settings: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to open camera settings: {}", error));
    }

    Ok(())
}
