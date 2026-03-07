# MetaLens — Deep Metadata Analyzer

MetaLens is a high-performance, premium Windows utility designed for forensic-grade metadata extraction. It provides a deep dive into photos, RAW files, videos, and audio, exposing thousands of hidden tags that standard explorers miss.

Built with **Rust** using the [egui](https://github.com/emilk/egui) framework, MetaLens seamlessly wraps the industry-standard [ExifTool](https://exiftool.org) engine in a modern, hardware-accelerated interface.

![MetaLens UI](assets/logo.png)

## 🚀 Key Features

### 📂 Explorer Integration
- **Smart Context Menu**: Automatically registers an "**Open in MetaLens**" right-click option in Windows Explorer for all supported file types.
- **Strict Filtering**: The context menu only appears for supported media formats (Images, Video, RAW, etc.), keeping your Windows shell clean.
- **One-Click Launch**: Instantly analyze any file directly from your folder without opening the app first.

### 🧠 Forensic Metadata Engine
- **400+ Formats**: Full support for JPEG, PNG, TIFF, and exotic RAW formats (CR3, ARW, DNG), plus Video (MP4, MOV, MKV) and Audio metadata.
- **Proprietary MakerNotes**: Extracts deep manufacturer-specific data for Sony, Canon, Nikon, Fujifilm, and more.
- **Internet Enrichment**:
  - **Reverse Geocoding**: Automatically translates GPS coordinates into human-readable street addresses via OpenStreetMap (Nominatim).
  - **Historical Weather**: Fetches precise local weather conditions (temperature, sky state) at the exact time and location the file was captured via Open-Meteo.

### 📸 Pro Camera Diagnostics
- **Live Shutter Count**: Instantly calculates the total shutter actuations for supported DSLRs and Mirrorless cameras.
- **Health Visualization**: Compares your current shutter count against known manufacturer lifespan ratings with a dynamic health gauge.
- **Camera Database**: Identifies exact camera models and lens hardware from internal EXIF codes.

### 🎨 Premium User Experience
- **Modern Dark UI**: Hardware-accelerated, high-DPI interface with silky smooth transitions and professional aesthetics.
- **Instant Filtering**: Real-time fuzzy search through thousands of metadata tags.
- **Standalone/Embedded**: ExifTool is securely embedded inside the binary; the app is a zero-dependency, single-file executable for the end user.

---

## 🛠 Usage & Installation

### Context Menu Setup
MetaLens **automatically registers itself** on the first run. 
1. Launch `MetaLens.exe`.
2. The "**Open in MetaLens**" entry is now live in your Windows right-click menu.
3. To remove the context menu entry:
   ```powershell
   .\MetaLens.exe --uninstall-context-menu
   ```

### Command Line Interface
MetaLens supports direct file passing via CLI:
```powershell
.\MetaLens.exe "C:\Path\To\MyPhoto.jpg"
```

---

## 🏗 Technical Stack

- **Core**: Rust (2021 Edition)
- **GUI**: `egui` / `eframe` (v0.31)
- **Engine**: Phil Harvey's ExifTool (Embedded ZIP Payload)
- **Networking**: `ureq` (Synchronous HTTP)
- **Serialization**: `serde` / `serde_json`
- **Image Processing**: `image` (Icon and Logo rendering)
- **Registry**: Direct Windows Shell registration via `std::process::Command`

---

## 📦 Building from Source

Requires the latest **Rust** toolchain on Windows.

1. **Clone**: `git clone https://github.com/Hamza-op/MetaLens`
2. **Assets**: Ensure `assets/logo.png` and `assets/payload.zip` (containing `exiftool.exe`) are present.
3. **Compile**:
   ```powershell
   cargo build --release
   ```
4. **Output**: The binary will be at `.\target\release\MetaLens.exe`.

---

## 🤝 Credits

Developed with ❤️ and 🦀 Rust by **[Hamza-op](https://github.com/Hamza-op)**.

---

### License
MIT License. ExifTool is licensed under the Artistic License / GPL. Data enrichment powered by OpenStreetMap and Open-Meteo.
