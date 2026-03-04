//! IDM (Internet Download Manager) registry operations.
//! Full port of the IDM activation.bat reset logic.

use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

use crate::debug_print;

// ─────────────────────────────────────────────────────────────
//  Architecture detection
// ─────────────────────────────────────────────────────────────

fn get_arch() -> &'static str {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = hklm.open_subkey(r"Hardware\Description\System\CentralProcessor\0") {
        if let Ok(id) = key.get_value::<String, _>("Identifier") {
            if id.to_lowercase().contains("x86") {
                return "x86";
            }
        }
    }
    "x64"
}

fn get_clsid_paths() -> Vec<&'static str> {
    if get_arch() == "x86" {
        vec![r"Software\Classes\CLSID"]
    } else {
        vec![
            r"Software\Classes\CLSID",
            r"Software\Classes\Wow6432Node\CLSID"
        ]
    }
}

fn get_hklm_idm_path() -> String {
    if get_arch() == "x86" {
        r"SOFTWARE\Internet Download Manager".to_string()
    } else {
        r"SOFTWARE\Wow6432Node\Internet Download Manager".to_string()
    }
}

// ─────────────────────────────────────────────────────────────
//  Reset IDM Activation / Trial
// ─────────────────────────────────────────────────────────────

pub fn reset_activation() {
    // Step 1: Unconditionally kill IDM before messing with its registry keys
    kill_idm();

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(key) = hkcu.open_subkey(r"Software\DownloadManager") {
        if key.get_value::<String, _>("Serial").is_ok() {
            debug_print("  [i] Serial key found in registry.");
        }
    }

    // Step 2: Delete settings.bak (bat line 293)
    delete_settings_backup();

    // Step 3: Delete queue — individual values + HKLM key (bat :delete_queue, lines 492-514)
    debug_print("  [⟳] Deleting activation registry values...");
    delete_queue();

    // Step 4: Scan CLSID and delete IDM tracking keys WITH permission takeover (bat :action with take_permission=1)
    debug_print("  [⟳] Scanning and deleting CLSID tracking keys...");
    delete_clsid_keys(true); // true = take_permission on failure

    // Step 5: Re-add the AdvIntDriverEnabled2 key (bat :add_key, lines 518-538)
    debug_print("  [⟳] Adding driver registry key...");
    add_driver_key();

    debug_print("  [✓] IDM Activation / Trial reset complete.");
}

// ─────────────────────────────────────────────────────────────
//  Fix IDM Popup  (port of Fix-IDM-Popup.ps1)
// ─────────────────────────────────────────────────────────────

pub fn fix_popup() {
    debug_print("[⟳] Fixing IDM update popup...");
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    match hkcu.open_subkey_with_flags(r"Software\DownloadManager", KEY_READ | KEY_WRITE) {
        Ok(key) => {
            let current: Result<u32, _> = key.get_value("CheckUpdtVM");
            match current {
                Ok(0) => debug_print("  [✓] Already disabled."),
                Ok(_) | Err(_) => {
                    match key.set_value("CheckUpdtVM", &0u32) {
                        Ok(_) => debug_print("  [✓] CheckUpdtVM set to 0 — popup disabled."),
                        Err(e) => debug_print(&format!("  [✗] Failed: {}", e)),
                    }
                }
            }
        }
        Err(_) => {
            // Try creating the key
            if let Ok((key, _)) = hkcu.create_subkey(r"Software\DownloadManager") {
                let _ = key.set_value("CheckUpdtVM", &0u32);
                debug_print("  [✓] Created CheckUpdtVM = 0");
            } else {
                debug_print("  [✗] IDM registry path not accessible.");
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────
//  Helpers — mirrors exact .bat logic
// ─────────────────────────────────────────────────────────────

fn kill_idm() {
    let output = Command::new("tasklist")
        .args(["/fi", "imagename eq idman.exe"])
        .output();

    let is_running = match &output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.to_lowercase().contains("idman.exe")
        }
        Err(_) => false,
    };

    if is_running {
        debug_print("  [i] IDM is running, terminating...");
        let _ = Command::new("taskkill")
            .args(["/f", "/im", "idman.exe"])
            .output();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn delete_settings_backup() {
    if let Ok(appdata) = std::env::var("APPDATA") {
        let backup_path = format!("{}\\DMCache\\settings.bak", appdata);
        if std::path::Path::new(&backup_path).exists() {
            match std::fs::remove_file(&backup_path) {
                Ok(_) => debug_print("  [✓] Deleted settings.bak"),
                Err(e) => debug_print(&format!("  [✗] Failed to delete settings.bak: {}", e)),
            }
        }
    }
}

/// Mirrors :delete_queue in the bat file (lines 492-514).
/// Deletes individual HKCU\Software\DownloadManager values and the HKLM IDM key.
fn delete_queue() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let values_to_delete = [
        "FName",
        "LName",
        "Email",
        "Serial",
        "scansk",
        "tvfrdt",
        "radxcnt",
        "LstCheck",
        "ptrk_scdt",
        "LastCheckQU",
    ];

    if let Ok(key) = hkcu.open_subkey_with_flags(r"Software\DownloadManager", KEY_ALL_ACCESS) {
        for val in &values_to_delete {
            match key.delete_value(val) {
                Ok(_) => debug_print(&format!("    Deleted — HKCU\\Software\\DownloadManager\\{}", val)),
                Err(_) => {} // value didn't exist — fine
            }
        }
    }

    // Delete HKLM IDM key (bat line 509)
    let hklm_path = get_hklm_idm_path();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    match hklm.delete_subkey_all(&hklm_path) {
        Ok(_) => debug_print(&format!("    Deleted — HKLM\\{}", hklm_path)),
        Err(_) => {} // didn't exist
    }

    // Extra methods for deeper cleanups
    let vs_machine = r"Software\Classes\VirtualStore\MACHINE\SOFTWARE\Internet Download Manager";
    match hkcu.delete_subkey_all(vs_machine) {
        Ok(_) => debug_print(&format!("    Deleted — HKCU\\{}", vs_machine)),
        Err(_) => {}
    }

    let vs_wow6432 = r"Software\Classes\VirtualStore\MACHINE\SOFTWARE\Wow6432Node\Internet Download Manager";
    match hkcu.delete_subkey_all(vs_wow6432) {
        Ok(_) => debug_print(&format!("    Deleted — HKCU\\{}", vs_wow6432)),
        Err(_) => {}
    }
}

/// Re-create the AdvIntDriverEnabled2 value (bat :add_key, lines 518-538).
fn add_driver_key() {
    let hklm_path = get_hklm_idm_path();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    match hklm.create_subkey(&hklm_path) {
        Ok((key, _)) => {
            match key.set_value("AdvIntDriverEnabled2", &1u32) {
                Ok(_) => debug_print(&format!(
                    "    Added — HKLM\\{}\\AdvIntDriverEnabled2 = 1",
                    hklm_path
                )),
                Err(e) => debug_print(&format!("    [✗] Failed to set AdvIntDriverEnabled2: {}", e)),
            }
        }
        Err(e) => debug_print(&format!("    [✗] Failed to create {}: {}", hklm_path, e)),
    }
}

// ─────────────────────────────────────────────────────────────
//  CLSID scanning  (bat :action → :scan_key → :delete_key)
// ─────────────────────────────────────────────────────────────

/// Enumerate CLSID, identify IDM tracking keys, and delete them.
/// When `take_permission` is true, on deletion failure it will
/// take ownership + reset ACL then retry (mirrors bat :delete_key logic).
fn delete_clsid_keys(take_permission: bool) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let clsid_paths = get_clsid_paths();

    for clsid_path in clsid_paths {
        debug_print(&format!("    [⟳] Scanning CLSID path: {}...", clsid_path));
        let clsid_key = match hkcu.open_subkey(clsid_path) {
            Ok(k) => k,
            Err(_) => {
                debug_print("    [i] CLSID path not found, skipping.");
                continue;
            }
        };

        // Collect names first to avoid borrow issues
        let keys_to_delete: Vec<String> = clsid_key
            .enum_keys()
            .filter_map(|r| r.ok())
            .filter(|name| is_guid_format(name) && is_idm_clsid_key(&clsid_key, name))
            .collect();

        if keys_to_delete.is_empty() {
            debug_print("    [i] No IDM CLSID tracking keys found in this path.");
            continue;
        }

        debug_print(&format!("    [i] Found {} IDM tracking key(s).", keys_to_delete.len()));

        // Open with write access for deletion
        let clsid_write = match hkcu.open_subkey_with_flags(clsid_path, KEY_ALL_ACCESS) {
            Ok(k) => k,
            Err(e) => {
                debug_print(&format!("    [✗] Cannot open CLSID with write access: {}", e));
                continue;
            }
        };

        for key_name in &keys_to_delete {
            // First try direct deletion (bat: reg delete %reg% /f)
            match clsid_write.delete_subkey_all(key_name) {
                Ok(_) => {
                    debug_print(&format!("    Deleted — {}", key_name));
                }
                Err(_) if take_permission => {
                    // Mirrors bat :delete_key lines 599-602:
                    // if errorlevel != 0 AND take_permission is set → call :reg_own then retry
                    let full_path = format!("HKCU\\{}\\{}", clsid_path, key_name);
                    debug_print(&format!("    [⟳] Taking ownership of {}...", key_name));
                    take_ownership_and_delete(&full_path);
                }
                Err(_) => {
                    debug_print(&format!("    [✗] Failed — {}", key_name));
                }
            }
        }
    }
}

/// GUID format check: {xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}
fn is_guid_format(name: &str) -> bool {
    name.starts_with('{') && name.ends_with('}') && name.contains('-')
}

/// Determine if a CLSID subkey is an IDM tracking key.
/// This mirrors the bat :scan_key logic (lines 562-591).
fn is_idm_clsid_key(parent: &RegKey, name: &str) -> bool {
    let subkey = match parent.open_subkey(name) {
        Ok(k) => k,
        Err(_) => return false,
    };

    // bat line 564: skip if has LocalServer32 / InProcServer32 / InProcHandler32
    for sub in ["LocalServer32", "InProcServer32", "InProcHandler32"] {
        if subkey.open_subkey(sub).is_ok() {
            return false;
        }
    }

    // bat line 566-569: if key has no subkeys with "H" → match
    let sub_names: Vec<String> = subkey.enum_keys().filter_map(|r| r.ok()).collect();
    let has_h_subkey = sub_names.iter().any(|s| s.contains('H') || s.contains('h'));
    if sub_names.is_empty() && subkey.enum_values().count() == 0 {
        // Empty key with no values and no subkeys
        return true;
    }
    if !has_h_subkey && !sub_names.is_empty() {
        // Has subkeys but none contain "H"
        return true;
    }

    // bat line 571-574: default value is purely numeric → match
    if let Ok(default_val) = subkey.get_value::<String, _>("") {
        let trimmed = default_val.trim();
        if !trimmed.is_empty() && trimmed.chars().all(|c| c.is_ascii_digit()) {
            return true;
        }
        // bat line 586-589: default value contains "+" → match
        if trimmed.contains('+') {
            return true;
        }
    }

    // bat line 576-579: Version subkey with numeric-only default → match
    if let Ok(ver_key) = subkey.open_subkey("Version") {
        if let Ok(ver_val) = ver_key.get_value::<String, _>("") {
            if !ver_val.trim().is_empty() && ver_val.trim().chars().all(|c| c.is_ascii_digit()) {
                return true;
            }
        }
    }

    // bat line 581-584: subkey names contain MData, Model, scansk, or Therad → match
    let patterns = ["mdata", "model", "scansk", "therad"];
    for sub_name in &sub_names {
        let lower = sub_name.to_lowercase();
        if patterns.iter().any(|p| lower.contains(p)) {
            return true;
        }
    }

    false
}

/// Take ownership, reset ACL, and delete a registry key via PowerShell.
/// Mirrors the bat :reg_own logic (lines 648-660).
fn take_ownership_and_delete(reg_path: &str) {
    let ps_script = format!(
        r#"
$ErrorActionPreference = 'Stop'
try {{
    # Enable privileges
    $d = [uri].Module.GetType('System.Diagnostics.Process').GetMethods(42) | Where-Object {{ $_.Name -eq 'SetPrivilege' }}
    @('SeSecurityPrivilege','SeTakeOwnershipPrivilege','SeBackupPrivilege','SeRestorePrivilege') | ForEach-Object {{
        $d.Invoke($null, @("$_", 2))
    }}

    $regPath = '{path}' -replace '^HKCU\\\\', 'HKCU:\\'
    $owner = [System.Security.Principal.WindowsIdentity]::GetCurrent().User

    # Take ownership
    $acl = Get-Acl $regPath
    $acl.SetOwner($owner)
    Set-Acl $regPath $acl

    # Grant ourselves FullControl
    $acl = Get-Acl $regPath
    $acl.SetAccessRuleProtection($true, $false)
    $rule = New-Object System.Security.AccessControl.RegistryAccessRule($owner, 'FullControl', 'ContainerInherit,ObjectInherit', 'None', 'Allow')
    $acl.AddAccessRule($rule)
    Set-Acl $regPath $acl

    # Now delete
    Remove-Item -Path $regPath -Recurse -Force
    exit 0
}} catch {{
    Write-Error $_.Exception.Message
    exit 1
}}
"#,
        path = reg_path
    );

    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &ps_script])
        .output();

    match output {
        Ok(o) if o.status.success() => {
            debug_print(&format!("    Deleted (with ownership) — {}", reg_path));
        }
        Ok(o) => {
            let stderr = String::from_utf8_lossy(&o.stderr);
            debug_print(&format!("    [✗] Failed — {} : {}", reg_path, stderr.trim()));
        }
        Err(e) => {
            debug_print(&format!("    [✗] PowerShell error — {}: {}", reg_path, e));
        }
    }
}
