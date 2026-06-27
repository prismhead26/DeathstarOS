use std::process::Command;

#[tauri::command]
pub fn toggle_do_not_disturb() -> Result<bool, String> {
    let current_state = get_do_not_disturb_state()?;
    let new_state = !current_state;

    // Use defaults command to toggle DND
    let defaults_output = Command::new("defaults")
        .arg("-currentHost")
        .arg("write")
        .arg("~/Library/Preferences/ByHost/com.apple.notificationcenterui")
        .arg("doNotDisturb")
        .arg("-boolean")
        .arg(if new_state { "true" } else { "false" })
        .output();

    match defaults_output {
        Ok(out) if out.status.success() => {
            // Restart NotificationCenter to apply changes
            let _ = Command::new("killall")
                .arg("NotificationCenter")
                .output();

            Ok(new_state)
        }
        _ => {
            Err("Failed to toggle Do Not Disturb. This feature may require manual setup or system permissions.".to_string())
        }
    }
}

#[tauri::command]
pub fn get_do_not_disturb_state() -> Result<bool, String> {
    // Read DND state from preferences
    let output = Command::new("defaults")
        .arg("-currentHost")
        .arg("read")
        .arg("~/Library/Preferences/ByHost/com.apple.notificationcenterui")
        .arg("doNotDisturb")
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let result = String::from_utf8_lossy(&out.stdout).trim().to_string();
            Ok(result == "1")
        }
        _ => {
            // Default to false if we can't read the setting
            Ok(false)
        }
    }
}

#[tauri::command]
pub fn open_focus_settings() -> Result<(), String> {
    let script = "tell application \"System Settings\"\n\
                 activate\n\
                 reveal pane id \"com.apple.Focus-Settings.extension\"\n\
                 end tell";

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to open focus settings: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to open focus settings: {}", error));
    }

    Ok(())
}
