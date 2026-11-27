use std::path::PathBuf;

use egui::{Color32, CornerRadius, Frame, Margin, Pos2, Rect, RichText, ScrollArea, Stroke, Ui};

use crate::model::{DesignTokens, PresetTheme};
use crate::templates::StarterTemplate;

use super::recent::RecentProjectsStore;
use super::state::{AppScreen, HomeSectionTab, StudioState};
use super::style::{
    hex_to_color32, primary_white_button, secondary_slate_button, section_heading, BG_INPUT,
    BG_PANEL, BG_SURFACE, BG_VOID, BORDER_STRONG, BORDER_SUBTLE, DANGER_INK, ICE_ACCENT,
    SLATE_INK, TEXT_DIM, TEXT_MUTED, WHITE,
};
use super::top_bar::open_project_dialog;

const CYAN_BRAND: Color32 = Color32::from_rgb(0, 188, 235);

/// Renders the Adobe Photoshop–style Creative Studio Start / Home screen.
pub fn draw_home_screen(ctx: &egui::Context, state: &mut StudioState) {
    // Left navigation rail matching Adobe Creative Cloud / Photoshop launcher
    egui::SidePanel::left("weysko_home_rail")
        .exact_width(228.0)
        .resizable(false)
        .frame(
            Frame::new()
                .fill(BG_PANEL)
                .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
                .inner_margin(Margin::symmetric(16, 18)),
        )
        .show(ctx, |ui| {
            // Brand lockup + version tag
            ui.horizontal(|ui| {
                let (mark_rect, _) =
                    ui.allocate_exact_size(egui::vec2(28.0, 24.0), egui::Sense::hover());
                super::branding::paint_logo_mark(ui.painter(), mark_rect, WHITE, CYAN_BRAND);
                ui.add_space(4.0);
                ui.label(RichText::new("WEYSKO").color(WHITE).strong().size(15.0));
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
                                    .size(10.5),
                            );
                        });
                });
            });

            ui.add_space(20.0);

            // Primary action buttons
            if ui
                .add_sized(
                    [ui.available_width(), 36.0],
                    egui::Button::new(
                        RichText::new("+ New Website")
                            .color(SLATE_INK)
                            .strong()
                            .size(13.0),
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
                    [ui.available_width(), 34.0],
                    egui::Button::new(
                        RichText::new("Import .weysko...")
                            .color(WHITE)
                            .size(12.5),
                    )
                    .fill(BG_SURFACE)
                    .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
                    .corner_radius(CornerRadius::same(8)),
                )
                .clicked()
            {
                open_project_dialog(state);
            }

            ui.add_space(18.0);
            ui.separator();
            ui.add_space(12.0);

            // Navigation items
            for (tab, label, sub) in [
                (HomeSectionTab::Home, "Home", "Recent projects & quick start"),
                (HomeSectionTab::Templates, "Templates", "Multi-page starter kits"),
                (HomeSectionTab::NewFromScratch, "New from Scratch", "Custom site skeleton"),
            ] {
                let active = state.home_tab == tab;
                let frame = Frame::new()
                    .fill(if active { BG_SURFACE } else { Color32::TRANSPARENT })
                    .stroke(Stroke::new(
                        1.0_f32,
                        if active { BORDER_STRONG } else { Color32::TRANSPARENT },
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
                                .size(13.0),
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
                ui.add_space(14.0);
                ui.separator();
                ui.add_space(12.0);
                if ui
                    .add_sized(
                        [ui.available_width(), 34.0],
                        egui::Button::new(
                            RichText::new(format!("<- Back to {}", truncate_str(&state.project.name, 14)))
                                .color(CYAN_BRAND)
                                .strong()
                                .size(12.0),
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

            // Footer metadata anchored cleanly at bottom
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.label(
                    RichText::new("Pure Rust • Zero AI • Local-First")
                        .color(TEXT_DIM)
                        .size(10.5),
                );
                ui.label(
                    RichText::new(".weysko & .weysko.json supported")
                        .color(TEXT_DIM)
                        .size(10.5),
                );
            });
        });

    // Main Start Screen workspace
    egui::CentralPanel::default()
        .frame(
            Frame::new()
                .fill(BG_VOID)
                .inner_margin(Margin::symmetric(28, 22)),
        )
        .show(ctx, |ui| {
            // Top bar: title + search + Meridian Cloud one-click demo
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Start designing in Weysko Studio")
                            .color(WHITE)
                            .strong()
                            .size(22.0),
                    );
                    ui.label(
                        RichText::new("Choose a visual style, launch a starter template, build from scratch, or open a saved .weysko project.")
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
                            .desired_width(210.0),
                    );
                });
            });

            ui.add_space(16.0);

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

fn draw_style_picker_strip(ui: &mut Ui, state: &mut StudioState) {
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

    ui.add_space(4.0);
    let avail_w = ui.available_width();
    let cols = if avail_w >= 900.0 {
        6
    } else if avail_w >= 580.0 {
        3
    } else {
        2
    };
    let card_w = ((avail_w - (cols as f32 - 1.0) * 10.0) / cols as f32).max(130.0);

    egui::Grid::new("home_style_picker_grid")
        .num_columns(cols)
        .spacing([10.0, 10.0])
        .show(ui, |ui| {
            for (idx, preset) in PresetTheme::all().iter().enumerate() {
                let selected = state.home_selected_theme == *preset;
                let tokens = preset.to_tokens();
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
                        ui.set_width((card_w - 22.0).max(96.0));
                        // Swatch strip showing real theme palette colors
                        let (swatch_rect, _) = ui.allocate_exact_size(
                            egui::vec2(ui.available_width(), 24.0),
                            egui::Sense::hover(),
                        );
                        paint_theme_preview_bar(ui.painter(), swatch_rect, &tokens);
                        ui.add_space(6.0);
                        ui.label(RichText::new(preset.label()).color(WHITE).strong().size(12.0));
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

                if (idx + 1) % cols == 0 {
                    ui.end_row();
                }
            }
        });
}

fn draw_home_overview(ui: &mut Ui, state: &mut StudioState) {
    section_heading(ui, "2. Start a New Site in This Style");
    ui.add_space(4.0);
    draw_template_cards_grid(ui, state, true);

    ui.add_space(22.0);
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
            .inner_margin(Margin::same(20))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("No past .weysko projects recorded yet")
                                .color(WHITE)
                                .strong()
                                .size(14.0),
                        );
                        ui.label(
                            RichText::new("Save any site with Ctrl+S or Export -> .weysko File, or import an existing project to list it here.")
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
            });
        return;
    }

    let mut open_path: Option<PathBuf> = None;
    let mut remove_path: Option<PathBuf> = None;

    for entry in &entries {
        let exists = entry.path.exists();
        Frame::new()
            .fill(BG_PANEL)
            .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::symmetric(14, 10))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
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
                                ui.label(RichText::new("(moved/missing)").color(DANGER_INK).size(10.5));
                            }
                        });
                        ui.label(
                            RichText::new(format!(
                                "{}  •  {} page(s), {} elements  •  {}",
                                entry.path.display(),
                                entry.page_count,
                                entry.element_count,
                                RecentProjectsStore::relative_time_label(entry.opened_at_unix)
                            ))
                            .color(TEXT_DIM)
                            .size(11.0),
                        );
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
        RichText::new("Every template adopts your selected Website Style above before opening in the Studio canvas.")
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

    let avail_w = ui.available_width();
    let cols = if avail_w >= 860.0 {
        3
    } else if avail_w >= 540.0 {
        2
    } else {
        1
    };
    let card_w = ((avail_w - (cols as f32 - 1.0) * 14.0) / cols as f32).max(200.0);
    let tokens = state.home_selected_theme.to_tokens();

    egui::Grid::new(if compact {
        "home_templates_compact"
    } else {
        "home_templates_full"
    })
    .num_columns(cols)
    .spacing([14.0, 14.0])
    .show(ui, |ui| {
        for (idx, tmpl) in templates.iter().enumerate() {
            let is_example = *tmpl == StarterTemplate::MeridianDemo;
            let frame = Frame::new()
                .fill(BG_PANEL)
                .stroke(Stroke::new(
                    1.0_f32,
                    if is_example { CYAN_BRAND } else { BORDER_STRONG },
                ))
                .corner_radius(CornerRadius::same(10))
                .inner_margin(Margin::same(12));

            frame.show(ui, |ui| {
                ui.set_width((card_w - 26.0).max(170.0));
                let preview_h = if compact { 68.0 } else { 88.0 };
                let (thumb_rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), preview_h),
                    egui::Sense::hover(),
                );
                paint_template_wireframe(ui.painter(), thumb_rect, *tmpl, &tokens);

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new(tmpl.label()).color(WHITE).strong().size(13.5));
                    if is_example {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            Frame::new()
                                .fill(Color32::from_rgb(7, 24, 45))
                                .stroke(Stroke::new(1.0_f32, CYAN_BRAND))
                                .corner_radius(CornerRadius::same(4))
                                .inner_margin(Margin::symmetric(6, 1))
                                .show(ui, |ui| {
                                    ui.label(RichText::new("DEMO").color(CYAN_BRAND).strong().size(9.5));
                                });
                        });
                    }
                });
                ui.label(RichText::new(tmpl.description()).color(TEXT_MUTED).size(11.5));
                ui.add_space(8.0);
                if primary_white_button(ui, "Use Template ->").clicked() {
                    state.launch_new_from_template(*tmpl, state.home_selected_theme, None);
                }
            });

            if (idx + 1) % cols == 0 {
                ui.end_row();
            }
        }
    });
}

fn draw_new_from_scratch(ui: &mut Ui, state: &mut StudioState) {
    let selected_theme = state.home_selected_theme;
    let tokens = selected_theme.to_tokens();
    let avail_w = ui.available_width();
    let two_col = avail_w >= 680.0;

    let draw_form = |ui: &mut Ui, state: &mut StudioState| {
        Frame::new()
            .fill(BG_PANEL)
            .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(18))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                section_heading(ui, "New Custom Website Details");
                ui.add_space(6.0);

                ui.label(RichText::new("Website Name").color(TEXT_MUTED).size(12.0));
                ui.add(
                    egui::TextEdit::singleline(&mut state.scratch_site_name)
                        .hint_text("My Studio Site")
                        .desired_width(f32::INFINITY),
                );
                ui.add_space(8.0);

                ui.label(RichText::new("Publish Base URL (optional)").color(TEXT_MUTED).size(12.0));
                ui.add(
                    egui::TextEdit::singleline(&mut state.scratch_base_url)
                        .hint_text("https://example.com")
                        .desired_width(f32::INFINITY),
                );
                ui.add_space(12.0);

                ui.label(RichText::new("Starting Canvas Structure").color(TEXT_MUTED).size(12.0));
                ui.add_space(4.0);
                for (mode, title, desc) in [
                    (0usize, "Blank Canvas (Single Page)", "Sticky header, empty 400px strip & footer"),
                    (1usize, "Landing Page Skeleton", "Header navbar, centered hero, 3-column features & footer"),
                    (2usize, "3-Page Multi-Page Starter", "Home, About, and Contact pages wired together cleanly"),
                ] {
                    ui.radio_value(&mut state.scratch_pages_mode, mode, RichText::new(title).color(WHITE).strong().size(12.5));
                    ui.label(RichText::new(format!("    {}", desc)).color(TEXT_DIM).size(11.0));
                    ui.add_space(4.0);
                }

                ui.add_space(14.0);
                if primary_white_button(ui, "Create & Open in Studio ->").clicked() {
                    state.create_from_scratch();
                }
            });
    };

    let draw_preview = |ui: &mut Ui| {
        Frame::new()
            .fill(BG_PANEL)
            .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(18))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                section_heading(ui, "Live Style Preview");
                ui.add_space(6.0);
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 190.0),
                    egui::Sense::hover(),
                );
                paint_template_wireframe(ui.painter(), rect, StarterTemplate::SaasLanding, &tokens);
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
        ui.columns(2, |cols| {
            draw_form(&mut cols[0], state);
            draw_preview(&mut cols[1]);
        });
    } else {
        draw_form(ui, state);
        ui.add_space(12.0);
        draw_preview(ui);
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
    // Mini navbar bar
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

    let body = Rect::from_min_max(
        Pos2::new(inner.min.x, inner.min.y + 13.0),
        inner.max,
    );

    match tmpl {
        StarterTemplate::MinimalistPortfolio | StarterTemplate::MeridianDemo | StarterTemplate::PersonalBio => {
            // Asymmetric split hero + right graphic block
            let left_w = body.width() * 0.52;
            painter.rect_filled(
                Rect::from_min_size(body.min, egui::vec2(left_w * 0.85, 7.0)),
                2.0,
                txt,
            );
            painter.rect_filled(
                Rect::from_min_size(Pos2::new(body.min.x, body.min.y + 11.0), egui::vec2(left_w * 0.70, 4.0)),
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
            // 3 lower cards
            let card_y = body.min.y + body.height() * 0.70;
            let card_h = (body.max.y - card_y).max(8.0);
            let cw = (body.width() - 10.0) / 3.0;
            for i in 0..3 {
                painter.rect_filled(
                    Rect::from_min_size(Pos2::new(body.min.x + (cw + 5.0) * i as f32, card_y), egui::vec2(cw, card_h)),
                    2.0,
                    surf,
                );
            }
        }
        StarterTemplate::SaasLanding | StarterTemplate::StudioAgency => {
            // Centered hero + 3 pricing/feature columns
            let cx = body.center().x;
            painter.rect_filled(
                Rect::from_center_size(Pos2::new(cx, body.min.y + 7.0), egui::vec2(body.width() * 0.62, 7.0)),
                2.0,
                txt,
            );
            painter.rect_filled(
                Rect::from_center_size(Pos2::new(cx, body.min.y + 18.0), egui::vec2(body.width() * 0.42, 4.0)),
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
                    Rect::from_min_size(Pos2::new(body.min.x + (cw + 5.0) * i as f32, row_y), egui::vec2(cw, row_h)),
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
