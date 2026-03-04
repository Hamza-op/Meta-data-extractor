//! Startup registration module.
//! Uses Task Scheduler with HIGHEST privileges so admin is only requested once.

use std::process::Command;

use crate::debug_print;

const TASK_NAME: &str = "IDMSystemTool";

/// Check if the startup task already exists.
pub fn is_startup_enabled() -> bool {
    let output = Command::new("schtasks")
        .args(["/Query", "/TN", TASK_NAME])
        .output();

    match output {
        Ok(o) => o.status.success(),
        Err(_) => false,
    }
}

/// Register the startup task if it doesn't already exist.
pub fn ensure_startup_registered() {
    if is_startup_enabled() {
        debug_print("[✓] Startup task already registered.");
        return;
    }

    let exe_path = match std::env::current_exe() {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(e) => {
            debug_print(&format!("[✗] Cannot resolve exe path: {}", e));
            return;
        }
    };

    debug_print("[⟳] Registering startup task (one-time)...");

    match create_scheduled_task(&exe_path) {
        Ok(_) => {
            debug_print("[✓] Startup task registered — will run as admin on every login.");
        }
        Err(e) => {
            debug_print(&format!("[✗] Task Scheduler failed: {}", e));
            debug_print("    Falling back to registry startup...");
            register_startup_registry(&exe_path);
        }
    }
}

/// Create a scheduled task that runs at logon with highest privileges.
fn create_scheduled_task(exe_path: &str) -> Result<(), String> {
    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.4" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
  <RegistrationInfo>
    <Description>IDM System Tool — Auto maintenance on startup</Description>
  </RegistrationInfo>
  <Triggers>
    <LogonTrigger>
      <Enabled>true</Enabled>
    </LogonTrigger>
  </Triggers>
  <Principals>
    <Principal id="Author">
      <LogonType>InteractiveToken</LogonType>
      <RunLevel>HighestAvailable</RunLevel>
    </Principal>
  </Principals>
  <Settings>
    <MultipleInstancesPolicy>IgnoreNew</MultipleInstancesPolicy>
    <DisallowStartIfOnBatteries>false</DisallowStartIfOnBatteries>
    <StopIfGoingOnBatteries>false</StopIfGoingOnBatteries>
    <AllowHardTerminate>true</AllowHardTerminate>
    <StartWhenAvailable>true</StartWhenAvailable>
    <RunOnlyIfNetworkAvailable>false</RunOnlyIfNetworkAvailable>
    <AllowStartOnDemand>true</AllowStartOnDemand>
    <Enabled>true</Enabled>
    <Hidden>false</Hidden>
    <ExecutionTimeLimit>PT0S</ExecutionTimeLimit>
    <Priority>7</Priority>
  </Settings>
  <Actions Context="Author">
    <Exec>
      <Command>{}</Command>
    </Exec>
  </Actions>
</Task>"#,
        exe_path
    );

    let temp_dir = std::env::var("TEMP").unwrap_or_else(|_| "C:\\Windows\\Temp".to_string());
    let xml_path = format!("{}\\{}.xml", temp_dir, TASK_NAME);

    // Write as UTF-16 LE with BOM (required by schtasks /XML)
    use std::io::Write;
    let mut file = std::fs::File::create(&xml_path).map_err(|e| e.to_string())?;
    file.write_all(&[0xFF, 0xFE]).map_err(|e| e.to_string())?;
    for code_unit in xml.encode_utf16() {
        file.write_all(&code_unit.to_le_bytes())
            .map_err(|e| e.to_string())?;
    }
    drop(file);

    let output = Command::new("schtasks")
        .args(["/Create", "/TN", TASK_NAME, "/XML", &xml_path, "/F"])
        .output()
        .map_err(|e| format!("schtasks exec failed: {}", e))?;

    let _ = std::fs::remove_file(&xml_path);

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(stderr.trim().to_string())
    }
}

/// Fallback: HKCU Run key (no admin on subsequent runs though).
fn register_startup_registry(exe_path: &str) {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    match hkcu.open_subkey_with_flags(
        r"Software\Microsoft\Windows\CurrentVersion\Run",
        KEY_WRITE,
    ) {
        Ok(key) => match key.set_value(TASK_NAME, &exe_path) {
            Ok(_) => debug_print("[✓] Added to registry Run key."),
            Err(e) => debug_print(&format!("[✗] Registry set failed: {}", e)),
        },
        Err(e) => debug_print(&format!("[✗] Cannot open Run key: {}", e)),
    }
}
