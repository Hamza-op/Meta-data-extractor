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
// GUI State
// ─────────────────────────────────────────────────────────────

struct TaskState {
    pub is_done: bool,
    pub current_phase: String,
}

struct MaintenanceApp {
    start_time: Instant,
    state: Arc<Mutex<TaskState>>,
}

impl eframe::App for MaintenanceApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (is_done, phase) = {
            let s = self.state.lock().unwrap();
            (s.is_done, s.current_phase.clone())
        };

        if is_done {
            // Close the window gracefully
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        let elapsed = self.start_time.elapsed().as_secs();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.heading(egui::RichText::new("System Optimizer & IDM Fix").size(24.0).strong());
                ui.add_space(20.0);

                let username = std::env::var("USERNAME").unwrap_or_else(|_| "User".to_string());

                ui.label(
                    egui::RichText::new(format!(
                        "Hey @{}, please wait for the optimizations to finish.",
                        username
                    ))
                    .size(16.0),
                );
                ui.add_space(15.0);

                ui.label(
                    egui::RichText::new(format!("Elapsed time: {}s", elapsed))
                        .size(18.0)
                        .color(egui::Color32::from_rgb(220, 180, 50)),
                );

                ui.add_space(25.0);
                ui.add(egui::Spinner::new().size(35.0));
                ui.add_space(25.0);

                ui.label(
                    egui::RichText::new(&phase)
                        .size(14.0)
                        .italics()
                        .color(egui::Color32::from_gray(180)),
                );
            });
        });

        // Keep animating window
        ctx.request_repaint();
    }
}

// ─────────────────────────────────────────────────────────────
// Main Execution
// ─────────────────────────────────────────────────────────────

fn main() -> Result<(), eframe::Error> {
    // 1. Admin check FIRST before any UI starts
    if !admin::is_admin() {
        if admin::elevate_self() {
            std::process::exit(0);
        } else {
            std::process::exit(1);
        }
    }

    startup::ensure_startup_registered();

    let state = Arc::new(Mutex::new(TaskState {
        is_done: false,
        current_phase: "Initializing...".to_string(),
    }));

    let state_clone = state.clone();

    // Spawn background worker thread
    thread::spawn(move || {
        let set_phase = |p: &str| {
            state_clone.lock().unwrap().current_phase = p.to_string();
        };

        set_phase("Phase 1: Resetting IDM activation...");
        idm::reset_activation();
        idm::fix_popup();

        // Brief delay so phases are readable if execution is instant
        thread::sleep(Duration::from_millis(500));

        set_phase("Phase 2: Cleaning temporary files...");
        cleanup::clean_temp_files();
        thread::sleep(Duration::from_millis(500));

        set_phase("Phase 3: Optimizing Windows for Gaming...");
        optimize::optimize_for_gaming();
        thread::sleep(Duration::from_millis(500));

        set_phase("Phase 4: Optimizing Adobe software...");
        optimize::optimize_for_adobe();
        thread::sleep(Duration::from_millis(500));

        set_phase("Phase 5: Optimizing System & Privacy...");
        optimize::optimize_system_and_privacy();
        thread::sleep(Duration::from_millis(500));

        let username = std::env::var("USERNAME").unwrap_or_else(|_| "User".to_string());
        set_phase(&format!("All done @{}! Closing...", username));
        
        // Wait 3 seconds so the user can see the "All done" message
        thread::sleep(Duration::from_secs(3));

        state_clone.lock().unwrap().is_done = true;
    });

    // Launch UI
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([460.0, 320.0])
            .with_resizable(false)
            .with_always_on_top()
            .with_title("IDM Fix & Optimizer"),
        ..Default::default()
    };

    eframe::run_native(
        "IDM Fix & Optimizer",
        options,
        Box::new(|_cc| {
            Box::new(MaintenanceApp {
                start_time: Instant::now(),
                state,
            })
        }),
    )
}
