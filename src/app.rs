use crate::exiftool;
use crate::metadata::{self, MetadataEntry, ShutterInfo};
use eframe::egui;
use std::path::PathBuf;
use std::sync::mpsc;

struct LoadResult {
    entries: Vec<MetadataEntry>,
    summary: Vec<MetadataEntry>,
    groups: Vec<String>,
    shutter: Option<ShutterInfo>,
    model: String,
    file_path: String,
}

pub struct MetaLensApp {
    exiftool_path: Option<PathBuf>,
    all_entries: Vec<MetadataEntry>,
    filtered_entries: Vec<MetadataEntry>,
    summary_entries: Vec<MetadataEntry>,
    groups: Vec<String>,
    active_tab: usize,
    search_query: String,
    file_loaded: bool,
    current_file: String,
    shutter_info: Option<ShutterInfo>,
    camera_model: String,
    loading: bool,
    rx: Option<mpsc::Receiver<LoadResult>>,
    anim_time: f64,
    status_msg: String,
    pending_file: Option<String>,
    logo_texture: Option<egui::TextureHandle>,
    tab_scroll_offset: f32,
}

impl MetaLensApp {
    pub fn new(cc: &eframe::CreationContext, initial_file: Option<String>) -> Self {
        let ctx = &cc.egui_ctx;
        ctx.set_visuals(egui::Visuals::dark());

        Self {
            exiftool_path: exiftool::find_exiftool(),
            all_entries: Vec::new(),
            filtered_entries: Vec::new(),
            summary_entries: Vec::new(),
            groups: Vec::new(),
            active_tab: 0,
            search_query: String::new(),
            file_loaded: false,
            current_file: String::new(),
            shutter_info: None,
            camera_model: String::new(),
            loading: false,
            rx: None,
            anim_time: 0.0,
            status_msg: "Ready — Drop a file or click Open".into(),
            pending_file: initial_file,
            logo_texture: None,
            tab_scroll_offset: 0.0,
        }
    }

    fn load_file(&mut self, path: String, ctx: egui::Context) {
        let Some(exiftool) = self.exiftool_path.clone() else {
            self.status_msg = "\u{26A0} ExifTool not found! Place exiftool.exe next to MetaLens.exe or add to PATH.".into();
            return;
        };
        self.loading = true;
        self.status_msg = "\u{23F3} Analyzing file...".into();
        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);

        std::thread::spawn(move || {
            let file_path_buf = PathBuf::from(&path);
            match exiftool::run_exiftool(&exiftool, &file_path_buf) {
                Ok(output) => {
                    let mut parsed = exiftool::parse_output(&output);
                    
                    let internet_entries = crate::net_enrich::fetch_internet_metadata(&parsed.entries);
                    if !internet_entries.is_empty() {
                        parsed.entries.extend(internet_entries);
                        if !parsed.groups.contains(&"Internet Data".to_string()) {
                            parsed.groups.insert(1, "Internet Data".into());
                        }
                    }

                    let summary = metadata::build_summary(&parsed.entries);
                    let shutter = metadata::extract_shutter_info(&parsed.entries, &parsed.found_model);
                    let _ = tx.send(LoadResult {
                        entries: parsed.entries,
                        summary,
                        groups: parsed.groups,
                        shutter,
                        model: parsed.found_model,
                        file_path: path,
                    });
                }
                Err(e) => {
                    let _ = tx.send(LoadResult {
                        entries: vec![MetadataEntry { group: "Error".into(), tag: "Error".into(), value: e }],
                        summary: Vec::new(),
                        groups: Vec::new(),
                        shutter: None,
                        model: String::new(),
                        file_path: path,
                    });
                }
            }
            ctx.request_repaint();
        });
    }

    fn apply_filter(&mut self) {
        self.filtered_entries = metadata::filter_entries(
            &self.all_entries,
            &self.summary_entries,
            self.active_tab,
            &self.groups,
            &self.search_query,
        );
    }

    fn clear(&mut self) {
        self.all_entries.clear();
        self.filtered_entries.clear();
        self.summary_entries.clear();
        self.groups.clear();
        self.active_tab = 0;
        self.search_query.clear();
        self.file_loaded = false;
        self.current_file.clear();
        self.shutter_info = None;
        self.camera_model.clear();
        self.status_msg = "Ready — Drop a file or click Open".into();
    }

    fn export(&self) {
        if self.all_entries.is_empty() { return; }
        if let Some(path) = rfd::FileDialog::new()
            .set_file_name("metadata_export.txt")
            .add_filter("Text File", &["txt"])
            .save_file()
        {
            let mut text = format!("MetaLens Metadata Export\n========================\nFile: {}\nTotal Fields: {}\n\n", self.current_file, self.all_entries.len());
            let mut last_group = String::new();
            for e in &self.all_entries {
                if e.group != last_group {
                    text += &format!("\n--- {} ---\n", e.group);
                    last_group = e.group.clone();
                }
                text += &format!("  {} : {}\n", e.tag, e.value);
            }
            let _ = std::fs::write(path, &text);
        }
    }

    fn copy_all(&self) {
        if self.filtered_entries.is_empty() { return; }
        let mut text = format!("MetaLens Metadata — {}\n", self.current_file);
        text += "═══════════════════════════════════════\n\n";
        let mut last_group = String::new();
        for e in &self.filtered_entries {
            if e.group != last_group {
                text += &format!("\n── {} ──\n", e.group);
                last_group = e.group.clone();
            }
            text += &format!("  {}  :  {}\n", e.tag, e.value);
        }
        if let Ok(mut clip) = arboard::Clipboard::new() {
            let _ = clip.set_text(&text);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// HELPERS — color interpolation & number formatting
// ═══════════════════════════════════════════════════════════════════

fn lerp_color(a: egui::Color32, b: egui::Color32, t: f32) -> egui::Color32 {
    let t = t.clamp(0.0, 1.0);
    egui::Color32::from_rgb(
        (a.r() as f32 + (b.r() as f32 - a.r() as f32) * t) as u8,
        (a.g() as f32 + (b.g() as f32 - a.g() as f32) * t) as u8,
        (a.b() as f32 + (b.b() as f32 - a.b() as f32) * t) as u8,
    )
}

fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, ch);
    }
    result
}

impl eframe::App for MetaLensApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.anim_time = ctx.input(|i| i.time);

        // Auto-load file passed via CLI argument (context menu)
        if let Some(path) = self.pending_file.take() {
            self.load_file(path, ctx.clone());
        }

        // Check for async load result
        if let Some(rx) = &self.rx {
            if let Ok(result) = rx.try_recv() {
                self.all_entries = result.entries;
                self.summary_entries = result.summary;
                self.groups = result.groups;
                self.shutter_info = result.shutter;
                self.camera_model = result.model;
                self.current_file = result.file_path;
                self.file_loaded = true;
                self.loading = false;
                self.active_tab = 0;
                self.search_query.clear();
                self.apply_filter();
                let found = self.summary_entries.iter().filter(|e| e.value != "\u{2014}").count();
                self.status_msg = format!("\u{2B50} Summary — {} / {} fields  |  {} total tags", found, self.summary_entries.len(), self.all_entries.len());
                self.rx = None;
            }
        }

        // Handle dropped files
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                if let Some(p) = i.raw.dropped_files[0].path.as_ref() {
                    let path_str = p.display().to_string();
                    return Some(path_str);
                }
            }
            None
        }).map(|p| self.load_file(p, ctx.clone()));

        // ═══════════════════════════════════════════════════════════
        // "DARK OBSERVATORY" PALETTE — Warm Obsidian + Amber Glow
        // ═══════════════════════════════════════════════════════════
        let bg_abyss       = egui::Color32::from_rgb(8, 8, 12);       // Deepest background
        let bg_panel       = egui::Color32::from_rgb(16, 15, 20);     // Sidebar / panels
        let bg_card        = egui::Color32::from_rgb(24, 23, 30);     // Cards / elevated surfaces
        let bg_input       = egui::Color32::from_rgb(32, 30, 40);     // Input fields

        // Amber / Gold accent — the hero color
        let amber          = egui::Color32::from_rgb(232, 168, 56);   // Primary warm gold
        let amber_bright   = egui::Color32::from_rgb(250, 196, 80);   // Hover / highlights
        let amber_dim      = egui::Color32::from_rgb(180, 120, 30);   // Muted gold

        // Supporting accents
        let teal           = egui::Color32::from_rgb(72, 202, 190);   // Field names / links
        let sage           = egui::Color32::from_rgb(120, 200, 140);  // Success / health good
        let copper         = egui::Color32::from_rgb(220, 140, 60);   // Warning / lens info
        let coral          = egui::Color32::from_rgb(235, 100, 90);   // Error / danger
        let mauve          = egui::Color32::from_rgb(180, 150, 210);  // File name / purple accent

        // Text hierarchy
        let text_cream     = egui::Color32::from_rgb(235, 230, 220);  // Primary text (warm white)
        let text_silver    = egui::Color32::from_rgb(160, 155, 165);  // Secondary labels
        let text_graphite  = egui::Color32::from_rgb(85, 80, 95);     // Muted / disabled

        // Borders & dividers
        let border_subtle  = egui::Color32::from_rgb(40, 38, 50);     // Card borders
        let border_warm    = egui::Color32::from_rgb(55, 48, 38);     // Warm-tinted separator

        // ═══ CUSTOM VISUALS ═══
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = bg_abyss;
        visuals.extreme_bg_color = bg_panel;
        visuals.widgets.noninteractive.bg_fill = bg_panel;
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgba_unmultiplied(90, 80, 60, 40);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_unmultiplied(110, 95, 60, 80);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgba_unmultiplied(140, 120, 70, 120);
        visuals.selection.bg_fill = amber.gamma_multiply(0.25);
        visuals.selection.stroke = egui::Stroke::new(1.0, amber);
        ctx.set_visuals(visuals);

        // ═══════════════════════════════════════════════════════
        // LEFT SIDEBAR
        // ═══════════════════════════════════════════════════════
        egui::SidePanel::left("sidebar")
            .frame(
                egui::Frame::new()
                    .fill(bg_panel)
                    .inner_margin(22.0)
                    .outer_margin(0.0)
                    .stroke(egui::Stroke::new(1.0, border_subtle)),
            )
            .exact_width(270.0)
            .show(ctx, |ui| {
                ui.add_space(14.0);

                // ── Logo ──
                if self.logo_texture.is_none() {
                    let logo_bytes = include_bytes!("../assets/logo.png");
                    let image_data = image::load_from_memory(logo_bytes).expect("Failed to load embedded logo");
                    let rgba = image_data.to_rgba8();
                    let (width, height) = rgba.dimensions();
                    let color_image = egui::ColorImage::from_rgba_unmultiplied(
                        [width as usize, height as usize],
                        &rgba,
                    );
                    self.logo_texture = Some(ctx.load_texture("app_logo", color_image, Default::default()));
                }

                if let Some(texture) = &self.logo_texture {
                    ui.vertical_centered(|ui| {
                        ui.add(egui::Image::new(texture).max_width(56.0).max_height(56.0));
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new("MetaLens").size(26.0).strong().color(text_cream));
                        ui.add_space(2.0);
                        ui.label(egui::RichText::new("Deep Metadata Analyzer").size(11.0).color(text_graphite));
                    });
                }

                ui.add_space(28.0);

                // ── Warm divider ──
                {
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), egui::Sense::hover());
                    let painter = ui.painter();
                    let mid = rect.center().x;
                    let half = rect.width() * 0.4;
                    // gradient fade from transparent → warm border → transparent
                    for i in 0..=20 {
                        let t = i as f32 / 20.0;
                        let dist = (t - 0.5).abs() * 2.0; // 0 at center, 1 at edges
                        let alpha = (1.0 - dist * dist) * 0.6;
                        let x = mid - half + half * 2.0 * t;
                        painter.line_segment(
                            [egui::pos2(x, rect.min.y), egui::pos2(x + half * 2.0 / 20.0, rect.min.y)],
                            egui::Stroke::new(1.0, amber_dim.gamma_multiply(alpha)),
                        );
                    }
                }
                ui.add_space(24.0);

                // ── Action buttons ──
                let sidebar_btn = |ui: &mut egui::Ui, label: &str, primary: bool, width: f32| -> bool {
                    let (bg, text_col, stroke_col) = if primary {
                        (amber, egui::Color32::from_rgb(20, 15, 5), amber)
                    } else {
                        (bg_card, text_silver, border_subtle)
                    };
                    let btn = egui::Button::new(egui::RichText::new(label).color(text_col).size(14.0).strong())
                        .fill(bg)
                        .corner_radius(10.0)
                        .stroke(egui::Stroke::new(1.0, stroke_col.gamma_multiply(0.5)))
                        .min_size(egui::vec2(width, 40.0));
                    let resp = ui.add(btn);
                    if resp.hovered() && primary {
                        ui.painter().rect_filled(resp.rect, 10.0, amber_bright.gamma_multiply(0.15));
                    } else if resp.hovered() {
                        ui.painter().rect_filled(resp.rect, 10.0, amber_dim.gamma_multiply(0.08));
                    }
                    resp.clicked()
                };

                if sidebar_btn(ui, "\u{1F4C2}  Open File", true, ui.available_width()) {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("All Supported", &[
                            "jpg","jpeg","png","tiff","tif","bmp","gif","webp","heic","heif","avif",
                            "cr2","cr3","nef","nrw","arw","srf","sr2","orf","rw2","raf","dng","pef","3fr","iiq","x3f",
                            "mp4","mov","avi","mkv","wmv","flv","webm","m4v","3gp","mts","m2ts",
                            "mp3","wav","flac","aac","ogg","wma","m4a",
                            "pdf","psd","ai","eps","svg","xml","xmp",
                        ])
                        .add_filter("Photos", &["jpg","jpeg","png","tiff","tif","bmp","gif","webp","heic","heif","avif"])
                        .add_filter("RAW Files", &["cr2","cr3","nef","nrw","arw","srf","sr2","orf","rw2","raf","dng","pef","3fr","iiq","x3f"])
                        .add_filter("Video", &["mp4","mov","avi","mkv","wmv","flv","webm","m4v","3gp","mts","m2ts"])
                        .add_filter("All Files", &["*"])
                        .pick_file()
                    {
                        self.load_file(path.display().to_string(), ctx.clone());
                    }
                }
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    let half_w = (ui.available_width() - 8.0) / 2.0;
                    if sidebar_btn(ui, "\u{1F4BE} Export", false, half_w) { self.export(); }
                    if sidebar_btn(ui, "\u{1F4CB} Copy", false, half_w) { self.copy_all(); }
                });
                ui.add_space(8.0);
                if sidebar_btn(ui, "\u{1F5D1}  Clear", false, ui.available_width()) { self.clear(); }

                ui.add_space(28.0);

                if self.file_loaded {
                    // ── Search bar ──
                    ui.label(egui::RichText::new("SEARCH FIELDS").size(10.0).color(text_graphite).strong());
                    ui.add_space(8.0);

                    let search_frame = egui::Frame::new()
                        .fill(bg_input)
                        .corner_radius(10.0)
                        .stroke(egui::Stroke::new(1.0, border_subtle))
                        .inner_margin(egui::Margin::symmetric(14, 12));

                    search_frame.show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("\u{1F50D}").size(14.0).color(amber_dim));
                            let resp = ui.add(
                                egui::TextEdit::singleline(&mut self.search_query)
                                    .hint_text("Filter tags & values…")
                                    .text_color(text_cream)
                                    .frame(false)
                                    .desired_width(ui.available_width()),
                            );
                            if resp.changed() {
                                self.apply_filter();
                            }
                        });
                    });

                    ui.add_space(20.0);

                    // ── Shutter count card ──
                    if let Some(ref info) = self.shutter_info {
                        let card = egui::Frame::new()
                            .fill(bg_card)
                            .corner_radius(14.0)
                            .stroke(egui::Stroke::new(1.0, border_warm))
                            .inner_margin(egui::Margin::symmetric(18, 18));
                        card.show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new("\u{1F4F7}  SHUTTER COUNT").size(11.0).color(text_graphite).strong());
                                ui.add_space(6.0);
                                ui.label(
                                    egui::RichText::new(format_number(info.count))
                                        .size(34.0)
                                        .color(amber)
                                        .strong(),
                                );
                                if !self.camera_model.is_empty() {
                                    ui.add_space(2.0);
                                    ui.label(egui::RichText::new(&self.camera_model).size(11.0).color(text_graphite));
                                }

                                if let (Some(rated), Some(pct)) = (info.rated_life, info.health_pct) {
                                    ui.add_space(18.0);
                                    ui.label(egui::RichText::new("SHUTTER LIFE USED").size(10.0).color(text_graphite).strong());
                                    ui.add_space(8.0);

                                    // Gradient progress bar
                                    let bar_height = 12.0;
                                    let (rect, _) = ui.allocate_exact_size(
                                        egui::vec2(ui.available_width(), bar_height),
                                        egui::Sense::hover(),
                                    );
                                    let painter = ui.painter();

                                    // Track
                                    painter.rect_filled(rect, 6.0, egui::Color32::from_rgb(14, 13, 18));
                                    painter.rect_stroke(rect, 6.0, egui::Stroke::new(1.0, border_subtle), egui::StrokeKind::Outside);

                                    // Fill with gradient
                                    let fill_frac = (pct / 100.0).min(1.0);
                                    let fill_w = rect.width() * fill_frac;
                                    let fill_start = if pct < 30.0 { sage }
                                        else if pct < 60.0 { egui::Color32::from_rgb(240, 200, 50) }
                                        else if pct < 80.0 { copper }
                                        else { coral };
                                    let fill_end = if pct < 30.0 { egui::Color32::from_rgb(80, 220, 120) }
                                        else if pct < 60.0 { amber }
                                        else if pct < 80.0 { egui::Color32::from_rgb(240, 120, 40) }
                                        else { egui::Color32::from_rgb(255, 60, 60) };

                                    // Draw gradient bar in segments
                                    let segments = (fill_w as i32).max(1);
                                    for seg in 0..segments {
                                        let t = seg as f32 / segments as f32;
                                        let c = lerp_color(fill_start, fill_end, t);
                                        let x = rect.min.x + seg as f32;
                                        let seg_rect = egui::Rect::from_min_size(
                                            egui::pos2(x, rect.min.y + 0.5),
                                            egui::vec2(1.5, bar_height - 1.0),
                                        );
                                        painter.rect_filled(seg_rect, 0.0, c);
                                    }

                                    ui.add_space(8.0);
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            egui::RichText::new(format!("{:.1}%", pct))
                                                .size(16.0)
                                                .color(fill_start)
                                                .strong(),
                                        );
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            ui.label(
                                                egui::RichText::new(format!("of {}", format_number(rated)))
                                                    .size(11.0)
                                                    .color(text_graphite),
                                            );
                                        });
                                    });
                                }
                            });
                        });
                    }
                }

                // ── Footer ──
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(8.0);
                    ui.hyperlink_to(
                        egui::RichText::new("github.com/Hamza-op").size(10.0).color(text_graphite),
                        "https://github.com/Hamza-op",
                    );
                    ui.add_space(2.0);
                    ui.label(egui::RichText::new("Made with \u{1F980} Rust").size(10.0).color(text_graphite));
                    ui.add_space(6.0);
                    let exif_status = if self.exiftool_path.is_some() {
                        egui::RichText::new("\u{2705} ExifTool Ready").size(11.0).color(sage)
                    } else {
                        egui::RichText::new("\u{26A0} ExifTool Missing").size(11.0).color(copper)
                    };
                    ui.label(exif_status);
                });
            });

        // ═══════════════════════════════════════════════════════
        // MAIN CONTENT AREA
        // ═══════════════════════════════════════════════════════
        let frame_bg = egui::Frame::new()
            .fill(bg_abyss)
            .inner_margin(egui::Margin::same(0));

        egui::CentralPanel::default().frame(frame_bg).show(ctx, |ui| {
            ui.set_min_size(ui.available_size());
            ui.add_space(24.0);

            // ─────────────────────────────────────
            // EMPTY STATE — Animated drop zone
            // ─────────────────────────────────────
            if !self.file_loaded {
                ui.vertical_centered(|ui| {
                    let avail = ui.available_size();
                    let center_y = avail.y / 2.5;
                    ui.add_space(center_y - 80.0);

                    let t = self.anim_time;
                    let icon_pulse = ((t * 1.2).sin() * 0.2 + 0.8) as f32;

                    // Animated concentric rings (painter-only block)
                    {
                        let painter = ui.painter();
                        let center = ui.cursor().min + egui::vec2(avail.x / 2.0, 0.0);
                        for ring in 0..3 {
                            let phase = t * 0.8 + ring as f64 * 2.1;
                            let pulse = (phase.sin() * 0.5 + 0.5) as f32;
                            let radius = 50.0 + ring as f32 * 30.0 + pulse * 10.0;
                            let alpha = (0.12 - ring as f32 * 0.03) * (0.6 + pulse * 0.4);
                            painter.circle_stroke(
                                center,
                                radius,
                                egui::Stroke::new(1.5, amber.gamma_multiply(alpha)),
                            );
                        }
                        // Amber glow behind icon
                        painter.circle_filled(center, 28.0, amber.gamma_multiply(0.06 * icon_pulse));
                    }

                    ui.label(
                        egui::RichText::new("\u{1F4C2}")
                            .size(56.0)
                            .color(amber.gamma_multiply(icon_pulse)),
                    );
                    ui.add_space(20.0);
                    ui.label(
                        egui::RichText::new("Drop a file here to analyze")
                            .size(22.0)
                            .color(text_cream),
                    );
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("Photos · RAW · Videos · Audio · Documents — 400+ formats")
                            .size(12.0)
                            .color(text_graphite),
                    );

                    // Loading shimmer
                    if self.loading {
                        ui.add_space(30.0);
                        let shimmer_w = 200.0;
                        let (bar_rect, _) = ui.allocate_exact_size(egui::vec2(shimmer_w, 3.0), egui::Sense::hover());
                        {
                            let painter = ui.painter();
                            let pos = ((t * 1.5).sin() * 0.5 + 0.5) as f32;
                            let glow_x = bar_rect.min.x + bar_rect.width() * pos;
                            for i in 0..=(shimmer_w as i32) {
                                let x = bar_rect.min.x + i as f32;
                                let dist = (x - glow_x).abs() / 40.0;
                                let alpha = (-dist * dist).exp();
                                painter.line_segment(
                                    [egui::pos2(x, bar_rect.min.y), egui::pos2(x, bar_rect.max.y)],
                                    egui::Stroke::new(1.0, amber.gamma_multiply(alpha * 0.8)),
                                );
                            }
                        }
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new("Analyzing…").size(12.0).color(amber_dim));
                    }
                });
                ctx.request_repaint_after(std::time::Duration::from_millis(50));
                return;
            }

            // ─────────────────────────────────────
            // TABS — Pill-style navigation
            // ─────────────────────────────────────
            ui.horizontal(|ui| {
                ui.add_space(16.0);

                // Arrow left
                let arrow_btn = |ui: &mut egui::Ui, icon: &str| -> bool {
                    let btn = egui::Button::new(egui::RichText::new(icon).size(13.0).color(text_silver))
                        .fill(bg_card)
                        .corner_radius(8.0)
                        .stroke(egui::Stroke::new(1.0, border_subtle))
                        .min_size(egui::vec2(30.0, 32.0));
                    ui.add(btn).clicked()
                };

                if arrow_btn(ui, "◀") {
                    self.tab_scroll_offset = (self.tab_scroll_offset - 150.0).max(0.0);
                }

                let tab_frame = egui::Frame::new()
                    .fill(bg_card)
                    .corner_radius(12.0)
                    .stroke(egui::Stroke::new(1.0, border_subtle))
                    .inner_margin(egui::Margin::symmetric(5, 5));

                tab_frame.show(ui, |ui| {
                    ui.set_width(ui.available_width() - 60.0);
                    let scroll_area = egui::ScrollArea::horizontal()
                        .auto_shrink([false; 2])
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .horizontal_scroll_offset(self.tab_scroll_offset);

                    let output = scroll_area.show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 4.0;
                            let mut tabs: Vec<String> = vec!["\u{2B50} Summary".into(), "All Fields".into()];
                            tabs.extend(self.groups.iter().cloned());

                            for (i, label) in tabs.iter().enumerate() {
                                let selected = self.active_tab == i;
                                let (text_col, bg, stroke) = if selected {
                                    (egui::Color32::from_rgb(18, 12, 4), amber, amber.gamma_multiply(0.6))
                                } else {
                                    (text_silver, egui::Color32::TRANSPARENT, egui::Color32::TRANSPARENT)
                                };
                                let btn = egui::Button::new(
                                    egui::RichText::new(label).size(12.5).color(text_col).strong(),
                                )
                                .fill(bg)
                                .corner_radius(8.0)
                                .stroke(egui::Stroke::new(if selected { 1.0 } else { 0.0 }, stroke))
                                .min_size(egui::vec2(0.0, 28.0));

                                let resp = ui.add(btn);
                                if !selected && resp.hovered() {
                                    ui.painter().rect_filled(resp.rect, 8.0, amber.gamma_multiply(0.06));
                                }
                                if resp.clicked() {
                                    self.active_tab = i;
                                    self.apply_filter();
                                }
                            }
                        });
                    });
                    self.tab_scroll_offset = output.state.offset.x;
                });

                if arrow_btn(ui, "▶") {
                    self.tab_scroll_offset += 150.0;
                }

                ui.add_space(16.0);
            });
            ui.add_space(14.0);

            // ─────────────────────────────────────
            // CONTENT TABLE
            // ─────────────────────────────────────
            let content_frame = egui::Frame::new()
                .fill(bg_panel)
                .corner_radius(14.0)
                .stroke(egui::Stroke::new(1.0, border_subtle))
                .inner_margin(egui::Margin::same(0))
                .outer_margin(egui::Margin::symmetric(16, 0));

            content_frame.show(ui, |ui| {
                let is_summary = self.active_tab == 0;

                egui::ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                    ui.set_min_width(ui.available_width());

                    egui::Grid::new("metadata_grid")
                        .num_columns(if is_summary { 2 } else { 3 })
                        .spacing([0.0, 0.0])
                        .striped(false)
                        .min_col_width(100.0)
                        .show(ui, |ui| {
                            // ── Header row ──
                            let hdr = |ui: &mut egui::Ui, text: &str, width: f32| {
                                let layout = egui::Layout::left_to_right(egui::Align::Center);
                                ui.allocate_ui_with_layout(egui::vec2(width, 38.0), layout, |ui| {
                                    ui.add_space(18.0);
                                    ui.label(
                                        egui::RichText::new(text)
                                            .size(10.0)
                                            .color(amber_dim)
                                            .strong(),
                                    );
                                });
                            };
                            if is_summary {
                                hdr(ui, "FIELD", 240.0);
                                hdr(ui, "VALUE", 500.0);
                            } else {
                                hdr(ui, "GROUP", 180.0);
                                hdr(ui, "TAG", 260.0);
                                hdr(ui, "VALUE", 500.0);
                            }
                            ui.end_row();

                            // ── Data rows ──
                            for (_row_idx, entry) in self.filtered_entries.iter().enumerate() {
                                let is_missing = entry.value == "\u{2014}";
                                let is_camera = entry.tag.contains("Identified Camera");
                                let is_lens = entry.tag.contains("Identified Lens")
                                    || (entry.group == "Camera Info" && entry.tag.contains("Lens"));

                                let cell = |ui: &mut egui::Ui, text: &str, color: egui::Color32, width: f32, bold: bool| {
                                    let layout = egui::Layout::left_to_right(egui::Align::Center);
                                    ui.allocate_ui_with_layout(egui::vec2(width, 34.0), layout, |ui| {
                                        ui.add_space(18.0);
                                        let mut rt = egui::RichText::new(text).size(13.0).color(color);
                                        if bold { rt = rt.strong(); }
                                        ui.label(rt);
                                    });
                                };

                                if is_summary {
                                    let field_col = if is_missing { text_graphite } else { teal };
                                    cell(ui, &entry.tag, field_col, 240.0, false);
                                    let val_col = if is_missing {
                                        egui::Color32::from_rgb(40, 38, 55)
                                    } else {
                                        text_cream
                                    };
                                    cell(ui, &entry.value, val_col, 500.0, false);
                                } else {
                                    let (grp_col, tag_col, val_col) = if is_camera {
                                        (sage, sage, sage)
                                    } else if is_lens {
                                        (copper, copper, copper)
                                    } else {
                                        (amber_dim, teal, text_cream)
                                    };
                                    cell(ui, &entry.group, grp_col, 180.0, is_camera || is_lens);
                                    cell(ui, &entry.tag, tag_col, 260.0, is_camera || is_lens);
                                    cell(ui, &entry.value, val_col, 500.0, is_camera || is_lens);
                                }
                                ui.end_row();
                            }
                        });
                });
            });

            ui.add_space(8.0);

            // ─────────────────────────────────────
            // STATUS BAR
            // ─────────────────────────────────────
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                let status_frame = egui::Frame::new()
                    .fill(bg_card)
                    .corner_radius(10.0)
                    .stroke(egui::Stroke::new(1.0, border_subtle))
                    .inner_margin(egui::Margin::symmetric(14, 7));
                status_frame.show(ui, |ui| {
                    ui.set_width(ui.available_width() - 32.0);
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(&self.status_msg).size(11.0).color(text_silver));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if !self.current_file.is_empty() {
                                let fname = std::path::Path::new(&self.current_file)
                                    .file_name()
                                    .map(|f| f.to_string_lossy().to_string())
                                    .unwrap_or_default();
                                ui.label(
                                    egui::RichText::new(format!("\u{1F4C4} {}", fname))
                                        .size(11.0)
                                        .color(mauve),
                                );
                            }
                        });
                    });
                });
                ui.add_space(16.0);
            });
            ui.add_space(8.0);
        });
    }
}
