use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MetadataEntry {
    pub group: String,
    pub tag: String,
    pub value: String,
}

pub struct SummaryField {
    pub display_name: &'static str,
    pub aliases: &'static [&'static str],
}

pub fn summary_fields() -> Vec<SummaryField> {
    vec![
        SummaryField { display_name: "Date Taken", aliases: &["date/time original", "datetimeoriginal", "create date", "createdate"] },
        SummaryField { display_name: "Camera Model", aliases: &["camera model name", "model"] },
        SummaryField { display_name: "Lens Model", aliases: &["lens model", "lens", "lens type"] },
        SummaryField { display_name: "ISO", aliases: &["iso", "iso speed", "iso speed ratings"] },
        SummaryField { display_name: "Aperture", aliases: &["aperture", "aperture value", "f number", "fnumber"] },
        SummaryField { display_name: "Shutter Speed", aliases: &["shutter speed", "shutter speed value", "exposure time", "exposuretime"] },
        SummaryField { display_name: "Focal Length", aliases: &["focal length", "focallength"] },
        SummaryField { display_name: "Image Width", aliases: &["image width", "imagewidth", "exif image width", "source image width"] },
        SummaryField { display_name: "Image Height", aliases: &["image height", "imageheight", "exif image height", "source image height"] },
        SummaryField { display_name: "File Size", aliases: &["file size", "filesize"] },
        SummaryField { display_name: "Frame Rate", aliases: &["frame rate", "video frame rate", "framerate"] },
        SummaryField { display_name: "Bitrate", aliases: &["bitrate", "avg bitrate", "nominal bitrate", "audio bitrate", "video bitrate"] },
        SummaryField { display_name: "Duration", aliases: &["duration", "media duration", "track duration"] },
        SummaryField { display_name: "Video Codec", aliases: &["video codec", "codec id", "compressor id", "compressor name"] },
        SummaryField { display_name: "Color Profile", aliases: &["color profile", "icc profile name", "profile description", "color space data"] },
        SummaryField { display_name: "White Balance", aliases: &["white balance", "whitebalance"] },
        SummaryField { display_name: "GPS Latitude", aliases: &["gps latitude", "gpslatitude"] },
        SummaryField { display_name: "GPS Longitude", aliases: &["gps longitude", "gpslongitude"] },
        SummaryField { display_name: "Exact Location", aliases: &["🌍 exact location"] },
        SummaryField { display_name: "City / Region", aliases: &["🏙️ city / region"] },
        SummaryField { display_name: "Historic Temp", aliases: &["🌡️ historic temperature"] },
        SummaryField { display_name: "Historic Weather", aliases: &["☁️ historic weather"] },
        SummaryField { display_name: "Exposure Compensation", aliases: &["exposure compensation", "exposurecompensation"] },
        SummaryField { display_name: "Metering Mode", aliases: &["metering mode", "meteringmode"] },
        SummaryField { display_name: "Focus Mode", aliases: &["focus mode", "focusmode", "af mode"] },
        SummaryField { display_name: "Flash Mode", aliases: &["flash", "flash mode", "flashmode"] },
        SummaryField { display_name: "Camera Make", aliases: &["make"] },
        SummaryField { display_name: "Bit Depth", aliases: &["bit depth", "bitdepth", "bits per sample", "bitspersample"] },
        SummaryField { display_name: "Color Space", aliases: &["color space", "colorspace"] },
        SummaryField { display_name: "Audio Channels", aliases: &["audio channels", "audiochannels", "channels", "channel count"] },
        SummaryField { display_name: "Audio Sample Rate", aliases: &["audio sample rate", "audiosamplerate", "sample rate", "samplerate"] },
        SummaryField { display_name: "Orientation", aliases: &["orientation"] },
        SummaryField { display_name: "Copyright", aliases: &["copyright", "copyright notice"] },
        SummaryField { display_name: "Artist / Creator", aliases: &["artist", "creator", "author"] },
        SummaryField { display_name: "Software Used", aliases: &["software", "processing software"] },
        SummaryField { display_name: "35mm Equivalent", aliases: &["focal length in 35mm format", "focallengthin35mmformat", "35mm focal length"] },
        SummaryField { display_name: "Camera Serial", aliases: &["serial number", "serialnumber", "camera serial number", "internal serial number"] },
        SummaryField { display_name: "Lens Serial", aliases: &["lens serial number", "lensserialnumber"] },
        SummaryField { display_name: "Firmware", aliases: &["firmware version", "firmware", "firmwareversion"] },
        SummaryField { display_name: "Focus Distance", aliases: &["focus distance", "focusdistance", "focus distance upper", "focus distance lower", "subject distance range"] },
        SummaryField { display_name: "Shutter Count", aliases: &["shutter count", "shuttercount", "image count", "actuations", "image number"] },
        SummaryField { display_name: "Date Modified", aliases: &["file modification date/time", "modify date", "modifydate"] },
        SummaryField { display_name: "Keywords", aliases: &["keywords", "subject"] },
        SummaryField { display_name: "Description", aliases: &["image description", "imagedescription", "description", "caption-abstract"] },
        SummaryField { display_name: "AF Points Used", aliases: &["focus points used", "af points in focus", "af point selected"] },
        SummaryField { display_name: "Digital Zoom", aliases: &["digital zoom ratio", "digitalzoomratio", "digital zoom"] },
        SummaryField { display_name: "Exposure Program", aliases: &["exposure program", "exposureprogram"] },
        SummaryField { display_name: "Light Source", aliases: &["light source", "lightsource"] },
        SummaryField { display_name: "Sensing Method", aliases: &["sensing method", "sensingmethod"] },
        SummaryField { display_name: "Exposure Mode", aliases: &["exposure mode", "exposuremode"] },
        SummaryField { display_name: "Scene Capture", aliases: &["scene capture type", "scenecapturetype"] },
        SummaryField { display_name: "Contrast", aliases: &["contrast"] },
        SummaryField { display_name: "Saturation", aliases: &["saturation"] },
        SummaryField { display_name: "Sharpness", aliases: &["sharpness"] },
        SummaryField { display_name: "Lens Info", aliases: &["lens info", "lensinfo"] },
        SummaryField { display_name: "GPS Altitude", aliases: &["gps altitude", "gpsaltitude"] },
        SummaryField { display_name: "GPS Direction", aliases: &["gps img direction", "gpsimgdirection", "gps dest bearing"] },
        SummaryField { display_name: "Compression", aliases: &["compression"] },
        SummaryField { display_name: "Rating", aliases: &["rating", "xmp:rating"] },
    ]
}

pub fn build_summary(entries: &[MetadataEntry]) -> Vec<MetadataEntry> {
    let fields = summary_fields();
    fields.iter().map(|field| {
        let found = entries.iter().find(|e| {
            let tag_low = e.tag.to_lowercase();
            field.aliases.iter().any(|a| tag_low == *a)
                && !e.value.is_empty()
                && e.value != "-"
                && e.value != "(none)"
                && e.value.to_lowercase() != "unknown"
        });
        MetadataEntry {
            group: found.map_or("Summary".into(), |e| e.group.clone()),
            tag: field.display_name.into(),
            value: found.map_or("\u{2014}".into(), |e| e.value.clone()),
        }
    }).collect()
}

pub fn filter_entries(
    all: &[MetadataEntry],
    summary: &[MetadataEntry],
    active_group: usize,
    groups: &[String],
    query: &str,
) -> Vec<MetadataEntry> {
    let q = query.to_lowercase();
    if active_group == 0 {
        return summary.iter().filter(|e| {
            q.is_empty() || e.tag.to_lowercase().contains(&q) || e.value.to_lowercase().contains(&q)
        }).cloned().collect();
    }
    let group_filter = if active_group >= 2 {
        groups.get(active_group - 2).map(|s| s.as_str())
    } else {
        None
    };
    all.iter().filter(|e| {
        if let Some(gf) = group_filter {
            if e.group != gf { return false; }
        }
        q.is_empty()
            || e.tag.to_lowercase().contains(&q)
            || e.value.to_lowercase().contains(&q)
            || e.group.to_lowercase().contains(&q)
    }).cloned().collect()
}

// Shutter count analysis
pub struct ShutterInfo {
    pub count: u64,
    pub rated_life: Option<u64>,
    pub health_pct: Option<f32>,
}

pub fn shutter_life_db() -> HashMap<&'static str, u64> {
    let mut m = HashMap::new();
    // Canon
    m.insert("canon eos-1d x mark iii", 500_000);
    m.insert("canon eos-1d x mark ii", 400_000);
    m.insert("canon eos 5d mark iv", 150_000);
    m.insert("canon eos 5d mark iii", 150_000);
    m.insert("canon eos 5ds", 150_000);
    m.insert("canon eos 6d mark ii", 100_000);
    m.insert("canon eos 6d", 100_000);
    m.insert("canon eos 7d mark ii", 200_000);
    m.insert("canon eos 90d", 120_000);
    m.insert("canon eos 80d", 100_000);
    m.insert("canon eos r5", 500_000);
    m.insert("canon eos r6", 300_000);
    m.insert("canon eos r3", 500_000);
    m.insert("canon eos r7", 200_000);
    // Nikon
    m.insert("nikon d6", 400_000);
    m.insert("nikon d5", 400_000);
    m.insert("nikon d850", 200_000);
    m.insert("nikon d810", 200_000);
    m.insert("nikon d780", 150_000);
    m.insert("nikon d750", 150_000);
    m.insert("nikon d500", 200_000);
    m.insert("nikon d7500", 150_000);
    m.insert("nikon z 9", 400_000);
    m.insert("nikon z 8", 400_000);
    m.insert("nikon z 6ii", 200_000);
    m.insert("nikon z 7ii", 200_000);
    // Sony
    m.insert("ilce-1", 500_000);
    m.insert("ilce-9m3", 500_000);
    m.insert("ilce-9m2", 500_000);
    m.insert("ilce-7rm5", 300_000);
    m.insert("ilce-7rm4", 300_000);
    m.insert("ilce-7rm3", 300_000);
    m.insert("ilce-7m4", 300_000);
    m.insert("ilce-7m3", 200_000);
    m.insert("ilce-7sm3", 300_000);
    m.insert("ilce-6700", 200_000);
    m.insert("ilce-6600", 200_000);
    m.insert("ilce-6400", 200_000);
    // Fujifilm
    m.insert("x-t5", 150_000);
    m.insert("x-t4", 150_000);
    m.insert("x-h2s", 150_000);
    m.insert("x-h2", 150_000);
    m
}

pub fn extract_shutter_info(entries: &[MetadataEntry], model: &str) -> Option<ShutterInfo> {
    let shutter_tags = ["shutter count", "shuttercount", "image count", "actuations", "image number", "mechanical shutter count"];
    let count_str = entries.iter().find_map(|e| {
        let tag_low = e.tag.to_lowercase();
        if shutter_tags.iter().any(|t| tag_low == *t) {
            let val = e.value.trim().replace(',', "");
            val.parse::<u64>().ok()
        } else {
            None
        }
    })?;
    let db = shutter_life_db();
    let model_low = model.to_lowercase();
    let rated = db.iter().find_map(|(k, v)| {
        if model_low.contains(k) { Some(*v) } else { None }
    });
    let health = rated.map(|r| (count_str as f32 / r as f32 * 100.0).min(100.0));
    Some(ShutterInfo { count: count_str, rated_life: rated, health_pct: health })
}

#[cfg(test)]
mod tests {
    use super::{build_summary, filter_entries, MetadataEntry};

    #[test]
    fn summary_prefers_matching_populated_fields() {
        let entries = vec![
            MetadataEntry {
                group: "EXIF".into(),
                tag: "Camera Model Name".into(),
                value: "ILCE-7RM5".into(),
            },
            MetadataEntry {
                group: "EXIF".into(),
                tag: "ISO".into(),
                value: "100".into(),
            },
            MetadataEntry {
                group: "Internet Data".into(),
                tag: "🌍 Exact Location".into(),
                value: "Test Street".into(),
            },
        ];

        let summary = build_summary(&entries);

        assert!(summary.iter().any(|e| e.tag == "Camera Model" && e.value == "ILCE-7RM5"));
        assert!(summary.iter().any(|e| e.tag == "ISO" && e.value == "100"));
        assert!(summary.iter().any(|e| e.tag == "Exact Location" && e.value == "Test Street"));
    }

    #[test]
    fn filter_entries_respects_group_and_query() {
        let all = vec![
            MetadataEntry {
                group: "EXIF".into(),
                tag: "ISO".into(),
                value: "100".into(),
            },
            MetadataEntry {
                group: "File".into(),
                tag: "File Size".into(),
                value: "10 MB".into(),
            },
        ];
        let summary = build_summary(&all);
        let groups = vec!["EXIF".into(), "File".into()];

        let exif_only = filter_entries(&all, &summary, 2, &groups, "");
        let queried = filter_entries(&all, &summary, 1, &groups, "10 mb");

        assert_eq!(exif_only.len(), 1);
        assert_eq!(exif_only[0].group, "EXIF");
        assert_eq!(queried.len(), 1);
        assert_eq!(queried[0].tag, "File Size");
    }
}
