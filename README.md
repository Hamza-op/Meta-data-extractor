# 🌌 MetaLens — Deep Metadata Analyzer

<div align="center">
  <img src="assets/logo.png" width="128" height="128" alt="MetaLens Logo">
  <br>
  <h1>The Forensic-Grade Metadata Explorer</h1>
  <p>
    <img src="https://img.shields.io/badge/Status-Experimental-amber.svg?style=flat-square" alt="Status">
    <img src="https://img.shields.io/badge/Platform-Windows%20|%20macOS%20|%20Linux-blueviolet.svg?style=flat-square" alt="Platforms">
    <img src="https://img.shields.io/badge/Built%20With-Rust-b7410e.svg?style=flat-square" alt="Rust">
  </p>
</div>

---

**MetaLens** is a high-performance, cinematic utility engineered for surgical metadata extraction. It exposes thousands of hidden tags in photos, RAW files, videos, and documents that traditional file explorers completely ignore.

Built with **Rust** using the [egui](https://github.com/emilk/egui) framework, MetaLens delivers a professional **"Dark Observatory"** experience — merging hardware-accelerated performance with a deep, atmospheric aesthetic.

---

## 🏛 The Dark Observatory Experience

Designed for the professional eye, MetaLens provides a workspace that feels like a high-end lens lab:

*   **Cinematic Palette**: A warm Obsidian & Amber Glow theme optimized for deep focus and long sessions.
*   **Silky-Smooth Performance**: Hardware-accelerated rendering at 144Hz+ via `wgpu`.
*   **Precise Typography**: Carefully weighted layouts for maximum information density without the clutter.
*   **Atmospheric Interaction**: Animated drop zones and shimmering load states create a living interface.

## 🛠 Pro-Grade Features

### 📂 Deep Shell Integration
*   **Smart Context Menu (Windows)**: Right-click any media or directory to "Open in MetaLens".
*   **Type-Aware Workflow**: The shell menu intelligently adapts to supported file types, keeping your workspace clean.

### 🧠 Expert Asset Analysis
*   **400+ Formats**: Exhaustive support for professional formats (JPG, PNG, TIFF, HEIC, AVIF).
*   **RAW Specialist**: Deep parsing of Sony (ARW), Canon (CR3), Nikon (NEF), Fuji (RAF), and DNG.
*   **Media Forensics**: Extract hidden streams from pro video (MOV, MKV, MP4) and audio (FLAC, WAV).

### 📸 Intelligent Diagnostics
*   **Live Shutter Count**: Precise extraction of shutter actuations (where supported by MakerNotes).
*   **Camera Health Gauge**: Immediate visual comparison of current usage against manufacturer lifespan.
*   **Forensic Enrichment**:
    *   📍 **Geo-Forensics**: Automated reverse geocoding to identify capture locations.
    *   ☁️ **Weather Reconstruction**: Historical weather data synced to the exact moment of capture.

---

## 🚀 Getting Started

### 📦 Binary Installation
Retrieve the latest forensic suite for your platform from the [Releases](https://github.com/Hamza-op/MetaLens/releases) page.

1.  **Windows**: Download `MetaLens-Windows.exe`. Launch it to automatically register the context menu.
2.  **macOS**: Download `MetaLens-macOS`. Grant execution permissions (`chmod +x`) if necessary.
3.  **Linux**: Download `MetaLens-Linux`. Requires `libgtk-3` and `libasound2`.

### 🛡 Manual Construction
Requires the latest **Rust** (2021 Edition) toolchain.

```bash
# 1. Acquire the source
git clone https://github.com/Hamza-op/MetaLens

# 2. Verify Assets
# Ensure assets/logo.png and assets/payload.zip are present

# 3. Forge the binary
cargo build --release
```

---

## 🏗 Engineering
*   **Core Engine**: Powered by Phil Harvey's peerless [ExifTool](https://exiftool.org).
*   **GUI Infrastructure**: `egui` / `eframe` (v0.31).
*   **Data Intelligence**: Enhanced via OpenStreetMap and Open-Meteo.
*   **Author**: [Hamza-op](https://github.com/Hamza-op)

---

### License
Released under the **MIT License**.
*ExifTool is licensed under the Artistic License / GPL.*

