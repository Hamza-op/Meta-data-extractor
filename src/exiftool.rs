use crate::camera_db;
use crate::metadata::MetadataEntry;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use zip::ZipArchive;

/// Embedded ExifTool payload (zip containing exiftool(-k).exe and exiftool_files)
const EXIFTOOL_PAYLOAD: &[u8] = include_bytes!("../assets/payload.zip");

pub fn find_exiftool() -> Option<PathBuf> {
    // 1. Ensure embedded ExifTool is extracted to %TEMP% and use it as priority
    if let Some(embedded_path) = ensure_embedded_exiftool() {
        if embedded_path.exists() {
            return Some(embedded_path);
        }
    }

    // 2. Next to our exe
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            for name in &["exiftool.exe", "exiftool(-k).exe", "tools\\exiftool.exe"] {
                let p = dir.join(name);
                if p.exists() {
                    return Some(p);
                }
            }
        }
    }

    // 3. Check PATH via `where`
    if let Ok(out) = Command::new("where").arg("exiftool.exe").output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout);
            if let Some(line) = s.lines().next() {
                let p = PathBuf::from(line.trim());
                if p.exists() {
                    return Some(p);
                }
            }
        }
    }

    // 4. Common locations
    for path in &[
        "C:\\exiftool\\exiftool.exe",
        "C:\\Windows\\exiftool.exe",
        "C:\\Program Files\\ExifTool\\exiftool.exe",
        "C:\\Program Files (x86)\\ExifTool\\exiftool.exe",
    ] {
        let p = PathBuf::from(path);
        if p.exists() {
            return Some(p);
        }
    }

    None
}

/// Ensures the embedded ExifTool payload is extracted to %TEMP%\MetaLens
fn ensure_embedded_exiftool() -> Option<PathBuf> {
    let temp_dir = std::env::var_os("TEMP")?;
    let dest_dir = PathBuf::from(temp_dir).join("MetaLens");
    let dest_exe = dest_dir.join("exiftool(-k).exe");

    // Check if it already exists and seems fully extracted
    if dest_exe.exists() && dest_dir.join("exiftool_files").exists() {
        return Some(dest_exe);
    }

    // Extraction needed
    let _ = std::fs::create_dir_all(&dest_dir);
    let payload_path = dest_dir.join("payload.zip");
    
    if std::fs::write(&payload_path, EXIFTOOL_PAYLOAD).is_ok() {
        let extracted = extract_payload(&payload_path, &dest_dir).is_ok();
        let _ = std::fs::remove_file(&payload_path); // Cleanup zip
        
        if extracted && dest_exe.exists() {
            return Some(dest_exe);
        }
    }
    
    None
}

fn extract_payload(zip_path: &Path, dest_dir: &Path) -> io::Result<()> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let Some(rel_path) = entry.enclosed_name().map(|p| p.to_path_buf()) else {
            continue;
        };
        let out_path = dest_dir.join(rel_path);

        if entry.is_dir() {
            std::fs::create_dir_all(&out_path)?;
            continue;
        }

        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut out_file = File::create(&out_path)?;
        io::copy(&mut entry, &mut out_file)?;
    }

    Ok(())
}

pub fn run_exiftool(exiftool_path: &Path, file_path: &Path) -> Result<String, String> {
    let output = Command::new(exiftool_path)
        .args(["-All", "-G:1", "-a", "-u", "-f", "-E", "-c", "%+.6f"])
        .arg(file_path)
        .output()
        .map_err(|e| format!("Failed to launch ExifTool: {}", e))?;

    if !output.status.success() && output.stdout.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("ExifTool error: {}", stderr));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub struct ParseResult {
    pub entries: Vec<MetadataEntry>,
    pub groups: Vec<String>,
    pub found_model: String,
}

pub fn parse_output(output: &str) -> ParseResult {
    let mut entries = Vec::new();
    let mut groups = Vec::new();
    let mut group_set = std::collections::HashSet::new();
    let mut found_model = String::new();
    let mut found_make = String::new();
    let mut found_lens_model = String::new();
    let mut found_lens_id = String::new();
    let mut found_lens_type = String::new();
    let mut found_lens_info = String::new();

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (group, rest) = if let (Some(open), Some(close)) = (line.find('['), line.find(']')) {
            if close > open {
                (line[open + 1..close].trim().to_string(), &line[close + 1..])
            } else {
                ("Other".to_string(), line)
            }
        } else {
            ("Other".to_string(), line)
        };

        let Some(colon) = rest.find(':') else { continue };
        let tag = rest[..colon].trim().to_string();
        let value = rest[colon + 1..].trim().to_string();
        if tag.is_empty() || tag.to_lowercase().starts_with("unknown") {
            continue;
        }
        if value.contains("(Binary data") || value.contains("use -b option to extract") {
            continue;
        }

        let tag_low = tag.to_lowercase();

        // Collect model/lens info
        if (tag_low == "camera model name" || tag_low == "model") && found_model.is_empty() {
            found_model = value.clone();
        }
        if tag_low == "make" && found_make.is_empty() {
            found_make = value.clone();
        }
        if (tag_low == "lens model" || tag_low == "lens") && found_lens_model.is_empty() {
            found_lens_model = value.clone();
        }
        if (tag_low == "lens id" || tag_low == "lensid") && found_lens_id.is_empty() {
            found_lens_id = value.clone();
        }
        if (tag_low == "lens type" || tag_low == "lenstype") && found_lens_type.is_empty() {
            found_lens_type = value.clone();
        }
        if (tag_low == "lens info" || tag_low == "lensinfo") && found_lens_info.is_empty() {
            found_lens_info = value.clone();
        }

        if group_set.insert(group.clone()) {
            groups.push(group.clone());
        }
        entries.push(MetadataEntry { group, tag, value });
    }

    // Resolve camera
    if !found_model.is_empty() {
        let resolved = camera_db::resolve_camera_model(&found_model);
        if resolved != found_model {
            entries.insert(0, MetadataEntry {
                group: "Camera Info".into(),
                tag: "\u{1F4F7} Identified Camera".into(),
                value: format!("{}  \u{2192}  {}", found_model, resolved),
            });
            if group_set.insert("Camera Info".into()) {
                groups.insert(0, "Camera Info".into());
            }
        }
    }

    // Resolve lens
    let best_lens = [&found_lens_model, &found_lens_id, &found_lens_type, &found_lens_info]
        .iter()
        .find(|s| !s.is_empty() && **s != "-" && s.to_lowercase() != "unknown")
        .map(|s| s.to_string());

    if let Some(lens_str) = best_lens {
        let resolved = camera_db::resolve_lens_model(&lens_str);
        let (tag, value) = if resolved != lens_str {
            ("\u{1F52D} Identified Lens".to_string(), format!("{}  \u{2192}  {}", lens_str, resolved))
        } else {
            ("\u{1F52D} Lens".to_string(), lens_str)
        };
        let pos = if !entries.is_empty() && entries[0].tag.contains("Identified Camera") { 1 } else { 0 };
        entries.insert(pos, MetadataEntry { group: "Camera Info".into(), tag, value });
        if group_set.insert("Camera Info".into()) {
            groups.insert(0, "Camera Info".into());
        }
    }

    // Sort groups by priority
    groups.sort_by_key(|g| {
        let gl = g.to_lowercase();
        if gl == "camera info" { 0 }
        else if gl.contains("exif") { 1 }
        else if gl.contains("ifd0") { 2 }
        else if gl.contains("makernotes") { 3 }
        else if gl.contains("xmp") { 4 }
        else if gl.contains("iptc") { 5 }
        else if gl.contains("icc") { 6 }
        else if gl.contains("composite") { 7 }
        else if gl.contains("file") { 8 }
        else if gl.contains("quicktime") || gl.contains("track") { 3 }
        else { 10 }
    });

    ParseResult { entries, groups, found_model }
}

#[cfg(test)]
mod tests {
    use super::parse_output;

    #[test]
    fn parse_output_adds_camera_and_lens_identification() {
        let output = "\
[EXIF] Camera Model Name : ILCE-7RM5
[EXIF] Lens Model : FE 24-70mm F2.8 GM II
[EXIF] ISO : 100
";

        let parsed = parse_output(output);

        assert_eq!(parsed.found_model, "ILCE-7RM5");
        assert_eq!(parsed.groups.first().map(String::as_str), Some("Camera Info"));
        assert!(parsed.entries.iter().any(|e| {
            e.group == "Camera Info"
                && e.tag.contains("Identified Camera")
                && e.value.contains("Sony Alpha 7R V")
        }));
        assert!(parsed.entries.iter().any(|e| {
            e.group == "Camera Info"
                && e.tag.contains("Identified Lens")
                && e.value.contains("Sony FE 24-70mm")
        }));
    }
}
