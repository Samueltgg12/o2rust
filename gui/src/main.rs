//! O2Rust GUI — an egui/eframe front-end for the emulator.
//!
//! This is the windowed front-end. It will eventually provide:
//!
//! - Windowed framebuffer display
//! - Keyboard/mouse input
//! - Debugger / inspector (registers, memory, disassembly)
//!
//! **Status:** milestone M4 (GUI complete). This is the initial window
//! skeleton; the full GUI is implemented in later milestones.

use eframe::egui;
use o2rust::system::Emulator;

fn main() -> eframe::Result {
    o2rust::log::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("O2Rust — SGI O2 Emulator"),
        ..Default::default()
    };

    eframe::run_native(
        "O2Rust",
        options,
        Box::new(|_cc| Ok(Box::new(O2RustApp::default()))),
    )
}

/// The main application state.
struct O2RustApp {
    /// The emulator instance.
    emulator: Emulator,
    /// Path to the PROM image.
    prom_path: String,
    /// Whether the emulator is running.
    running: bool,
}

impl Default for O2RustApp {
    fn default() -> Self {
        Self {
            emulator: Emulator::new(),
            prom_path: "samples/ip32prom.rev4.18.bin".to_string(),
            running: false,
        }
    }
}

impl eframe::App for O2RustApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load PROM...").clicked() {
                        self.load_prom();
                        ui.close_menu();
                    }
                    if ui.button("Reset").clicked() {
                        self.emulator.reset();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Emulation", |ui| {
                    if ui.button("Run").clicked() {
                        self.running = true;
                        ui.close_menu();
                    }
                    if ui.button("Pause").clicked() {
                        self.running = false;
                        ui.close_menu();
                    }
                    if ui.button("Step").clicked() {
                        self.emulator.step();
                        ui.close_menu();
                    }
                });
            });
        });

        egui::SidePanel::right("inspector")
            .default_width(320.0)
            .show(ctx, |ui| {
                ui.heading("CPU Inspector");
                ui.separator();
                ui.label(format!("PC: 0x{:08x}", self.emulator.pc()));
                ui.label(format!("Model: {}", self.emulator.model.name()));
                ui.label(format!("Running: {}", self.running));
                ui.separator();
                ui.label("Registers");
                egui::Grid::new("registers").num_columns(2).show(ui, |ui| {
                    for i in 0..32 {
                        ui.monospace(format!("${i:02}"));
                        ui.monospace(format!("0x{:016x}", self.emulator.cpu.state.gpr[i]));
                        ui.end_row();
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("O2Rust");
            ui.label(format!("SGI O2 (IP32) emulator — v{}", o2rust::VERSION));
            ui.separator();
            ui.label("Framebuffer display will appear here (milestone M4).");
            ui.add_space(16.0);
            ui.horizontal(|ui| {
                if ui.button("Load PROM").clicked() {
                    self.load_prom();
                }
                if ui.button("Run").clicked() {
                    self.running = true;
                }
                if ui.button("Pause").clicked() {
                    self.running = false;
                }
                if ui.button("Step").clicked() {
                    self.emulator.step();
                }
            });
        });

        // Run the emulator if requested.
        if self.running {
            self.emulator.run(100_000);
            ctx.request_repaint();
        }
    }
}

impl O2RustApp {
    /// Load the PROM image into the emulator.
    fn load_prom(&mut self) {
        match self.emulator.load_prom(&self.prom_path) {
            Ok(()) => {
                o2rust::log::info_msg("PROM loaded successfully");
            }
            Err(e) => {
                o2rust::log::error_msg(&format!("Failed to load PROM: {e}"));
            }
        }
    }
}