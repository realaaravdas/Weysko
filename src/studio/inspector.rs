use egui::{Frame, Margin, RichText, ScrollArea, Stroke, Ui};

use crate::model::{
    available_font_families, AccordionItem, BorderStyle, BoxShadowPreset, Element, ElementBgType,
    ElementKind, EntranceAnimation, FormActionMode, FormField, FormFieldType, HoverEffect,
    ListStyle, MobileLayoutMode, NavLink, ObjectFit, Rect2D, SectionBackground, SocialLink,
    SocialPlatform, TextAlign, ViewportMode,
};

use super::state::{InspectorTab, StudioState};
use super::style::{
    color_picker_hex, ghost_button, secondary_slate_button, section_heading, segmented_tabs,
    BG_PANEL, BORDER_SUBTLE, DANGER_INK, TEXT_DIM, TEXT_MUTED, WHITE,
};

pub fn draw_inspector(ctx: &egui::Context, state: &mut StudioState) {
    if !state.show_inspector {
        return;
    }
    egui::SidePanel::right("websir_inspector")
        .default_width(284.0)
        .width_range(236.0..=380.0)
        .resizable(true)
        .frame(
            Frame::new()
                .fill(BG_PANEL)
                .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
                .inner_margin(Margin::same(12)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Inspector").color(WHITE).strong().size(13.0));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ghost_button(ui, "x", false)
                        .on_hover_text("Collapse Inspector")
                        .clicked()
                    {
                        state.show_inspector = false;
                    }
                });
            });
            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    if let Some(el_id) = state.selected_element_id.clone() {
                        draw_element_inspector(ui, state, &el_id);
                    } else if let Some(sec_id) = state.selected_section_id.clone() {
                        draw_section_inspector(ui, state, &sec_id);
                    } else {
                        draw_page_site_inspector(ui, state);
                    }
                });
        });
}

fn draw_element_inspector(ui: &mut Ui, state: &mut StudioState, el_id: &str) {
    let page_id = state.selected_page_id.clone();
    let assets: Vec<(String, String)> = state
        .project
        .assets
        .iter()
        .map(|a| (a.id.clone(), a.name.clone()))
        .collect();
    let page_slugs: Vec<(String, String)> = state
        .project
        .pages
        .iter()
        .map(|p| (p.slug.clone(), p.title.clone()))
        .collect();

    let Some(page) = state.project.find_page_mut(&page_id) else {
        return;
    };
    let Some(el) = page.find_element_mut(el_id) else {
        return;
    };

    // Element header
    let mut do_delete = false;
    let mut do_duplicate = false;
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(el.kind.short_icon())
                .color(WHITE)
                .strong()
                .size(12.0),
        );
        ui.add(
            egui::TextEdit::singleline(&mut el.name)
                .desired_width(136.0)
                .font(egui::FontId::proportional(12.5)),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .small_button(RichText::new("Del").color(DANGER_INK))
                .on_hover_text("Delete element")
                .clicked()
            {
                do_delete = true;
            }
            if ui.small_button("Dup").on_hover_text("Duplicate element").clicked() {
                do_duplicate = true;
            }
        });
    });

    if do_delete {
        state.delete_selected_element();
        return;
    }
    if do_duplicate {
        state.duplicate_selected_element();
        return;
    }

    // Re-borrow cleanly after potential early return above
    let before = state.project.clone();
    let Some(page) = state.project.find_page_mut(&page_id) else {
        return;
    };
    let Some(el) = page.find_element_mut(el_id) else {
        return;
    };

    ui.add_space(6.0);
    segmented_tabs(
        ui,
        &mut state.inspector_tab,
        &[
            (InspectorTab::Content, "Content"),
            (InspectorTab::Layout, "Layout"),
            (InspectorTab::Style, "Style"),
            (InspectorTab::Effects, "Effects"),
        ],
    );
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(6.0);

    let mut changed = false;
    match state.inspector_tab {
        InspectorTab::Content => {
            changed |= draw_element_content_tab(ui, el, &assets, &page_slugs);
        }
        InspectorTab::Layout => {
            changed |= draw_element_layout_tab(ui, el, state.viewport);
        }
        InspectorTab::Style => {
            changed |= draw_element_style_tab(ui, el);
        }
        InspectorTab::Effects => {
            changed |= draw_element_effects_tab(ui, el);
        }
    }

    if changed {
        state.record_undo_from(before);
    }
}

fn draw_element_content_tab(
    ui: &mut Ui,
    el: &mut Element,
    assets: &[(String, String)],
    page_slugs: &[(String, String)],
) -> bool {
    let mut changed = false;

    match el.kind {
        ElementKind::Heading => {
            section_heading(ui, "Heading Text");
            changed |= ui
                .add(
                    egui::TextEdit::multiline(&mut el.content.text)
                        .desired_rows(3)
                        .desired_width(f32::INFINITY),
                )
                .changed();
            ui.horizontal(|ui| {
                ui.label(RichText::new("Tag Level").color(TEXT_MUTED).size(12.0));
                for lvl in 1..=6u8 {
                    if ghost_button(ui, &format!("H{}", lvl), el.content.heading_level == lvl)
                        .clicked()
                    {
                        el.content.heading_level = lvl;
                        changed = true;
                    }
                }
            });
        }
        ElementKind::Paragraph => {
            section_heading(ui, "Paragraph Body");
            changed |= ui
                .add(
                    egui::TextEdit::multiline(&mut el.content.text)
                        .desired_rows(6)
                        .desired_width(f32::INFINITY),
                )
                .changed();
        }
        ElementKind::Button => {
            section_heading(ui, "Button Label & Action");
            ui.label(RichText::new("Label").color(TEXT_MUTED).size(11.5));
            changed |= ui
                .add(egui::TextEdit::singleline(&mut el.content.text).desired_width(f32::INFINITY))
                .changed();

            ui.label(RichText::new("Link to Page (optional)").color(TEXT_MUTED).size(11.5));
            let current_target = el
                .content
                .link_target_page_slug
                .as_deref()
                .unwrap_or("Custom URL / Anchor");
            egui::ComboBox::from_id_salt("btn_page_target")
                .selected_text(current_target)
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_label(el.content.link_target_page_slug.is_none(), "Custom URL / Anchor")
                        .clicked()
                    {
                        el.content.link_target_page_slug = None;
                        changed = true;
                    }
                    for (slug, title) in page_slugs {
                        let sel = el.content.link_target_page_slug.as_deref() == Some(slug);
                        if ui.selectable_label(sel, format!("{} ({})", title, slug)).clicked() {
                            el.content.link_target_page_slug = Some(slug.clone());
                            el.content.href = format!("{}.html", slug);
                            changed = true;
                        }
                    }
                });

            ui.label(RichText::new("URL or #anchor").color(TEXT_MUTED).size(11.5));
            changed |= ui
                .add(egui::TextEdit::singleline(&mut el.content.href).desired_width(f32::INFINITY))
                .changed();
            changed |= ui
                .checkbox(&mut el.content.open_in_new_tab, "Open in new tab")
                .changed();
        }
        ElementKind::Badge => {
            section_heading(ui, "Badge Text");
            changed |= ui
                .add(egui::TextEdit::singleline(&mut el.content.text).desired_width(f32::INFINITY))
                .changed();
        }
        ElementKind::Image => {
            section_heading(ui, "Image Source");
            let selected_asset_name = el
                .content
                .asset_id
                .as_deref()
                .and_then(|aid| assets.iter().find(|(id, _)| id == aid))
                .map(|(_, name)| name.as_str())
                .unwrap_or("None (Use URL or Placeholder)");

            egui::ComboBox::from_id_salt("img_asset_picker")
                .selected_text(selected_asset_name)
                .width(f32::INFINITY)
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_label(el.content.asset_id.is_none(), "None")
                        .clicked()
                    {
                        el.content.asset_id = None;
                        changed = true;
                    }
                    for (aid, name) in assets {
                        let sel = el.content.asset_id.as_deref() == Some(aid);
                        if ui.selectable_label(sel, name).clicked() {
                            el.content.asset_id = Some(aid.clone());
                            changed = true;
                        }
                    }
                });

            ui.label(RichText::new("Or External Image URL").color(TEXT_MUTED).size(11.5));
            changed |= ui
                .add(
                    egui::TextEdit::singleline(&mut el.content.image_url)
                        .desired_width(f32::INFINITY),
                )
                .changed();
            ui.label(RichText::new("Alt Text").color(TEXT_MUTED).size(11.5));
            changed |= ui
                .add(
                    egui::TextEdit::singleline(&mut el.content.alt_text)
                        .desired_width(f32::INFINITY),
                )
                .changed();

            ui.horizontal(|ui| {
                ui.label(RichText::new("Object Fit").color(TEXT_MUTED).size(12.0));
                for (fit, lbl) in [
                    (ObjectFit::Cover, "Cover"),
                    (ObjectFit::Contain, "Contain"),
                    (ObjectFit::Fill, "Fill"),
                ] {
                    if ghost_button(ui, lbl, el.content.object_fit == fit).clicked() {
                        el.content.object_fit = fit;
                        changed = true;
                    }
                }
            });
        }
        ElementKind::Navbar => {
            section_heading(ui, "Navbar Configuration");
            ui.label(RichText::new("Brand Wordmark").color(TEXT_MUTED).size(11.5));
            changed |= ui
                .add(
                    egui::TextEdit::singleline(&mut el.content.nav_brand)
                        .desired_width(f32::INFINITY),
                )
                .changed();
            changed |= ui
                .checkbox(
                    &mut el.content.nav_auto_pages,
                    "Auto-list all site pages (when multi-page)",
                )
                .changed();

            section_heading(ui, "Custom Nav Links");
            let mut remove_idx = None;
            for (idx, link) in el.content.nav_links.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    changed |= ui
                        .add(egui::TextEdit::singleline(&mut link.label).desired_width(90.0))
                        .changed();
                    changed |= ui
                        .add(egui::TextEdit::singleline(&mut link.href).desired_width(130.0))
                        .changed();
                    if ui.small_button("×").clicked() {
                        remove_idx = Some(idx);
                    }
                });
            }
            if let Some(i) = remove_idx {
                el.content.nav_links.remove(i);
                changed = true;
            }
            if secondary_slate_button(ui, "+ Add Nav Link").clicked() {
                el.content.nav_links.push(NavLink {
                    label: "Link".into(),
                    href: "#".into(),
                    page_slug: None,
                });
                changed = true;
            }

            section_heading(ui, "Navbar CTA Button");
            changed |= ui
                .add(
                    egui::TextEdit::singleline(&mut el.content.nav_cta_label)
                        .hint_text("CTA Label (leave blank to hide)")
                        .desired_width(f32::INFINITY),
                )
                .changed();
            changed |= ui
                .add(
                    egui::TextEdit::singleline(&mut el.content.nav_cta_href)
                        .hint_text("CTA Href")
                        .desired_width(f32::INFINITY),
                )
                .changed();
        }
        ElementKind::Footer => {
            section_heading(ui, "Footer Content");
            ui.label(RichText::new("Brand Name").color(TEXT_MUTED).size(11.5));
            changed |= ui
                .add(
                    egui::TextEdit::singleline(&mut el.content.nav_brand)
                        .desired_width(f32::INFINITY),
                )
                .changed();
            ui.label(RichText::new("Copyright Line").color(TEXT_MUTED).size(11.5));
            changed |= ui
                .add(
                    egui::TextEdit::singleline(&mut el.content.footer_copyright)
                        .desired_width(f32::INFINITY),
                )
                .changed();
        }
        ElementKind::ContactForm => {
            section_heading(ui, "Form Settings");
            ui.label(RichText::new("Form Title").color(TEXT_MUTED).size(11.5));
            changed |= ui
                .add(
                    egui::TextEdit::singleline(&mut el.content.form_title)
                        .desired_width(f32::INFINITY),
                )
                .changed();
            ui.label(RichText::new("Submit Button Label").color(TEXT_MUTED).size(11.5));
            changed |= ui
                .add(
                    egui::TextEdit::singleline(&mut el.content.form_submit_label)
                        .desired_width(f32::INFINITY),
                )
                .changed();
            ui.label(RichText::new("Submission Mode").color(TEXT_MUTED).size(11.5));
            egui::ComboBox::from_id_salt("form_mode_combo")
                .selected_text(el.content.form_action_mode.label())
                .width(f32::INFINITY)
                .show_ui(ui, |ui| {
                    for mode in [
                        FormActionMode::ToastDemo,
                        FormActionMode::Mailto,
                        FormActionMode::PostUrl,
                        FormActionMode::NetlifyForms,
                    ] {
                        if ui
                            .selectable_value(&mut el.content.form_action_mode, mode, mode.label())
                            .changed()
                        {
                            changed = true;
                        }
                    }
                });
            if matches!(
                el.content.form_action_mode,
                FormActionMode::Mailto | FormActionMode::PostUrl
            ) {
                ui.label(RichText::new("Target Email / Endpoint URL").color(TEXT_MUTED).size(11.5));
                changed |= ui
                    .add(
                        egui::TextEdit::singleline(&mut el.content.form_action_value)
                            .desired_width(f32::INFINITY),
                    )
                    .changed();
            }

            section_heading(ui, "Fields");
            let mut rem = None;
            for (idx, field) in el.content.form_fields.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    changed |= ui
                        .add(egui::TextEdit::singleline(&mut field.label).desired_width(86.0))
                        .changed();
                    egui::ComboBox::from_id_salt(format!("ft_{}", idx))
                        .selected_text(field.field_type.label())
                        .width(80.0)
                        .show_ui(ui, |ui| {
                            for ft in [
                                FormFieldType::Text,
                                FormFieldType::Email,
                                FormFieldType::Textarea,
                            ] {
                                if ui
                                    .selectable_value(&mut field.field_type, ft, ft.label())
                                    .changed()
                                {
                                    changed = true;
                                }
                            }
                        });
                    if ui.small_button("×").clicked() {
                        rem = Some(idx);
                    }
                });
            }
            if let Some(i) = rem {
                el.content.form_fields.remove(i);
                changed = true;
            }
            if secondary_slate_button(ui, "+ Add Field").clicked() {
                el.content.form_fields.push(FormField {
                    label: "Company".into(),
                    placeholder: "Acme Inc.".into(),
                    field_type: FormFieldType::Text,
                    required: false,
                });
                changed = true;
            }
        }
        ElementKind::List => {
            section_heading(ui, "List Style & Items");
            ui.horizontal(|ui| {
                for (st, lbl) in [
                    (ListStyle::Bullet, "Bullet"),
                    (ListStyle::Numbered, "Number"),
                    (ListStyle::Check, "Check"),
                ] {
                    if ghost_button(ui, lbl, el.content.list_style == st).clicked() {
                        el.content.list_style = st;
                        changed = true;
                    }
                }
            });
            let mut rem = None;
            for (idx, item) in el.content.list_items.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    changed |= ui
                        .add(egui::TextEdit::singleline(item).desired_width(230.0))
                        .changed();
                    if ui.small_button("×").clicked() {
                        rem = Some(idx);
                    }
                });
            }
            if let Some(i) = rem {
                el.content.list_items.remove(i);
                changed = true;
            }
            if secondary_slate_button(ui, "+ Add List Item").clicked() {
                el.content.list_items.push("New list item".into());
                changed = true;
            }
        }
        ElementKind::Card => {
            section_heading(ui, "Card Content");
            for (lbl, field) in [
                ("Eyebrow", &mut el.content.card_eyebrow),
                ("Title", &mut el.content.card_title),
                ("Metadata", &mut el.content.card_meta),
                ("CTA Label", &mut el.content.card_cta_label),
                ("CTA Href", &mut el.content.card_cta_href),
            ] {
                ui.label(RichText::new(lbl).color(TEXT_MUTED).size(11.0));
                changed |= ui
                    .add(egui::TextEdit::singleline(field).desired_width(f32::INFINITY))
                    .changed();
            }
            ui.label(RichText::new("Body Copy").color(TEXT_MUTED).size(11.0));
            changed |= ui
                .add(
                    egui::TextEdit::multiline(&mut el.content.card_body)
                        .desired_rows(3)
                        .desired_width(f32::INFINITY),
                )
                .changed();
        }
        ElementKind::Accordion => {
            section_heading(ui, "FAQ Accordion Items");
            let mut rem = None;
            for (idx, item) in el.content.accordion_items.iter_mut().enumerate() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("Q{}", idx + 1)).color(TEXT_DIM).size(11.0));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("×").clicked() {
                                rem = Some(idx);
                            }
                        });
                    });
                    changed |= ui
                        .add(
                            egui::TextEdit::singleline(&mut item.question)
                                .desired_width(f32::INFINITY),
                        )
                        .changed();
                    changed |= ui
                        .add(
                            egui::TextEdit::multiline(&mut item.answer)
                                .desired_rows(2)
                                .desired_width(f32::INFINITY),
                        )
                        .changed();
                });
            }
            if let Some(i) = rem {
                el.content.accordion_items.remove(i);
                changed = true;
            }
            if secondary_slate_button(ui, "+ Add FAQ Item").clicked() {
                el.content.accordion_items.push(AccordionItem {
                    question: "New Question?".into(),
                    answer: "Detailed answer here.".into(),
                });
                changed = true;
            }
        }
        ElementKind::VideoEmbed => {
            section_heading(ui, "Video URL (YouTube, Vimeo, or MP4)");
            changed |= ui
                .add(
                    egui::TextEdit::singleline(&mut el.content.video_url)
                        .desired_width(f32::INFINITY),
                )
                .changed();
            changed |= ui.checkbox(&mut el.content.video_controls, "Show controls").changed();
            changed |= ui.checkbox(&mut el.content.video_autoplay, "Autoplay").changed();
            changed |= ui.checkbox(&mut el.content.video_muted, "Muted").changed();
            changed |= ui.checkbox(&mut el.content.video_loop, "Loop").changed();
        }
        ElementKind::CustomHtml => {
            section_heading(ui, "Raw HTML Embed");
            changed |= ui
                .add(
                    egui::TextEdit::multiline(&mut el.content.raw_html)
                        .code_editor()
                        .desired_rows(8)
                        .desired_width(f32::INFINITY),
                )
                .changed();
        }
        ElementKind::SocialLinks => {
            section_heading(ui, "Social Profiles");
            let mut rem = None;
            for (idx, link) in el.content.social_links.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    egui::ComboBox::from_id_salt(format!("soc_{}", idx))
                        .selected_text(link.platform.label())
                        .width(86.0)
                        .show_ui(ui, |ui| {
                            for p in SocialPlatform::all() {
                                if ui
                                    .selectable_value(&mut link.platform, *p, p.label())
                                    .changed()
                                {
                                    changed = true;
                                }
                            }
                        });
                    changed |= ui
                        .add(egui::TextEdit::singleline(&mut link.url).desired_width(142.0))
                        .changed();
                    if ui.small_button("×").clicked() {
                        rem = Some(idx);
                    }
                });
            }
            if let Some(i) = rem {
                el.content.social_links.remove(i);
                changed = true;
            }
            if secondary_slate_button(ui, "+ Add Social Link").clicked() {
                el.content.social_links.push(SocialLink {
                    platform: SocialPlatform::Website,
                    url: "https://example.com".into(),
                });
                changed = true;
            }
        }
        ElementKind::Container | ElementKind::Divider => {
            ui.label(
                RichText::new("Use the Layout & Style tabs to customize dimensions, colors, borders, and radius.")
                    .color(TEXT_MUTED)
                    .size(12.0),
            );
        }
    }

    changed
}

fn draw_element_layout_tab(ui: &mut Ui, el: &mut Element, viewport: ViewportMode) -> bool {
    let mut changed = false;
    section_heading(ui, &format!("Geometry ({})", viewport.label()));

    let active_rect: &mut Rect2D = match viewport {
        ViewportMode::Desktop => &mut el.rect,
        ViewportMode::Tablet => {
            if el.tablet_rect.is_none() && secondary_slate_button(ui, "Create Tablet Override").clicked() {
                let scale = 768.0 / 1200.0;
                el.tablet_rect = Some(Rect2D::new(
                    el.rect.x * scale,
                    el.rect.y,
                    el.rect.w * scale,
                    el.rect.h,
                ));
                changed = true;
            }
            if let Some(tr) = &mut el.tablet_rect {
                tr
            } else {
                &mut el.rect
            }
        }
        ViewportMode::Mobile => {
            if el.mobile_rect.is_none() && secondary_slate_button(ui, "Create Mobile Override").clicked() {
                el.mobile_rect = Some(Rect2D::new(16.0, el.rect.y, 358.0, el.rect.h));
                changed = true;
            }
            if let Some(mr) = &mut el.mobile_rect {
                mr
            } else {
                &mut el.rect
            }
        }
    };

    egui::Grid::new("layout_xywh")
        .num_columns(4)
        .spacing([8.0, 6.0])
        .show(ui, |ui| {
            ui.label(RichText::new("X").color(TEXT_MUTED).size(11.5));
            changed |= ui.add(egui::DragValue::new(&mut active_rect.x).speed(1.0)).changed();
            ui.label(RichText::new("Y").color(TEXT_MUTED).size(11.5));
            changed |= ui.add(egui::DragValue::new(&mut active_rect.y).speed(1.0)).changed();
            ui.end_row();

            ui.label(RichText::new("W").color(TEXT_MUTED).size(11.5));
            changed |= ui
                .add(egui::DragValue::new(&mut active_rect.w).speed(1.0).range(16.0..=2400.0))
                .changed();
            ui.label(RichText::new("H").color(TEXT_MUTED).size(11.5));
            changed |= ui
                .add(egui::DragValue::new(&mut active_rect.h).speed(1.0).range(2.0..=2400.0))
                .changed();
            ui.end_row();
        });

    if viewport == ViewportMode::Tablet
        && el.tablet_rect.is_some()
        && ui.small_button("Reset Tablet Override").clicked()
    {
        el.tablet_rect = None;
        changed = true;
    }
    if viewport == ViewportMode::Mobile
        && el.mobile_rect.is_some()
        && ui.small_button("Reset Mobile Override").clicked()
    {
        el.mobile_rect = None;
        changed = true;
    }

    changed |= ui.checkbox(&mut el.full_width, "Full Width (100% strip)").changed();
    changed |= ui.checkbox(&mut el.locked, "Lock position").changed();

    section_heading(ui, "Layer Order & Align");
    ui.horizontal(|ui| {
        ui.label(RichText::new("Z-Index").color(TEXT_MUTED).size(11.5));
        changed |= ui.add(egui::DragValue::new(&mut el.z_index)).changed();
        if ui.small_button("Front").clicked() {
            el.z_index += 10;
            changed = true;
        }
        if ui.small_button("Back").clicked() {
            el.z_index = (el.z_index - 10).max(0);
            changed = true;
        }
    });

    ui.horizontal(|ui| {
        if ui.small_button("Align L").clicked() {
            el.rect.x = 60.0;
            changed = true;
        }
        if ui.small_button("Center X").clicked() {
            el.rect.x = (1200.0 - el.rect.w) * 0.5;
            changed = true;
        }
        if ui.small_button("Align R").clicked() {
            el.rect.x = 1140.0 - el.rect.w;
            changed = true;
        }
    });

    section_heading(ui, "Inner Padding (T / R / B / L)");
    ui.horizontal(|ui| {
        for p in &mut el.style.padding {
            changed |= ui
                .add(egui::DragValue::new(p).range(0.0..=120.0).speed(1.0))
                .changed();
        }
    });

    section_heading(ui, "Breakpoint Visibility");
    ui.horizontal(|ui| {
        changed |= ui.checkbox(&mut el.visible_desktop, "Desktop").changed();
        changed |= ui.checkbox(&mut el.visible_tablet, "Tablet").changed();
        changed |= ui.checkbox(&mut el.visible_mobile, "Mobile").changed();
    });

    changed
}

fn draw_element_style_tab(ui: &mut Ui, el: &mut Element) -> bool {
    let mut changed = false;
    let s = &mut el.style;

    section_heading(ui, "Background");
    ui.horizontal(|ui| {
        for (mode, lbl) in [
            (ElementBgType::None, "None"),
            (ElementBgType::Solid, "Solid"),
            (ElementBgType::LinearGradient, "Gradient"),
        ] {
            if ghost_button(ui, lbl, s.bg_type == mode).clicked() {
                s.bg_type = mode;
                changed = true;
            }
        }
    });
    if s.bg_type != ElementBgType::None {
        changed |= color_picker_hex(ui, "Fill Color", &mut s.bg_color);
    }
    if s.bg_type == ElementBgType::LinearGradient {
        changed |= color_picker_hex(ui, "Gradient End", &mut s.bg_gradient_end);
        ui.horizontal(|ui| {
            ui.label(RichText::new("Angle").color(TEXT_MUTED).size(12.0));
            changed |= ui
                .add(egui::Slider::new(&mut s.bg_gradient_angle, 0.0..=360.0).suffix("°"))
                .changed();
        });
    }

    section_heading(ui, "Typography & Color");
    changed |= color_picker_hex(ui, "Text Color", &mut s.text_color);

    ui.horizontal(|ui| {
        ui.label(RichText::new("Font").color(TEXT_MUTED).size(12.0));
        let display = if s.font_family.is_empty() {
            "Inherit Theme"
        } else {
            &s.font_family
        };
        egui::ComboBox::from_id_salt("el_font_family")
            .selected_text(display)
            .show_ui(ui, |ui| {
                if ui.selectable_label(s.font_family.is_empty(), "Inherit Theme").clicked() {
                    s.font_family.clear();
                    changed = true;
                }
                for f in available_font_families() {
                    if ui
                        .selectable_value(&mut s.font_family, (*f).into(), *f)
                        .changed()
                    {
                        changed = true;
                    }
                }
            });
    });

    egui::Grid::new("el_typo_grid").num_columns(4).show(ui, |ui| {
        ui.label(RichText::new("Size").color(TEXT_MUTED).size(11.5));
        changed |= ui
            .add(egui::DragValue::new(&mut s.font_size).range(8.0..=140.0).suffix("px"))
            .changed();
        ui.label(RichText::new("Weight").color(TEXT_MUTED).size(11.5));
        changed |= ui
            .add(egui::DragValue::new(&mut s.font_weight).speed(100.0).range(100..=900))
            .changed();
        ui.end_row();

        ui.label(RichText::new("Line H").color(TEXT_MUTED).size(11.5));
        changed |= ui
            .add(egui::DragValue::new(&mut s.line_height).speed(0.02).range(0.8..=3.0))
            .changed();
        ui.label(RichText::new("Tracking").color(TEXT_MUTED).size(11.5));
        changed |= ui
            .add(egui::DragValue::new(&mut s.letter_spacing).speed(0.005).range(-0.1..=0.4))
            .changed();
        ui.end_row();
    });

    ui.horizontal(|ui| {
        for (al, lbl) in [
            (TextAlign::Left, "Left"),
            (TextAlign::Center, "Center"),
            (TextAlign::Right, "Right"),
            (TextAlign::Justify, "Justify"),
        ] {
            if ghost_button(ui, lbl, s.text_align == al).clicked() {
                s.text_align = al;
                changed = true;
            }
        }
    });

    section_heading(ui, "Border, Radius & Opacity");
    ui.horizontal(|ui| {
        ui.label(RichText::new("Border W").color(TEXT_MUTED).size(11.5));
        changed |= ui
            .add(egui::DragValue::new(&mut s.border_width).range(0.0..=16.0).suffix("px"))
            .changed();
        egui::ComboBox::from_id_salt("el_border_style")
            .selected_text(s.border_style.as_css())
            .width(80.0)
            .show_ui(ui, |ui| {
                for bs in [BorderStyle::Solid, BorderStyle::Dashed, BorderStyle::Dotted] {
                    if ui
                        .selectable_value(&mut s.border_style, bs, bs.as_css())
                        .changed()
                    {
                        changed = true;
                    }
                }
            });
    });
    if s.border_width > 0.0 {
        changed |= color_picker_hex(ui, "Border Color", &mut s.border_color);
    }

    let mut uniform_r = s.border_radius[0];
    ui.horizontal(|ui| {
        ui.label(RichText::new("Radius").color(TEXT_MUTED).size(11.5));
        if ui
            .add(egui::Slider::new(&mut uniform_r, 0.0..=64.0).suffix("px"))
            .changed()
        {
            s.border_radius = [uniform_r; 4];
            changed = true;
        }
    });

    ui.horizontal(|ui| {
        ui.label(RichText::new("Opacity").color(TEXT_MUTED).size(11.5));
        changed |= ui.add(egui::Slider::new(&mut s.opacity, 0.05..=1.0)).changed();
    });

    changed
}

fn draw_element_effects_tab(ui: &mut Ui, el: &mut Element) -> bool {
    let mut changed = false;

    section_heading(ui, "Box Shadow");
    egui::ComboBox::from_id_salt("el_shadow_combo")
        .selected_text(el.style.shadow.label())
        .width(f32::INFINITY)
        .show_ui(ui, |ui| {
            for sh in [
                BoxShadowPreset::None,
                BoxShadowPreset::Subtle,
                BoxShadowPreset::Medium,
                BoxShadowPreset::Elevated,
                BoxShadowPreset::Glow,
                BoxShadowPreset::Custom,
            ] {
                if ui
                    .selectable_value(&mut el.style.shadow, sh, sh.label())
                    .changed()
                {
                    changed = true;
                }
            }
        });

    section_heading(ui, "Hover Effect");
    let hover_label = match &el.hover {
        HoverEffect::None => "None",
        HoverEffect::Lift => "Lift Up",
        HoverEffect::Scale => "Scale Up",
        HoverEffect::Brighten => "Brighten",
        HoverEffect::Invert => "Invert Colors",
        HoverEffect::BorderHighlight => "Border Highlight",
        HoverEffect::Custom { .. } => "Custom Colors",
    };
    egui::ComboBox::from_id_salt("el_hover_combo")
        .selected_text(hover_label)
        .width(f32::INFINITY)
        .show_ui(ui, |ui| {
            for (eff, lbl) in [
                (HoverEffect::None, "None"),
                (HoverEffect::Lift, "Lift Up"),
                (HoverEffect::Scale, "Scale Up"),
                (HoverEffect::Brighten, "Brighten"),
                (HoverEffect::Invert, "Invert Colors"),
                (HoverEffect::BorderHighlight, "Border Highlight"),
            ] {
                if ui
                    .selectable_label(std::mem::discriminant(&el.hover) == std::mem::discriminant(&eff), lbl)
                    .clicked()
                {
                    el.hover = eff;
                    changed = true;
                }
            }
            if ui
                .selectable_label(matches!(el.hover, HoverEffect::Custom { .. }), "Custom Colors")
                .clicked()
            {
                el.hover = HoverEffect::Custom {
                    bg_color: "#FFFFFF".into(),
                    text_color: "#0F172A".into(),
                };
                changed = true;
            }
        });

    if let HoverEffect::Custom {
        bg_color,
        text_color,
    } = &mut el.hover
    {
        changed |= color_picker_hex(ui, "Hover Fill", bg_color);
        changed |= color_picker_hex(ui, "Hover Ink", text_color);
    }

    section_heading(ui, "Entrance Animation");
    egui::ComboBox::from_id_salt("el_anim_combo")
        .selected_text(el.animation.label())
        .width(f32::INFINITY)
        .show_ui(ui, |ui| {
            for anim in [
                EntranceAnimation::None,
                EntranceAnimation::FadeIn,
                EntranceAnimation::SlideUp,
                EntranceAnimation::SlideLeft,
                EntranceAnimation::ZoomIn,
            ] {
                if ui
                    .selectable_value(&mut el.animation, anim, anim.label())
                    .changed()
                {
                    changed = true;
                }
            }
        });

    section_heading(ui, "Advanced Attributes & CSS");
    ui.label(RichText::new("HTML #id").color(TEXT_MUTED).size(11.0));
    changed |= ui
        .add(egui::TextEdit::singleline(&mut el.html_id).desired_width(f32::INFINITY))
        .changed();
    ui.label(RichText::new("Custom CSS Classes").color(TEXT_MUTED).size(11.0));
    changed |= ui
        .add(egui::TextEdit::singleline(&mut el.custom_class).desired_width(f32::INFINITY))
        .changed();
    ui.label(RichText::new("Custom Element CSS").color(TEXT_MUTED).size(11.0));
    changed |= ui
        .add(
            egui::TextEdit::multiline(&mut el.custom_css)
                .code_editor()
                .desired_rows(4)
                .desired_width(f32::INFINITY),
        )
        .changed();

    changed
}

fn draw_section_inspector(ui: &mut Ui, state: &mut StudioState, sec_id: &str) {
    let before = state.project.clone();
    let page_id = state.selected_page_id.clone();
    let assets: Vec<(String, String)> = state
        .project
        .assets
        .iter()
        .map(|a| (a.id.clone(), a.name.clone()))
        .collect();
    let theme_bg = state.project.theme.palette.background.clone();
    let theme_surface = state.project.theme.palette.surface.clone();

    let Some(page) = state.project.find_page_mut(&page_id) else {
        return;
    };
    let sec_idx_opt = page.sections.iter().position(|s| s.id == sec_id);
    let sec_count = page.sections.len();
    let Some(sec) = page.find_section_mut(sec_id) else {
        return;
    };

    section_heading(ui, "Section Strip Settings");
    let mut changed = false;

    ui.label(RichText::new("Section Name").color(TEXT_MUTED).size(11.5));
    changed |= ui
        .add(egui::TextEdit::singleline(&mut sec.name).desired_width(f32::INFINITY))
        .changed();

    ui.label(RichText::new("Anchor ID (for #links)").color(TEXT_MUTED).size(11.5));
    changed |= ui
        .add(egui::TextEdit::singleline(&mut sec.anchor_id).desired_width(f32::INFINITY))
        .changed();

    ui.horizontal(|ui| {
        ui.label(RichText::new("Height").color(TEXT_MUTED).size(12.0));
        changed |= ui
            .add(
                egui::DragValue::new(&mut sec.height)
                    .speed(4.0)
                    .range(48.0..=2400.0)
                    .suffix("px"),
            )
            .changed();
    });

    changed |= ui.checkbox(&mut sec.full_bleed, "Full-bleed inner container").changed();
    changed |= ui.checkbox(&mut sec.sticky, "Sticky top header").changed();
    changed |= ui.checkbox(&mut sec.border_top, "Top border line").changed();
    changed |= ui.checkbox(&mut sec.border_bottom, "Bottom border line").changed();

    section_heading(ui, "Background");
    ui.horizontal(|ui| {
        if ghost_button(
            ui,
            "Solid",
            matches!(sec.background, SectionBackground::Solid { .. }),
        )
        .clicked()
        {
            sec.background = SectionBackground::Solid {
                color: theme_bg.clone(),
            };
            changed = true;
        }
        if ghost_button(
            ui,
            "Gradient",
            matches!(sec.background, SectionBackground::LinearGradient { .. }),
        )
        .clicked()
        {
            sec.background = SectionBackground::LinearGradient {
                start: theme_bg,
                end: theme_surface,
                angle: 180.0,
            };
            changed = true;
        }
        if ghost_button(
            ui,
            "Image",
            matches!(sec.background, SectionBackground::Image { .. }),
        )
        .clicked()
        {
            sec.background = SectionBackground::Image {
                asset_id: assets.first().map(|(id, _)| id.clone()),
                overlay_color: "#090D16".into(),
                overlay_opacity: 0.65,
            };
            changed = true;
        }
    });

    match &mut sec.background {
        SectionBackground::Solid { color } => {
            changed |= color_picker_hex(ui, "Color", color);
        }
        SectionBackground::LinearGradient { start, end, angle } => {
            changed |= color_picker_hex(ui, "Start", start);
            changed |= color_picker_hex(ui, "End", end);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Angle").color(TEXT_MUTED).size(12.0));
                changed |= ui
                    .add(egui::Slider::new(angle, 0.0..=360.0).suffix("°"))
                    .changed();
            });
        }
        SectionBackground::Image {
            asset_id,
            overlay_color,
            overlay_opacity,
        } => {
            let current_name = asset_id
                .as_deref()
                .and_then(|aid| assets.iter().find(|(id, _)| id == aid))
                .map(|(_, n)| n.as_str())
                .unwrap_or("Pick Asset...");
            egui::ComboBox::from_id_salt("sec_bg_asset")
                .selected_text(current_name)
                .width(f32::INFINITY)
                .show_ui(ui, |ui| {
                    for (aid, name) in &assets {
                        if ui
                            .selectable_label(asset_id.as_deref() == Some(aid), name)
                            .clicked()
                        {
                            *asset_id = Some(aid.clone());
                            changed = true;
                        }
                    }
                });
            changed |= color_picker_hex(ui, "Overlay", overlay_color);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Overlay Opacity").color(TEXT_MUTED).size(11.5));
                changed |= ui.add(egui::Slider::new(overlay_opacity, 0.0..=0.95)).changed();
            });
        }
    }

    section_heading(ui, "Mobile Behavior");
    egui::ComboBox::from_id_salt("sec_mobile_mode")
        .selected_text(sec.mobile_layout.label())
        .width(f32::INFINITY)
        .show_ui(ui, |ui| {
            for mode in [MobileLayoutMode::AutoStack, MobileLayoutMode::Scale] {
                if ui
                    .selectable_value(&mut sec.mobile_layout, mode, mode.label())
                    .changed()
                {
                    changed = true;
                }
            }
        });

    if changed {
        state.record_undo_from(before);
    }

    section_heading(ui, "Section Actions");
    let mut act: Option<SecAction> = None;
    ui.horizontal(|ui| {
        if let Some(idx) = sec_idx_opt {
            if idx > 0 && secondary_slate_button(ui, "Move Up").clicked() {
                act = Some(SecAction::Move(idx, idx - 1));
            }
            if idx + 1 < sec_count && secondary_slate_button(ui, "Move Down").clicked() {
                act = Some(SecAction::Move(idx, idx + 1));
            }
        }
    });
    ui.horizontal(|ui| {
        if secondary_slate_button(ui, "Duplicate Section").clicked() {
            act = Some(SecAction::Duplicate);
        }
        if ui
            .button(RichText::new("Delete Section").color(DANGER_INK))
            .clicked()
        {
            act = Some(SecAction::Delete);
        }
    });

    if let Some(a) = act {
        state.push_undo_snapshot();
        if let Some(p) = state.project.find_page_mut(&page_id) {
            match a {
                SecAction::Move(from, to) => p.sections.swap(from, to),
                SecAction::Duplicate => {
                    if let Some(pos) = p.sections.iter().position(|s| s.id == sec_id) {
                        let copy = p.sections[pos].duplicate();
                        let new_id = copy.id.clone();
                        p.sections.insert(pos + 1, copy);
                        state.selected_section_id = Some(new_id);
                    }
                }
                SecAction::Delete => {
                    p.sections.retain(|s| s.id != sec_id);
                    state.reconcile_selection();
                }
            }
        }
    }
}

enum SecAction {
    Move(usize, usize),
    Duplicate,
    Delete,
}

fn draw_page_site_inspector(ui: &mut Ui, state: &mut StudioState) {
    let before = state.project.clone();
    section_heading(ui, "Site Settings");
    let mut changed = false;
    ui.label(RichText::new("Production Base URL (for sitemap.xml)").color(TEXT_MUTED).size(11.5));
    changed |= ui
        .add(egui::TextEdit::singleline(&mut state.project.base_url).desired_width(f32::INFINITY))
        .changed();

    let page_id = state.selected_page_id.clone();
    if let Some(page) = state.project.find_page_mut(&page_id) {
        section_heading(ui, "Current Page SEO");
        ui.label(RichText::new("Page Title").color(TEXT_MUTED).size(11.5));
        changed |= ui
            .add(egui::TextEdit::singleline(&mut page.title).desired_width(f32::INFINITY))
            .changed();
        ui.label(RichText::new("Slug").color(TEXT_MUTED).size(11.5));
        if ui
            .add(egui::TextEdit::singleline(&mut page.slug).desired_width(f32::INFINITY))
            .changed()
        {
            changed = true;
        }
        ui.label(RichText::new("Meta Description").color(TEXT_MUTED).size(11.5));
        changed |= ui
            .add(
                egui::TextEdit::multiline(&mut page.meta_description)
                    .desired_rows(3)
                    .desired_width(f32::INFINITY),
            )
            .changed();
        ui.label(RichText::new("Custom <head> HTML").color(TEXT_MUTED).size(11.5));
        changed |= ui
            .add(
                egui::TextEdit::multiline(&mut page.custom_head_html)
                    .code_editor()
                    .desired_rows(4)
                    .desired_width(f32::INFINITY),
            )
            .changed();
    }

    if changed {
        state.record_undo_from(before);
    }
}
