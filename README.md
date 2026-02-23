# MetaLens — Deep Metadata Analyzer

A lightweight Windows utility that performs deep metadata analysis on photos, RAW files, and videos. Built as a native Win32 application wrapping [ExifTool](https://exiftool.org) by Phil Harvey.

![Windows](https://img.shields.io/badge/platform-Windows-blue) ![C++17](https://img.shields.io/badge/language-C%2B%2B17-orange) ![License](https://img.shields.io/badge/license-MIT-green)

## Features

- **Standalone Executable**: ExifTool is now embedded directly inside the exe — no extra files needed!
- **Curated Summary View**: Immediately see the most important 50+ fields (Camera, Lens, Exposure, GPS, Codec) in a clean, prioritized layout instead of raw ExifTool output
- **Missing Data Indicator**: Missing summary fields are gracefully muted (`—`) so you can instantly see what data a photo does and doesn't have
- **Deep Extraction**: Extracts ALL metadata including proprietary MakerNotes, unknown tags, and duplicates
- **Camera Name Resolution**: Translates internal EXIF model codes (like `ILCE-7RM5`, `FC3582`, `SM-S928B`) to their real-world names
- **Lens Identification**: Resolves over 150+ cryptic lens IDs to their actual retail names
- **Drag & Drop**: Just drop a file onto the window
- **200+ Camera Models**: Built-in database covering Canon, Nikon, Sony, Fujifilm, Panasonic, DJI, GoPro, Apple, Samsung, Google, Leica, Hasselblad, and more
- **Live Search/Filter**: Instantly filter through 1000+ metadata fields
- **Grouped View**: Tab-based group filtering (Summary, All Fields, EXIF, MakerNotes, XMP, IPTC, Video tracks, etc.)
- **Dark Themed UI**: Modern dark interface with color-coded fields and dynamic 2-column/3-column layouts
- **Export & Copy**: Export to text file or copy all data to clipboard
- **Zero Dependencies**: Pure Win32 API C++ — no .NET, no Electron, no bloat

## Supported Formats

| Type | Formats |
|------|---------|
| **Photos** | JPEG, PNG, TIFF, WebP, HEIC, AVIF, BMP, GIF |
| **RAW** | CR2, CR3, NEF, NRW, ARW, SRF, SR2, ORF, RW2, RAF, DNG, PEF, 3FR, IIQ, X3F |
| **Video** | MP4, MOV, AVI, MKV, WMV, FLV, WebM, M4V, 3GP, MTS, M2TS |
| **Sidecars**| XML, XMP (Perfect for Sony video workflows) |
| **Audio** | MP3, WAV, FLAC, AAC, OGG, WMA, M4A |
| **Other** | PDF, PSD, AI, EPS, SVG |

## Requirements

1. **Windows 10/11**
2. **Setup**: None! Download the standalone release and run it. 
   *(If building from source, you'll need Visual Studio 2019+ with C++ Desktop workload)*

## Building

### Option 1: Direct Compile (Easiest)
```batch
compile.bat
```
This uses `cl.exe` directly — no CMake needed.

### Option 2: CMake Build
```batch
build.bat
```

### Option 3: Manual
```batch
:: Open a "Developer Command Prompt for VS 2022" and run:
cd MetaLens
mkdir bin
rc /nologo /fo bin\resource.res src\resource.rc
cl /nologo /std:c++17 /O2 /EHsc /DUNICODE /D_UNICODE /Isrc src\main.cpp bin\resource.res /Fe:bin\MetaLens.exe /link /SUBSYSTEM:WINDOWS comctl32.lib uxtheme.lib dwmapi.lib shlwapi.lib gdi32.lib user32.lib shell32.lib ole32.lib comdlg32.lib advapi32.lib
```

## Usage

1. **Launch** `MetaLens.exe`
2. **Drop** any photo/video/xml file onto the window, or click **Open File**
3. **Review Summary** — The "⭐ Summary" tab immediately shows you the most important curated fields in a clean 2-column layout.
4. **Deep Dive** — Click the "All Fields" tab to see everything ExifTool found. 
5. **Search** — type in the filter bar to find specific tags (model, lens, ISO, shutter, etc.)
6. **Browse groups** — click tabs to view by category (EXIF, MakerNotes, XMP, etc.)
7. **Double-click** any row to copy that single field to clipboard
8. **Export** — save all metadata to a text file
9. **Copy All** — copy visible (filtered) metadata to clipboard

## Camera Model Resolution

MetaLens includes a built-in database that translates cryptic EXIF model strings to recognizable names:

| EXIF Model String | MetaLens Shows |
|---|---|
| `ILCE-7RM5` | Sony Alpha 7R V (A7R V) |
| `FC3582` | DJI Mini 4 Pro |
| `SM-S928B` | Samsung Galaxy S25 Ultra |
| `Canon EOS 2000D` | Canon EOS 2000D / Rebel T7 / Kiss X90 |
| `X-T5` | Fujifilm X-T5 |
| `DC-GH6` | Panasonic Lumix GH6 |

The resolved name appears as **"📷 Identified Camera"** at the top of the metadata list, highlighted in green.

## Architecture

```
MetaLens/
├── CMakeLists.txt          # CMake build config
├── compile.bat             # Direct MSVC compile script (Resource embedder)
├── .github/workflows/      # CI/CD CD pipeline (Auto-embeds ExifTool)
├── README.md
└── src/
    ├── main.cpp            # Win32 GUI application (~1600 lines)
    ├── camera_db.h         # Camera & Lens model db (350+ models)
    ├── resource.h          # Resource IDs
    └── resource.rc         # App manifest, ExifTool embedder
```

- **No custom parsers** — all metadata extraction delegated to ExifTool
- **ExifTool flags used**: `-All -G:1 -a -u -f -E` for maximum extraction with grouping
- **Single-threaded UI** with subprocess execution for ExifTool

## License

MIT License. ExifTool is separately licensed under the Artistic License / GPL.
