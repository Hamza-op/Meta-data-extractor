use crate::metadata::MetadataEntry;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Deserialize, Default)]
struct NominatimAddress {
    city: Option<String>,
    town: Option<String>,
    village: Option<String>,
    state: Option<String>,
    country: Option<String>,
}

#[derive(Deserialize, Default)]
struct NominatimResponse {
    display_name: Option<String>,
    address: Option<NominatimAddress>,
}

#[derive(Deserialize, Default)]
struct WikiPage {
    title: String,
    extract: String,
}

#[derive(Deserialize, Default)]
struct WikiQuery {
    pages: HashMap<String, WikiPage>,
}

#[derive(Deserialize, Default)]
struct WikiResponse {
    query: Option<WikiQuery>,
}

pub fn fetch_internet_metadata(entries: &[MetadataEntry], file_path: &Path) -> Vec<MetadataEntry> {
    let mut results = Vec::new();

    if let Some(topic) = format_topic(file_path) {
        if let Some((title, description)) = fetch_wikipedia_intro(topic) {
            push_online(&mut results, "File format", &title);
            push_online(
                &mut results,
                "Format reference",
                truncate(&description, 420),
            );
            push_online(
                &mut results,
                "Format source",
                format!(
                    "Wikipedia · https://en.wikipedia.org/wiki/{}",
                    title.replace(' ', "_")
                ),
            );
        }
    }

    if let Some(camera) = camera_topic(entries) {
        if let Some((title, description)) = fetch_wikipedia_intro(&camera) {
            push_online(&mut results, "Device reference", title);
            push_online(&mut results, "Device context", truncate(&description, 420));
        }
    }

    let Some((latitude, longitude)) = gps_coordinates(entries) else {
        return results;
    };

    let nominatim = ureq::get("https://nominatim.openstreetmap.org/reverse")
        .query("format", "json")
        .query("lat", &latitude.to_string())
        .query("lon", &longitude.to_string())
        .query("zoom", "18")
        .query("addressdetails", "1")
        .set(
            "User-Agent",
            "MetaLens/1.1 (https://github.com/Hamza-op/Meta-data-extractor)",
        )
        .timeout(TIMEOUT)
        .call();

    if let Ok(response) = nominatim {
        if let Ok(response) = response.into_json::<NominatimResponse>() {
            if let Some(display) = response.display_name {
                push_online(&mut results, "Exact location", display);
            }
            if let Some(address) = response.address {
                let mut parts = Vec::new();
                if let Some(city) = address.city.or(address.town).or(address.village) {
                    parts.push(city);
                }
                if let Some(state) = address.state {
                    parts.push(state);
                }
                if let Some(country) = address.country {
                    parts.push(country);
                }
                if !parts.is_empty() {
                    push_online(&mut results, "City / region", parts.join(", "));
                }
            }
            push_online(&mut results, "Location source", "OpenStreetMap Nominatim");
        }
    }

    if let Some(date) = capture_date(entries) {
        fetch_weather(latitude, longitude, &date, &mut results);
    }

    results
}

fn fetch_wikipedia_intro(topic: &str) -> Option<(String, String)> {
    let response = ureq::get("https://en.wikipedia.org/w/api.php")
        .query("action", "query")
        .query("prop", "extracts")
        .query("exintro", "1")
        .query("explaintext", "1")
        .query("redirects", "1")
        .query("format", "json")
        .query("titles", topic)
        .set(
            "User-Agent",
            "MetaLens/1.1 (https://github.com/Hamza-op/Meta-data-extractor)",
        )
        .timeout(TIMEOUT)
        .call()
        .ok()?
        .into_json::<WikiResponse>()
        .ok()?;
    response
        .query?
        .pages
        .into_values()
        .find(|page| !page.extract.trim().is_empty())
        .map(|page| (page.title, page.extract))
}

fn fetch_weather(latitude: f64, longitude: f64, date: &str, results: &mut Vec<MetadataEntry>) {
    #[derive(Deserialize)]
    struct Daily {
        temperature_2m_max: Vec<Option<f32>>,
        temperature_2m_min: Vec<Option<f32>>,
        weathercode: Vec<Option<u32>>,
    }
    #[derive(Deserialize)]
    struct MeteoHistory {
        elevation: Option<f32>,
        timezone: Option<String>,
        daily: Option<Daily>,
    }

    let response = ureq::get("https://archive-api.open-meteo.com/v1/archive")
        .query("latitude", &latitude.to_string())
        .query("longitude", &longitude.to_string())
        .query("start_date", date)
        .query("end_date", date)
        .query("daily", "temperature_2m_max,temperature_2m_min,weathercode")
        .query("timezone", "auto")
        .timeout(TIMEOUT)
        .call();

    let Ok(response) = response else { return };
    let Ok(history) = response.into_json::<MeteoHistory>() else {
        return;
    };
    if let Some(timezone) = history.timezone {
        push_online(results, "Capture timezone", timezone);
    }
    if let Some(elevation) = history.elevation {
        push_online(results, "Location elevation", format!("{elevation:.0} m"));
    }
    if let Some(daily) = history.daily {
        if let (Some(maximum), Some(minimum)) = (
            daily.temperature_2m_max.first().copied().flatten(),
            daily.temperature_2m_min.first().copied().flatten(),
        ) {
            push_online(
                results,
                "Historic temperature",
                format!("{maximum:.1}°C high · {minimum:.1}°C low"),
            );
        }
        if let Some(code) = daily.weathercode.first().copied().flatten() {
            push_online(results, "Historic weather", weather_description(code));
        }
        push_online(results, "Weather source", "Open-Meteo historical archive");
    }
}

fn push_online(results: &mut Vec<MetadataEntry>, tag: &str, value: impl Into<String>) {
    results.push(MetadataEntry {
        group: "Online Intelligence".into(),
        tag: tag.into(),
        value: value.into(),
    });
}

fn gps_coordinates(entries: &[MetadataEntry]) -> Option<(f64, f64)> {
    let find = |tag: &str| {
        entries
            .iter()
            .find(|entry| entry.tag.eq_ignore_ascii_case(tag))
            .and_then(|entry| entry.value.trim().parse::<f64>().ok())
    };
    Some((find("GPS Latitude")?, find("GPS Longitude")?))
}

fn capture_date(entries: &[MetadataEntry]) -> Option<String> {
    entries
        .iter()
        .find(|entry| {
            matches!(
                entry.tag.to_ascii_lowercase().as_str(),
                "date/time original" | "create date" | "datetimeoriginal"
            )
        })
        .and_then(|entry| normalize_exif_date(&entry.value))
}

fn normalize_exif_date(value: &str) -> Option<String> {
    let prefix = value.get(..10)?;
    let normalized = prefix.replace(':', "-");
    let bytes = normalized.as_bytes();
    if bytes.len() == 10 && bytes[4] == b'-' && bytes[7] == b'-' {
        Some(normalized)
    } else {
        None
    }
}

fn camera_topic(entries: &[MetadataEntry]) -> Option<String> {
    entries
        .iter()
        .find(|entry| entry.tag.contains("Identified Camera"))
        .map(|entry| {
            entry
                .value
                .rsplit_once('→')
                .map(|(_, resolved)| resolved.trim())
                .unwrap_or(entry.value.trim())
                .to_string()
        })
}

fn format_topic(path: &Path) -> Option<&'static str> {
    match path.extension()?.to_str()?.to_ascii_lowercase().as_str() {
        "jpg" | "jpeg" => Some("JPEG"),
        "png" => Some("PNG"),
        "gif" => Some("GIF"),
        "webp" => Some("WebP"),
        "heic" | "heif" => Some("High Efficiency Image File Format"),
        "avif" => Some("AVIF"),
        "tif" | "tiff" => Some("TIFF"),
        "dng" => Some("Digital Negative"),
        "mp4" => Some("MP4 file format"),
        "mov" => Some("QuickTime File Format"),
        "mkv" => Some("Matroska"),
        "mp3" => Some("MP3"),
        "flac" => Some("FLAC"),
        "wav" => Some("WAV"),
        "pdf" => Some("PDF"),
        "svg" => Some("SVG"),
        _ => None,
    }
}

fn weather_description(code: u32) -> &'static str {
    match code {
        0 => "Clear sky",
        1..=3 => "Partly cloudy",
        45 | 48 => "Fog",
        51..=55 => "Drizzle",
        61..=65 => "Rain",
        71..=75 => "Snow",
        95 | 96 | 99 => "Thunderstorm",
        _ => "Overcast",
    }
}

fn truncate(value: &str, maximum: usize) -> String {
    let mut chars = value.chars();
    let truncated: String = chars.by_ref().take(maximum).collect();
    if chars.next().is_some() {
        format!("{}…", truncated.trim_end())
    } else {
        truncated
    }
}

#[cfg(test)]
mod tests {
    use super::{camera_topic, format_topic, normalize_exif_date, truncate};
    use crate::metadata::MetadataEntry;
    use std::path::Path;

    #[test]
    fn normalizes_valid_exif_dates() {
        assert_eq!(
            normalize_exif_date("2024:05:06 12:30:00"),
            Some("2024-05-06".into())
        );
        assert_eq!(normalize_exif_date("invalid"), None);
    }

    #[test]
    fn maps_known_formats_to_reference_topics() {
        assert_eq!(format_topic(Path::new("sample.jpg")), Some("JPEG"));
        assert_eq!(format_topic(Path::new("sample.unknown")), None);
    }

    #[test]
    fn extracts_resolved_camera_name() {
        let entries = vec![MetadataEntry {
            group: "Camera Info".into(),
            tag: "Identified Camera".into(),
            value: "ILCE-7RM5 → Sony Alpha 7R V".into(),
        }];
        assert_eq!(camera_topic(&entries), Some("Sony Alpha 7R V".into()));
    }

    #[test]
    fn truncates_long_reference_text() {
        assert_eq!(truncate("abcdef", 4), "abcd…");
        assert_eq!(truncate("abc", 4), "abc");
    }
}
