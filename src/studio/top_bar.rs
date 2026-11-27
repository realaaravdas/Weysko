use std::fs;

use egui::{Color32, CornerRadius, Frame, Margin, RichText, Stroke, Ui};

use crate::generator::{compile_site, exporter::generate_google_sites_embed};
use crate::model::{SiteProject, ViewportMode};
use crate::preview::PreviewServer;

use super::state::StudioState;
use super::style::{
    ghost_button, primary_white_button, secondary_slate_button, BG_INPUT, BG_PANEL, BG_SURFACE,
    BORDER_STRONG, BORDER_SUBTLE, SLATE_INK, TEXT_DIM, TEXT_MUTED, TEXT_PRIMARY, WHITE,
};

pub fn draw_top_bar(ctx: &egui::Context, state: &mut StudioState) {
    egui::TopBottomPanel::top("weysko_top_bar")
        .exact_height(52.0)
        .frame(
            Frame::new()
                .fill(BG_PANEL)
                .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
                .inner_margin(Margin::symmetric(14, 8)),
        )
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.x = 5.0;
            let avail_w = ui.available_width();
            let compact = avail_w < 1120.0;
            let show_pages_btn = avail_w >= 1220.0;
            let show_proj_name = avail_w >= 960.0;

            ui.horizontal_centered(|ui| {
                // Vector Brand Mark + Wordmark
                let (mark_rect, _) =
                    ui.allocate_exact_size(egui::vec2(24.0, 20.0), egui::Sense::hover());
                super::branding::paint_logo_mark(
                    ui.painter(),
                    mark_rect,
                    WHITE,
                    Color32::from_rgb(0, 188, 235),
                );
                if !compact {
                    ui.label(RichText::new("WEYSKO").color(WHITE).strong().size(13.0));
                }

                let file_lbl = if state.dirty { "File*" } else { "File" };
                ui.menu_button(RichText::new(file_lbl).color(TEXT_MUTED).size(12.0), |ui| {
                    if ui.button("New Blank Site").clicked() {
                        state.apply_template(crate::templates::StarterTemplate::BlankCanvas);
                        ui.close_menu();
                    }
                    if ui.button("Open Project...").clicked() {
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
                    ui.separator();
                    if ui.button("Manage Pages...").clicked() {
                        state.show_pages_modal = true;
                        ui.close_menu();
                    }
                    if ui.button("Export / Publish...").clicked() {
                        state.google_sites_embed_cache =
                            generate_google_sites_embed(&state.project);
                        state.show_export_modal = true;
                        ui.close_menu();
                    }
                });

                separator_v(ui);

                // Editable Project Name (auto-hides on narrow windows)
                if show_proj_name {
                    let name_resp = ui.add(
                        egui::TextEdit::singleline(&mut state.project.name)
                            .desired_width(108.0)
                            .font(egui::FontId::proportional(12.0)),
                    );
                    if name_resp.changed() {
                        state.mark_mutated();
                    }
                }

                // Page Dropdown (includes Manage Pages action)
                let current_page_title = state
                    .project
                    .find_page(&state.selected_page_id)
                    .map(|p| format!("/{}", p.html_filename()))
                    .unwrap_or_else(|| "Page".into());

                egui::ComboBox::from_id_salt("top_page_selector")
                    .selected_text(RichText::new(current_page_title).size(11.5))
                    .width(122.0)
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

                if show_pages_btn && secondary_slate_button(ui, "Pages").clicked() {
                    state.show_pages_modal = true;
                }

                separator_v(ui);

                // Viewport Switcher + Zoom + Grid/Snap
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
                                let label = if compact {
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
                                .min_size(egui::vec2(0.0, 24.0));
                                if ui.add(btn).on_hover_text(vp.label()).clicked() {
                                    state.viewport = vp;
                                    state.auto_fit_zoom = true;
                                }
                            }
                        });
                    });

                if ghost_button(ui, "-", false).on_hover_text("Zoom out (Ctrl+-)").clicked() {
                    state.auto_fit_zoom = false;
                    state.zoom = (state.zoom - 0.1).max(0.35);
                }
                ui.label(
                    RichText::new(format!("{}%", (state.zoom * 100.0).round() as i32))
                        .size(11.0)
                        .color(TEXT_MUTED),
                );
                if ghost_button(ui, "+", false).on_hover_text("Zoom in (Ctrl+=)").clicked() {
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

                // Right-aligned Undo/Redo, Live Preview, Shortcuts, Inspector toggle, Export CTA
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let export_lbl = if compact { "Export" } else { "Export / Publish" };
                    if primary_white_button(ui, export_lbl).clicked() {
                        state.google_sites_embed_cache =
                            generate_google_sites_embed(&state.project);
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
                    } else if compact {
                        "Preview"
                    } else {
                        "Live Preview"
                    };
                    if secondary_slate_button(ui, preview_label)
                        .on_hover_text("Compile and open live local HTTP preview in browser")
                        .clicked()
                    {
                        launch_or_open_preview(state);
                    }

                    if !compact {
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
            "{}.weysko.json",
            crate::model::project::sanitize_slug(&state.project.name)
        );
        rfd::FileDialog::new()
            .set_file_name(&default_name)
            .add_filter("Weysko Project", &["json", "weysko.json"])
            .save_file()
    });

    if let Some(p) = path {
        match state.project.to_json_pretty() {
            Ok(json) => match fs::write(&p, json) {
                Ok(_) => {
                    state.current_file_path = Some(p.clone());
                    state.dirty = false;
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
        .add_filter("Weysko Project", &["json"])
        .pick_file()
    {
        match fs::read_to_string(&path) {
            Ok(raw) => match SiteProject::from_json(&raw) {
                Ok(proj) => {
                    state.project = proj;
                    state.history.clear();
                    state.textures.clear();
                    state.current_file_path = Some(path.clone());
                    state.dirty = false;
                    state.preview_dirty = true;
                    state.selected_element_id = None;
                    state.selected_page_id = state
                        .project
                        .pages
                        .first()
                        .map(|p| p.id.clone())
                        .unwrap_or_default();
                    state.reconcile_selection();
                    state.notify(format!("Opened {}", path.display()), false);
                }
                Err(e) => state.notify(format!("Invalid project JSON: {}", e), true),
            },
            Err(e) => state.notify(format!("Read error: {}", e), true),
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
