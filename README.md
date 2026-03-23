# MetaLens — Deep Metadata Analyzer

<div align="center">
  <img src="assets/logo.png" width="96" height="96" alt="MetaLens Logo">
  <h3>The Forensic-Grade Metadata Explorer</h3>
</div>

---

**MetaLens** is a high-performance, premium utility designed for deep metadata extraction. It provides a surgical look into your photos, RAW files, videos, and documents, exposing thousands of hidden tags that standard file explorers ignore.

Built with **Rust** using the [egui](https://github.com/emilk/egui) framework, MetaLens blends hardware acceleration with a cinematic **"Dark Observatory"** aesthetic inspired by professional photography software.

---

## 🌌 The Dark Observatory Experience
- **Cinematic Palette**: A warm obsidian & amber glow theme designed for long-session analysis.
- **Hardware Accelerated**: Silky-smooth 144Hz+ rendering via `wgpu`.
- **Atmospheric UI**: Animated drop zones, shimmering load states, and precise typography.
- **Cross-Platform**: Designed for Windows (with deep shell integration), Linux, and macOS.

## 🚀 Key Features

### 📂 Shell Integration (Windows)
- **Smart Context Menu**: Right-click any supported media file to "Open in MetaLens".
- **Ultra-Selective**: The menu intelligently hides for unsupported file types, keeping your shell clean.
- **Zero-Launch Workflow**: Instantly analyze files without pre-opening the application.

### 🧠 Expert Metadata Support
- **400+ Formats**: Full support for JPG, PNG, TIFF, HEIC, and exotic RAW (CR3, ARW, DNG, RAF).
- **Pro Media Support**: Extract deep metadata from MP4, MOV, MKV, MP3, and PDF.
- **Proprietary MakerNotes**: Forensic-grade extraction for Sony, Canon, Nikon, Fuji, and more.

### 📸 Intelligent Diagnostics
- **Live Shutter Count**: Direct calculation of total shutter actuations (where supported).
- **Camera Health Gauge**: Compares current count against manufacturer-rated lifespan.
- **Smart Enrichment**:
  - 📍 **Reverse Geocoding**: Translates GPS into human-readable locations.
  - ☁️ **Weather Forensics**: Syncs capture time with historical weather data.

---

## 🛠 Installation

### Binary Release
Download the latest `MetaLens.exe` from the [Releases](https://github.com/Hamza-op/MetaLens/releases) page.
- **First Run**: Simply launch the app; it will automatically register the "Open in MetaLens" context menu.
- **Uninstall Menu**: Run `metaLens.exe --uninstall-context-menu` from PowerShell.

### Manual Build
Requires the **Rust** (2021 Edition) toolchain.

1. **Clone**: `git clone https://github.com/Hamza-op/MetaLens`
2. **Assets**: Ensure `assets/logo.png` and `assets/payload.zip` (containing `exiftool.exe`) are present.
3. **Build**:
   ```bash
   cargo build --release
   ```

---

## 🤝 Project Info

- **Core Engine**: Phil Harvey's [ExifTool](https://exiftool.org) (Securely Embedded)
- **GUI Engine**: `egui` / `eframe` (v0.31)
- **Data Enrichment**: Powered by OpenStreetMap and Open-Meteo.
- **Developed by**: [Hamza-op](https://github.com/Hamza-op)

---

### License
MIT License. ExifTool is licensed under Artistic License / GPL.
