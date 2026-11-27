use std::fs;

use egui::{Color32, CornerRadius, Frame, Margin, RichText, Stroke, Ui};

use crate::generator::{compile_site, exporter::generate_google_sites_embed};
use crate::model::ViewportMode;
use crate::preview::PreviewServer;

use super::state::{AppScreen, ExportModalTab, StudioState};
use super::style::{
    ghost_button, primary_white_button, secondary_slate_button, BG_INPUT, BG_PANEL, BG_SURFACE,
    BORDER_STRONG, BORDER_SUBTLE, SLATE_INK, SUCCESS_INK, TEXT_DIM, TEXT_MUTED, TEXT_PRIMARY,
    WHITE,
};

pub fn draw_top_bar(ctx: &egui::Context, state: &mut StudioState) {
    let screen_w = ctx.screen_rect().width();
    let multi_row = screen_w < 1360.0;
    let bar_h = if multi_row { 82.0 } else { 52.0 };

    egui::TopBottomPanel::top("weysko_top_bar")
        .exact_height(bar_h)
        .frame(
            Frame::new()
                .fill(BG_PANEL)
                .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
                .inner_margin(Margin::symmetric(12, 8)),
        )
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.x = 5.0;
            if multi_row {
                draw_top_bar_two_row(ui, state, screen_w);
            } else {
                draw_top_bar_single_row(ui, state, ui.available_width());
            }
        });
}

fn draw_brand_and_file_controls(ui: &mut Ui, state: &mut StudioState, show_wordmark: bool) {
    let (mark_rect, mark_resp) =
        ui.allocate_exact_size(egui::vec2(22.0, 18.0), egui::Sense::click());
    super::branding::paint_logo_mark(
        ui.painter(),
        mark_rect,
        WHITE,
        Color32::from_rgb(0, 188, 235),
    );
    if mark_resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    if mark_resp
        .on_hover_text("Return to Start Screen (Home)")
        .clicked()
    {
        state.screen = AppScreen::Home;
    }

    if show_wordmark {
        ui.label(RichText::new("WEYSKO").color(WHITE).strong().size(13.0));
    }

    if ghost_button(ui, "Home", false)
        .on_hover_text("Open Start / Home screen (templates, styles & recents)")
        .clicked()
    {
        state.screen = AppScreen::Home;
    }

    let file_lbl = if state.dirty { "File*" } else { "File" };
    ui.menu_button(RichText::new(file_lbl).color(TEXT_MUTED).size(12.0), |ui| {
        if ui.button("Home / Start Screen...").clicked() {
            state.screen = AppScreen::Home;
            ui.close_menu();
        }
        if ui.button("New Blank Site").clicked() {
            state.apply_template(crate::templates::StarterTemplate::BlankCanvas);
            ui.close_menu();
        }
        if ui.button("Open Project (.weysko / .json)...").clicked() {
            open_project_dialog(state);
            ui.close_menu();
        }
        ui.separator();
        if ui.button("Save (Ctrl+S)").clicked() {
            save_project_dialog(state, false);
            ui.close_menu();
        }
        if ui.button("Save As... (Ctrl+Shift+S)").clicked() {
            save_project_dialog(state, true);
            ui.close_menu();
        }
        if ui.button("Export .weysko File...").clicked() {
            state.export_tab = ExportModalTab::WeyskoFile;
            state.show_export_modal = true;
            ui.close_menu();
        }
        ui.separator();
        if ui.button("Manage Pages...").clicked() {
            state.show_pages_modal = true;
            ui.close_menu();
        }
        if ui.button("Export / Publish...").clicked() {
            state.google_sites_embed_cache = generate_google_sites_embed(&state.project);
            state.show_export_modal = true;
            ui.close_menu();
        }
    });
}

fn draw_page_dropdown(ui: &mut Ui, state: &mut StudioState, width: f32) {
    let current_page_title = state
        .project
        .find_page(&state.selected_page_id)
        .map(|p| format!("/{}", p.html_filename()))
        .unwrap_or_else(|| "Page".into());

    egui::ComboBox::from_id_salt("top_page_selector")
        .selected_text(RichText::new(current_page_title).size(11.5))
        .width(width)
        .show_ui(ui, |ui| {
            let pages: Vec<(String, String, String)> = state
                .project
                .pages
                .iter()
                .map(|p| (p.id.clone(), p.title.clone(), p.html_filename()))
                .collect();
            for (pid, title, file) in pages {
                let selected = pid == state.selected_page_id;
                if ui
                    .selectable_label(selected, format!("{} ({})", title, file))
                    .clicked()
                {
                    state.selected_page_id = pid;
                    state.selected_element_id = None;
                    state.reconcile_selection();
                }
            }
            ui.separator();
            if ui.button("+ Manage Pages...").clicked() {
                state.show_pages_modal = true;
            }
        });
}

fn draw_viewport_and_zoom_controls(ui: &mut Ui, state: &mut StudioState, short_vp: bool) {
    Frame::new()
        .fill(BG_INPUT)
        .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
        .corner_radius(CornerRadius::same(7))
        .inner_margin(Margin::same(2))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 2.0;
                for vp in [
                    ViewportMode::Desktop,
                    ViewportMode::Tablet,
                    ViewportMode::Mobile,
                ] {
                    let active = state.viewport == vp;
                    let label = if short_vp {
                        match vp {
                            ViewportMode::Desktop => "D",
                            ViewportMode::Tablet => "T",
                            ViewportMode::Mobile => "M",
                        }
                    } else {
                        match vp {
                            ViewportMode::Desktop => "Desktop",
                            ViewportMode::Tablet => "Tablet",
                            ViewportMode::Mobile => "Mobile",
                        }
                    };
                    let btn = egui::Button::new(
                        RichText::new(label)
                            .size(11.5)
                            .color(if active { SLATE_INK } else { TEXT_MUTED })
                            .strong(),
                    )
                    .fill(if active { WHITE } else { Color32::TRANSPARENT })
                    .stroke(Stroke::NONE)
                    .corner_radius(CornerRadius::same(5))
                    .min_size(egui::vec2(0.0, 23.0));
                    if ui.add(btn).on_hover_text(vp.label()).clicked() {
                        state.viewport = vp;
                        state.auto_fit_zoom = true;
                    }
                }
            });
        });

    if ghost_button(ui, "-", false)
        .on_hover_text("Zoom out (Ctrl+-)")
        .clicked()
    {
        state.auto_fit_zoom = false;
        state.zoom = (state.zoom - 0.1).max(0.24);
    }
    ui.label(
        RichText::new(format!("{}%", (state.zoom * 100.0).round() as i32))
            .size(11.0)
            .color(TEXT_MUTED),
    );
    if ghost_button(ui, "+", false)
        .on_hover_text("Zoom in (Ctrl+=)")
        .clicked()
    {
        state.auto_fit_zoom = false;
        state.zoom = (state.zoom + 0.1).min(1.5);
    }
    if ghost_button(ui, "Fit", state.auto_fit_zoom)
        .on_hover_text("Auto-fit canvas to window width (Ctrl+0)")
        .clicked()
    {
        state.auto_fit_zoom = true;
    }

    if ghost_button(ui, "Grid", state.show_grid)
        .on_hover_text("Toggle canvas dot grid")
        .clicked()
    {
        state.show_grid = !state.show_grid;
    }
    if ghost_button(ui, "Snap", state.snap_to_grid)
        .on_hover_text("Snap to 8px grid & alignment guides")
        .clicked()
    {
        state.snap_to_grid = !state.snap_to_grid;
    }
}

fn draw_top_bar_single_row(ui: &mut Ui, state: &mut StudioState, avail_w: f32) {
    let show_pages_btn = avail_w >= 1240.0;
    ui.horizontal_centered(|ui| {
        draw_brand_and_file_controls(ui, state, true);
        separator_v(ui);

        let name_resp = ui.add(
            egui::TextEdit::singleline(&mut state.project.name)
                .desired_width(104.0)
                .font(egui::FontId::proportional(12.0)),
        );
        if name_resp.changed() {
            state.mark_mutated();
        }

        draw_page_dropdown(ui, state, 118.0);
        if show_pages_btn && secondary_slate_button(ui, "Pages").clicked() {
            state.show_pages_modal = true;
        }

        separator_v(ui);
        draw_viewport_and_zoom_controls(ui, state, false);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if primary_white_button(ui, "Export / Publish").clicked() {
                state.google_sites_embed_cache = generate_google_sites_embed(&state.project);
                state.show_export_modal = true;
            }
            if ghost_button(ui, "Props", state.show_inspector)
                .on_hover_text("Toggle right Inspector panel")
                .clicked()
            {
                state.show_inspector = !state.show_inspector;
            }
            if ghost_button(ui, "?", state.show_shortcuts_modal)
                .on_hover_text("Keyboard shortcuts & quick guide (F1)")
                .clicked()
            {
                state.show_shortcuts_modal = !state.show_shortcuts_modal;
            }
            let preview_label = if state.preview_server.is_some() {
                "Preview ●"
            } else {
                "Live Preview"
            };
            if secondary_slate_button(ui, preview_label)
                .on_hover_text("Compile and open live local HTTP preview in browser")
                .clicked()
            {
                launch_or_open_preview(state);
            }

            let can_redo = state.history.can_redo();
            ui.add_enabled_ui(can_redo, |ui| {
                if ghost_button(ui, "Redo", false).clicked() {
                    state.redo();
                }
            });
            let can_undo = state.history.can_undo();
            ui.add_enabled_ui(can_undo, |ui| {
                if ghost_button(ui, "Undo", false).clicked() {
                    state.undo();
                }
            });
        });
    });
}

fn draw_top_bar_two_row(ui: &mut Ui, state: &mut StudioState, screen_w: f32) {
    let narrow_controls = screen_w < 820.0;
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 6.0;

        // Row 1: Brand, Home, File, Project Name/Page, Preview, Props, Export
        ui.horizontal(|ui| {
            draw_brand_and_file_controls(ui, state, screen_w >= 860.0);
            separator_v(ui);

            if screen_w >= 820.0 {
                let name_w = (screen_w - 720.0).clamp(80.0, 130.0);
                let name_resp = ui.add(
                    egui::TextEdit::singleline(&mut state.project.name)
                        .desired_width(name_w)
                        .font(egui::FontId::proportional(12.0)),
                );
                if name_resp.changed() {
                    state.mark_mutated();
                }
            }

            draw_page_dropdown(ui, state, if narrow_controls { 96.0 } else { 112.0 });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if primary_white_button(ui, "Export").clicked() {
                    state.google_sites_embed_cache = generate_google_sites_embed(&state.project);
                    state.show_export_modal = true;
                }
                if ghost_button(ui, "Props", state.show_inspector)
                    .on_hover_text("Toggle right Inspector panel")
                    .clicked()
                {
                    state.show_inspector = !state.show_inspector;
                    if state.show_inspector && screen_w < 920.0 {
                        state.active_drawer = None;
                    }
                }
                let preview_label = if state.preview_server.is_some() {
                    "Preview ●"
                } else {
                    "Preview"
                };
                if secondary_slate_button(ui, preview_label)
                    .on_hover_text("Compile and open live local HTTP preview in browser")
                    .clicked()
                {
                    launch_or_open_preview(state);
                }
            });
        });

        // Row 2: Viewport switcher, Zoom/Fit, Grid/Snap, Pages, Undo/Redo, Help
        ui.horizontal(|ui| {
            draw_viewport_and_zoom_controls(ui, state, screen_w < 760.0);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ghost_button(ui, "?", state.show_shortcuts_modal)
                    .on_hover_text("Keyboard shortcuts & quick guide (F1)")
                    .clicked()
                {
                    state.show_shortcuts_modal = !state.show_shortcuts_modal;
                }
                if screen_w >= 760.0 && ghost_button(ui, "Pages", state.show_pages_modal).clicked() {
                    state.show_pages_modal = true;
                }
                let can_redo = state.history.can_redo();
                ui.add_enabled_ui(can_redo, |ui| {
                    if ghost_button(ui, "Redo", false).clicked() {
                        state.redo();
                    }
                });
                let can_undo = state.history.can_undo();
                ui.add_enabled_ui(can_undo, |ui| {
                    if ghost_button(ui, "Undo", false).clicked() {
                        state.undo();
                    }
                });
            });
        });
    });
}

/// Compact bottom status bar surfacing live coordinates, theme, and file status without collisions.
pub fn draw_status_bar(ctx: &egui::Context, state: &StudioState) {
    egui::TopBottomPanel::bottom("weysko_status_bar")
        .exact_height(26.0)
        .frame(
            Frame::new()
                .fill(Color32::from_rgb(11, 17, 32))
                .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
                .inner_margin(Margin::symmetric(12, 4)),
        )
        .show(ctx, |ui| {
            let avail_w = ui.available_width();
            ui.horizontal_centered(|ui| {
                if let Some(page) = state.project.find_page(&state.selected_page_id) {
                    let elem_count: usize = page.sections.iter().map(|s| s.elements.len()).sum();
                    ui.label(
                        RichText::new(format!(
                            "/{}  •  {} strips  •  {} els",
                            page.html_filename(),
                            page.sections.len(),
                            elem_count
                        ))
                        .color(TEXT_MUTED)
                        .size(11.0),
                    );

                    if let Some(el_id) = &state.selected_element_id {
                        if let Some((_, el)) = page.find_element(el_id) {
                            separator_v_small(ui);
                            let short_name: String = el.name.chars().take(18).collect();
                            ui.label(
                                RichText::new(format!(
                                    "{}  [{:.0},{:.0} · {:.0}×{:.0}]",
                                    short_name, el.rect.x, el.rect.y, el.rect.w, el.rect.h
                                ))
                                .color(WHITE)
                                .size(11.0),
                            );
                        }
                    } else if avail_w >= 1060.0 {
                        separator_v_small(ui);
                        ui.label(
                            RichText::new("Double-click text to edit inline • Right-click for actions")
                                .color(TEXT_DIM)
                                .size(11.0),
                        );
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let file_raw = state
                        .current_file_path
                        .as_ref()
                        .and_then(|p| p.file_name())
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unsaved draft");
                    let file_label: String = if file_raw.chars().count() > 22 {
                        let head: String = file_raw.chars().take(19).collect();
                        format!("{}…", head)
                    } else {
                        file_raw.to_string()
                    };
                    let dirty_mark = if state.dirty { " *" } else { "" };
                    ui.label(
                        RichText::new(format!("{}{}", file_label, dirty_mark))
                            .color(TEXT_MUTED)
                            .size(11.0),
                    );

                    if avail_w >= 760.0 {
                        separator_v_small(ui);
                        ui.label(
                            RichText::new(format!(
                                "{} · {:.0}px",
                                state.project.theme.preset.label(),
                                state.viewport.width_px()
                            ))
                            .color(TEXT_DIM)
                            .size(11.0),
                        );
                    }

                    if avail_w >= 880.0 {
                        if let Some(srv) = &state.preview_server {
                            separator_v_small(ui);
                            ui.label(
                                RichText::new(format!("● Live {}", srv.url()))
                                    .color(SUCCESS_INK)
                                    .size(11.0),
                            );
                        }
                    }
                });
            });
        });
}

pub fn save_project_dialog(state: &mut StudioState, force_save_as: bool) {
    let target_path = if !force_save_as {
        state.current_file_path.clone()
    } else {
        None
    };

    let path = target_path.or_else(|| {
        let default_name = format!(
            "{}.weysko",
            crate::model::project::sanitize_slug(&state.project.name)
        );
        rfd::FileDialog::new()
            .set_file_name(&default_name)
            .add_filter("Weysko Project", &["weysko", "json"])
            .save_file()
    });

    if let Some(p) = path {
        let is_legacy_json = p
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("json"))
            .unwrap_or(false);
        let serialized = if is_legacy_json {
            state.project.to_json_pretty()
        } else {
            state.project.to_weysko_pretty()
        };

        match serialized {
            Ok(raw) => match fs::write(&p, raw) {
                Ok(_) => {
                    state.current_file_path = Some(p.clone());
                    state.dirty = false;
                    state.recent_projects.record(&state.project, &p);
                    state.recent_projects.save();
                    state.notify(format!("Saved {}", p.display()), false);
                }
                Err(e) => state.notify(format!("Save failed: {}", e), true),
            },
            Err(e) => state.notify(format!("Serialization error: {}", e), true),
        }
    }
}

pub fn open_project_dialog(state: &mut StudioState) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("Weysko Project", &["weysko", "json"])
        .pick_file()
    {
        if let Err(err) = state.open_project_from_path(&path) {
            state.notify(err, true);
        }
    }
}

pub fn launch_or_open_preview(state: &mut StudioState) {
    let compiled = compile_site(&state.project);
    if let Some(server) = &state.preview_server {
        server.update_site(compiled);
        let url = server.url();
        let _ = open::that(&url);
        state.preview_dirty = false;
        state.notify(format!("Preview updated at {}", url), false);
    } else {
        match PreviewServer::start(compiled, None) {
            Ok(server) => {
                let url = server.url();
                let _ = open::that(&url);
                state.preview_server = Some(server);
                state.preview_dirty = false;
                state.notify(format!("Live preview running at {}", url), false);
            }
            Err(e) => state.notify(format!("Preview server failed: {}", e), true),
        }
    }
}

fn separator_v(ui: &mut Ui) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(1.0, 22.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, BG_SURFACE);
    let _ = TEXT_PRIMARY;
    let _ = TEXT_DIM;
}

fn separator_v_small(ui: &mut Ui) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(1.0, 14.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, BG_SURFACE);
}
