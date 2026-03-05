#![windows_subsystem = "windows"]

use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

mod admin;
mod cleanup;
mod idm;
mod optimize;
mod startup;

/// Toggle this to `false` for production silent mode.
const DEBUG: bool = true;

pub fn debug_print(msg: &str) {
    if DEBUG {
        println!("  {}", msg);
    }
}

// ─────────────────────────────────────────────────────────────
// Phase Tracking
// ─────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
enum PhaseStatus {
    Pending,
    Running,
    Done,
}

#[derive(Clone)]
struct Phase {
    icon: &'static str,
    name: &'static str,
    status: PhaseStatus,
}

struct TaskState {
    pub is_done: bool,
    pub phases: Vec<Phase>,
    pub active_phase: usize,
}

impl TaskState {
    fn new() -> Self {
        Self {
            is_done: false,
            active_phase: 0,
            phases: vec![
                Phase { icon: "🔑", name: "IDM Activation Reset", status: PhaseStatus::Pending },
                Phase { icon: "🧹", name: "Temporary File Cleanup", status: PhaseStatus::Pending },
                Phase { icon: "🎮", name: "Gaming Optimizations", status: PhaseStatus::Pending },
                Phase { icon: "🎨", name: "Adobe Optimization", status: PhaseStatus::Pending },
                Phase { icon: "🛡", name: "System & Privacy", status: PhaseStatus::Pending },
            ],
        }
    }

    fn start_phase(&mut self, idx: usize) {
        if idx < self.phases.len() {
            self.active_phase = idx;
            self.phases[idx].status = PhaseStatus::Running;
        }
    }

    fn complete_phase(&mut self, idx: usize) {
        if idx < self.phases.len() {
            self.phases[idx].status = PhaseStatus::Done;
        }
    }

    fn progress(&self) -> f32 {
        let done = self.phases.iter().filter(|p| p.status == PhaseStatus::Done).count();
        done as f32 / self.phases.len() as f32
    }
}

// ─────────────────────────────────────────────────────────────
// Color Palette
// ─────────────────────────────────────────────────────────────

const BG_DARK: egui::Color32 = egui::Color32::from_rgb(13, 13, 26);
const BG_CARD: egui::Color32 = egui::Color32::from_rgb(22, 22, 40);
const BG_CARD_ACTIVE: egui::Color32 = egui::Color32::from_rgb(30, 28, 55);
const ACCENT_CYAN: egui::Color32 = egui::Color32::from_rgb(0, 212, 255);
const ACCENT_PURPLE: egui::Color32 = egui::Color32::from_rgb(124, 58, 237);
const TEXT_PRIMARY: egui::Color32 = egui::Color32::from_rgb(240, 240, 255);
const TEXT_DIM: egui::Color32 = egui::Color32::from_rgb(130, 130, 170);
const TEXT_SUCCESS: egui::Color32 = egui::Color32::from_rgb(52, 211, 153);
const PROGRESS_BG: egui::Color32 = egui::Color32::from_rgb(35, 35, 60);

// ─────────────────────────────────────────────────────────────
// GUI App
// ─────────────────────────────────────────────────────────────

struct MaintenanceApp {
    start_time: Instant,
    state: Arc<Mutex<TaskState>>,
}

impl eframe::App for MaintenanceApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let state = self.state.lock().unwrap();
        let is_done = state.is_done;
        let progress = state.progress();
        let phases = state.phases.clone();
        drop(state);

        if is_done {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        let elapsed = self.start_time.elapsed().as_secs();
        let anim_t = ctx.input(|i| i.time) as f32;

        // Full dark background
        let panel_frame = egui::Frame::none()
            .fill(BG_DARK)
            .inner_margin(egui::Margin::same(24.0));

        egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            ui.style_mut().visuals.override_text_color = Some(TEXT_PRIMARY);

            // ── Header ──
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);

                // Animated title with gradient feel via alternating color
                let pulse = ((anim_t * 1.5).sin() * 0.5 + 0.5).clamp(0.0, 1.0);
                let title_color = lerp_color(ACCENT_CYAN, ACCENT_PURPLE, pulse);

                ui.label(
                    egui::RichText::new("⚡ SYSTEM OPTIMIZER")
                        .size(26.0)
                        .strong()
                        .color(title_color),
                );
                ui.add_space(2.0);
                ui.label(
                    egui::RichText::new("IDM Fix  ·  Gaming  ·  Privacy")
                        .size(12.0)
                        .color(TEXT_DIM),
                );
            });

            ui.add_space(16.0);

            // ── Username greeting badge ──
            let username = std::env::var("USERNAME").unwrap_or_else(|_| "User".to_string());
            ui.horizontal(|ui| {
                ui.add_space(4.0);
                let badge_frame = egui::Frame::none()
                    .fill(egui::Color32::from_rgb(28, 28, 50))
                    .rounding(egui::Rounding::same(12.0))
                    .inner_margin(egui::Margin::symmetric(12.0, 5.0));
                badge_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(format!("👤 {}", username))
                            .size(13.0)
                            .color(TEXT_DIM),
                    );
                });

                // Elapsed time badge
                let time_frame = egui::Frame::none()
                    .fill(egui::Color32::from_rgb(28, 28, 50))
                    .rounding(egui::Rounding::same(12.0))
                    .inner_margin(egui::Margin::symmetric(12.0, 5.0));
                time_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(format!("⏱ {}s", elapsed))
                            .size(13.0)
                            .color(egui::Color32::from_rgb(250, 204, 21)),
                    );
                });
            });

            ui.add_space(14.0);

            // ── Progress Bar ──
            let progress_rect = ui.available_rect_before_wrap();
            let bar_height = 8.0;
            let bar_rect = egui::Rect::from_min_size(
                egui::pos2(progress_rect.min.x, progress_rect.min.y),
                egui::vec2(progress_rect.width(), bar_height),
            );

            // Background track
            ui.painter().rect_filled(
                bar_rect,
                egui::Rounding::same(4.0),
                PROGRESS_BG,
            );

            // Filled portion with gradient shimmer
            let fill_width = bar_rect.width() * progress;
            if fill_width > 0.0 {
                let fill_rect = egui::Rect::from_min_size(
                    bar_rect.min,
                    egui::vec2(fill_width, bar_height),
                );
                let shimmer = ((anim_t * 2.0).sin() * 0.3 + 0.7).clamp(0.4, 1.0);
                let bar_color = lerp_color(ACCENT_CYAN, ACCENT_PURPLE, shimmer);
                ui.painter().rect_filled(fill_rect, egui::Rounding::same(4.0), bar_color);
            }

            ui.allocate_space(egui::vec2(0.0, bar_height + 4.0));

            // Percentage label
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(format!("{}%", (progress * 100.0) as u32))
                        .size(13.0)
                        .color(TEXT_DIM),
                );
            });

            ui.add_space(10.0);

            // ── Phase Cards ──
            phases.iter().enumerate().for_each(|(i, phase)| {
                let (bg, text_col, status_icon) = match phase.status {
                    PhaseStatus::Done => (BG_CARD, TEXT_SUCCESS, "✓"),
                    PhaseStatus::Running => (BG_CARD_ACTIVE, ACCENT_CYAN, "⟳"),
                    PhaseStatus::Pending => (BG_CARD, TEXT_DIM, "·"),
                };

                let card_frame = egui::Frame::none()
                    .fill(bg)
                    .rounding(egui::Rounding::same(8.0))
                    .inner_margin(egui::Margin::symmetric(14.0, 8.0));

                // Active glow border
                let card_frame = if phase.status == PhaseStatus::Running {
                    card_frame.stroke(egui::Stroke::new(1.0, ACCENT_CYAN))
                } else {
                    card_frame.stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 40, 65)))
                };

                card_frame.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        // Status icon
                        ui.label(
                            egui::RichText::new(status_icon)
                                .size(16.0)
                                .color(text_col),
                        );
                        ui.add_space(6.0);
                        // Phase icon
                        ui.label(egui::RichText::new(phase.icon).size(16.0));
                        ui.add_space(4.0);
                        // Phase name
                        let name_size = if phase.status == PhaseStatus::Running { 15.0 } else { 14.0 };
                        let mut name_text = egui::RichText::new(phase.name).size(name_size).color(text_col);
                        if phase.status == PhaseStatus::Running {
                            name_text = name_text.strong();
                        }
                        ui.label(name_text);

                        // Step number on the right
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(format!("{}/{}", i + 1, phases.len()))
                                    .size(12.0)
                                    .color(egui::Color32::from_rgb(70, 70, 100)),
                            );
                        });
                    });
                });
                ui.add_space(4.0);
            });

            ui.add_space(8.0);

            // ── Footer ──
            ui.vertical_centered(|ui| {
                let dot_pulse = ((anim_t * 3.0).sin() * 0.5 + 0.5).clamp(0.0, 1.0);
                let dot_alpha = (dot_pulse * 255.0) as u8;
                let footer_color = egui::Color32::from_rgba_premultiplied(
                    TEXT_DIM.r(),
                    TEXT_DIM.g(),
                    TEXT_DIM.b(),
                    (128 + (dot_alpha / 2)) .min(255),
                );
                ui.label(
                    egui::RichText::new("Running as Administrator")
                        .size(11.0)
                        .color(footer_color),
                );
            });
        });

        ctx.request_repaint();
    }
}

fn lerp_color(a: egui::Color32, b: egui::Color32, t: f32) -> egui::Color32 {
    let inv = 1.0 - t;
    egui::Color32::from_rgb(
        (a.r() as f32 * inv + b.r() as f32 * t) as u8,
        (a.g() as f32 * inv + b.g() as f32 * t) as u8,
        (a.b() as f32 * inv + b.b() as f32 * t) as u8,
    )
}

// ─────────────────────────────────────────────────────────────
// Main Execution
// ─────────────────────────────────────────────────────────────

fn main() -> Result<(), eframe::Error> {
    if !admin::is_admin() {
        if admin::elevate_self() {
            std::process::exit(0);
        } else {
            std::process::exit(1);
        }
    }

    startup::ensure_startup_registered();

    let state = Arc::new(Mutex::new(TaskState::new()));
    let state_clone = state.clone();

    thread::spawn(move || {
        let run_phase = |idx: usize, work: fn()| {
            state_clone.lock().unwrap().start_phase(idx);
            work();
            thread::sleep(Duration::from_millis(400));
            state_clone.lock().unwrap().complete_phase(idx);
        };

        run_phase(0, || {
            idm::reset_activation();
            idm::fix_popup();
        });

        run_phase(1, || {
            cleanup::clean_temp_files();
        });

        run_phase(2, || {
            optimize::optimize_for_gaming();
        });

        run_phase(3, || {
            optimize::optimize_for_adobe();
        });

        run_phase(4, || {
            optimize::optimize_system_and_privacy();
        });

        // Show completed state for a few seconds
        thread::sleep(Duration::from_secs(3));
        state_clone.lock().unwrap().is_done = true;
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 520.0])
            .with_resizable(false)
            .with_always_on_top()
            .with_title("System Optimizer"),
        ..Default::default()
    };

    eframe::run_native(
        "System Optimizer",
        options,
        Box::new(|_cc| {
            Box::new(MaintenanceApp {
                start_time: Instant::now(),
                state,
            })
        }),
    )
}
