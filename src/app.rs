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
                    
                    // Attempt Internet Metadata Enrichment (Reverse Geocoding & Weather)
                    let internet_entries = crate::net_enrich::fetch_internet_metadata(&parsed.entries);
                    if !internet_entries.is_empty() {
                        parsed.entries.extend(internet_entries);
                        if !parsed.groups.contains(&"Internet Data".to_string()) {
                            parsed.groups.insert(1, "Internet Data".into()); // Right after Camera Info
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

        // Colors
        let bg_dark = egui::Color32::from_rgb(10, 10, 14);
        let bg_panel = egui::Color32::from_rgb(22, 22, 28);
        let bg_card = egui::Color32::from_rgb(30, 30, 38);
        let bg_input = egui::Color32::from_rgb(38, 38, 48);
        let accent = egui::Color32::from_rgb(99, 102, 241);
        let accent_hover = egui::Color32::from_rgb(129, 132, 255);
        let text_primary = egui::Color32::from_rgb(240, 240, 245);
        let text_secondary = egui::Color32::from_rgb(148, 148, 165);
        let text_muted = egui::Color32::from_rgb(82, 82, 100);
        let cyan = egui::Color32::from_rgb(34, 211, 238);
        let green = egui::Color32::from_rgb(52, 211, 153);
        let orange = egui::Color32::from_rgb(251, 146, 60);
        let red = egui::Color32::from_rgb(248, 113, 113);
        let purple = egui::Color32::from_rgb(167, 139, 250);
        let border = egui::Color32::from_rgb(45, 45, 58);
        
        // ═══ CUSTOM VISUALS ═══
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = bg_dark;
        visuals.extreme_bg_color = bg_panel; // Scrollbar track matches panel
        visuals.widgets.noninteractive.bg_fill = bg_panel;
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgba_unmultiplied(100, 100, 100, 40); // Subtle handle
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_unmultiplied(120, 120, 120, 80);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgba_unmultiplied(140, 140, 140, 120);
        ctx.set_visuals(visuals);

        // ═══ LEFT SIDEBAR ═══
        egui::SidePanel::left("sidebar")
            .frame(egui::Frame::new().fill(bg_panel).inner_margin(20.0).outer_margin(0.0).stroke(egui::Stroke::NONE))
            .exact_width(260.0)
            .show(ctx, |ui| {
                ui.add_space(12.0);
                
                // Logo display
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
                        ui.add(egui::Image::new(texture).max_width(64.0).max_height(64.0));
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new("MetaLens").size(24.0).strong().color(text_primary));
                        ui.label(egui::RichText::new("Deep Metadata").size(11.0).color(text_muted));
                    });
                }

                ui.add_space(32.0);
                
                let btn = |ui: &mut egui::Ui, label: &str, primary: bool, width: f32| -> bool {
                    let (bg, hover_bg, text_col) = if primary {
                        (accent, accent_hover, egui::Color32::WHITE)
                    } else {
                        (bg_card, egui::Color32::from_rgb(50, 50, 62), text_secondary)
                    };
                    let btn = egui::Button::new(egui::RichText::new(label).color(text_col).size(14.0))
                        .fill(bg)
                        .corner_radius(8.0)
                        .stroke(egui::Stroke::NONE)
                        .min_size(egui::vec2(width, 38.0));
                    let resp = ui.add(btn);
                    if resp.hovered() {
                        ui.painter().rect_filled(resp.rect, 8.0, hover_bg.gamma_multiply(0.3));
                    }
                    resp.clicked()
                };

                if btn(ui, "\u{1F4C2} Open File", true, ui.available_width()) {
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
                    if btn(ui, "\u{1F4BE} Export", false, half_w) { self.export(); }
                    if btn(ui, "\u{1F4CB} Copy", false, half_w) { self.copy_all(); }
                });
                ui.add_space(8.0);
                if btn(ui, "\u{1F5D1} Clear", false, ui.available_width()) { self.clear(); }
                
                ui.add_space(32.0);
                
                if self.file_loaded {
                    // Search bar
                    ui.label(egui::RichText::new("Search Fields").size(12.0).color(text_muted).strong());
                    ui.add_space(6.0);
                    let search_frame = egui::Frame::new()
                        .fill(bg_input)
                        .corner_radius(8.0)
                        .stroke(egui::Stroke::NONE)
                        .inner_margin(egui::Margin::symmetric(12, 12));

                    search_frame.show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("\u{1F50D}").size(14.0).color(text_muted));
                            let resp = ui.add(
                                egui::TextEdit::singleline(&mut self.search_query)
                                    .hint_text("Tags & values...")
                                    .text_color(text_primary)
                                    .frame(false)
                                    .desired_width(ui.available_width()),
                            );
                            if resp.changed() {
                                self.apply_filter();
                            }
                        });
                    });
                    
                    ui.add_space(24.0);
                    
                    if let Some(ref info) = self.shutter_info {
                        let card = egui::Frame::new()
                            .fill(bg_card)
                            .corner_radius(12.0)
                            .stroke(egui::Stroke::NONE)
                            .inner_margin(egui::Margin::symmetric(16, 16));
                        card.show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new("\u{1F4F7} Shutter Count").size(13.0).color(text_secondary));
                                ui.add_space(4.0);
                                ui.label(egui::RichText::new(format!("{}", format_number(info.count))).size(32.0).color(text_primary).strong());
                                if !self.camera_model.is_empty() {
                                    ui.add_space(2.0);
                                    ui.label(egui::RichText::new(&self.camera_model).size(11.0).color(text_muted));
                                }
                                
                                if let (Some(rated), Some(pct)) = (info.rated_life, info.health_pct) {
                                    ui.add_space(16.0);
                                    ui.label(egui::RichText::new("Shutter Life Used").size(12.0).color(text_secondary));
                                    ui.add_space(6.0);
                                    
                                    let bar_height = 10.0;
                                    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), bar_height), egui::Sense::hover());
                                    let painter = ui.painter();
                                    painter.rect_filled(rect, 5.0, egui::Color32::from_rgb(20, 20, 28));
                                    
                                    let fill_w = rect.width() * (pct / 100.0).min(1.0);
                                    let fill_color = if pct < 30.0 { green }
                                        else if pct < 60.0 { egui::Color32::from_rgb(250, 204, 21) }
                                        else if pct < 80.0 { orange }
                                        else { red };
                                        
                                    let fill_rect = egui::Rect::from_min_size(rect.min, egui::vec2(fill_w, bar_height));
                                    painter.rect_filled(fill_rect, 5.0, fill_color);
                                    painter.rect_stroke(rect, 5.0, egui::Stroke::new(1.0, border), egui::StrokeKind::Outside);
                                    
                                    ui.add_space(6.0);
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(format!("{:.1}%", pct)).size(14.0).color(fill_color).strong());
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            ui.label(egui::RichText::new(format!("of {}", format_number(rated))).size(11.0).color(text_muted));
                                        });
                                    });
                                }
                            });
                        });
                    }
                }
                
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(8.0);
                    // GitHub link
                    ui.hyperlink_to(
                        egui::RichText::new("github.com/Hamza-op").size(10.0).color(text_muted),
                        "https://github.com/Hamza-op",
                    );
                    ui.add_space(2.0);
                    ui.label(egui::RichText::new("Made with \u{1F980} Rust").size(10.0).color(text_muted));
                    ui.add_space(6.0);
                    let exif_status = if self.exiftool_path.is_some() {
                        egui::RichText::new("\u{2705} ExifTool Ready").size(11.0).color(green)
                    } else {
                        egui::RichText::new("\u{26A0} ExifTool Missing").size(11.0).color(orange)
                    };
                    ui.label(exif_status);
                });
            });

        // ═══ MAIN AREA ═══
        let frame_bg = egui::Frame::new()
            .fill(bg_dark)
            .inner_margin(egui::Margin::same(0));

        egui::CentralPanel::default().frame(frame_bg).show(ctx, |ui| {
            ui.set_min_size(ui.available_size());
            ui.add_space(24.0);

            if !self.file_loaded {
                // Drop zone
                ui.vertical_centered(|ui| {
                    ui.add_space(ui.available_height() / 3.0);
                    let pulse = ((self.anim_time * 1.5).sin() * 0.15 + 0.85) as f32;
                    ui.label(egui::RichText::new("\u{1F4C2}").size(64.0).color(text_muted.gamma_multiply(pulse)));
                    ui.add_space(16.0);
                    ui.label(egui::RichText::new("Drop a file here to analyze").size(22.0).color(text_secondary));
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new("Photos, RAW (CR2, NEF, ARW), Videos (MP4, MOV), and 400+ formats")
                        .size(13.0).color(text_muted));
                });
                ctx.request_repaint_after(std::time::Duration::from_millis(50));
                return; // skip the rest
            }

            // Tabs with arrow navigation
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                
                // Left arrow
                let left_btn = egui::Button::new(egui::RichText::new("◀").size(14.0).color(text_secondary))
                    .fill(bg_card)
                    .corner_radius(6.0)
                    .stroke(egui::Stroke::new(1.0, border))
                    .min_size(egui::vec2(28.0, 30.0));
                if ui.add(left_btn).clicked() {
                    self.tab_scroll_offset = (self.tab_scroll_offset - 150.0).max(0.0);
                }

                let tab_frame = egui::Frame::new()
                    .fill(bg_panel)
                    .corner_radius(10.0)
                    .inner_margin(egui::Margin::symmetric(4, 4));
                tab_frame.show(ui, |ui| {
                    ui.set_width(ui.available_width() - 60.0);
                    let scroll_area = egui::ScrollArea::horizontal()
                        .auto_shrink([false; 2])
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .horizontal_scroll_offset(self.tab_scroll_offset);
                    
                    let output = scroll_area.show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let mut tabs: Vec<String> = vec!["\u{2B50} Summary".into(), "All Fields".into()];
                            tabs.extend(self.groups.iter().cloned());

                            for (i, label) in tabs.iter().enumerate() {
                                let selected = self.active_tab == i;
                                let text_col = if selected { egui::Color32::WHITE } else { text_secondary };
                                let bg = if selected { accent } else { egui::Color32::TRANSPARENT };
                                let btn = egui::Button::new(egui::RichText::new(label).size(13.0).color(text_col))
                                    .fill(bg)
                                    .corner_radius(7.0)
                                    .stroke(egui::Stroke::NONE)
                                    .min_size(egui::vec2(0.0, 30.0));
                                if ui.add(btn).clicked() {
                                    self.active_tab = i;
                                    self.apply_filter();
                                }
                            }
                        });
                    });
                    // Sync offset back
                    self.tab_scroll_offset = output.state.offset.x;
                });

                // Right arrow
                let right_btn = egui::Button::new(egui::RichText::new("▶").size(14.0).color(text_secondary))
                    .fill(bg_card)
                    .corner_radius(6.0)
                    .stroke(egui::Stroke::new(1.0, border))
                    .min_size(egui::vec2(28.0, 30.0));
                if ui.add(right_btn).clicked() {
                    self.tab_scroll_offset += 150.0;
                }

                ui.add_space(16.0);
            });
            ui.add_space(16.0);

            // ═══ CONTENT AREA ═══
            let content_frame = egui::Frame::new()
                .fill(bg_panel)
                .corner_radius(12.0)
                .stroke(egui::Stroke::NONE)
                .inner_margin(egui::Margin::same(0))
                .outer_margin(egui::Margin::symmetric(16, 0));

            content_frame.show(ui, |ui| {
                let is_summary = self.active_tab == 0;

                egui::ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    egui::Grid::new("metadata_grid")
                        .num_columns(if is_summary { 2 } else { 3 })
                        .spacing([0.0, 0.0])
                        .striped(true)
                        .min_col_width(100.0)
                        .show(ui, |ui| {
                            // Header
                            let hdr_frame = |ui: &mut egui::Ui, text: &str, width: f32| {
                                let layout = egui::Layout::left_to_right(egui::Align::Center);
                                ui.allocate_ui_with_layout(egui::vec2(width, 36.0), layout, |ui| {
                                    ui.add_space(16.0);
                                    ui.label(egui::RichText::new(text).size(12.0).color(text_muted).strong());
                                });
                            };

                            if is_summary {
                                hdr_frame(ui, "FIELD", 220.0);
                                hdr_frame(ui, "VALUE", 500.0);
                            } else {
                                hdr_frame(ui, "GROUP", 160.0);
                                hdr_frame(ui, "TAG", 240.0);
                                hdr_frame(ui, "VALUE", 500.0);
                            }
                            ui.end_row();

                            // Rows
                            for entry in &self.filtered_entries {
                                let is_missing = entry.value == "\u{2014}";
                                let is_camera = entry.tag.contains("Identified Camera");
                                let is_lens = entry.tag.contains("Identified Lens") || (entry.group == "Camera Info" && entry.tag.contains("Lens"));

                                if is_summary {
                                    // Field column
                                    let field_col = if is_missing { text_muted } else { cyan };
                                    let cell = |ui: &mut egui::Ui, text: &str, color: egui::Color32, width: f32| {
                                        ui.allocate_ui_with_layout(egui::vec2(width, 32.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                            ui.add_space(16.0);
                                            ui.label(egui::RichText::new(text).size(13.0).color(color));
                                        });
                                    };
                                    cell(ui, &entry.tag, field_col, 220.0);
                                    let val_col = if is_missing { egui::Color32::from_rgb(45, 45, 65) } else { text_primary };
                                    cell(ui, &entry.value, val_col, 500.0);
                                } else {
                                    let cell = |ui: &mut egui::Ui, text: &str, color: egui::Color32, width: f32| {
                                        ui.allocate_ui_with_layout(egui::vec2(width, 32.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                            ui.add_space(16.0);
                                            ui.label(egui::RichText::new(text).size(13.0).color(color));
                                        });
                                    };
                                    let grp_col = if is_camera { green } else if is_lens { orange } else { accent };
                                    let tag_col = if is_camera { green } else if is_lens { orange } else { cyan };
                                    let val_col = if is_camera { green } else if is_lens { orange } else { text_primary };
                                    cell(ui, &entry.group, grp_col, 160.0);
                                    cell(ui, &entry.tag, tag_col, 240.0);
                                    cell(ui, &entry.value, val_col, 500.0);
                                }
                                ui.end_row();
                            }
                        });
                });
            });

            ui.add_space(8.0);
            
            // Status bar (File info + loaded fields)
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                let status_frame = egui::Frame::new()
                    .fill(bg_card)
                    .corner_radius(8.0)
                    .inner_margin(egui::Margin::symmetric(12, 6));
                status_frame.show(ui, |ui| {
                    ui.set_width(ui.available_width() - 32.0);
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(&self.status_msg).size(11.0).color(text_secondary));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if !self.current_file.is_empty() {
                                let fname = std::path::Path::new(&self.current_file)
                                    .file_name()
                                    .map(|f| f.to_string_lossy().to_string())
                                    .unwrap_or_default();
                                ui.label(egui::RichText::new(format!("\u{1F4C4} {}", fname)).size(11.0).color(purple));
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
