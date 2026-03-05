use crate::metadata::MetadataEntry;
use serde::Deserialize;

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

pub fn fetch_internet_metadata(entries: &[MetadataEntry]) -> Vec<MetadataEntry> {
    let mut results = Vec::new();

    // Find GPS coordinates
    let mut lat = None;
    let mut lon = None;
    let mut date_str = None;

    for e in entries {
        let t = e.tag.to_lowercase();
        if t == "gps latitude" {
            // value is like "+38.123456"
            lat = e.value.trim().parse::<f64>().ok();
        } else if t == "gps longitude" {
            lon = e.value.trim().parse::<f64>().ok();
        } else if t == "date/time original" || t == "create date" || t == "datetimeoriginal" {
            date_str = Some(e.value.clone());
        }
    }

    let (Some(lat), Some(lon)) = (lat, lon) else {
        return results; // No GPS, no enrichment possible
    };

    // 1. Reverse Geocoding (Nominatim API - Free, OpenStreetMap)
    // Note: requires a custom User-Agent
    let nominatim_url = format!(
        "https://nominatim.openstreetmap.org/reverse?format=json&lat={}&lon={}&zoom=18&addressdetails=1",
        lat, lon
    );

    if let Ok(resp) = ureq::get(&nominatim_url)
        .set("User-Agent", "MetaLens-App/1.0 (Contact: metalens@example.com)")
        .call()
    {
        if let Ok(json) = resp.into_json::<NominatimResponse>() {
            if let Some(display) = json.display_name {
                results.push(MetadataEntry {
                    group: "Internet Data".into(),
                    tag: "🌍 Exact Location".into(),
                    value: display,
                });
            }
            if let Some(addr) = json.address {
                let mut components = Vec::new();
                if let Some(c) = addr.city.or(addr.town).or(addr.village) {
                    components.push(c);
                }
                if let Some(s) = addr.state {
                    components.push(s);
                }
                if let Some(c) = addr.country {
                    components.push(c);
                }
                if !components.is_empty() {
                    results.push(MetadataEntry {
                        group: "Internet Data".into(),
                        tag: "🏙️ City / Region".into(),
                        value: components.join(", "),
                    });
                }
            }
        }
    }

    // 2. Historical Weather data
    // Open-Meteo provides free historical weather without API keys!
    // Requires Date in format YYYY-MM-DD
    if let Some(date_exif) = date_str {
        // Exif Date format: "2018:05:27 08:03:00" -> extract "2018-05-27"
        if date_exif.len() >= 10 {
            let yyyy_mm_dd = date_exif[0..10].replace(':', "-");
            
            // Note: Open-Meteo historical needs start_date and end_date
            let meteo_url = format!(
                "https://archive-api.open-meteo.com/v1/archive?latitude={}&longitude={}&start_date={}&end_date={}&daily=temperature_2m_max,temperature_2m_min,weathercode&timezone=auto",
                lat, lon, yyyy_mm_dd, yyyy_mm_dd
            );

            #[derive(Deserialize)]
            struct Daily {
                temperature_2m_max: Vec<Option<f32>>,
                temperature_2m_min: Vec<Option<f32>>,
                weathercode: Vec<Option<u32>>,
            }
            #[derive(Deserialize)]
            struct MeteoHist {
                daily: Option<Daily>,
            }

            if let Ok(resp) = ureq::get(&meteo_url).call() {
                if let Ok(json) = resp.into_json::<MeteoHist>() {
                    if let Some(daily) = json.daily {
                        if !daily.temperature_2m_max.is_empty() {
                            let max_t = daily.temperature_2m_max[0];
                            let min_t = daily.temperature_2m_min[0];
                            let wc = daily.weathercode.get(0).copied().flatten();

                            if let (Some(max), Some(min)) = (max_t, min_t) {
                                results.push(MetadataEntry {
                                    group: "Internet Data".into(),
                                    tag: "🌡️ Historic Temperature".into(),
                                    value: format!("{:.1}°C / {:.1}°C (Max/Min)", max, min),
                                });
                            }

                            if let Some(code) = wc {
                                let weather_desc = match code {
                                    0 => "☀️ Clear sky",
                                    1|2|3 => "🌤️ Partly cloudy",
                                    45|48 => "🌫️ Fog",
                                    51..=55 => "🌧️ Drizzle",
                                    61..=65 => "🌧️ Rain",
                                    71..=75 => "❄️ Snow",
                                    95|96|99 => "⛈️ Thunderstorm",
                                    _ => "☁️ Overcast",
                                };
                                results.push(MetadataEntry {
                                    group: "Internet Data".into(),
                                    tag: "☁️ Historic Weather".into(),
                                    value: weather_desc.into(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort to make sure location is first
    results
}
