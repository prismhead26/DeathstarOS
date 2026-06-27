use std::process::Command;

#[cfg(target_os = "macos")]
mod macos {
    use std::os::raw::{c_float, c_int};

    // Private DisplayServices framework — works on macOS 11+
    #[link(name = "DisplayServices", kind = "framework")]
    extern "C" {
        fn DisplayServicesGetBrightness(display_id: u32, brightness: *mut c_float) -> c_int;
        fn DisplayServicesSetBrightness(display_id: u32, brightness: c_float) -> c_int;
    }

    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        fn CGMainDisplayID() -> u32;
    }

    pub fn get_brightness() -> Result<f32, String> {
        unsafe {
            let display = CGMainDisplayID();
            let mut brightness: c_float = 0.0;
            let ret = DisplayServicesGetBrightness(display, &mut brightness);
            if ret != 0 {
                return Err(format!("DisplayServicesGetBrightness failed: {}", ret));
            }
            Ok(brightness * 100.0)
        }
    }

    pub fn set_brightness(level: f32) -> Result<(), String> {
        unsafe {
            let display = CGMainDisplayID();
            let normalized = (level / 100.0) as c_float;
            let ret = DisplayServicesSetBrightness(display, normalized);
            if ret != 0 {
                return Err(format!("DisplayServicesSetBrightness failed: {}", ret));
            }
            Ok(())
        }
    }
}

#[tauri::command]
pub fn get_brightness() -> Result<f32, String> {
    #[cfg(target_os = "macos")]
    {
        return macos::get_brightness();
    }

    #[cfg(not(target_os = "macos"))]
    {
        let output = Command::new("brightness").arg("-l").output();
        match output {
            Ok(out) => {
                let result = String::from_utf8_lossy(&out.stdout);
                for line in result.lines() {
                    if line.contains("brightness") && !line.contains("failed") {
                        if let Some(value_str) = line.split_whitespace().last() {
                            if let Ok(value) = value_str.parse::<f32>() {
                                return Ok(value * 100.0);
                            }
                        }
                    }
                }
                Err("Could not read brightness value".to_string())
            }
            Err(_) => Err("Brightness tool not available".to_string()),
        }
    }
}

#[tauri::command]
pub fn set_brightness(level: f32) -> Result<(), String> {
    let clamped = level.max(0.0).min(100.0);

    #[cfg(target_os = "macos")]
    {
        return macos::set_brightness(clamped);
    }

    #[cfg(not(target_os = "macos"))]
    {
        let normalized = clamped / 100.0;
        let output = Command::new("brightness")
            .arg(normalized.to_string())
            .output();
        match output {
            Ok(out) if out.status.success() => Ok(()),
            Ok(out) => Err(format!(
                "Failed to set brightness: {}",
                String::from_utf8_lossy(&out.stderr)
            )),
            Err(_) => Err("Brightness tool not available".to_string()),
        }
    }
}

#[tauri::command]
pub fn open_accessibility_settings() -> Result<(), String> {
    Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
        .output()
        .map_err(|e| format!("Failed to open settings: {}", e))?;
    Ok(())
}
