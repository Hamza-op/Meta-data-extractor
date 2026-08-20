use chrono::{DateTime, Local};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Clone, Debug, Default)]
pub struct FileFacts {
    pub name: String,
    pub format: String,
    pub media_kind: String,
    pub size: String,
    pub modified: String,
    pub sha256: String,
    pub readonly: bool,
}

impl FileFacts {
    pub fn inspect(path: &Path) -> io::Result<Self> {
        let metadata = path.metadata()?;
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        let modified = metadata
            .modified()
            .ok()
            .map(|time| {
                let local: DateTime<Local> = time.into();
                local.format("%Y-%m-%d · %H:%M").to_string()
            })
            .unwrap_or_else(|| "Unavailable".into());

        Ok(Self {
            name: path
                .file_name()
                .map(|value| value.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.display().to_string()),
            format: if extension.is_empty() {
                "Unknown".into()
            } else {
                extension.to_ascii_uppercase()
            },
            media_kind: media_kind(&extension).into(),
            size: human_size(metadata.len()),
            modified,
            sha256: sha256(path)?,
            readonly: metadata.permissions().readonly(),
        })
    }
}

fn sha256(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{} {}", bytes, UNITS[unit])
    } else {
        format!("{value:.2} {}", UNITS[unit])
    }
}

pub fn media_kind(extension: &str) -> &'static str {
    match extension.to_ascii_lowercase().as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "tif" | "tiff" | "heic" | "heif"
        | "avif" => "Image",
        "cr2" | "cr3" | "nef" | "nrw" | "arw" | "orf" | "rw2" | "raf" | "dng" | "pef" => {
            "RAW image"
        }
        "mp4" | "mov" | "avi" | "mkv" | "wmv" | "webm" | "m4v" | "mts" | "m2ts" => "Video",
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" => "Audio",
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" => "Document",
        _ => "File",
    }
}

#[cfg(test)]
mod tests {
    use super::{human_size, media_kind};

    #[test]
    fn formats_file_sizes() {
        assert_eq!(human_size(42), "42 B");
        assert_eq!(human_size(1_048_576), "1.00 MB");
    }

    #[test]
    fn classifies_common_formats() {
        assert_eq!(media_kind("JPG"), "Image");
        assert_eq!(media_kind("cr3"), "RAW image");
        assert_eq!(media_kind("mkv"), "Video");
    }
}
