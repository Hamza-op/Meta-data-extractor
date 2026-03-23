#![cfg_attr(windows, windows_subsystem = "windows")]

#[cfg(windows)]
use std::os::windows::process::CommandExt;

mod app;
mod camera_db;
mod exiftool;
mod metadata;
mod net_enrich;

fn load_icon() -> Option<egui::IconData> {
    let logo_bytes = include_bytes!("../assets/logo.png");
    if let Ok(img) = image::load_from_memory(logo_bytes) {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        return Some(egui::IconData {
            rgba: rgba.into_raw(),
            width,
            height,
        });
    }
    None
}

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Handle --uninstall-context-menu flag
    if args.iter().any(|a| a == "--uninstall-context-menu") {
        #[cfg(windows)]
        uninstall_context_menu();
        return Ok(());
    }

    #[cfg(windows)]
    install_context_menu();

    // Check if a file path was passed as a CLI argument (e.g. from context menu)
    let initial_file = args.get(1).filter(|p| {
        !p.starts_with("--") && std::path::Path::new(p).exists()
    }).cloned();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("MetaLens — Deep Metadata Analyzer")
            .with_inner_size([960.0, 640.0])
            .with_min_inner_size([750.0, 500.0])
            .with_drag_and_drop(true)
            .with_icon(load_icon().unwrap_or_else(|| egui::IconData::default())),
        ..Default::default()
    };
    eframe::run_native(
        "MetaLens",
        options,
        Box::new(move |cc| Ok(Box::new(app::MetaLensApp::new(cc, initial_file)))),
    )
}

/// Registers "Open in MetaLens" in the Windows right-click context menu for all file types.
/// Uses HKCU so no admin rights needed. Idempotent — safe to call on every launch.
#[cfg(windows)]
fn install_context_menu() {
    let exe = std::env::current_exe().unwrap_or_default();
    let exe_str = exe.display().to_string();

    let reg_key = r#"HKCU\Software\Classes\*\shell\MetaLens"#;
    let reg_cmd = format!(r#"HKCU\Software\Classes\*\shell\MetaLens\command"#);

    // Set display name
    let _ = std::process::Command::new("reg")
        .args(["add", reg_key, "/ve", "/d", "Open in MetaLens", "/f"])
        .creation_flags(0x08000000)
        .output();

    // Limit to supported files only
    let extensions = [
        ".jpg",".jpeg",".png",".tiff",".tif",".bmp",".gif",".webp",".heic",".heif",".avif",
        ".cr2",".cr3",".nef",".nrw",".arw",".srf",".sr2",".orf",".rw2",".raf",".dng",".pef",".3fr",".iiq",".x3f",
        ".mp4",".mov",".avi",".mkv",".wmv",".flv",".webm",".m4v",".3gp",".mts",".m2ts",
        ".mp3",".wav",".flac",".aac",".ogg",".wma",".m4a",
        ".pdf",".psd",".ai",".eps",".svg",".xml",".xmp"
    ];
    let applies_to = extensions.iter()
        .map(|ext| format!("System.FileExtension:={}", ext))
        .collect::<Vec<_>>()
        .join(" OR ");

    let _ = std::process::Command::new("reg")
        .args(["add", reg_key, "/v", "AppliesTo", "/d", &applies_to, "/f"])
        .creation_flags(0x08000000)
        .output();

    // Set icon to the exe itself
    let _ = std::process::Command::new("reg")
        .args(["add", reg_key, "/v", "Icon", "/d", &format!("\"{}\",0", exe_str), "/f"])
        .creation_flags(0x08000000)
        .output();

    // Set command
    let command_value = format!("\"{}\" \"%1\"", exe_str);
    let _ = std::process::Command::new("reg")
        .args(["add", &reg_cmd, "/ve", "/d", &command_value, "/f"])
        .creation_flags(0x08000000)
        .output();
}

/// Removes the context menu registry entry.
#[cfg(windows)]
fn uninstall_context_menu() {
    let _ = std::process::Command::new("reg")
        .args(["delete", r#"HKCU\Software\Classes\*\shell\MetaLens"#, "/f"])
        .creation_flags(0x08000000)
        .output();
}
