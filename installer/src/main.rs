#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod engine;

use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use eframe::egui::{self, Color32, CornerRadius, Frame, IconData, Margin, Pos2, Rect, RichText, Stroke, Vec2};

use engine::{run_install, run_uninstall, InstallConfig, InstallSummary};

const BG_VOID: Color32 = Color32::from_rgb(9, 13, 22);
const BG_PANEL: Color32 = Color32::from_rgb(15, 23, 42);
const BG_SURFACE: Color32 = Color32::from_rgb(30, 41, 59);
const BG_INPUT: Color32 = Color32::from_rgb(12, 19, 34);
const BORDER_STRONG: Color32 = Color32::from_rgb(51, 65, 85);
const TEXT_PRIMARY: Color32 = Color32::from_rgb(248, 250, 252);
const TEXT_MUTED: Color32 = Color32::from_rgb(148, 163, 184);
const WHITE: Color32 = Color32::WHITE;
const SLATE_INK: Color32 = Color32::from_rgb(15, 23, 42);
const CYAN_BRAND: Color32 = Color32::from_rgb(0, 188, 235);
const DANGER_INK: Color32 = Color32::from_rgb(248, 113, 113);

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // Headless silent install mode for automation / enterprise deployment
    if args.iter().any(|a| a == "--silent" || a == "/S" || a == "/s") {
        let mut cfg = InstallConfig::default();
        if let Some(pos) = args.iter().position(|a| a == "--dir") {
            if let Some(dir) = args.get(pos + 1) {
                cfg.install_dir = PathBuf::from(dir);
            }
        }
        if args.iter().any(|a| a == "--no-shortcuts") {
            cfg.create_start_menu = false;
            cfg.create_desktop_shortcut = false;
        }
        if args.iter().any(|a| a == "--no-path") {
            cfg.add_to_path = false;
        }
        if args.iter().any(|a| a == "--no-assoc") {
            cfg.associate_files = false;
        }
        if args.iter().any(|a| a == "--no-registry") {
            cfg.register_uninstaller = false;
        }

        if let Err(e) = run_install(&cfg, |_, _| {}) {
            eprintln!("Silent install failed: {}", e);
            std::process::exit(1);
        }
        return;
    }

    let is_uninstall = args.iter().any(|a| a == "--uninstall");
    let title = if is_uninstall {
        "Uninstall Weysko Visual Website Studio"
    } else {
        "Weysko v1.3.0 Setup"
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(title)
            .with_inner_size([660.0, 480.0])
            .with_resizable(false)
            .with_icon(setup_icon_data()),
        ..Default::default()
    };

    let _ = eframe::run_native(
        title,
        options,
        Box::new(move |_cc| Ok(Box::new(SetupApp::new(is_uninstall)))),
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WizardStep {
    Welcome,
    Options,
    Installing,
    Complete,
    UninstallConfirm,
    Uninstalling,
    UninstallDone,
}

enum WorkerEvent {
    Progress(f32, String),
    InstallDone(Result<InstallSummary, String>),
    UninstallDone(Result<(), String>),
}

struct SetupApp {
    step: WizardStep,
    config: InstallConfig,
    accepted_license: bool,
    launch_after_finish: bool,
    progress: f32,
    status_line: String,
    logs: Vec<String>,
    error_msg: Option<String>,
    summary: Option<InstallSummary>,
    rx: Option<Receiver<WorkerEvent>>,
    themed: bool,
}

impl SetupApp {
    fn new(is_uninstall: bool) -> Self {
        let default_dir = if is_uninstall {
            env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.to_path_buf()))
                .unwrap_or_else(engine::default_install_dir)
        } else {
            engine::default_install_dir()
        };

        let cfg = InstallConfig {
            install_dir: default_dir,
            ..Default::default()
        };

        Self {
            step: if is_uninstall {
                WizardStep::UninstallConfirm
            } else {
                WizardStep::Welcome
            },
            config: cfg,
            accepted_license: true,
            launch_after_finish: true,
            progress: 0.0,
            status_line: "Ready".into(),
            logs: Vec::new(),
            error_msg: None,
            summary: None,
            rx: None,
            themed: false,
        }
    }

    fn start_install(&mut self) {
        self.step = WizardStep::Installing;
        self.progress = 0.05;
        self.logs.clear();
        self.error_msg = None;

        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);
        let cfg = self.config.clone();

        thread::spawn(move || {
            let res = run_install(&cfg, |p, msg| {
                let _ = tx.send(WorkerEvent::Progress(p, msg.to_string()));
            });
            let _ = tx.send(WorkerEvent::InstallDone(res));
        });
    }

    fn start_uninstall(&mut self) {
        self.step = WizardStep::Uninstalling;
        self.progress = 0.05;
        self.logs.clear();
        self.error_msg = None;

        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);
        let dir = self.config.install_dir.clone();

        thread::spawn(move || {
            let res = run_uninstall(&dir, |p, msg| {
                let _ = tx.send(WorkerEvent::Progress(p, msg.to_string()));
            });
            let _ = tx.send(WorkerEvent::UninstallDone(res));
        });
    }

    fn poll_worker(&mut self, ctx: &egui::Context) {
        let mut finished = false;
        if let Some(rx) = &self.rx {
            while let Ok(ev) = rx.try_recv() {
                match ev {
                    WorkerEvent::Progress(p, msg) => {
                        self.progress = p;
                        self.status_line = msg.clone();
                        self.logs.push(msg);
                    }
                    WorkerEvent::InstallDone(res) => {
                        finished = true;
                        match res {
                            Ok(sum) => {
                                self.summary = Some(sum);
                                self.step = WizardStep::Complete;
                            }
                            Err(err) => {
                                self.error_msg = Some(err);
                                self.step = WizardStep::Complete;
                            }
                        }
                    }
                    WorkerEvent::UninstallDone(res) => {
                        finished = true;
                        if let Err(err) = res {
                            self.error_msg = Some(err);
                        }
                        self.step = WizardStep::UninstallDone;
                    }
                }
            }
            ctx.request_repaint();
        }
        if finished {
            self.rx = None;
        }
    }
}

impl eframe::App for SetupApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.themed {
            apply_theme(ctx);
            self.themed = true;
        }
        self.poll_worker(ctx);

        // Top Slate Header Strip
        egui::TopBottomPanel::top("setup_header")
            .exact_height(64.0)
            .frame(
                Frame::new()
                    .fill(BG_PANEL)
                    .stroke(Stroke::new(1.0_f32, BG_SURFACE))
                    .inner_margin(Margin::symmetric(22, 14)),
            )
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    let (mark_rect, _) =
                        ui.allocate_exact_size(egui::vec2(28.0, 24.0), egui::Sense::hover());
                    paint_logo_mark(ui.painter(), mark_rect);
                    ui.add_space(6.0);
                    ui.label(RichText::new("WEYSKO").color(WHITE).strong().size(15.0));
                    ui.label(RichText::new("v1.3.0 Setup").color(TEXT_MUTED).size(12.5));
                });
            });

        // Bottom Footer Bar
        egui::TopBottomPanel::bottom("setup_footer")
            .exact_height(58.0)
            .frame(
                Frame::new()
                    .fill(BG_PANEL)
                    .stroke(Stroke::new(1.0_f32, BG_SURFACE))
                    .inner_margin(Margin::symmetric(22, 12)),
            )
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.label(
                        RichText::new("Pure Rust • Zero AI • MIT License")
                            .color(TEXT_MUTED)
                            .size(11.5),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        match self.step {
                            WizardStep::Welcome => {
                                ui.add_enabled_ui(self.accepted_license, |ui| {
                                    if white_btn(ui, "Next ›").clicked() {
                                        self.step = WizardStep::Options;
                                    }
                                });
                                if slate_btn(ui, "Cancel").clicked() {
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                }
                            }
                            WizardStep::Options => {
                                if white_btn(ui, "Install").clicked() {
                                    self.start_install();
                                }
                                if slate_btn(ui, "‹ Back").clicked() {
                                    self.step = WizardStep::Welcome;
                                }
                            }
                            WizardStep::Installing | WizardStep::Uninstalling => {
                                ui.label(RichText::new("Please wait...").color(TEXT_MUTED).size(12.0));
                            }
                            WizardStep::Complete => {
                                if white_btn(ui, "Finish").clicked() {
                                    if self.error_msg.is_none() && self.launch_after_finish {
                                        if let Some(sum) = &self.summary {
                                            let _ = Command::new(&sum.exe_path).spawn();
                                        }
                                    }
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                }
                            }
                            WizardStep::UninstallConfirm => {
                                if danger_btn(ui, "Uninstall").clicked() {
                                    self.start_uninstall();
                                }
                                if slate_btn(ui, "Cancel").clicked() {
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                }
                            }
                            WizardStep::UninstallDone => {
                                if white_btn(ui, "Close").clicked() {
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                }
                            }
                        }
                    });
                });
            });

        // Main Body
        egui::CentralPanel::default()
            .frame(Frame::new().fill(BG_VOID).inner_margin(Margin::same(26)))
            .show(ctx, |ui| match self.step {
                WizardStep::Welcome => {
                    ui.label(
                        RichText::new("Install Weysko Visual Website Studio")
                            .color(WHITE)
                            .strong()
                            .size(21.0),
                    );
                    ui.add_space(10.0);
                    ui.label(
                        RichText::new(
                            "Weysko is a deterministic, zero-AI visual website designer and static site compiler written in Rust. Build responsive multi-page sites and export directly to any host or Google Sites.",
                        )
                        .color(TEXT_MUTED)
                        .size(13.5),
                    );
                    ui.add_space(16.0);
                    Frame::new()
                        .fill(BG_PANEL)
                        .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::same(14))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            for item in [
                                "• Native desktop WYSIWYG designer with dark minimalist slate workspace",
                                "• 16 element primitives, 13 section strips, 5 templates, 6 curated themes",
                                "• Export to Static Folder, ZIP Archive, Single-File HTML, or Google Sites",
                                "• Includes headless CLI (`weysko --export`, `weysko --serve`)",
                            ] {
                                ui.label(RichText::new(item).color(TEXT_PRIMARY).size(12.5));
                            }
                        });
                    ui.add_space(18.0);
                    ui.checkbox(
                        &mut self.accepted_license,
                        "I accept the MIT License terms (2026 Weysko Project)",
                    );
                }
                WizardStep::Options => {
                    ui.label(
                        RichText::new("Destination & Integration Options")
                            .color(WHITE)
                            .strong()
                            .size(18.0),
                    );
                    ui.add_space(12.0);
                    ui.label(RichText::new("Install Location").color(TEXT_MUTED).size(12.0));
                    ui.add_space(4.0);
                    let mut dir_str = self.config.install_dir.to_string_lossy().to_string();
                    ui.horizontal(|ui| {
                        let resp = ui.add(
                            egui::TextEdit::singleline(&mut dir_str)
                                .desired_width(480.0),
                        );
                        if resp.changed() {
                            self.config.install_dir = PathBuf::from(&dir_str);
                        }
                        if slate_btn(ui, "Browse...").clicked() {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                self.config.install_dir = folder.join("Weysko");
                            }
                        }
                    });

                    ui.add_space(18.0);
                    ui.label(RichText::new("System Integration").color(TEXT_MUTED).size(12.0));
                    ui.add_space(6.0);
                    ui.checkbox(&mut self.config.create_start_menu, "Create Start Menu shortcuts (Weysko + Uninstall)");
                    ui.checkbox(&mut self.config.create_desktop_shortcut, "Create Desktop shortcut");
                    ui.checkbox(&mut self.config.add_to_path, "Add Weysko CLI (`weysko`) to User PATH");
                    ui.checkbox(&mut self.config.associate_files, "Associate `.weysko.json` project files with Weysko");
                    ui.checkbox(&mut self.config.register_uninstaller, "Register in Windows Apps & Features");
                }
                WizardStep::Installing | WizardStep::Uninstalling => {
                    let heading = if self.step == WizardStep::Installing {
                        "Installing Weysko..."
                    } else {
                        "Uninstalling Weysko..."
                    };
                    ui.label(RichText::new(heading).color(WHITE).strong().size(18.0));
                    ui.add_space(10.0);
                    ui.label(RichText::new(&self.status_line).color(TEXT_MUTED).size(13.0));
                    ui.add_space(12.0);
                    ui.add(
                        egui::ProgressBar::new(self.progress)
                            .show_percentage()
                            .desired_width(f32::INFINITY),
                    );
                    ui.add_space(14.0);
                    Frame::new()
                        .fill(BG_INPUT)
                        .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::same(12))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.set_min_height(150.0);
                            for line in &self.logs {
                                ui.label(
                                    RichText::new(format!("› {}", line))
                                        .font(egui::FontId::monospace(11.5))
                                        .color(TEXT_MUTED),
                                );
                            }
                        });
                }
                WizardStep::Complete => {
                    if let Some(err) = &self.error_msg {
                        ui.label(
                            RichText::new("Installation Failed")
                                .color(DANGER_INK)
                                .strong()
                                .size(20.0),
                        );
                        ui.add_space(10.0);
                        ui.label(RichText::new(err).color(TEXT_PRIMARY).size(13.0));
                    } else {
                        ui.label(
                            RichText::new("Weysko v1.3.0 Installed")
                                .color(WHITE)
                                .strong()
                                .size(21.0),
                        );
                        ui.add_space(10.0);
                        if let Some(sum) = &self.summary {
                            let mb = sum.installed_bytes as f64 / (1024.0 * 1024.0);
                            ui.label(
                                RichText::new(format!(
                                    "Installed {:.2} MB successfully to {}",
                                    mb,
                                    sum.install_dir.display()
                                ))
                                .color(TEXT_MUTED)
                                .size(13.5),
                            );
                        }
                        ui.add_space(18.0);
                        ui.checkbox(
                            &mut self.launch_after_finish,
                            "Launch Weysko Visual Website Studio when setup closes",
                        );
                    }
                }
                WizardStep::UninstallConfirm => {
                    ui.label(
                        RichText::new("Uninstall Weysko Visual Website Studio?")
                            .color(WHITE)
                            .strong()
                            .size(19.0),
                    );
                    ui.add_space(12.0);
                    ui.label(
                        RichText::new(format!(
                            "This will remove Weysko from {}, delete Start Menu and Desktop shortcuts, clean up Windows Registry entries, and remove Weysko from your User PATH.",
                            self.config.install_dir.display()
                        ))
                        .color(TEXT_MUTED)
                        .size(13.5),
                    );
                }
                WizardStep::UninstallDone => {
                    if let Some(err) = &self.error_msg {
                        ui.label(RichText::new("Uninstall Encountered an Error").color(DANGER_INK).strong().size(18.0));
                        ui.add_space(8.0);
                        ui.label(RichText::new(err).color(TEXT_PRIMARY));
                    } else {
                        ui.label(RichText::new("Weysko Uninstalled").color(WHITE).strong().size(20.0));
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new("All Weysko files, shortcuts, and registry entries have been removed.")
                                .color(TEXT_MUTED)
                                .size(13.5),
                        );
                    }
                }
            });
    }
}

fn apply_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = BG_PANEL;
    visuals.window_fill = BG_PANEL;
    visuals.extreme_bg_color = BG_INPUT;
    visuals.widgets.inactive.bg_fill = BG_SURFACE;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, BORDER_STRONG);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(6);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(44, 58, 82);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, WHITE);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(6);
    ctx.set_visuals(visuals);
}

fn white_btn(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.add(
        egui::Button::new(RichText::new(label).color(SLATE_INK).strong().size(12.5))
            .fill(WHITE)
            .stroke(Stroke::new(1.0_f32, WHITE))
            .corner_radius(CornerRadius::same(6))
            .min_size(egui::vec2(88.0, 32.0)),
    )
}

fn slate_btn(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.add(
        egui::Button::new(RichText::new(label).color(TEXT_PRIMARY).size(12.5))
            .fill(BG_SURFACE)
            .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
            .corner_radius(CornerRadius::same(6))
            .min_size(egui::vec2(82.0, 32.0)),
    )
}

fn danger_btn(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.add(
        egui::Button::new(RichText::new(label).color(WHITE).strong().size(12.5))
            .fill(Color32::from_rgb(185, 28, 28))
            .stroke(Stroke::new(1.0_f32, DANGER_INK))
            .corner_radius(CornerRadius::same(6))
            .min_size(egui::vec2(94.0, 32.0)),
    )
}

fn paint_logo_mark(painter: &egui::Painter, rect: Rect) {
    let w = rect.width();
    let h = rect.height();
    let bars: [(f32, f32, f32, Color32); 7] = [
        (0.06, 0.18, 0.56, WHITE),
        (0.20, 0.34, 0.54, CYAN_BRAND),
        (0.34, 0.22, 0.56, WHITE),
        (0.48, 0.38, 0.36, TEXT_MUTED),
        (0.62, 0.22, 0.56, WHITE),
        (0.76, 0.34, 0.54, CYAN_BRAND),
        (0.90, 0.18, 0.56, WHITE),
    ];
    let bar_w = (w * 0.095).max(1.8);
    let cr = CornerRadius::same((bar_w * 0.5).round() as u8);
    for (cx_frac, y_frac, h_frac, col) in bars {
        let pos = Pos2::new(rect.min.x + w * cx_frac - bar_w * 0.5, rect.min.y + h * y_frac);
        painter.rect_filled(Rect::from_min_size(pos, Vec2::new(bar_w, h * h_frac)), cr, col);
    }
}

fn setup_icon_data() -> IconData {
    let size = 64u32;
    let mut rgba = vec![0u8; (size * size * 4) as usize];
    let bg = [15u8, 23, 42, 255];
    let border = [51u8, 65, 85, 255];
    let white = [255u8, 255, 255, 255];
    let cyan = [0u8, 188, 235, 255];
    let muted = [148u8, 163, 184, 255];

    for y in 0..size {
        for x in 0..size {
            let fx = x as f32 + 0.5;
            let fy = y as f32 + 0.5;
            let dx = (fx - 32.0).abs() - 18.0;
            let dy = (fy - 32.0).abs() - 18.0;
            let dist = dx.max(0.0).hypot(dy.max(0.0)) + dx.max(dy).min(0.0) - 13.0;
            let idx = ((y * size + x) * 4) as usize;
            if dist <= 0.0 {
                rgba[idx..idx + 4].copy_from_slice(if dist > -1.8 { &border } else { &bg });
            }
        }
    }

    let bars: [(f32, f32, f32, f32, [u8; 4]); 7] = [
        (11.5, 17.0, 4.5, 24.0, white),
        (18.0, 24.0, 4.5, 23.0, cyan),
        (24.5, 20.0, 4.5, 24.0, white),
        (31.0, 27.0, 4.5, 15.0, muted),
        (37.5, 20.0, 4.5, 24.0, white),
        (44.0, 24.0, 4.5, 23.0, cyan),
        (50.5, 17.0, 4.5, 24.0, white),
    ];
    for (bx, by, bw, bh, col) in bars {
        for y in (by as u32)..((by + bh) as u32) {
            for x in (bx as u32)..((bx + bw) as u32) {
                let idx = ((y * size + x) * 4) as usize;
                rgba[idx..idx + 4].copy_from_slice(&col);
            }
        }
    }

    IconData { rgba, width: size, height: size }
}
