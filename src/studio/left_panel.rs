use std::fs;

use egui::{Color32, CornerRadius, Frame, Margin, RichText, ScrollArea, Stroke, Ui};

use crate::model::{available_font_families, mime_from_extension, AssetItem, ElementKind, PresetTheme};
use crate::templates::{SectionPreset, StarterTemplate};

use super::state::{DrawerTab, StudioState};
use super::style::{
    color_picker_hex, ghost_button, primary_white_button, secondary_slate_button, section_heading,
    slate_card_frame, BG_INPUT, BG_PANEL, BG_SURFACE, BORDER_STRONG, BORDER_SUBTLE, DANGER_INK,
    ICE_ACCENT, SLATE_INK, TEXT_DIM, TEXT_MUTED, TEXT_PRIMARY, WHITE,
};

pub fn draw_left_panel(ctx: &egui::Context, state: &mut StudioState) {
    // 1. Wix-style narrow vertical icon rail
    egui::SidePanel::left("websir_left_rail")
        .exact_width(54.0)
        .resizable(false)
        .frame(
            Frame::new()
                .fill(BG_PANEL)
                .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
                .inner_margin(Margin::symmetric(6, 10)),
        )
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.spacing_mut().item_spacing.y = 8.0;
                for tab in DrawerTab::all() {
                    let active = state.active_drawer == Some(*tab);
                    let btn_frame = Frame::new()
                        .fill(if active { WHITE } else { BG_SURFACE })
                        .stroke(Stroke::new(1.0_f32, if active { WHITE } else { BORDER_STRONG }))
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::symmetric(4, 6));

                    let resp = btn_frame
                        .show(ui, |ui| {
                            ui.set_width(36.0);
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    RichText::new(tab.icon())
                                        .size(12.5)
                                        .strong()
                                        .color(if active { SLATE_INK } else { WHITE }),
                                );
                                ui.label(
                                    RichText::new(tab.short_label())
                                        .size(9.5)
                                        .color(if active { SLATE_INK } else { TEXT_MUTED }),
                                );
                            });
                        })
                        .response
                        .interact(egui::Sense::click());

                    if resp.clicked() {
                        if active {
                            state.active_drawer = None;
                        } else {
                            state.active_drawer = Some(*tab);
                        }
                    }
                }
            });
        });

    // 2. Resizable Collapsible Drawer
    if let Some(tab) = state.active_drawer {
        egui::SidePanel::left("websir_left_drawer")
            .default_width(244.0)
            .width_range(200.0..=340.0)
            .resizable(true)
            .frame(
                Frame::new()
                    .fill(BG_PANEL)
                    .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
                    .inner_margin(Margin::same(12)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let title = match tab {
                        DrawerTab::Add => "Add Elements",
                        DrawerTab::Sections => "Section Strips",
                        DrawerTab::Templates => "Site Templates",
                        DrawerTab::Layers => "Page Layers",
                        DrawerTab::Theme => "Global Theme",
                        DrawerTab::Assets => "Media Library",
                    };
                    ui.label(RichText::new(title).color(WHITE).strong().size(14.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ghost_button(ui, "x", false).clicked() {
                            state.active_drawer = None;
                        }
                    });
                });
                ui.add_space(6.0);
                ui.separator();
                ui.add_space(6.0);

                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| match tab {
                        DrawerTab::Add => draw_add_elements_drawer(ui, state),
                        DrawerTab::Sections => draw_sections_drawer(ui, state),
                        DrawerTab::Templates => draw_templates_drawer(ui, state),
                        DrawerTab::Layers => draw_layers_drawer(ui, state),
                        DrawerTab::Theme => draw_theme_drawer(ui, state),
                        DrawerTab::Assets => draw_assets_drawer(ui, state),
                    });
            });
    }
}

fn draw_add_elements_drawer(ui: &mut Ui, state: &mut StudioState) {
    ui.add(
        egui::TextEdit::singleline(&mut state.add_search)
            .hint_text("Search 16 elements...")
            .desired_width(f32::INFINITY),
    );
    ui.add_space(6.0);

    let query = state.add_search.trim().to_ascii_lowercase();
    let categories = [
        "Typography & Actions",
        "Media & Social",
        "Layout & Cards",
        "Components & Embeds",
    ];

    for cat in categories {
        let items: Vec<ElementKind> = ElementKind::all()
            .iter()
            .copied()
            .filter(|k| k.category() == cat)
            .filter(|k| query.is_empty() || k.label().to_ascii_lowercase().contains(&query))
            .collect();

        if items.is_empty() {
            continue;
        }

        section_heading(ui, cat);
        let tile_w = ((ui.available_width() - 8.0) * 0.5 - 18.0).max(74.0);
        egui::Grid::new(format!("add_grid_{}", cat))
            .num_columns(2)
            .spacing([8.0, 8.0])
            .show(ui, |ui| {
                for (idx, kind) in items.iter().enumerate() {
                    let tile = Frame::new()
                        .fill(BG_SURFACE)
                        .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
                        .corner_radius(CornerRadius::same(7))
                        .inner_margin(Margin::symmetric(8, 9));

                    let resp = tile
                        .show(ui, |ui| {
                            ui.set_width(tile_w);
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    RichText::new(kind.short_icon())
                                        .color(WHITE)
                                        .strong()
                                        .size(13.0),
                                );
                                ui.add_space(2.0);
                                ui.label(
                                    RichText::new(kind.label())
                                        .color(TEXT_MUTED)
                                        .size(11.0),
                                );
                            });
                        })
                        .response
                        .interact(egui::Sense::click());

                    if resp.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                    if resp.clicked() {
                        state.insert_element(*kind);
                    }

                    if idx % 2 == 1 {
                        ui.end_row();
                    }
                }
            });
        ui.add_space(6.0);
    }
}

fn draw_sections_drawer(ui: &mut Ui, state: &mut StudioState) {
    ui.label(
        RichText::new("Click any pre-built strip to insert it below the selected section.")
            .color(TEXT_MUTED)
            .size(11.5),
    );
    ui.add_space(6.0);

    for preset in SectionPreset::all() {
        slate_card_frame().show(ui, |ui| {
            ui.set_width(ui.available_width());
            // Wireframe thumbnail preview
            let (thumb_rect, _) =
                ui.allocate_exact_size(egui::vec2(ui.available_width(), 34.0), egui::Sense::hover());
            draw_section_wireframe(ui.painter(), thumb_rect, *preset);

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new(preset.label()).color(WHITE).strong().size(12.5));
                    ui.label(
                        RichText::new(preset.category())
                            .color(TEXT_DIM)
                            .size(10.5),
                    );
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if primary_white_button(ui, "+ Insert").clicked() {
                        state.insert_section_preset(*preset);
                    }
                });
            });
            ui.label(
                RichText::new(preset.description())
                    .color(TEXT_MUTED)
                    .size(11.0),
            );
        });
        ui.add_space(6.0);
    }
}

fn draw_section_wireframe(painter: &egui::Painter, rect: egui::Rect, preset: SectionPreset) {
    painter.rect_filled(rect, CornerRadius::same(4), BG_INPUT);
    painter.rect_stroke(
        rect,
        CornerRadius::same(4),
        Stroke::new(1.0_f32, BORDER_SUBTLE),
        egui::StrokeKind::Inside,
    );
    let inner = rect.shrink(6.0);
    match preset {
        SectionPreset::NavbarStrip => {
            painter.rect_filled(
                egui::Rect::from_min_size(inner.min, egui::vec2(36.0, 6.0)),
                2.0,
                WHITE,
            );
            painter.rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(inner.max.x - 42.0, inner.min.y),
                    egui::vec2(42.0, 8.0),
                ),
                3.0,
                ICE_ACCENT,
            );
        }
        SectionPreset::HeroCentered | SectionPreset::CtaBanner => {
            let cx = inner.center().x;
            painter.rect_filled(
                egui::Rect::from_center_size(egui::pos2(cx, inner.min.y + 6.0), egui::vec2(120.0, 6.0)),
                2.0,
                WHITE,
            );
            painter.rect_filled(
                egui::Rect::from_center_size(egui::pos2(cx, inner.min.y + 16.0), egui::vec2(84.0, 4.0)),
                2.0,
                TEXT_DIM,
            );
        }
        SectionPreset::HeroSplit | SectionPreset::AboutSplit | SectionPreset::ContactStrip => {
            painter.rect_filled(
                egui::Rect::from_min_size(inner.min, egui::vec2(inner.width() * 0.45, 7.0)),
                2.0,
                WHITE,
            );
            painter.rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(inner.min.x, inner.min.y + 11.0),
                    egui::vec2(inner.width() * 0.35, 4.0),
                ),
                2.0,
                TEXT_DIM,
            );
            painter.rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(inner.center().x + 8.0, inner.min.y),
                    egui::vec2(inner.width() * 0.44, inner.height()),
                ),
                3.0,
                BG_SURFACE,
            );
        }
        SectionPreset::FeaturesGrid | SectionPreset::PortfolioGallery | SectionPreset::PricingTiers => {
            let col_w = (inner.width() - 10.0) / 3.0;
            for i in 0..3 {
                let x = inner.min.x + (col_w + 5.0) * i as f32;
                painter.rect_filled(
                    egui::Rect::from_min_size(egui::pos2(x, inner.min.y), egui::vec2(col_w, inner.height())),
                    3.0,
                    BG_SURFACE,
                );
            }
        }
        SectionPreset::Testimonials => {
            let col_w = (inner.width() - 6.0) / 2.0;
            for i in 0..2 {
                let x = inner.min.x + (col_w + 6.0) * i as f32;
                painter.rect_filled(
                    egui::Rect::from_min_size(egui::pos2(x, inner.min.y), egui::vec2(col_w, inner.height())),
                    3.0,
                    BG_SURFACE,
                );
            }
        }
        SectionPreset::FaqAccordion => {
            for i in 0..3 {
                let y = inner.min.y + i as f32 * 8.0;
                painter.rect_filled(
                    egui::Rect::from_min_size(egui::pos2(inner.min.x, y), egui::vec2(inner.width(), 5.0)),
                    2.0,
                    BG_SURFACE,
                );
            }
        }
        SectionPreset::FooterStrip => {
            painter.rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(inner.min.x, inner.max.y - 5.0),
                    egui::vec2(inner.width(), 4.0),
                ),
                2.0,
                TEXT_DIM,
            );
        }
        SectionPreset::BlankSection => {}
    }
}

fn draw_templates_drawer(ui: &mut Ui, state: &mut StudioState) {
    ui.label(
        RichText::new("Zero-AI, handcrafted starter sites. Applying replaces current pages (undoable).")
            .color(TEXT_MUTED)
            .size(11.5),
    );
    ui.add_space(6.0);

    for tmpl in StarterTemplate::all() {
        slate_card_frame().show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(RichText::new(tmpl.label()).color(WHITE).strong().size(13.0));
            ui.label(RichText::new(tmpl.description()).color(TEXT_MUTED).size(11.5));
            ui.add_space(6.0);
            if state.confirm_template == Some(*tmpl) {
                ui.horizontal(|ui| {
                    if primary_white_button(ui, "Confirm Apply").clicked() {
                        state.apply_template(*tmpl);
                    }
                    if secondary_slate_button(ui, "Cancel").clicked() {
                        state.confirm_template = None;
                    }
                });
            } else if secondary_slate_button(ui, "Load Template").clicked() {
                if state.dirty {
                    state.confirm_template = Some(*tmpl);
                } else {
                    state.apply_template(*tmpl);
                }
            }
        });
        ui.add_space(6.0);
    }
}

fn draw_layers_drawer(ui: &mut Ui, state: &mut StudioState) {
    let page_opt = state.project.find_page(&state.selected_page_id).cloned();
    let Some(page) = page_opt else {
        ui.label("No active page.");
        return;
    };

    section_heading(ui, &format!("Page: {}", page.title));

    let mut action: Option<LayerAction> = None;

    for (sec_idx, sec) in page.sections.iter().enumerate() {
        let sec_selected =
            state.selected_section_id.as_deref() == Some(&sec.id) && state.selected_element_id.is_none();

        let frame = Frame::new()
            .fill(if sec_selected { BG_SURFACE } else { BG_INPUT })
            .stroke(Stroke::new(
                1.0_f32,
                if sec_selected { WHITE } else { BORDER_SUBTLE },
            ))
            .corner_radius(CornerRadius::same(6))
            .inner_margin(Margin::same(8));

        frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(sec_selected, RichText::new(&sec.name).strong().size(12.0))
                    .clicked()
                {
                    state.selected_section_id = Some(sec.id.clone());
                    state.selected_element_id = None;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.small_button("×").on_hover_text("Delete section").clicked() {
                        action = Some(LayerAction::DeleteSection(sec.id.clone()));
                    }
                    if sec_idx + 1 < page.sections.len()
                        && ui.small_button("↓").on_hover_text("Move section down").clicked()
                    {
                        action = Some(LayerAction::MoveSection(sec_idx, sec_idx + 1));
                    }
                    if sec_idx > 0
                        && ui.small_button("↑").on_hover_text("Move section up").clicked()
                    {
                        action = Some(LayerAction::MoveSection(sec_idx, sec_idx - 1));
                    }
                });
            });

            ui.add_space(4.0);
            let mut sorted = sec.elements.clone();
            sorted.sort_by_key(|e| -e.z_index);

            for el in sorted {
                let el_selected = state.selected_element_id.as_deref() == Some(&el.id);
                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    let eye = if el.hidden { "○" } else { "●" };
                    if ui
                        .small_button(eye)
                        .on_hover_text("Toggle visibility")
                        .clicked()
                    {
                        action = Some(LayerAction::ToggleHidden(el.id.clone()));
                    }
                    let lock = if el.locked { "L" } else { "·" };
                    if ui.small_button(lock).on_hover_text("Toggle lock").clicked() {
                        action = Some(LayerAction::ToggleLock(el.id.clone()));
                    }
                    let label = format!("{}  {}", el.kind.short_icon(), el.name);
                    if ui
                        .selectable_label(
                            el_selected,
                            RichText::new(label)
                                .size(11.5)
                                .color(if el_selected { WHITE } else { TEXT_MUTED }),
                        )
                        .clicked()
                    {
                        state.selected_section_id = Some(sec.id.clone());
                        state.selected_element_id = Some(el.id.clone());
                    }
                });
            }
        });
        ui.add_space(5.0);
    }

    if let Some(act) = action {
        state.push_undo_snapshot();
        if let Some(p) = state.project.find_page_mut(&state.selected_page_id) {
            match act {
                LayerAction::MoveSection(from, to) => {
                    if from < p.sections.len() && to < p.sections.len() {
                        p.sections.swap(from, to);
                    }
                }
                LayerAction::DeleteSection(sid) => {
                    p.sections.retain(|s| s.id != sid);
                    state.reconcile_selection();
                }
                LayerAction::ToggleHidden(eid) => {
                    if let Some(el) = p.find_element_mut(&eid) {
                        el.hidden = !el.hidden;
                    }
                }
                LayerAction::ToggleLock(eid) => {
                    if let Some(el) = p.find_element_mut(&eid) {
                        el.locked = !el.locked;
                    }
                }
            }
        }
    }
}

enum LayerAction {
    MoveSection(usize, usize),
    DeleteSection(String),
    ToggleHidden(String),
    ToggleLock(String),
}

fn draw_theme_drawer(ui: &mut Ui, state: &mut StudioState) {
    section_heading(ui, "Curated Theme Presets");
    for preset in PresetTheme::all() {
        let is_current = state.project.theme.preset == *preset;
        let tokens = preset.to_tokens();
        let frame = Frame::new()
            .fill(if is_current { BG_SURFACE } else { BG_INPUT })
            .stroke(Stroke::new(
                1.0_f32,
                if is_current { WHITE } else { BORDER_STRONG },
            ))
            .corner_radius(CornerRadius::same(7))
            .inner_margin(Margin::same(8));

        let resp = frame
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    for hex in [
                        &tokens.palette.background,
                        &tokens.palette.surface,
                        &tokens.palette.primary,
                        &tokens.palette.text_muted,
                    ] {
                        let (r, _) =
                            ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());
                        ui.painter().rect_filled(
                            r,
                            CornerRadius::same(3),
                            super::style::hex_to_color32(hex),
                        );
                    }
                    ui.label(
                        RichText::new(preset.label())
                            .color(WHITE)
                            .strong()
                            .size(12.0),
                    );
                });
                ui.label(
                    RichText::new(preset.description())
                        .color(TEXT_MUTED)
                        .size(10.5),
                );
            })
            .response
            .interact(egui::Sense::click());

        if resp.clicked() {
            state.push_undo_snapshot();
            state.project.theme = tokens;
            state.notify(format!("Applied {} theme", preset.label()), false);
        }
        ui.add_space(4.0);
    }

    section_heading(ui, "Global Color Palette");
    let before = state.project.clone();
    let mut changed = false;
    {
        let pal = &mut state.project.theme.palette;
        changed |= color_picker_hex(ui, "Background", &mut pal.background);
        changed |= color_picker_hex(ui, "Surface", &mut pal.surface);
        changed |= color_picker_hex(ui, "Surface Alt", &mut pal.surface_alt);
        changed |= color_picker_hex(ui, "Primary CTA", &mut pal.primary);
        changed |= color_picker_hex(ui, "Primary Ink", &mut pal.primary_text);
        changed |= color_picker_hex(ui, "Accent", &mut pal.accent);
        changed |= color_picker_hex(ui, "Text", &mut pal.text);
        changed |= color_picker_hex(ui, "Muted Text", &mut pal.text_muted);
        changed |= color_picker_hex(ui, "Border", &mut pal.border);
    }

    section_heading(ui, "Typography & Layout");
    {
        let typo = &mut state.project.theme.typography;
        ui.horizontal(|ui| {
            ui.label(RichText::new("Heading Font").color(TEXT_MUTED).size(12.0));
            egui::ComboBox::from_id_salt("theme_heading_font")
                .selected_text(&typo.heading_font)
                .show_ui(ui, |ui| {
                    for f in available_font_families() {
                        if ui.selectable_value(&mut typo.heading_font, (*f).into(), *f).changed() {
                            changed = true;
                        }
                    }
                });
        });
        ui.horizontal(|ui| {
            ui.label(RichText::new("Body Font").color(TEXT_MUTED).size(12.0));
            egui::ComboBox::from_id_salt("theme_body_font")
                .selected_text(&typo.body_font)
                .show_ui(ui, |ui| {
                    for f in available_font_families() {
                        if ui.selectable_value(&mut typo.body_font, (*f).into(), *f).changed() {
                            changed = true;
                        }
                    }
                });
        });
        changed |= ui
            .checkbox(&mut typo.use_google_fonts, "Load Google Fonts CDN")
            .changed();
        ui.horizontal(|ui| {
            ui.label(RichText::new("Base Size").color(TEXT_MUTED).size(12.0));
            changed |= ui
                .add(egui::DragValue::new(&mut typo.base_font_size).range(12.0..=24.0).suffix("px"))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label(RichText::new("Line Height").color(TEXT_MUTED).size(12.0));
            changed |= ui
                .add(egui::DragValue::new(&mut typo.line_height).speed(0.02).range(1.1..=2.2))
                .changed();
        });
    }

    ui.horizontal(|ui| {
        ui.label(RichText::new("Max Content W").color(TEXT_MUTED).size(12.0));
        changed |= ui
            .add(
                egui::DragValue::new(&mut state.project.theme.content_max_width)
                    .speed(10.0)
                    .range(800.0..=1600.0)
                    .suffix("px"),
            )
            .changed();
    });
    ui.horizontal(|ui| {
        ui.label(RichText::new("Global Radius").color(TEXT_MUTED).size(12.0));
        changed |= ui
            .add(
                egui::DragValue::new(&mut state.project.theme.border_radius)
                    .range(0.0..=32.0)
                    .suffix("px"),
            )
            .changed();
    });
    changed |= ui
        .checkbox(&mut state.project.theme.smooth_scroll, "Smooth anchor scrolling")
        .changed();

    section_heading(ui, "Custom Global CSS");
    changed |= ui
        .add(
            egui::TextEdit::multiline(&mut state.project.theme.custom_global_css)
                .code_editor()
                .desired_rows(5)
                .desired_width(f32::INFINITY),
        )
        .changed();

    if changed {
        state.record_undo_from(before);
    }
}

fn draw_assets_drawer(ui: &mut Ui, state: &mut StudioState) {
    if primary_white_button(ui, "+ Import Image (PNG/JPG/WebP)").clicked() {
        if let Some(files) = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg", "webp", "gif", "svg"])
            .pick_files()
        {
            state.push_undo_snapshot();
            let mut added = 0;
            for path in files {
                if let Ok(bytes) = fs::read(&path) {
                    let fname = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("image.png");
                    let mime = mime_from_extension(fname);
                    let asset = AssetItem::from_bytes(fname, mime, &bytes);
                    state.project.assets.push(asset);
                    added += 1;
                }
            }
            if added > 0 {
                state.notify(format!("Imported {} image asset(s)", added), false);
            }
        }
    }

    ui.add_space(8.0);

    if state.project.assets.is_empty() {
        ui.label(
            RichText::new("No imported assets yet. Import images to embed them locally in your site & single-file exports.")
                .color(TEXT_MUTED)
                .size(11.5),
        );
        return;
    }

    let mut delete_id: Option<String> = None;
    let mut assign_id: Option<String> = None;

    for asset in &state.project.assets {
        slate_card_frame().show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                if let Some(tex) = state.textures.get(&asset.id) {
                    ui.add(
                        egui::Image::new(tex)
                            .fit_to_exact_size(egui::vec2(48.0, 48.0))
                            .corner_radius(CornerRadius::same(4)),
                    );
                } else {
                    let (r, _) =
                        ui.allocate_exact_size(egui::vec2(48.0, 48.0), egui::Sense::hover());
                    ui.painter().rect_filled(r, CornerRadius::same(4), BG_INPUT);
                }

                ui.vertical(|ui| {
                    ui.label(RichText::new(&asset.name).color(WHITE).strong().size(12.0));
                    ui.label(
                        RichText::new(format!("{}x{} • {}", asset.width, asset.height, asset.mime_type))
                            .color(TEXT_DIM)
                            .size(10.5),
                    );
                    ui.horizontal(|ui| {
                        if ui.small_button("Use on Selected").clicked() {
                            assign_id = Some(asset.id.clone());
                        }
                        if ui
                            .small_button(RichText::new("Delete").color(DANGER_INK))
                            .clicked()
                        {
                            delete_id = Some(asset.id.clone());
                        }
                    });
                });
            });
        });
        ui.add_space(6.0);
    }

    if let Some(aid) = assign_id {
        if let Some(el_id) = state.selected_element_id.clone() {
            state.push_undo_snapshot();
            if let Some(page) = state.project.find_page_mut(&state.selected_page_id) {
                if let Some(el) = page.find_element_mut(&el_id) {
                    el.content.asset_id = Some(aid);
                    state.notify("Assigned image asset to selected element", false);
                }
            }
        } else {
            state.notify("Select an Image element first to assign this asset", true);
        }
    }

    if let Some(del_id) = delete_id {
        state.push_undo_snapshot();
        state.project.assets.retain(|a| a.id != del_id);
        state.textures.remove(&del_id);
        state.notify("Removed asset", false);
    }
    let _ = Color32::TRANSPARENT;
    let _ = TEXT_PRIMARY;
}
