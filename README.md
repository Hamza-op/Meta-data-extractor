# MetaLens — Deep Metadata Analyzer

A lightweight, powerful Windows utility that performs deep metadata analysis on photos, RAW files, and videos. Built in **Rust** using the [egui](https://github.com/emilk/egui) framework, wrapping the industry-standard [ExifTool](https://exiftool.org) by Phil Harvey.

![Windows](https://img.shields.io/badge/platform-Windows-blue) ![Rust](https://img.shields.io/badge/language-Rust-orange) ![License](https://img.shields.io/badge/license-MIT-green)

## ✨ New in this Version

- **Total Rust Rewrite**: Ported from legacy C++ to a modern, memory-safe Rust codebase.
- **Dynamic UI**: A beautiful, hardware-accelerated dark theme built with `egui`, featuring a persistent left dashboard/sidebar.
- **Internet Enrichment**: Automatically fetches exact street addresses/cities (via OpenStreetMap) and historical local weather conditions (via Open-Meteo) based on GPS and timestamps!
- **Shutter Health Analysis**: Instantly calculates and displays your camera's exact shutter count and visualizes its health against expected lifespan ratings.

## Features

- **Standalone Executable**: ExifTool is securely embedded directly inside the `.exe` as a compressed ZIP payload. It automatically extracts to a temporary folder gracefully under the hood. No extra files required!
- **Curated Summary View**: Immediately see the most important fields (Camera, Lens, Exposure, GPS, Codec, Internet Data) in a clean layout instead of raw ExifTool terminal output.
- **Deep Extraction**: Extracts ALL metadata including proprietary MakerNotes, unknown tags, and duplicates (while intelligently filtering out useless binary data).
- **Camera Configuration Database**: Translates internal EXIF model codes (like `ILCE-7RM5`) to their real-world names and estimates shutter lifespans.
- **Live Search/Filter**: Instantly filter through 1000+ metadata fields in milliseconds.
- **Export & Copy**: Export to text file or copy all data to clipboard.

## Supported Formats

| Type | Formats |
|------|---------|
| **Photos** | JPEG, PNG, TIFF, WebP, HEIC, AVIF, BMP, GIF |
| **RAW** | CR2, CR3, NEF, NRW, ARW, SRF, SR2, ORF, RW2, RAF, DNG, PEF, 3FR, IIQ, X3F |
| **Video** | MP4, MOV, AVI, MKV, WMV, FLV, WebM, M4V, 3GP, MTS, M2TS |
| **Audio** | MP3, WAV, FLAC, AAC, OGG, WMA, M4A |

## Building from Source

You will need the standard Rust toolchain installed.

1. Clone the repository.
2. Ensure you have the ExifTool payload: `assets/payload.zip` must exist securely packaging the `exiftool.exe` binary.
3. Build the project:
   ```bash
   cargo build --release
   ```
4. The executable will be found in `target/release/MetaLens.exe`. 

*Note: Running `cargo run` will also work for development testing.*

## Usage

1. **Launch** `MetaLens.exe`
2. **Drop** any photo/video file anywhere onto the window, or click **Open File**.
3. **Review Dashboard** — The left sidebar will immediately display summary info and your camera's total Shutter Count health gauge (if supported).
4. **Summary Tab** — Check the curated highlight of critical metadata, including automatically fetched GPS Address and Weather History.
5. **Deep Dive** — Click the "All Fields" tab or specific Metadata Groups (EXIF, MakerNotes, XMP) to see everything ExifTool found. 
6. **Search** — Type in the left sidebar's search box to instantly find specific tags.

## Architecture & Crates

- `eframe` / `egui` — GUI framework.
- `zip` — In-memory extraction of the embedded ExifTool executable.
- `ureq` / `serde_json` — Blocking HTTP requests and parsing for Nominatim (Geocoding) and Open-Meteo (Weather).
- `rfd` — Native file dialogs.
- `arboard` — Clipboard operations.

## License

MIT License. ExifTool is separately licensed under the Artistic License / GPL. Data fetched via OpenStreetMap and Open-Meteo are subject to their respective free-tier usage policies.
