use egui::{
    Align2, Color32, CornerRadius, CursorIcon, FontId, Frame, Margin, Pos2, Rect, RichText,
    ScrollArea, Sense, Stroke, Ui, Vec2,
};

use crate::model::{
    Element, ElementBgType, ElementKind, ListStyle, MobileLayoutMode, Rect2D, Section,
    SectionBackground, SiteProject, TextAlign, ViewportMode,
};
use crate::templates::SectionPreset;

use super::state::{DragMode, InlineEditState, ResizeHandle, StudioState};
use super::style::{
    hex_to_color32, primary_white_button, secondary_slate_button, BG_INPUT, BG_PANEL, BG_SURFACE,
    BG_VOID, BORDER_STRONG, DANGER_INK, ICE_ACCENT, SLATE_INK, TEXT_DIM, TEXT_MUTED,
    WHITE,
};

struct GuideLine {
    vertical: bool,
    coord_canvas: f32,
}

enum CanvasAction {
    MoveSection(usize, usize),
    DuplicateSection(String),
    DeleteSection(String),
    SelectElement(String, String),
    SelectSection(String),
    StartInlineEdit(String, String),
    AppendBlankSection,
    AddElementInSection(String, ElementKind),
    DuplicateElement(String),
    CopyElement(String),
    BringToFront(String),
    SendToBack(String),
    ToggleLock(String),
    DeleteElement(String),
}

/// Renders the interactive Wix-style WYSIWYG artboard with drag/resize, smart guides, and inline text editing.
pub fn draw_canvas(ctx: &egui::Context, state: &mut StudioState) {
    egui::CentralPanel::default()
        .frame(Frame::new().fill(BG_VOID).inner_margin(Margin::same(0)))
        .show(ctx, |ui| {
            let panel_rect = ui.max_rect();
            if state.show_grid {
                draw_workspace_dot_grid(ui.painter(), panel_rect);
            }

            let Some(page_snapshot) = state.project.find_page(&state.selected_page_id).cloned() else {
                ui.centered_and_justified(|ui| {
                    ui.label(RichText::new("No active page selected").color(TEXT_MUTED));
                });
                return;
            };

            let vp = state.viewport;
            let vp_width = vp.width_px();
            if state.auto_fit_zoom {
                state.zoom = ((panel_rect.width() - 56.0).max(240.0) / vp_width).clamp(0.35, 1.0);
            }
            let zoom = state.zoom;
            let artboard_w_screen = vp_width * zoom;
            let theme = state.project.theme.clone();
            let assets = state.project.assets.clone();
            let all_pages = state.project.pages.clone();

            let mut pending_action: Option<CanvasAction> = None;
            let mut active_guides: Vec<GuideLine> = Vec::new();
            let mut hover_cursor: Option<CursorIcon> = None;

            // Handle active drag/resize globally on pointer move/release
            handle_active_drag(ctx, state, zoom, vp_width, &mut active_guides);

            ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.add_space(22.0);

                    let avail_w = ui.available_width();
                    let side_pad = ((avail_w - artboard_w_screen) * 0.5).max(28.0);

                    ui.horizontal(|ui| {
                        ui.add_space(side_pad);
                        ui.vertical(|ui| {
                            ui.set_width(artboard_w_screen);

                            // Artboard chrome header
                            Frame::new()
                                .fill(BG_PANEL)
                                .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
                                .corner_radius(CornerRadius {
                                    nw: 8,
                                    ne: 8,
                                    sw: 0,
                                    se: 0,
                                })
                                .inner_margin(Margin::symmetric(12, 6))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new(format!("/{}", page_snapshot.html_filename()))
                                                .font(FontId::monospace(11.5))
                                                .color(WHITE),
                                        );
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.label(
                                                    RichText::new(format!(
                                                        "{} • {}px • {}%",
                                                        vp.label(),
                                                        vp_width as i32,
                                                        (zoom * 100.0).round() as i32
                                                    ))
                                                    .size(11.0)
                                                    .color(TEXT_MUTED),
                                                );
                                            },
                                        );
                                    });
                                });

                            // Render each section strip
                            let sec_count = page_snapshot.sections.len();
                            for (sec_idx, sec) in page_snapshot.sections.iter().enumerate() {
                                let is_sec_selected = state.selected_section_id.as_deref()
                                    == Some(&sec.id)
                                    && state.selected_element_id.is_none();

                                let effective_sec_h = compute_section_display_height(sec, vp);
                                let (sec_rect, sec_resp) = ui.allocate_exact_size(
                                    egui::vec2(artboard_w_screen, effective_sec_h * zoom),
                                    Sense::click(),
                                );

                                let painter = ui.painter_at(sec_rect);
                                draw_section_background(
                                    &painter,
                                    sec_rect,
                                    sec,
                                    &state.textures,
                                    &theme.palette.border,
                                );

                                // Subtle canvas grid lines inside selected section when grid is enabled
                                if state.show_grid && state.selected_section_id.as_deref() == Some(&sec.id) {
                                    let step = 40.0 * zoom;
                                    let mut gx = sec_rect.min.x + step;
                                    while gx < sec_rect.max.x {
                                        painter.line_segment(
                                            [Pos2::new(gx, sec_rect.min.y), Pos2::new(gx, sec_rect.max.y)],
                                            Stroke::new(0.5_f32, Color32::from_rgba_unmultiplied(148, 163, 184, 18)),
                                        );
                                        gx += step;
                                    }
                                }

                                // Compute inner content horizontal mapping (max 1200px on Desktop, full on Mobile/Tablet/full_bleed)
                                let content_w_canvas = if sec.full_bleed || vp != ViewportMode::Desktop {
                                    vp_width
                                } else {
                                    theme.content_max_width.min(vp_width)
                                };
                                let content_origin_x =
                                    sec_rect.min.x + ((vp_width - content_w_canvas) * 0.5) * zoom;
                                let scale_x = content_w_canvas / 1200.0;

                                // Sort elements by z_index
                                let mut elements: Vec<&Element> = sec.elements.iter().collect();
                                elements.sort_by_key(|e| e.z_index);

                                // Track mobile auto-stack Y cursor if needed
                                let mut auto_stack_y = 24.0_f32;
                                let mut clicked_element = false;

                                // Order for auto-stack matches CSS compiler (y, x)
                                let mut mobile_order_map = std::collections::HashMap::new();
                                if vp == ViewportMode::Mobile && sec.mobile_layout == MobileLayoutMode::AutoStack {
                                    let mut sorted_m = elements.clone();
                                    sorted_m.sort_by(|a, b| {
                                        a.rect
                                            .y
                                            .partial_cmp(&b.rect.y)
                                            .unwrap_or(std::cmp::Ordering::Equal)
                                            .then_with(|| {
                                                a.rect
                                                    .x
                                                    .partial_cmp(&b.rect.x)
                                                    .unwrap_or(std::cmp::Ordering::Equal)
                                            })
                                    });
                                    for m_el in sorted_m {
                                        if m_el.hidden || !m_el.visible_mobile {
                                            continue;
                                        }
                                        let h = m_el.mobile_rect.map(|r| r.h).unwrap_or(match m_el.kind {
                                            ElementKind::Image | ElementKind::VideoEmbed => 200.0,
                                            ElementKind::Divider => 2.0,
                                            ElementKind::Navbar => 64.0,
                                            _ => m_el.rect.h.min(280.0),
                                        });
                                        let r = m_el.mobile_rect.unwrap_or(Rect2D::new(
                                            16.0,
                                            auto_stack_y,
                                            vp_width - 32.0,
                                            h,
                                        ));
                                        mobile_order_map.insert(m_el.id.clone(), r);
                                        if m_el.mobile_rect.is_none() {
                                            auto_stack_y += h + 14.0;
                                        }
                                    }
                                }

                                for el in elements {
                                    if el.hidden || !is_visible_on_viewport(el, vp) {
                                        continue;
                                    }

                                    let el_canvas_rect = if let Some(mr) = mobile_order_map.get(&el.id) {
                                        *mr
                                    } else {
                                        resolve_canvas_rect(el, vp, content_w_canvas, scale_x)
                                    };

                                    let el_screen_rect = if vp == ViewportMode::Desktop && !el.full_width {
                                        Rect::from_min_size(
                                            Pos2::new(
                                                content_origin_x + el_canvas_rect.x * scale_x * zoom,
                                                sec_rect.min.y + el_canvas_rect.y * zoom,
                                            ),
                                            egui::vec2(
                                                el_canvas_rect.w * scale_x * zoom,
                                                el_canvas_rect.h * zoom,
                                            ),
                                        )
                                    } else if el.full_width {
                                        Rect::from_min_size(
                                            Pos2::new(
                                                sec_rect.min.x,
                                                sec_rect.min.y + el_canvas_rect.y * zoom,
                                            ),
                                            egui::vec2(artboard_w_screen, el_canvas_rect.h * zoom),
                                        )
                                    } else {
                                        Rect::from_min_size(
                                            Pos2::new(
                                                sec_rect.min.x + el_canvas_rect.x * zoom,
                                                sec_rect.min.y + el_canvas_rect.y * zoom,
                                            ),
                                            egui::vec2(
                                                el_canvas_rect.w * zoom,
                                                el_canvas_rect.h * zoom,
                                            ),
                                        )
                                    };

                                    let is_el_selected =
                                        state.selected_element_id.as_deref() == Some(&el.id);

                                    // Inline text edit overlay if active for this element
                                    let is_inline_editing = state
                                        .inline_edit
                                        .as_ref()
                                        .map(|ie| ie.element_id == el.id)
                                        .unwrap_or(false);

                                    if is_inline_editing {
                                        draw_inline_editor(ui, state, el_screen_rect, el);
                                        clicked_element = true;
                                        continue;
                                    }

                                    // Register 8 resize handles BEFORE element body interaction so corner/edge drags take priority
                                    if is_el_selected && !el.locked {
                                        for (handle, h_pos) in resize_handle_positions(el_screen_rect) {
                                            let h_rect = Rect::from_center_size(h_pos, Vec2::splat(9.0));
                                            let h_resp = ui.interact(
                                                h_rect.expand(4.0),
                                                ui.make_persistent_id(format!(
                                                    "rh_{}_{:?}",
                                                    el.id, handle
                                                )),
                                                Sense::drag(),
                                            );
                                            if h_resp.hovered() {
                                                hover_cursor = Some(cursor_for_handle(handle));
                                            }
                                            if h_resp.drag_started() && state.drag_mode.is_none() {
                                                if let Some(pos) = ctx.pointer_interact_pos() {
                                                    let base_rect = match vp {
                                                        ViewportMode::Desktop => el.rect,
                                                        ViewportMode::Tablet => {
                                                            el.tablet_rect.unwrap_or(el_canvas_rect)
                                                        }
                                                        ViewportMode::Mobile => {
                                                            el.mobile_rect.unwrap_or(el_canvas_rect)
                                                        }
                                                    };
                                                    state.drag_mode = Some(DragMode::ResizeElement {
                                                        section_id: sec.id.clone(),
                                                        element_id: el.id.clone(),
                                                        handle,
                                                        start_rect: base_rect,
                                                        start_pointer: pos,
                                                        pre_drag_snapshot: state.project.clone(),
                                                    });
                                                    clicked_element = true;
                                                }
                                            }
                                        }
                                    }

                                    // Paint element WYSIWYG visuals clipped to element bounds
                                    let el_painter = painter.with_clip_rect(el_screen_rect.intersect(sec_rect));
                                    paint_element(
                                        &el_painter,
                                        el_screen_rect,
                                        el,
                                        zoom,
                                        &state.textures,
                                        &assets,
                                        &theme.palette,
                                        &all_pages,
                                    );

                                    // Element interaction (click / double-click / drag / right-click)
                                    let el_resp = ui.interact(
                                        el_screen_rect,
                                        ui.make_persistent_id(format!("el_hit_{}", el.id)),
                                        Sense::click_and_drag(),
                                    );
                                    el_resp.context_menu(|ui| {
                                        if ui.button("Duplicate (Ctrl+D)").clicked() {
                                            pending_action = Some(CanvasAction::DuplicateElement(el.id.clone()));
                                            ui.close_menu();
                                        }
                                        if ui.button("Copy (Ctrl+C)").clicked() {
                                            pending_action = Some(CanvasAction::CopyElement(el.id.clone()));
                                            ui.close_menu();
                                        }
                                        ui.separator();
                                        if ui.button("Bring to Front").clicked() {
                                            pending_action = Some(CanvasAction::BringToFront(el.id.clone()));
                                            ui.close_menu();
                                        }
                                        if ui.button("Send to Back").clicked() {
                                            pending_action = Some(CanvasAction::SendToBack(el.id.clone()));
                                            ui.close_menu();
                                        }
                                        let lock_lbl = if el.locked { "Unlock Position" } else { "Lock Position" };
                                        if ui.button(lock_lbl).clicked() {
                                            pending_action = Some(CanvasAction::ToggleLock(el.id.clone()));
                                            ui.close_menu();
                                        }
                                        ui.separator();
                                        if ui.button(RichText::new("Delete").color(DANGER_INK)).clicked() {
                                            pending_action = Some(CanvasAction::DeleteElement(el.id.clone()));
                                            ui.close_menu();
                                        }
                                    });

                                    if el_resp.hovered() && hover_cursor.is_none() {
                                        hover_cursor = Some(if el.locked {
                                            CursorIcon::NotAllowed
                                        } else {
                                            CursorIcon::Grab
                                        });
                                        if !is_el_selected {
                                            ui.painter().rect_stroke(
                                                el_screen_rect,
                                                CornerRadius::same(2),
                                                Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(203, 213, 225, 140)),
                                                egui::StrokeKind::Outside,
                                            );
                                        }
                                    }

                                    if el_resp.clicked() {
                                        pending_action = Some(CanvasAction::SelectElement(
                                            sec.id.clone(),
                                            el.id.clone(),
                                        ));
                                        clicked_element = true;
                                    }

                                    if el_resp.double_clicked() && is_inline_editable(el.kind) && !el.locked {
                                        pending_action = Some(CanvasAction::StartInlineEdit(
                                            el.id.clone(),
                                            el.content.text.clone(),
                                        ));
                                        clicked_element = true;
                                    }

                                    if el_resp.drag_started() && !el.locked && state.drag_mode.is_none() {
                                        if let Some(pos) = ctx.pointer_interact_pos() {
                                            let base_rect = match vp {
                                                ViewportMode::Desktop => el.rect,
                                                ViewportMode::Tablet => el.tablet_rect.unwrap_or(el_canvas_rect),
                                                ViewportMode::Mobile => el.mobile_rect.unwrap_or(el_canvas_rect),
                                            };
                                            state.selected_section_id = Some(sec.id.clone());
                                            state.selected_element_id = Some(el.id.clone());
                                            state.drag_mode = Some(DragMode::MoveElement {
                                                section_id: sec.id.clone(),
                                                element_id: el.id.clone(),
                                                start_rect: base_rect,
                                                start_pointer: pos,
                                                pre_drag_snapshot: state.project.clone(),
                                            });
                                            clicked_element = true;
                                        }
                                    }

                                    // Selection bounding box & 8 resize handles painted on unclipped scroll-area painter
                                    if is_el_selected {
                                        let overlay = ui.painter();
                                        overlay.rect_stroke(
                                            el_screen_rect,
                                            CornerRadius::same(2),
                                            Stroke::new(1.5_f32, WHITE),
                                            egui::StrokeKind::Outside,
                                        );

                                        // Element floating label pill
                                        let badge_pos = Pos2::new(
                                            el_screen_rect.min.x,
                                            (el_screen_rect.min.y - 18.0).max(panel_rect.min.y + 4.0),
                                        );
                                        let pill_rect = Rect::from_min_size(
                                            badge_pos,
                                            egui::vec2(124.0, 16.0),
                                        );
                                        overlay.rect_filled(pill_rect, CornerRadius::same(3), WHITE);
                                        overlay.text(
                                            pill_rect.min + egui::vec2(6.0, 8.0),
                                            Align2::LEFT_CENTER,
                                            el.name.chars().take(16).collect::<String>(),
                                            FontId::proportional(10.0),
                                            SLATE_INK,
                                        );

                                        if !el.locked {
                                            for (_, h_pos) in resize_handle_positions(el_screen_rect) {
                                                let h_rect = Rect::from_center_size(h_pos, Vec2::splat(9.0));
                                                overlay.rect_filled(h_rect, CornerRadius::same(2), WHITE);
                                                overlay.rect_stroke(
                                                    h_rect,
                                                    CornerRadius::same(2),
                                                    Stroke::new(1.0_f32, SLATE_INK),
                                                    egui::StrokeKind::Inside,
                                                );
                                            }
                                        }
                                    }
                                }

                                // Smart alignment guide lines
                                for g in &active_guides {
                                    if g.vertical {
                                        let gx = if vp == ViewportMode::Desktop {
                                            content_origin_x + g.coord_canvas * scale_x * zoom
                                        } else {
                                            sec_rect.min.x + g.coord_canvas * zoom
                                        };
                                        painter.line_segment(
                                            [Pos2::new(gx, sec_rect.min.y), Pos2::new(gx, sec_rect.max.y)],
                                            Stroke::new(1.0_f32, ICE_ACCENT),
                                        );
                                    } else {
                                        let gy = sec_rect.min.y + g.coord_canvas * zoom;
                                        painter.line_segment(
                                            [Pos2::new(sec_rect.min.x, gy), Pos2::new(sec_rect.max.x, gy)],
                                            Stroke::new(1.0_f32, ICE_ACCENT),
                                        );
                                    }
                                }

                                // Clicking or right-clicking empty section
                                if !clicked_element && sec_resp.clicked() {
                                    pending_action = Some(CanvasAction::SelectSection(sec.id.clone()));
                                }
                                sec_resp.context_menu(|ui| {
                                    if ui.button("Add Heading").clicked() {
                                        pending_action = Some(CanvasAction::AddElementInSection(
                                            sec.id.clone(),
                                            ElementKind::Heading,
                                        ));
                                        ui.close_menu();
                                    }
                                    if ui.button("Add Paragraph").clicked() {
                                        pending_action = Some(CanvasAction::AddElementInSection(
                                            sec.id.clone(),
                                            ElementKind::Paragraph,
                                        ));
                                        ui.close_menu();
                                    }
                                    if ui.button("Add Button").clicked() {
                                        pending_action = Some(CanvasAction::AddElementInSection(
                                            sec.id.clone(),
                                            ElementKind::Button,
                                        ));
                                        ui.close_menu();
                                    }
                                    if ui.button("Add Card").clicked() {
                                        pending_action = Some(CanvasAction::AddElementInSection(
                                            sec.id.clone(),
                                            ElementKind::Card,
                                        ));
                                        ui.close_menu();
                                    }
                                    ui.separator();
                                    if ui.button("Duplicate Section").clicked() {
                                        pending_action = Some(CanvasAction::DuplicateSection(sec.id.clone()));
                                        ui.close_menu();
                                    }
                                    if ui.button(RichText::new("Delete Section").color(DANGER_INK)).clicked() {
                                        pending_action = Some(CanvasAction::DeleteSection(sec.id.clone()));
                                        ui.close_menu();
                                    }
                                });

                                // Section outline & floating section toolbar
                                let sec_hovered = sec_resp.hovered();
                                if is_sec_selected || sec_hovered {
                                    painter.rect_stroke(
                                        sec_rect,
                                        CornerRadius::ZERO,
                                        Stroke::new(
                                            if is_sec_selected { 1.5_f32 } else { 1.0_f32 },
                                            if is_sec_selected {
                                                WHITE
                                            } else {
                                                Color32::from_rgba_unmultiplied(148, 163, 184, 120)
                                            },
                                        ),
                                        egui::StrokeKind::Inside,
                                    );
                                }

                                if is_sec_selected {
                                    draw_section_floating_toolbar(
                                        ui,
                                        sec_rect,
                                        sec,
                                        sec_idx,
                                        sec_count,
                                        &mut pending_action,
                                    );
                                }

                                // Bottom edge resize handle for section height
                                let bot_handle = Rect::from_min_size(
                                    Pos2::new(sec_rect.center().x - 28.0, sec_rect.max.y - 5.0),
                                    egui::vec2(56.0, 10.0),
                                );
                                let bot_resp = ui.interact(
                                    bot_handle,
                                    ui.make_persistent_id(format!("sec_resize_{}", sec.id)),
                                    Sense::drag(),
                                );
                                if bot_resp.hovered() || is_sec_selected {
                                    painter.rect_filled(
                                        Rect::from_center_size(
                                            Pos2::new(sec_rect.center().x, sec_rect.max.y),
                                            egui::vec2(44.0, 5.0),
                                        ),
                                        CornerRadius::same(3),
                                        if bot_resp.hovered() { WHITE } else { BORDER_STRONG },
                                    );
                                }
                                if bot_resp.hovered() {
                                    hover_cursor = Some(CursorIcon::ResizeVertical);
                                }
                                if bot_resp.drag_started() && state.drag_mode.is_none() {
                                    if let Some(pos) = ctx.pointer_interact_pos() {
                                        state.drag_mode = Some(DragMode::ResizeSection {
                                            section_id: sec.id.clone(),
                                            start_height: sec.height,
                                            start_pointer_y: pos.y,
                                            pre_drag_snapshot: state.project.clone(),
                                        });
                                    }
                                }
                            }

                            ui.add_space(16.0);
                            ui.horizontal(|ui| {
                                if primary_white_button(ui, "+ Add Section Strip").clicked() {
                                    pending_action = Some(CanvasAction::AppendBlankSection);
                                }
                                if secondary_slate_button(ui, "Open Strip Library").clicked() {
                                    state.active_drawer = Some(super::state::DrawerTab::Sections);
                                }
                            });
                            ui.add_space(48.0);
                        });
                    });
                });

            if let Some(c) = hover_cursor {
                ctx.set_cursor_icon(c);
            }

            // Live coordinate HUD pill while dragging or resizing
            if let Some(drag) = &state.drag_mode {
                if let Some(ptr) = ctx.pointer_interact_pos() {
                    let hud_text = match drag {
                        DragMode::MoveElement { element_id, .. }
                        | DragMode::ResizeElement { element_id, .. } => {
                            state
                                .project
                                .find_page(&state.selected_page_id)
                                .and_then(|p| p.find_element(element_id))
                                .map(|(_, e)| {
                                    let r = match vp {
                                        ViewportMode::Desktop => e.rect,
                                        ViewportMode::Tablet => e.tablet_rect.unwrap_or(e.rect),
                                        ViewportMode::Mobile => e.mobile_rect.unwrap_or(e.rect),
                                    };
                                    format!(
                                        "X: {}  Y: {}  |  {} × {}",
                                        r.x.round() as i32,
                                        r.y.round() as i32,
                                        r.w.round() as i32,
                                        r.h.round() as i32
                                    )
                                })
                                .unwrap_or_default()
                        }
                        DragMode::ResizeSection { section_id, .. } => {
                            state
                                .project
                                .find_page(&state.selected_page_id)
                                .and_then(|p| p.find_section(section_id))
                                .map(|s| format!("Section Height: {}px", s.height.round() as i32))
                                .unwrap_or_default()
                        }
                    };
                    if !hud_text.is_empty() {
                        let hud_pos = ptr + egui::vec2(14.0, 16.0);
                        let painter = ctx.layer_painter(egui::LayerId::new(
                            egui::Order::Tooltip,
                            egui::Id::new("weysko_drag_hud"),
                        ));
                        let rect = Rect::from_min_size(hud_pos, egui::vec2(176.0, 24.0));
                        painter.rect_filled(rect, CornerRadius::same(5), BG_PANEL);
                        painter.rect_stroke(
                            rect,
                            CornerRadius::same(5),
                            Stroke::new(1.0_f32, WHITE),
                            egui::StrokeKind::Inside,
                        );
                        painter.text(
                            rect.center(),
                            Align2::CENTER_CENTER,
                            hud_text,
                            FontId::monospace(10.5),
                            WHITE,
                        );
                    }
                }
            }

            // Apply deferred canvas actions cleanly outside immutable borrows
            if let Some(act) = pending_action {
                apply_canvas_action(state, act);
            }
        });
}

fn apply_canvas_action(state: &mut StudioState, act: CanvasAction) {
    let page_id = state.selected_page_id.clone();
    match act {
        CanvasAction::SelectElement(sid, eid) => {
            state.selected_section_id = Some(sid);
            state.selected_element_id = Some(eid);
            state.inline_edit = None;
        }
        CanvasAction::SelectSection(sid) => {
            state.selected_section_id = Some(sid);
            state.selected_element_id = None;
            state.inline_edit = None;
        }
        CanvasAction::StartInlineEdit(eid, text) => {
            state.inline_edit = Some(InlineEditState {
                element_id: eid,
                buffer: text,
                pre_edit_snapshot: state.project.clone(),
            });
        }
        CanvasAction::MoveSection(from, to) => {
            state.push_undo_snapshot();
            if let Some(p) = state.project.find_page_mut(&page_id) {
                if from < p.sections.len() && to < p.sections.len() {
                    p.sections.swap(from, to);
                }
            }
        }
        CanvasAction::DuplicateSection(sid) => {
            state.push_undo_snapshot();
            if let Some(p) = state.project.find_page_mut(&page_id) {
                if let Some(pos) = p.sections.iter().position(|s| s.id == sid) {
                    let copy = p.sections[pos].duplicate();
                    let new_id = copy.id.clone();
                    p.sections.insert(pos + 1, copy);
                    state.selected_section_id = Some(new_id);
                    state.selected_element_id = None;
                    state.notify("Duplicated section", false);
                }
            }
        }
        CanvasAction::DeleteSection(sid) => {
            state.push_undo_snapshot();
            if let Some(p) = state.project.find_page_mut(&page_id) {
                p.sections.retain(|s| s.id != sid);
                state.reconcile_selection();
                state.notify("Deleted section", false);
            }
        }
        CanvasAction::AppendBlankSection => {
            state.push_undo_snapshot();
            let tokens = state.project.theme.clone();
            if let Some(p) = state.project.find_page_mut(&page_id) {
                let sec = SectionPreset::BlankSection.build(&tokens);
                let sid = sec.id.clone();
                p.sections.push(sec);
                state.selected_section_id = Some(sid);
                state.selected_element_id = None;
            }
        }
        CanvasAction::AddElementInSection(sid, kind) => {
            state.selected_section_id = Some(sid);
            state.insert_element(kind);
        }
        CanvasAction::DuplicateElement(eid) => {
            state.selected_element_id = Some(eid);
            state.duplicate_selected_element();
        }
        CanvasAction::CopyElement(eid) => {
            if let Some(page) = state.project.find_page(&page_id) {
                if let Some((_, el)) = page.find_element(&eid) {
                    state.clipboard_element = Some(el.clone());
                    state.notify("Copied element", false);
                }
            }
        }
        CanvasAction::BringToFront(eid) => state.bring_element_to_front(&eid),
        CanvasAction::SendToBack(eid) => state.send_element_to_back(&eid),
        CanvasAction::ToggleLock(eid) => state.toggle_element_lock(&eid),
        CanvasAction::DeleteElement(eid) => {
            state.selected_element_id = Some(eid);
            state.delete_selected_element();
        }
    }
}

fn handle_active_drag(
    ctx: &egui::Context,
    state: &mut StudioState,
    zoom: f32,
    vp_width: f32,
    guides: &mut Vec<GuideLine>,
) {
    let Some(drag) = state.drag_mode.clone() else {
        return;
    };

    let pointer_down = ctx.input(|i| i.pointer.primary_down());
    let pointer_pos = ctx.input(|i| i.pointer.interact_pos());

    if !pointer_down {
        // Drag finished: commit pre_drag_snapshot to undo stack if project changed
        let pre = match &drag {
            DragMode::MoveElement { pre_drag_snapshot, .. }
            | DragMode::ResizeElement { pre_drag_snapshot, .. }
            | DragMode::ResizeSection { pre_drag_snapshot, .. } => pre_drag_snapshot,
        };
        if *pre != state.project {
            state.history.push(pre.clone());
            state.dirty = true;
            state.preview_dirty = true;
        }
        state.drag_mode = None;
        return;
    }

    let Some(pos) = pointer_pos else {
        return;
    };

    let page_id = state.selected_page_id.clone();
    let vp = state.viewport;
    let snap = state.snap_to_grid;
    let max_content = state.project.theme.content_max_width.min(vp_width);

    match drag {
        DragMode::MoveElement {
            section_id,
            element_id,
            start_rect,
            start_pointer,
            ..
        } => {
            let delta = (pos - start_pointer) / zoom;
            let scale_x = if vp == ViewportMode::Desktop {
                max_content / 1200.0
            } else {
                1.0
            };
            let mut nx = start_rect.x + delta.x / scale_x;
            let mut ny = start_rect.y + delta.y;

            if snap {
                nx = (nx / 8.0).round() * 8.0;
                ny = (ny / 8.0).round() * 8.0;
            }

            // Smart alignment guides against section center & sibling edges
            let canvas_w = if vp == ViewportMode::Desktop {
                1200.0
            } else {
                vp_width
            };
            if let Some(page) = state.project.find_page(&page_id) {
                if let Some(sec) = page.find_section(&section_id) {
                    let threshold = 6.0;
                    let center_x = canvas_w * 0.5;
                    if ((nx + start_rect.w * 0.5) - center_x).abs() < threshold {
                        nx = center_x - start_rect.w * 0.5;
                        guides.push(GuideLine {
                            vertical: true,
                            coord_canvas: center_x,
                        });
                    }
                    let center_y = sec.height * 0.5;
                    if ((ny + start_rect.h * 0.5) - center_y).abs() < threshold {
                        ny = center_y - start_rect.h * 0.5;
                        guides.push(GuideLine {
                            vertical: false,
                            coord_canvas: center_y,
                        });
                    }
                    for sib in &sec.elements {
                        if sib.id == element_id || sib.hidden {
                            continue;
                        }
                        if (nx - sib.rect.x).abs() < threshold {
                            nx = sib.rect.x;
                            guides.push(GuideLine {
                                vertical: true,
                                coord_canvas: sib.rect.x,
                            });
                        }
                        if (ny - sib.rect.y).abs() < threshold {
                            ny = sib.rect.y;
                            guides.push(GuideLine {
                                vertical: false,
                                coord_canvas: sib.rect.y,
                            });
                        }
                    }
                }
            }

            nx = nx.clamp(0.0, (canvas_w - 24.0).max(0.0));
            ny = ny.max(0.0);

            if let Some(page) = state.project.find_page_mut(&page_id) {
                if let Some(el) = page.find_element_mut(&element_id) {
                    let updated = Rect2D::new(nx, ny, start_rect.w, start_rect.h);
                    match vp {
                        ViewportMode::Desktop => el.rect = updated,
                        ViewportMode::Tablet => el.tablet_rect = Some(updated),
                        ViewportMode::Mobile => el.mobile_rect = Some(updated),
                    }
                }
            }
        }
        DragMode::ResizeElement {
            element_id,
            handle,
            start_rect,
            start_pointer,
            ..
        } => {
            let delta = (pos - start_pointer) / zoom;
            let scale_x = if vp == ViewportMode::Desktop {
                max_content / 1200.0
            } else {
                1.0
            };
            let dx = delta.x / scale_x;
            let dy = delta.y;

            let mut r = start_rect;
            let orig_right = start_rect.x + start_rect.w;
            let orig_bottom = start_rect.y + start_rect.h;

            match handle {
                ResizeHandle::E | ResizeHandle::NE | ResizeHandle::SE => {
                    let mut nw = (start_rect.w + dx).max(20.0);
                    if snap {
                        nw = ((nw / 8.0).round() * 8.0).max(20.0);
                    }
                    r.w = nw;
                }
                ResizeHandle::W | ResizeHandle::NW | ResizeHandle::SW => {
                    let mut nx = (start_rect.x + dx).min(orig_right - 20.0).max(0.0);
                    if snap {
                        nx = ((nx / 8.0).round() * 8.0).min(orig_right - 20.0).max(0.0);
                    }
                    r.x = nx;
                    r.w = (orig_right - nx).max(20.0);
                }
                _ => {}
            }

            match handle {
                ResizeHandle::S | ResizeHandle::SE | ResizeHandle::SW => {
                    let mut nh = (start_rect.h + dy).max(2.0);
                    if snap {
                        nh = ((nh / 8.0).round() * 8.0).max(2.0);
                    }
                    r.h = nh;
                }
                ResizeHandle::N | ResizeHandle::NE | ResizeHandle::NW => {
                    let mut ny = (start_rect.y + dy).min(orig_bottom - 2.0).max(0.0);
                    if snap {
                        ny = ((ny / 8.0).round() * 8.0).min(orig_bottom - 2.0).max(0.0);
                    }
                    r.y = ny;
                    r.h = (orig_bottom - ny).max(2.0);
                }
                _ => {}
            }

            if let Some(page) = state.project.find_page_mut(&page_id) {
                if let Some(el) = page.find_element_mut(&element_id) {
                    match vp {
                        ViewportMode::Desktop => el.rect = r,
                        ViewportMode::Tablet => el.tablet_rect = Some(r),
                        ViewportMode::Mobile => el.mobile_rect = Some(r),
                    }
                }
            }
        }
        DragMode::ResizeSection {
            section_id,
            start_height,
            start_pointer_y,
            ..
        } => {
            let dy = (pos.y - start_pointer_y) / zoom;
            let mut nh = (start_height + dy).clamp(56.0, 2400.0);
            if snap {
                nh = (nh / 8.0).round() * 8.0;
            }
            if let Some(page) = state.project.find_page_mut(&page_id) {
                if let Some(sec) = page.find_section_mut(&section_id) {
                    sec.height = nh;
                }
            }
        }
    }
}

fn draw_inline_editor(ui: &mut Ui, state: &mut StudioState, rect: Rect, el: &Element) {
    let Some(ie) = &mut state.inline_edit else {
        return;
    };

    let mut commit = false;
    let mut cancel = false;

    egui::Area::new(egui::Id::new(format!("inline_edit_{}", el.id)))
        .fixed_pos(rect.min)
        .show(ui.ctx(), |ui| {
            ui.set_min_size(rect.size());
            let resp = if el.kind == ElementKind::Paragraph {
                ui.add_sized(
                    rect.size(),
                    egui::TextEdit::multiline(&mut ie.buffer)
                        .font(FontId::proportional((el.style.font_size * state.zoom).clamp(11.0, 32.0))),
                )
            } else {
                ui.add_sized(
                    rect.size(),
                    egui::TextEdit::singleline(&mut ie.buffer)
                        .font(FontId::proportional((el.style.font_size * state.zoom).clamp(11.0, 36.0))),
                )
            };
            resp.request_focus();

            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                cancel = true;
            } else if (el.kind != ElementKind::Paragraph && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                || resp.lost_focus()
            {
                commit = true;
            }
        });

    if cancel {
        state.inline_edit = None;
    } else if commit {
        if let Some(finished) = state.inline_edit.take() {
            if let Some(page) = state.project.find_page_mut(&state.selected_page_id) {
                if let Some(el_mut) = page.find_element_mut(&finished.element_id) {
                    if el_mut.content.text != finished.buffer {
                        state.history.push(finished.pre_edit_snapshot);
                        el_mut.content.text = finished.buffer;
                        state.dirty = true;
                        state.preview_dirty = true;
                    }
                }
            }
        }
    }
}

fn draw_section_floating_toolbar(
    ui: &mut Ui,
    sec_rect: Rect,
    sec: &Section,
    sec_idx: usize,
    sec_count: usize,
    pending_action: &mut Option<CanvasAction>,
) {
    let tb_pos = Pos2::new(sec_rect.min.x + 8.0, sec_rect.min.y + 6.0);
    egui::Area::new(egui::Id::new(format!("sec_tb_{}", sec.id)))
        .fixed_pos(tb_pos)
        .show(ui.ctx(), |ui| {
            Frame::new()
                .fill(BG_PANEL)
                .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
                .corner_radius(CornerRadius::same(6))
                .inner_margin(Margin::symmetric(8, 4))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 5.0;
                        ui.label(RichText::new(&sec.name).color(WHITE).strong().size(11.0));
                        if sec_idx > 0 && ui.small_button("↑").clicked() {
                            *pending_action = Some(CanvasAction::MoveSection(sec_idx, sec_idx - 1));
                        }
                        if sec_idx + 1 < sec_count && ui.small_button("↓").clicked() {
                            *pending_action = Some(CanvasAction::MoveSection(sec_idx, sec_idx + 1));
                        }
                        if ui.small_button("Dup").clicked() {
                            *pending_action = Some(CanvasAction::DuplicateSection(sec.id.clone()));
                        }
                        if ui
                            .small_button(RichText::new("×").color(DANGER_INK))
                            .clicked()
                        {
                            *pending_action = Some(CanvasAction::DeleteSection(sec.id.clone()));
                        }
                    });
                });
        });
}

fn draw_workspace_dot_grid(painter: &egui::Painter, rect: Rect) {
    let step = 22.0_f32;
    let dot_col = Color32::from_rgba_unmultiplied(51, 65, 85, 95);
    let mut y = rect.min.y + 11.0;
    while y < rect.max.y {
        let mut x = rect.min.x + 11.0;
        while x < rect.max.x {
            painter.circle_filled(Pos2::new(x, y), 1.0, dot_col);
            x += step;
        }
        y += step;
    }
}

fn draw_section_background(
    painter: &egui::Painter,
    rect: Rect,
    sec: &Section,
    textures: &std::collections::HashMap<String, egui::TextureHandle>,
    border_hex: &str,
) {
    match &sec.background {
        SectionBackground::Solid { color } => {
            painter.rect_filled(rect, CornerRadius::ZERO, hex_to_color32(color));
        }
        SectionBackground::LinearGradient { start, end, .. } => {
            let c1 = hex_to_color32(start);
            let c2 = hex_to_color32(end);
            let bands = 16;
            let bh = rect.height() / bands as f32;
            for i in 0..bands {
                let t = i as f32 / (bands - 1) as f32;
                let col = lerp_color(c1, c2, t);
                let band_rect = Rect::from_min_size(
                    Pos2::new(rect.min.x, rect.min.y + i as f32 * bh),
                    egui::vec2(rect.width(), bh + 1.0),
                );
                painter.rect_filled(band_rect, CornerRadius::ZERO, col);
            }
        }
        SectionBackground::Image {
            asset_id,
            overlay_color,
            overlay_opacity,
        } => {
            painter.rect_filled(rect, CornerRadius::ZERO, BG_SURFACE);
            if let Some(aid) = asset_id {
                if let Some(tex) = textures.get(aid) {
                    painter.image(
                        tex.id(),
                        rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );
                }
            }
            let ov = hex_to_color32(overlay_color);
            let alpha = (*overlay_opacity * 255.0).clamp(0.0, 255.0) as u8;
            painter.rect_filled(
                rect,
                CornerRadius::ZERO,
                Color32::from_rgba_unmultiplied(ov.r(), ov.g(), ov.b(), alpha),
            );
        }
    }

    let bcol = hex_to_color32(border_hex);
    if sec.border_top {
        painter.line_segment([rect.left_top(), rect.right_top()], Stroke::new(1.0_f32, bcol));
    }
    if sec.border_bottom {
        painter.line_segment([rect.left_bottom(), rect.right_bottom()], Stroke::new(1.0_f32, bcol));
    }
}

#[allow(clippy::too_many_arguments)]
fn paint_element(
    painter: &egui::Painter,
    rect: Rect,
    el: &Element,
    zoom: f32,
    textures: &std::collections::HashMap<String, egui::TextureHandle>,
    _assets: &[crate::model::AssetItem],
    palette: &crate::model::ColorPalette,
    project_pages: &[crate::model::Page],
) {
    let s = &el.style;
    let r = (s.border_radius[0] * zoom).clamp(0.0, 255.0) as u8;
    let cr = CornerRadius::same(r);
    let alpha_mul = s.opacity.clamp(0.05, 1.0);

    // Paint background fill
    match s.bg_type {
        ElementBgType::None => {}
        ElementBgType::Solid => {
            let fill = with_opacity(hex_to_color32(&s.bg_color), alpha_mul);
            painter.rect_filled(rect, cr, fill);
        }
        ElementBgType::LinearGradient => {
            let c1 = with_opacity(hex_to_color32(&s.bg_color), alpha_mul);
            let c2 = with_opacity(hex_to_color32(&s.bg_gradient_end), alpha_mul);
            painter.rect_filled(rect, cr, lerp_color(c1, c2, 0.45));
        }
    }

    if s.border_width > 0.0 {
        let bcol = with_opacity(hex_to_color32(&s.border_color), alpha_mul);
        painter.rect_stroke(
            rect,
            cr,
            Stroke::new((s.border_width * zoom).max(1.0_f32), bcol),
            egui::StrokeKind::Inside,
        );
    }

    let text_col = with_opacity(hex_to_color32(&s.text_color), alpha_mul);
    let muted_col = with_opacity(hex_to_color32(&palette.text_muted), alpha_mul);
    let border_col = with_opacity(hex_to_color32(&palette.border), alpha_mul);
    let primary_col = with_opacity(hex_to_color32(&palette.primary), alpha_mul);
    let primary_text_col = with_opacity(hex_to_color32(&palette.primary_text), alpha_mul);

    let pad_l = s.padding[3] * zoom;
    let pad_t = s.padding[0] * zoom;
    let pad_r = s.padding[1] * zoom;
    let pad_b = s.padding[2] * zoom;
    let inner = Rect::from_min_max(
        Pos2::new((rect.min.x + pad_l).min(rect.max.x), (rect.min.y + pad_t).min(rect.max.y)),
        Pos2::new((rect.max.x - pad_r).max(rect.min.x), (rect.max.y - pad_b).max(rect.min.y)),
    );

    match el.kind {
        ElementKind::Heading | ElementKind::Paragraph => {
            let font_sz = (s.font_size * zoom).clamp(9.0, 96.0);
            let galley = painter.layout(
                el.content.text.clone(),
                FontId::proportional(font_sz),
                text_col,
                inner.width().max(24.0),
            );
            let x = match s.text_align {
                TextAlign::Center => inner.center().x - galley.size().x * 0.5,
                TextAlign::Right => inner.max.x - galley.size().x,
                _ => inner.min.x,
            };
            painter.galley(Pos2::new(x.max(inner.min.x), inner.min.y), galley, text_col);
        }
        ElementKind::Button | ElementKind::Badge => {
            let font_sz = (s.font_size * zoom).clamp(9.0, 32.0);
            painter.text(
                rect.center(),
                Align2::CENTER_CENTER,
                &el.content.text,
                FontId::proportional(font_sz),
                text_col,
            );
        }
        ElementKind::Image => {
            if let Some(aid) = &el.content.asset_id {
                if let Some(tex) = textures.get(aid) {
                    painter.image(
                        tex.id(),
                        rect.shrink(1.0),
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );
                    return;
                }
            }
            painter.line_segment([rect.left_top(), rect.right_bottom()], Stroke::new(1.0_f32, border_col));
            painter.line_segment([rect.right_top(), rect.left_bottom()], Stroke::new(1.0_f32, border_col));
            painter.text(
                rect.center(),
                Align2::CENTER_CENTER,
                format!("IMAGE: {}", el.content.alt_text),
                FontId::proportional((12.0 * zoom).max(9.0)),
                muted_col,
            );
        }
        ElementKind::Container | ElementKind::Divider => {}
        ElementKind::Navbar => {
            let pad_x = 24.0 * zoom;
            painter.text(
                Pos2::new(rect.min.x + pad_x, rect.center().y),
                Align2::LEFT_CENTER,
                &el.content.nav_brand,
                FontId::proportional((16.0 * zoom).max(10.0)),
                text_col,
            );

            let cta_w = if el.content.nav_cta_label.is_empty() {
                0.0
            } else {
                108.0 * zoom
            };
            if cta_w > 0.0 {
                let cta_rect = Rect::from_center_size(
                    Pos2::new(rect.max.x - pad_x - cta_w * 0.5, rect.center().y),
                    egui::vec2(cta_w, 34.0 * zoom),
                );
                painter.rect_filled(cta_rect, CornerRadius::same(6), primary_col);
                painter.text(
                    cta_rect.center(),
                    Align2::CENTER_CENTER,
                    &el.content.nav_cta_label,
                    FontId::proportional((12.0 * zoom).max(9.0)),
                    primary_text_col,
                );
            }

            let mut lx = rect.max.x - pad_x - cta_w - 28.0 * zoom;
            let labels: Vec<String> = if el.content.nav_auto_pages && project_pages.len() > 1 {
                project_pages.iter().map(|p| p.title.clone()).collect()
            } else {
                el.content.nav_links.iter().map(|l| l.label.clone()).collect()
            };
            for lbl in labels.iter().rev() {
                painter.text(
                    Pos2::new(lx, rect.center().y),
                    Align2::RIGHT_CENTER,
                    lbl,
                    FontId::proportional((13.0 * zoom).max(9.0)),
                    muted_col,
                );
                lx -= (lbl.len() as f32 * 8.0 + 24.0) * zoom;
            }
        }
        ElementKind::Footer => {
            painter.text(
                Pos2::new(inner.min.x, inner.min.y + 12.0 * zoom),
                Align2::LEFT_TOP,
                &el.content.nav_brand,
                FontId::proportional((16.0 * zoom).max(10.0)),
                text_col,
            );
            painter.line_segment(
                [
                    Pos2::new(inner.min.x, inner.max.y - 30.0 * zoom),
                    Pos2::new(inner.max.x, inner.max.y - 30.0 * zoom),
                ],
                Stroke::new(1.0_f32, border_col),
            );
            painter.text(
                Pos2::new(inner.min.x, inner.max.y - 12.0 * zoom),
                Align2::LEFT_CENTER,
                &el.content.footer_copyright,
                FontId::proportional((12.0 * zoom).max(9.0)),
                muted_col,
            );
        }
        ElementKind::Card => {
            let mut y = inner.min.y;
            if !el.content.card_eyebrow.is_empty() {
                painter.text(
                    Pos2::new(inner.min.x, y),
                    Align2::LEFT_TOP,
                    &el.content.card_eyebrow,
                    FontId::proportional((10.5 * zoom).max(8.0)),
                    muted_col,
                );
                y += 20.0 * zoom;
            }
            painter.text(
                Pos2::new(inner.min.x, y),
                Align2::LEFT_TOP,
                &el.content.card_title,
                FontId::proportional((19.0 * zoom).max(11.0)),
                text_col,
            );
            y += 30.0 * zoom;
            let body_galley = painter.layout(
                el.content.card_body.clone(),
                FontId::proportional((13.5 * zoom).max(9.5)),
                muted_col,
                inner.width().max(24.0),
            );
            painter.galley(Pos2::new(inner.min.x, y), body_galley, muted_col);

            if !el.content.card_meta.is_empty() || !el.content.card_cta_label.is_empty() {
                painter.line_segment(
                    [
                        Pos2::new(inner.min.x, inner.max.y - 26.0 * zoom),
                        Pos2::new(inner.max.x, inner.max.y - 26.0 * zoom),
                    ],
                    Stroke::new(1.0_f32, border_col),
                );
                painter.text(
                    Pos2::new(inner.min.x, inner.max.y - 10.0 * zoom),
                    Align2::LEFT_CENTER,
                    &el.content.card_meta,
                    FontId::proportional((11.5 * zoom).max(8.5)),
                    muted_col,
                );
                painter.text(
                    Pos2::new(inner.max.x, inner.max.y - 10.0 * zoom),
                    Align2::RIGHT_CENTER,
                    &el.content.card_cta_label,
                    FontId::proportional((12.0 * zoom).max(9.0)),
                    text_col,
                );
            }
        }
        ElementKind::List => {
            let mut y = inner.min.y;
            let font_sz = (s.font_size * zoom).clamp(9.0, 26.0);
            for (idx, item) in el.content.list_items.iter().enumerate() {
                if y > inner.max.y - 10.0 {
                    break;
                }
                let prefix = match el.content.list_style {
                    ListStyle::Bullet => "•  ".to_string(),
                    ListStyle::Numbered => format!("{}.  ", idx + 1),
                    ListStyle::Check => "✓  ".to_string(),
                };
                painter.text(
                    Pos2::new(inner.min.x, y),
                    Align2::LEFT_TOP,
                    format!("{}{}", prefix, item),
                    FontId::proportional(font_sz),
                    text_col,
                );
                y += font_sz * 1.65;
            }
        }
        ElementKind::ContactForm => {
            let mut y = inner.min.y;
            painter.text(
                Pos2::new(inner.min.x, y),
                Align2::LEFT_TOP,
                &el.content.form_title,
                FontId::proportional((20.0 * zoom).max(11.0)),
                text_col,
            );
            y += 34.0 * zoom;
            for field in &el.content.form_fields {
                if y > inner.max.y - 54.0 * zoom {
                    break;
                }
                painter.text(
                    Pos2::new(inner.min.x, y),
                    Align2::LEFT_TOP,
                    &field.label,
                    FontId::proportional((11.5 * zoom).max(8.5)),
                    muted_col,
                );
                y += 16.0 * zoom;
                let fh = match field.field_type {
                    crate::model::FormFieldType::Textarea => 56.0 * zoom,
                    _ => 32.0 * zoom,
                };
                let frect = Rect::from_min_size(Pos2::new(inner.min.x, y), egui::vec2(inner.width(), fh));
                painter.rect_filled(frect, CornerRadius::same(5), BG_INPUT);
                painter.rect_stroke(frect, CornerRadius::same(5), Stroke::new(1.0_f32, border_col), egui::StrokeKind::Inside);
                painter.text(
                    Pos2::new(frect.min.x + 10.0 * zoom, frect.min.y + 10.0 * zoom),
                    Align2::LEFT_TOP,
                    &field.placeholder,
                    FontId::proportional((11.5 * zoom).max(8.5)),
                    TEXT_DIM,
                );
                y += fh + 12.0 * zoom;
            }
            let btn_rect = Rect::from_min_size(
                Pos2::new(inner.min.x, (inner.max.y - 38.0 * zoom).max(y)),
                egui::vec2(inner.width(), 38.0 * zoom),
            );
            painter.rect_filled(btn_rect, CornerRadius::same(6), primary_col);
            painter.text(
                btn_rect.center(),
                Align2::CENTER_CENTER,
                &el.content.form_submit_label,
                FontId::proportional((13.0 * zoom).max(9.5)),
                primary_text_col,
            );
        }
        ElementKind::Accordion => {
            let mut y = inner.min.y;
            for item in &el.content.accordion_items {
                if y > inner.max.y - 32.0 * zoom {
                    break;
                }
                let row_rect = Rect::from_min_size(
                    Pos2::new(inner.min.x, y),
                    egui::vec2(inner.width(), 44.0 * zoom),
                );
                painter.rect_filled(row_rect, CornerRadius::same(6), BG_INPUT);
                painter.rect_stroke(row_rect, CornerRadius::same(6), Stroke::new(1.0_f32, border_col), egui::StrokeKind::Inside);
                painter.text(
                    Pos2::new(row_rect.min.x + 14.0 * zoom, row_rect.center().y),
                    Align2::LEFT_CENTER,
                    &item.question,
                    FontId::proportional((13.5 * zoom).max(9.5)),
                    text_col,
                );
                painter.text(
                    Pos2::new(row_rect.max.x - 14.0 * zoom, row_rect.center().y),
                    Align2::RIGHT_CENTER,
                    "+",
                    FontId::proportional((16.0 * zoom).max(10.0)),
                    muted_col,
                );
                y += 52.0 * zoom;
            }
        }
        ElementKind::VideoEmbed => {
            painter.circle_stroke(rect.center(), 24.0 * zoom, Stroke::new(1.5_f32, WHITE));
            painter.text(
                rect.center(),
                Align2::CENTER_CENTER,
                "▶",
                FontId::proportional((18.0 * zoom).max(10.0)),
                WHITE,
            );
            painter.text(
                Pos2::new(rect.center().x, rect.center().y + 38.0 * zoom),
                Align2::CENTER_CENTER,
                &el.content.video_url,
                FontId::proportional((11.0 * zoom).max(8.0)),
                muted_col,
            );
        }
        ElementKind::CustomHtml => {
            painter.rect_stroke(rect, CornerRadius::same(6), Stroke::new(1.0_f32, border_col), egui::StrokeKind::Inside);
            painter.text(
                Pos2::new(inner.min.x, inner.min.y),
                Align2::LEFT_TOP,
                "</> CUSTOM HTML",
                FontId::monospace((11.0 * zoom).max(8.5)),
                WHITE,
            );
            let preview: String = el.content.raw_html.chars().take(120).collect();
            let galley = painter.layout(
                preview,
                FontId::monospace((10.5 * zoom).max(8.0)),
                muted_col,
                inner.width().max(24.0),
            );
            painter.galley(Pos2::new(inner.min.x, inner.min.y + 18.0 * zoom), galley, muted_col);
        }
        ElementKind::SocialLinks => {
            let mut x = inner.min.x;
            for link in &el.content.social_links {
                let chip = Rect::from_min_size(Pos2::new(x, inner.min.y), Vec2::splat(34.0 * zoom));
                painter.rect_filled(chip, CornerRadius::same(7), BG_SURFACE);
                painter.rect_stroke(chip, CornerRadius::same(7), Stroke::new(1.0_f32, border_col), egui::StrokeKind::Inside);
                let abbr: String = link.platform.label().chars().take(2).collect();
                painter.text(
                    chip.center(),
                    Align2::CENTER_CENTER,
                    abbr,
                    FontId::proportional((11.0 * zoom).max(8.5)),
                    text_col,
                );
                x += 42.0 * zoom;
            }
        }
    }
}

fn is_inline_editable(kind: ElementKind) -> bool {
    matches!(
        kind,
        ElementKind::Heading | ElementKind::Paragraph | ElementKind::Button | ElementKind::Badge
    )
}

fn is_visible_on_viewport(el: &Element, vp: ViewportMode) -> bool {
    match vp {
        ViewportMode::Desktop => el.visible_desktop,
        ViewportMode::Tablet => el.visible_tablet,
        ViewportMode::Mobile => el.visible_mobile,
    }
}

fn compute_section_display_height(sec: &Section, vp: ViewportMode) -> f32 {
    if vp == ViewportMode::Mobile && sec.mobile_layout == MobileLayoutMode::AutoStack {
        let mut sum = 48.0_f32;
        for el in &sec.elements {
            if !el.hidden && el.visible_mobile {
                let h = el.mobile_rect.map(|r| r.h).unwrap_or(match el.kind {
                    ElementKind::Image | ElementKind::VideoEmbed => 200.0,
                    ElementKind::Divider => 2.0,
                    ElementKind::Navbar => 64.0,
                    _ => el.rect.h.min(280.0),
                });
                sum += h + 14.0;
            }
        }
        sum.max(sec.height.min(220.0))
    } else {
        sec.height
    }
}

fn resolve_canvas_rect(
    el: &Element,
    vp: ViewportMode,
    content_w: f32,
    _scale_x: f32,
) -> Rect2D {
    SiteProject::effective_rect(el, vp, content_w)
}

fn resize_handle_positions(rect: Rect) -> [(ResizeHandle, Pos2); 8] {
    [
        (ResizeHandle::NW, rect.left_top()),
        (ResizeHandle::N, Pos2::new(rect.center().x, rect.top())),
        (ResizeHandle::NE, rect.right_top()),
        (ResizeHandle::E, Pos2::new(rect.right(), rect.center().y)),
        (ResizeHandle::SE, rect.right_bottom()),
        (ResizeHandle::S, Pos2::new(rect.center().x, rect.bottom())),
        (ResizeHandle::SW, rect.left_bottom()),
        (ResizeHandle::W, Pos2::new(rect.left(), rect.center().y)),
    ]
}

fn cursor_for_handle(h: ResizeHandle) -> CursorIcon {
    match h {
        ResizeHandle::N | ResizeHandle::S => CursorIcon::ResizeVertical,
        ResizeHandle::E | ResizeHandle::W => CursorIcon::ResizeHorizontal,
        ResizeHandle::NW | ResizeHandle::SE => CursorIcon::ResizeNwSe,
        ResizeHandle::NE | ResizeHandle::SW => CursorIcon::ResizeNeSw,
    }
}

fn with_opacity(c: Color32, opacity: f32) -> Color32 {
    let a = ((c.a() as f32) * opacity).clamp(0.0, 255.0) as u8;
    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), a)
}

fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    Color32::from_rgba_unmultiplied(
        (a.r() as f32 + (b.r() as f32 - a.r() as f32) * t) as u8,
        (a.g() as f32 + (b.g() as f32 - a.g() as f32) * t) as u8,
        (a.b() as f32 + (b.b() as f32 - a.b() as f32) * t) as u8,
        (a.a() as f32 + (b.a() as f32 - a.a() as f32) * t) as u8,
    )
}
