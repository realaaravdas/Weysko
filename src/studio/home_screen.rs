use std::path::{Path, PathBuf};

use egui::{Color32, CornerRadius, Frame, Margin, Pos2, Rect, RichText, ScrollArea, Stroke, Ui};

use crate::model::{DesignTokens, PresetTheme};
use crate::templates::StarterTemplate;

use super::recent::RecentProjectsStore;
use super::state::{AppScreen, HomeSectionTab, StudioState};
use super::style::{
    ghost_button, hex_to_color32, primary_white_button, secondary_slate_button, section_heading,
    BG_INPUT, BG_PANEL, BG_SURFACE, BG_VOID, BORDER_STRONG, BORDER_SUBTLE, DANGER_INK, ICE_ACCENT,
    SLATE_INK, TEXT_DIM, TEXT_MUTED, WHITE,
};
use super::top_bar::open_project_dialog;

const CYAN_BRAND: Color32 = Color32::from_rgb(0, 188, 235);

/// Renders the responsive Adobe Photoshop–style Creative Studio Start / Home screen.
pub fn draw_home_screen(ctx: &egui::Context, state: &mut StudioState) {
    let screen_w = ctx.screen_rect().width();
    let compact_home = screen_w < 860.0;

    if compact_home {
        draw_compact_home_header(ctx, state);
    } else {
        draw_home_left_rail(ctx, state, screen_w);
    }

    let pad_x = if compact_home {
        14
    } else if screen_w < 1080.0 {
        18
    } else {
        26
    };

    egui::CentralPanel::default()
        .frame(
            Frame::new()
                .fill(BG_VOID)
                .inner_margin(Margin::symmetric(pad_x, 16)),
        )
        .show(ctx, |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
            let avail_w = ui.available_width();
            draw_home_top_header(ui, state, avail_w);
            ui.add_space(14.0);

            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    draw_style_picker_strip(ui, state);
                    ui.add_space(18.0);

                    match state.home_tab {
                        HomeSectionTab::Home => draw_home_overview(ui, state),
                        HomeSectionTab::Templates => draw_templates_gallery(ui, state),
                        HomeSectionTab::NewFromScratch => draw_new_from_scratch(ui, state),
                    }
                });
        });
}

fn draw_compact_home_header(ctx: &egui::Context, state: &mut StudioState) {
    egui::TopBottomPanel::top("weysko_home_compact_bar")
        .exact_height(84.0)
        .frame(
            Frame::new()
                .fill(BG_PANEL)
                .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
                .inner_margin(Margin::symmetric(14, 10)),
        )
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 8.0;
                ui.horizontal(|ui| {
                    let (mark_rect, _) =
                        ui.allocate_exact_size(egui::vec2(24.0, 20.0), egui::Sense::hover());
                    super::branding::paint_logo_mark(ui.painter(), mark_rect, WHITE, CYAN_BRAND);
                    ui.label(RichText::new("WEYSKO").color(WHITE).strong().size(14.0));
                    ui.label(
                        RichText::new(concat!("v", env!("CARGO_PKG_VERSION")))
                            .color(TEXT_DIM)
                            .size(11.0),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if primary_white_button(ui, "+ New").clicked() {
                            state.home_tab = HomeSectionTab::NewFromScratch;
                        }
                        if secondary_slate_button(ui, "Import .weysko").clicked() {
                            open_project_dialog(state);
                        }
                        if state.has_active_session && ghost_button(ui, "<- Editor", false).clicked()
                        {
                            state.screen = AppScreen::Editor;
                        }
                    });
                });

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 6.0;
                    for (tab, label) in [
                        (HomeSectionTab::Home, "Home"),
                        (HomeSectionTab::Templates, "Templates"),
                        (HomeSectionTab::NewFromScratch, "New from Scratch"),
                    ] {
                        let active = state.home_tab == tab;
                        if ghost_button(ui, label, active).clicked() {
                            state.home_tab = tab;
                        }
                    }
                });
            });
        });
}

fn draw_home_left_rail(ctx: &egui::Context, state: &mut StudioState, screen_w: f32) {
    let rail_w = if screen_w < 1060.0 { 196.0 } else { 228.0 };

    egui::SidePanel::left("weysko_home_rail")
        .exact_width(rail_w)
        .resizable(false)
        .frame(
            Frame::new()
                .fill(BG_PANEL)
                .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
                .inner_margin(Margin::symmetric(14, 16)),
        )
        .show(ctx, |ui| {
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let (mark_rect, _) =
                            ui.allocate_exact_size(egui::vec2(26.0, 22.0), egui::Sense::hover());
                        super::branding::paint_logo_mark(ui.painter(), mark_rect, WHITE, CYAN_BRAND);
                        ui.add_space(4.0);
                        ui.label(RichText::new("WEYSKO").color(WHITE).strong().size(14.5));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            Frame::new()
                                .fill(BG_SURFACE)
                                .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
                                .corner_radius(CornerRadius::same(4))
                                .inner_margin(Margin::symmetric(6, 2))
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(concat!("v", env!("CARGO_PKG_VERSION")))
                                            .color(TEXT_MUTED)
                                            .size(10.0),
                                    );
                                });
                        });
                    });

                    ui.add_space(18.0);

                    if ui
                        .add_sized(
                            [ui.available_width(), 34.0],
                            egui::Button::new(
                                RichText::new("+ New Website")
                                    .color(SLATE_INK)
                                    .strong()
                                    .size(12.5),
                            )
                            .fill(WHITE)
                            .stroke(Stroke::new(1.0_f32, WHITE))
                            .corner_radius(CornerRadius::same(8)),
                        )
                        .clicked()
                    {
                        state.home_tab = HomeSectionTab::NewFromScratch;
                    }

                    ui.add_space(8.0);

                    if ui
                        .add_sized(
                            [ui.available_width(), 32.0],
                            egui::Button::new(
                                RichText::new("Import .weysko...").color(WHITE).size(12.0),
                            )
                            .fill(BG_SURFACE)
                            .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
                            .corner_radius(CornerRadius::same(8)),
                        )
                        .clicked()
                    {
                        open_project_dialog(state);
                    }

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(10.0);

                    for (tab, label, sub) in [
                        (HomeSectionTab::Home, "Home", "Recents & quick start"),
                        (HomeSectionTab::Templates, "Templates", "Multi-page starter kits"),
                        (
                            HomeSectionTab::NewFromScratch,
                            "New from Scratch",
                            "Custom site skeleton",
                        ),
                    ] {
                        let active = state.home_tab == tab;
                        let frame = Frame::new()
                            .fill(if active {
                                BG_SURFACE
                            } else {
                                Color32::TRANSPARENT
                            })
                            .stroke(Stroke::new(
                                1.0_f32,
                                if active {
                                    BORDER_STRONG
                                } else {
                                    Color32::TRANSPARENT
                                },
                            ))
                            .corner_radius(CornerRadius::same(8))
                            .inner_margin(Margin::symmetric(10, 8));

                        let resp = frame
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                ui.label(
                                    RichText::new(label)
                                        .color(if active { WHITE } else { TEXT_MUTED })
                                        .strong()
                                        .size(12.5),
                                );
                                ui.label(RichText::new(sub).color(TEXT_DIM).size(10.5));
                            })
                            .response
                            .interact(egui::Sense::click());

                        if resp.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                        if resp.clicked() {
                            state.home_tab = tab;
                        }
                        ui.add_space(4.0);
                    }

                    if state.has_active_session {
                        ui.add_space(12.0);
                        ui.separator();
                        ui.add_space(10.0);
                        if ui
                            .add_sized(
                                [ui.available_width(), 32.0],
                                egui::Button::new(
                                    RichText::new(format!(
                                        "<- Back to {}",
                                        truncate_str(&state.project.name, 12)
                                    ))
                                    .color(CYAN_BRAND)
                                    .strong()
                                    .size(11.5),
                                )
                                .fill(BG_INPUT)
                                .stroke(Stroke::new(1.0_f32, CYAN_BRAND))
                                .corner_radius(CornerRadius::same(8)),
                            )
                            .clicked()
                        {
                            state.screen = AppScreen::Editor;
                        }
                    }

                    ui.add_space(18.0);
                    ui.label(
                        RichText::new("Pure Rust • Zero AI • Local-First")
                            .color(TEXT_DIM)
                            .size(10.0),
                    );
                    ui.label(
                        RichText::new(".weysko & .weysko.json supported")
                            .color(TEXT_DIM)
                            .size(10.0),
                    );
                });
        });
}

fn draw_home_top_header(ui: &mut Ui, state: &mut StudioState, avail_w: f32) {
    if avail_w >= 860.0 {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    RichText::new("Start designing in Weysko Studio")
                        .color(WHITE)
                        .strong()
                        .size(21.0),
                );
                ui.label(
                    RichText::new("Choose a visual style, launch a template, build from scratch, or open a saved .weysko project.")
                        .color(TEXT_MUTED)
                        .size(12.5),
                );
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if secondary_slate_button(ui, "Open Example: Meridian Cloud").clicked() {
                    state.launch_new_from_template(
                        StarterTemplate::MeridianDemo,
                        state.home_selected_theme,
                        None,
                    );
                }
                ui.add(
                    egui::TextEdit::singleline(&mut state.home_search)
                        .hint_text("Search templates or recents...")
                        .desired_width(196.0),
                );
            });
        });
    } else {
        ui.vertical(|ui| {
            ui.label(
                RichText::new("Start designing in Weysko Studio")
                    .color(WHITE)
                    .strong()
                    .size(18.5),
            );
            ui.label(
                RichText::new(
                    "Choose a style, launch a template, or open a saved .weysko project.",
                )
                .color(TEXT_MUTED)
                .size(12.0),
            );
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(8.0, 6.0);
                let search_w = (avail_w - 220.0).clamp(150.0, 260.0);
                ui.add(
                    egui::TextEdit::singleline(&mut state.home_search)
                        .hint_text("Search templates or recents...")
                        .desired_width(search_w),
                );
                if secondary_slate_button(ui, "Meridian Cloud Demo").clicked() {
                    state.launch_new_from_template(
                        StarterTemplate::MeridianDemo,
                        state.home_selected_theme,
                        None,
                    );
                }
            });
        });
    }
}

fn draw_style_picker_strip(ui: &mut Ui, state: &mut StudioState) {
    let avail_w = ui.available_width();
    if avail_w >= 540.0 {
        ui.horizontal(|ui| {
            section_heading(ui, "1. Choose Your Website Style");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new(format!(
                        "Selected: {} • Font: {}",
                        state.home_selected_theme.label(),
                        state.home_selected_theme.to_tokens().typography.heading_font
                    ))
                    .color(CYAN_BRAND)
                    .size(11.5),
                );
            });
        });
    } else {
        section_heading(ui, "1. Choose Your Website Style");
        ui.label(
            RichText::new(format!(
                "Selected: {} • {}",
                state.home_selected_theme.label(),
                state.home_selected_theme.to_tokens().typography.heading_font
            ))
            .color(CYAN_BRAND)
            .size(11.5),
        );
    }

    ui.add_space(4.0);
    // Reserve 14px for vertical scrollbar gutter so rightmost column never clips
    let usable_w = (avail_w - 14.0).max(200.0);
    let cols = if usable_w >= 920.0 {
        6
    } else if usable_w >= 520.0 {
        3
    } else {
        2
    };
    let card_outer_w = ((usable_w - (cols as f32 - 1.0) * 10.0) / cols as f32).floor();
    let inner_w = (card_outer_w - 22.0).max(80.0);

    let presets = PresetTheme::all();
    for row in presets.chunks(cols) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 10.0;
            for preset in row {
                let selected = state.home_selected_theme == *preset;
                let tokens = preset.to_tokens();
                ui.allocate_ui_with_layout(
                    egui::vec2(card_outer_w, 78.0),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        ui.set_max_width(card_outer_w);
                        let frame = Frame::new()
                            .fill(if selected { BG_SURFACE } else { BG_PANEL })
                            .stroke(Stroke::new(
                                if selected { 1.6_f32 } else { 1.0_f32 },
                                if selected { WHITE } else { BORDER_STRONG },
                            ))
                            .corner_radius(CornerRadius::same(9))
                            .inner_margin(Margin::same(10));

                        let resp = frame
                            .show(ui, |ui| {
                                ui.set_width(inner_w);
                                ui.set_max_width(inner_w);
                                let (swatch_rect, _) = ui.allocate_exact_size(
                                    egui::vec2(inner_w, 22.0),
                                    egui::Sense::hover(),
                                );
                                paint_theme_preview_bar(ui.painter(), swatch_rect, &tokens);
                                ui.add_space(6.0);
                                ui.label(
                                    RichText::new(preset.label())
                                        .color(WHITE)
                                        .strong()
                                        .size(12.0),
                                );
                                ui.label(
                                    RichText::new(&tokens.typography.heading_font)
                                        .color(TEXT_DIM)
                                        .size(10.5),
                                );
                            })
                            .response
                            .interact(egui::Sense::click());

                        if resp.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                        if resp.clicked() {
                            state.home_selected_theme = *preset;
                        }
                    },
                );
            }
        });
        ui.add_space(10.0);
    }
}

fn draw_home_overview(ui: &mut Ui, state: &mut StudioState) {
    section_heading(ui, "2. Start a New Site in This Style");
    ui.add_space(4.0);
    draw_template_cards_grid(ui, state, true);

    ui.add_space(20.0);
    let avail_w = ui.available_width();
    ui.horizontal(|ui| {
        section_heading(ui, "3. Past Projects");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if secondary_slate_button(ui, "Import .weysko File").clicked() {
                open_project_dialog(state);
            }
        });
    });
    ui.add_space(6.0);

    let q = state.home_search.trim().to_ascii_lowercase();
    let entries: Vec<_> = state
        .recent_projects
        .entries
        .iter()
        .filter(|e| {
            q.is_empty()
                || e.name.to_ascii_lowercase().contains(&q)
                || e.path.to_string_lossy().to_ascii_lowercase().contains(&q)
        })
        .cloned()
        .collect();

    if entries.is_empty() {
        Frame::new()
            .fill(BG_PANEL)
            .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .show(ui, |ui| {
                ui.set_width((avail_w - 48.0).max(180.0));
                ui.set_max_width((avail_w - 48.0).max(180.0));
                if avail_w >= 720.0 {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new("No past .weysko projects recorded yet")
                                    .color(WHITE)
                                    .strong()
                                    .size(13.5),
                            );
                            ui.label(
                                RichText::new("Save any site with Ctrl+S or Export -> .weysko File, or import a file.")
                                    .color(TEXT_MUTED)
                                    .size(12.0),
                            );
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if primary_white_button(ui, "Open Meridian Cloud Example").clicked() {
                                state.launch_new_from_template(
                                    StarterTemplate::MeridianDemo,
                                    state.home_selected_theme,
                                    None,
                                );
                            }
                            if secondary_slate_button(ui, "Import .weysko...").clicked() {
                                open_project_dialog(state);
                            }
                        });
                    });
                } else {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("No past .weysko projects recorded yet")
                                .color(WHITE)
                                .strong()
                                .size(13.5),
                        );
                        ui.label(
                            RichText::new("Save any site with Ctrl+S or import an existing .weysko project.")
                                .color(TEXT_MUTED)
                                .size(12.0),
                        );
                        ui.add_space(10.0);
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(8.0, 6.0);
                            if primary_white_button(ui, "Open Meridian Cloud Example").clicked() {
                                state.launch_new_from_template(
                                    StarterTemplate::MeridianDemo,
                                    state.home_selected_theme,
                                    None,
                                );
                            }
                            if secondary_slate_button(ui, "Import .weysko...").clicked() {
                                open_project_dialog(state);
                            }
                        });
                    });
                }
            });
        return;
    }

    let mut open_path: Option<PathBuf> = None;
    let mut remove_path: Option<PathBuf> = None;
    let row_w = (avail_w - 44.0).max(180.0);
    let max_path_chars = if avail_w < 720.0 { 38 } else { 64 };

    for entry in &entries {
        let exists = entry.path.exists();
        let path_display = truncate_path_middle(&entry.path, max_path_chars);
        let meta_line = format!(
            "{}  •  {} page(s), {} elements  •  {}",
            path_display,
            entry.page_count,
            entry.element_count,
            RecentProjectsStore::relative_time_label(entry.opened_at_unix)
        );

        Frame::new()
            .fill(BG_PANEL)
            .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::symmetric(14, 10))
            .show(ui, |ui| {
                ui.set_width(row_w);
                if avail_w >= 640.0 {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(&entry.name).color(WHITE).strong().size(13.5));
                                Frame::new()
                                    .fill(BG_SURFACE)
                                    .corner_radius(CornerRadius::same(4))
                                    .inner_margin(Margin::symmetric(6, 2))
                                    .show(ui, |ui| {
                                        ui.label(
                                            RichText::new(&entry.theme_label)
                                                .color(ICE_ACCENT)
                                                .size(10.5),
                                        );
                                    });
                                if !exists {
                                    ui.label(
                                        RichText::new("(moved/missing)")
                                            .color(DANGER_INK)
                                            .size(10.5),
                                    );
                                }
                            });
                            ui.label(RichText::new(&meta_line).color(TEXT_DIM).size(11.0));
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .small_button(RichText::new("×").color(TEXT_MUTED))
                                .on_hover_text("Remove from recent list")
                                .clicked()
                            {
                                remove_path = Some(entry.path.clone());
                            }
                            ui.add_enabled_ui(exists, |ui| {
                                if primary_white_button(ui, "Open").clicked() {
                                    open_path = Some(entry.path.clone());
                                }
                            });
                        });
                    });
                } else {
                    ui.vertical(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(RichText::new(&entry.name).color(WHITE).strong().size(13.5));
                            Frame::new()
                                .fill(BG_SURFACE)
                                .corner_radius(CornerRadius::same(4))
                                .inner_margin(Margin::symmetric(6, 2))
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(&entry.theme_label)
                                            .color(ICE_ACCENT)
                                            .size(10.5),
                                    );
                                });
                            if !exists {
                                ui.label(
                                    RichText::new("(moved/missing)")
                                        .color(DANGER_INK)
                                        .size(10.5),
                                );
                            }
                        });
                        ui.label(RichText::new(&meta_line).color(TEXT_DIM).size(11.0));
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.add_enabled_ui(exists, |ui| {
                                if primary_white_button(ui, "Open").clicked() {
                                    open_path = Some(entry.path.clone());
                                }
                            });
                            if ui.small_button(RichText::new("Remove").color(TEXT_MUTED)).clicked()
                            {
                                remove_path = Some(entry.path.clone());
                            }
                        });
                    });
                }
            });
        ui.add_space(6.0);
    }

    if let Some(p) = remove_path {
        state.recent_projects.remove(&p);
        state.recent_projects.save();
    }
    if let Some(p) = open_path {
        if let Err(e) = state.open_project_from_path(&p) {
            state.notify(e, true);
        }
    }
}

fn draw_templates_gallery(ui: &mut Ui, state: &mut StudioState) {
    section_heading(ui, "Starter Website Templates");
    ui.label(
        RichText::new(
            "Every template adopts your selected Website Style above before opening in the Studio canvas.",
        )
        .color(TEXT_MUTED)
        .size(12.0),
    );
    ui.add_space(8.0);
    draw_template_cards_grid(ui, state, false);
}

fn draw_template_cards_grid(ui: &mut Ui, state: &mut StudioState, compact: bool) {
    let q = state.home_search.trim().to_ascii_lowercase();
    let templates: Vec<StarterTemplate> = StarterTemplate::all()
        .iter()
        .copied()
        .filter(|t| {
            q.is_empty()
                || t.label().to_ascii_lowercase().contains(&q)
                || t.description().to_ascii_lowercase().contains(&q)
        })
        .collect();

    let usable_w = (ui.available_width() - 14.0).max(220.0);
    let cols = if usable_w >= 860.0 {
        3
    } else if usable_w >= 520.0 {
        2
    } else {
        1
    };
    let card_outer_w = ((usable_w - (cols as f32 - 1.0) * 12.0) / cols as f32).floor();
    let inner_w = (card_outer_w - 26.0).max(150.0);
    let card_min_h = if compact { 184.0 } else { 206.0 };
    let tokens = state.home_selected_theme.to_tokens();

    for row in templates.chunks(cols) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 12.0;
            for tmpl in row {
                let is_example = *tmpl == StarterTemplate::MeridianDemo;
                ui.allocate_ui_with_layout(
                    egui::vec2(card_outer_w, card_min_h),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        ui.set_max_width(card_outer_w);
                        let frame = Frame::new()
                            .fill(BG_PANEL)
                            .stroke(Stroke::new(
                                1.0_f32,
                                if is_example { CYAN_BRAND } else { BORDER_STRONG },
                            ))
                            .corner_radius(CornerRadius::same(10))
                            .inner_margin(Margin::same(12));

                        frame.show(ui, |ui| {
                            ui.set_width(inner_w);
                            ui.set_max_width(inner_w);
                            ui.set_min_height(card_min_h - 26.0);
                            let preview_h = if compact { 60.0 } else { 80.0 };
                            let (thumb_rect, _) = ui.allocate_exact_size(
                                egui::vec2(inner_w, preview_h),
                                egui::Sense::hover(),
                            );
                            paint_template_wireframe(ui.painter(), thumb_rect, *tmpl, &tokens);

                            ui.add_space(8.0);
                            ui.horizontal_wrapped(|ui| {
                                ui.label(
                                    RichText::new(tmpl.label())
                                        .color(WHITE)
                                        .strong()
                                        .size(13.0),
                                );
                                if is_example {
                                    Frame::new()
                                        .fill(Color32::from_rgb(7, 24, 45))
                                        .stroke(Stroke::new(1.0_f32, CYAN_BRAND))
                                        .corner_radius(CornerRadius::same(4))
                                        .inner_margin(Margin::symmetric(5, 1))
                                        .show(ui, |ui| {
                                            ui.label(
                                                RichText::new("DEMO")
                                                    .color(CYAN_BRAND)
                                                    .strong()
                                                    .size(9.5),
                                            );
                                        });
                                }
                            });
                            ui.label(
                                RichText::new(tmpl.description())
                                    .color(TEXT_MUTED)
                                    .size(11.5),
                            );
                            ui.add_space(8.0);
                            if primary_white_button(ui, "Use Template ->").clicked() {
                                state.launch_new_from_template(
                                    *tmpl,
                                    state.home_selected_theme,
                                    None,
                                );
                            }
                        });
                    },
                );
            }
        });
        ui.add_space(12.0);
    }
}

fn draw_new_from_scratch(ui: &mut Ui, state: &mut StudioState) {
    let selected_theme = state.home_selected_theme;
    let tokens = selected_theme.to_tokens();
    let avail_w = (ui.available_width() - 16.0).max(220.0);
    let two_col = avail_w >= 720.0;

    let draw_form = |ui: &mut Ui, state: &mut StudioState, card_w: f32| {
        Frame::new()
            .fill(BG_PANEL)
            .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .show(ui, |ui| {
                ui.set_width((card_w - 34.0).max(180.0));
                section_heading(ui, "New Custom Website Details");
                ui.add_space(6.0);

                ui.label(RichText::new("Website Name").color(TEXT_MUTED).size(12.0));
                ui.add(
                    egui::TextEdit::singleline(&mut state.scratch_site_name)
                        .hint_text("My Studio Site")
                        .desired_width(f32::INFINITY),
                );
                ui.add_space(8.0);

                ui.label(
                    RichText::new("Publish Base URL (optional)")
                        .color(TEXT_MUTED)
                        .size(12.0),
                );
                ui.add(
                    egui::TextEdit::singleline(&mut state.scratch_base_url)
                        .hint_text("https://example.com")
                        .desired_width(f32::INFINITY),
                );
                ui.add_space(12.0);

                ui.label(
                    RichText::new("Starting Canvas Structure")
                        .color(TEXT_MUTED)
                        .size(12.0),
                );
                ui.add_space(4.0);
                for (mode, title, desc) in [
                    (
                        0usize,
                        "Blank Canvas (Single Page)",
                        "Sticky header, empty 400px strip & footer",
                    ),
                    (
                        1usize,
                        "Landing Page Skeleton",
                        "Header navbar, centered hero, 3-column features & footer",
                    ),
                    (
                        2usize,
                        "3-Page Multi-Page Starter",
                        "Home, About, and Contact pages wired together cleanly",
                    ),
                ] {
                    ui.radio_value(
                        &mut state.scratch_pages_mode,
                        mode,
                        RichText::new(title).color(WHITE).strong().size(12.5),
                    );
                    ui.label(RichText::new(format!("    {}", desc)).color(TEXT_DIM).size(11.0));
                    ui.add_space(4.0);
                }

                ui.add_space(12.0);
                if primary_white_button(ui, "Create & Open in Studio ->").clicked() {
                    state.create_from_scratch();
                }
            });
    };

    let draw_preview = |ui: &mut Ui, card_w: f32| {
        Frame::new()
            .fill(BG_PANEL)
            .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .show(ui, |ui| {
                let inner_w = (card_w - 34.0).max(180.0);
                ui.set_width(inner_w);
                section_heading(ui, "Live Style Preview");
                ui.add_space(6.0);
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(inner_w, 184.0), egui::Sense::hover());
                paint_template_wireframe(
                    ui.painter(),
                    rect,
                    StarterTemplate::SaasLanding,
                    &tokens,
                );
                ui.add_space(10.0);
                ui.label(
                    RichText::new(format!("Style: {}", selected_theme.label()))
                        .color(WHITE)
                        .strong()
                        .size(13.0),
                );
                ui.label(
                    RichText::new(selected_theme.description())
                        .color(TEXT_MUTED)
                        .size(11.5),
                );
            });
    };

    if two_col {
        let half_w = (avail_w - 14.0) * 0.5;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 14.0;
            draw_form(ui, state, half_w);
            draw_preview(ui, half_w);
        });
    } else {
        draw_form(ui, state, avail_w);
        ui.add_space(12.0);
        draw_preview(ui, avail_w);
    }
}

fn paint_theme_preview_bar(painter: &egui::Painter, rect: Rect, tokens: &DesignTokens) {
    let bg = hex_to_color32(&tokens.palette.background);
    let surf = hex_to_color32(&tokens.palette.surface);
    let pri = hex_to_color32(&tokens.palette.primary);
    let txt = hex_to_color32(&tokens.palette.text);
    let border = hex_to_color32(&tokens.palette.border);

    let w = rect.width() / 4.0;
    let cols = [bg, surf, pri, txt];
    for (i, c) in cols.iter().enumerate() {
        let seg = Rect::from_min_size(
            Pos2::new(rect.min.x + w * i as f32, rect.min.y),
            egui::vec2(w, rect.height()),
        );
        painter.rect_filled(seg, CornerRadius::same(if i == 0 || i == 3 { 4 } else { 0 }), *c);
    }
    painter.rect_stroke(
        rect,
        CornerRadius::same(4),
        Stroke::new(1.0_f32, border),
        egui::StrokeKind::Inside,
    );
}

fn paint_template_wireframe(
    painter: &egui::Painter,
    rect: Rect,
    tmpl: StarterTemplate,
    tokens: &DesignTokens,
) {
    let bg = hex_to_color32(&tokens.palette.background);
    let surf = hex_to_color32(&tokens.palette.surface);
    let pri = hex_to_color32(&tokens.palette.primary);
    let txt = hex_to_color32(&tokens.palette.text);
    let muted = hex_to_color32(&tokens.palette.text_muted);
    let border = hex_to_color32(&tokens.palette.border);

    painter.rect_filled(rect, CornerRadius::same(6), bg);
    painter.rect_stroke(
        rect,
        CornerRadius::same(6),
        Stroke::new(1.0_f32, border),
        egui::StrokeKind::Inside,
    );

    let inner = rect.shrink(8.0);
    painter.rect_filled(
        Rect::from_min_size(inner.min, egui::vec2(inner.width(), 8.0)),
        CornerRadius::same(2),
        surf,
    );
    painter.rect_filled(
        Rect::from_min_size(
            Pos2::new(inner.max.x - 24.0, inner.min.y + 1.5),
            egui::vec2(20.0, 5.0),
        ),
        CornerRadius::same(2),
        pri,
    );

    let body = Rect::from_min_max(Pos2::new(inner.min.x, inner.min.y + 13.0), inner.max);

    match tmpl {
        StarterTemplate::MinimalistPortfolio
        | StarterTemplate::MeridianDemo
        | StarterTemplate::PersonalBio => {
            let left_w = body.width() * 0.52;
            painter.rect_filled(
                Rect::from_min_size(body.min, egui::vec2(left_w * 0.85, 7.0)),
                2.0,
                txt,
            );
            painter.rect_filled(
                Rect::from_min_size(
                    Pos2::new(body.min.x, body.min.y + 11.0),
                    egui::vec2(left_w * 0.70, 4.0),
                ),
                2.0,
                muted,
            );
            painter.rect_filled(
                Rect::from_min_size(Pos2::new(body.min.x, body.min.y + 20.0), egui::vec2(38.0, 8.0)),
                3.0,
                pri,
            );
            painter.rect_filled(
                Rect::from_min_size(
                    Pos2::new(body.min.x + left_w + 8.0, body.min.y),
                    egui::vec2(body.width() - left_w - 8.0, body.height() * 0.62),
                ),
                3.0,
                surf,
            );
            let card_y = body.min.y + body.height() * 0.70;
            let card_h = (body.max.y - card_y).max(8.0);
            let cw = (body.width() - 10.0) / 3.0;
            for i in 0..3 {
                painter.rect_filled(
                    Rect::from_min_size(
                        Pos2::new(body.min.x + (cw + 5.0) * i as f32, card_y),
                        egui::vec2(cw, card_h),
                    ),
                    2.0,
                    surf,
                );
            }
        }
        StarterTemplate::SaasLanding | StarterTemplate::StudioAgency => {
            let cx = body.center().x;
            painter.rect_filled(
                Rect::from_center_size(
                    Pos2::new(cx, body.min.y + 7.0),
                    egui::vec2(body.width() * 0.62, 7.0),
                ),
                2.0,
                txt,
            );
            painter.rect_filled(
                Rect::from_center_size(
                    Pos2::new(cx, body.min.y + 18.0),
                    egui::vec2(body.width() * 0.42, 4.0),
                ),
                2.0,
                muted,
            );
            painter.rect_filled(
                Rect::from_center_size(Pos2::new(cx, body.min.y + 29.0), egui::vec2(44.0, 8.0)),
                3.0,
                pri,
            );
            let row_y = body.min.y + 42.0;
            let row_h = (body.max.y - row_y).max(10.0);
            let cw = (body.width() - 10.0) / 3.0;
            for i in 0..3 {
                painter.rect_filled(
                    Rect::from_min_size(
                        Pos2::new(body.min.x + (cw + 5.0) * i as f32, row_y),
                        egui::vec2(cw, row_h),
                    ),
                    3.0,
                    surf,
                );
            }
        }
        StarterTemplate::BlankCanvas => {
            painter.rect_stroke(
                body.shrink(2.0),
                CornerRadius::same(4),
                Stroke::new(1.0_f32, border),
                egui::StrokeKind::Inside,
            );
        }
    }
}

fn truncate_str(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max_chars.saturating_sub(1)).collect();
        out.push('…');
        out
    }
}

/// Truncates long file paths cleanly in the middle while keeping filename visibility.
pub fn truncate_path_middle(path: &Path, max_chars: usize) -> String {
    let raw = path.to_string_lossy();
    let count = raw.chars().count();
    if count <= max_chars || max_chars < 10 {
        return raw.into_owned();
    }
    let keep_head = (max_chars - 3) / 3;
    let keep_tail = max_chars - 3 - keep_head;
    let head: String = raw.chars().take(keep_head).collect();
    let tail: String = raw
        .chars()
        .skip(count.saturating_sub(keep_tail))
        .collect();
    format!("{}...{}", head, tail)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_path_middle_preserves_short_and_compacts_long() {
        let short = Path::new("site.weysko");
        assert_eq!(truncate_path_middle(short, 38), "site.weysko");

        let long = Path::new(r"C:\Users\developer\Documents\clients\meridian\meridian-cloud.weysko");
        let compacted = truncate_path_middle(long, 32);
        assert!(compacted.chars().count() <= 32);
        assert!(compacted.contains("..."));
        assert!(compacted.ends_with(".weysko"));
    }
}
