#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod camera_db;
mod exiftool;
mod metadata;
mod net_enrich;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("MetaLens — Deep Metadata Analyzer")
            .with_inner_size([1280.0, 820.0])
            .with_min_inner_size([750.0, 500.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "MetaLens",
        options,
        Box::new(|cc| Ok(Box::new(app::MetaLensApp::new(cc)))),
    )
}
