use egui::{Align2, Color32, CornerRadius, Frame, Margin, RichText, ScrollArea, Stroke};

use crate::generator::exporter::{
    export_to_folder, export_to_single_file, export_to_weysko_file, export_to_zip,
    generate_google_sites_embed,
};
use crate::model::{sanitize_slug, Page};
use crate::templates::SectionPreset;

use super::state::{ExportModalTab, StudioState};
use super::style::{
    primary_white_button, secondary_slate_button, section_heading, segmented_tabs, BG_INPUT,
    BG_PANEL, BG_SURFACE, BORDER_STRONG, DANGER_INK, SUCCESS_INK, TEXT_DIM, TEXT_MUTED, WHITE,
};

pub fn draw_modals_and_toasts(ctx: &egui::Context, state: &mut StudioState) {
    if state.show_export_modal {
        draw_export_modal(ctx, state);
    }
    if state.show_pages_modal {
        draw_pages_modal(ctx, state);
    }
    if state.show_shortcuts_modal {
        draw_shortcuts_modal(ctx, state);
    }
    draw_toast(ctx, state);
}

fn draw_export_modal(ctx: &egui::Context, state: &mut StudioState) {
    let mut open = state.show_export_modal;
    egui::Window::new("Export & Publish Website")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .fixed_size(egui::vec2(620.0, 450.0))
        .frame(
            Frame::new()
                .fill(BG_PANEL)
                .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
                .corner_radius(CornerRadius::same(12))
                .inner_margin(Margin::same(18)),
        )
        .show(ctx, |ui| {
            ui.label(
                RichText::new("Compile 100% deterministic HTML5, CSS3 & vanilla JS or save a portable .weysko project package.")
                    .color(TEXT_MUTED)
                    .size(12.0),
            );
            ui.add_space(10.0);

            segmented_tabs(
                ui,
                &mut state.export_tab,
                &[
                    (ExportModalTab::Folder, "Static Folder"),
                    (ExportModalTab::Zip, "ZIP Archive"),
                    (ExportModalTab::SingleFile, "Single-File HTML"),
                    (ExportModalTab::GoogleSites, "Google Sites"),
                    (ExportModalTab::WeyskoFile, ".weysko File"),
                ],
            );
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(10.0);

            match state.export_tab {
                ExportModalTab::Folder => {
                    section_heading(ui, "Deploy to Any Web Server / GitHub Pages / S3");
                    ui.label(
                        RichText::new("Exports clean multi-page .html files, styles.css, site.js, assets/, 404.html, and optional SEO files.")
                            .color(TEXT_MUTED)
                            .size(12.0),
                    );
                    ui.add_space(8.0);
                    ui.checkbox(&mut state.export_options.include_sitemap, "Generate sitemap.xml");
                    ui.checkbox(&mut state.export_options.include_robots, "Generate robots.txt");
                    ui.checkbox(&mut state.export_options.include_nojekyll, "Include .nojekyll (for GitHub Pages)");
                    ui.add_space(12.0);

                    ui.horizontal(|ui| {
                        if primary_white_button(ui, "Choose Output Folder & Export").clicked() {
                            if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                                match export_to_folder(&state.project, &dir, &state.export_options) {
                                    Ok(summary) => {
                                        state.last_export_folder = Some(summary.destination.clone());
                                        state.notify(
                                            format!(
                                                "Exported {} files ({} KB) to {}",
                                                summary.file_count,
                                                (summary.total_bytes / 1024).max(1),
                                                summary.destination.display()
                                            ),
                                            false,
                                        );
                                    }
                                    Err(e) => state.notify(format!("Export failed: {}", e), true),
                                }
                            }
                        }
                        if let Some(last) = &state.last_export_folder {
                            if secondary_slate_button(ui, "Open Folder").clicked() {
                                let _ = open::that(last);
                            }
                        }
                    });
                }
                ExportModalTab::Zip => {
                    section_heading(ui, "Single Archive for Netlify Drop, Cloudflare Pages & cPanel");
                    ui.label(
                        RichText::new("Creates a compressed .zip archive containing the complete static website ready for drag-and-drop hosting.")
                            .color(TEXT_MUTED)
                            .size(12.0),
                    );
                    ui.add_space(8.0);
                    ui.checkbox(&mut state.export_options.include_sitemap, "Include sitemap.xml");
                    ui.checkbox(&mut state.export_options.include_robots, "Include robots.txt");
                    ui.add_space(12.0);

                    if primary_white_button(ui, "Export .ZIP Archive").clicked() {
                        let default_zip = format!("{}.zip", sanitize_slug(&state.project.name));
                        if let Some(zip_path) = rfd::FileDialog::new()
                            .set_file_name(&default_zip)
                            .add_filter("ZIP Archive", &["zip"])
                            .save_file()
                        {
                            match export_to_zip(&state.project, &zip_path, &state.export_options) {
                                Ok(summary) => state.notify(
                                    format!(
                                        "Packaged {} files into {}",
                                        summary.file_count,
                                        summary.destination.display()
                                    ),
                                    false,
                                ),
                                Err(e) => state.notify(format!("ZIP export failed: {}", e), true),
                            }
                        }
                    }
                }
                ExportModalTab::SingleFile => {
                    section_heading(ui, "Self-Contained Portable HTML");
                    ui.label(
                        RichText::new("All pages, CSS, JavaScript, and embedded images are bundled into one standalone .html file with built-in page switching.")
                            .color(TEXT_MUTED)
                            .size(12.0),
                    );
                    ui.add_space(12.0);
                    if primary_white_button(ui, "Save Single-File HTML").clicked() {
                        let default_html = format!("{}.html", sanitize_slug(&state.project.name));
                        if let Some(path) = rfd::FileDialog::new()
                            .set_file_name(&default_html)
                            .add_filter("HTML Document", &["html"])
                            .save_file()
                        {
                            match export_to_single_file(&state.project, &path) {
                                Ok(summary) => state.notify(
                                    format!(
                                        "Saved single-file site ({} KB) to {}",
                                        (summary.total_bytes / 1024).max(1),
                                        summary.destination.display()
                                    ),
                                    false,
                                ),
                                Err(e) => state.notify(format!("Export failed: {}", e), true),
                            }
                        }
                    }
                }
                ExportModalTab::GoogleSites => {
                    section_heading(ui, "Publish Inside Google Sites");
                    ui.label(
                        RichText::new("1. Click 'Copy Embed HTML' below.\n2. Open Google Sites → Insert → Embed → Embed code.\n3. Paste the code, click Next → Insert, and resize the frame full-width.")
                            .color(TEXT_MUTED)
                            .size(12.0),
                    );
                    ui.add_space(8.0);
                    let kb = (state.google_sites_embed_cache.len() / 1024).max(1);
                    ui.label(
                        RichText::new(format!(
                            "Bundle size: {} KB ({} chars) — self-contained with inline CSS/JS & images.",
                            kb,
                            state.google_sites_embed_cache.len()
                        ))
                        .color(TEXT_DIM)
                        .size(11.0),
                    );
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        if primary_white_button(ui, "Copy Embed HTML to Clipboard").clicked() {
                            let bundle = generate_google_sites_embed(&state.project);
                            state.google_sites_embed_cache = bundle.clone();
                            ctx.copy_text(bundle);
                            state.notify("Copied Google Sites embed HTML to clipboard", false);
                        }
                        if secondary_slate_button(ui, "Save as .html").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .set_file_name("google-sites-embed.html")
                                .add_filter("HTML", &["html"])
                                .save_file()
                            {
                                let _ = export_to_single_file(&state.project, &path);
                                state.notify("Saved embed HTML file", false);
                            }
                        }
                    });
                    ui.add_space(8.0);
                    ScrollArea::vertical().max_height(130.0).show(ui, |ui| {
                        let mut preview_slice: String = state
                            .google_sites_embed_cache
                            .chars()
                            .take(1600)
                            .collect();
                        ui.add(
                            egui::TextEdit::multiline(&mut preview_slice)
                                .code_editor()
                                .desired_width(f32::INFINITY),
                        );
                    });
                }
                ExportModalTab::WeyskoFile => {
                    section_heading(ui, "Portable .weysko Editable Project File");
                    ui.label(
                        RichText::new("Saves the complete editable website into a single .weysko file preserving all pages, sections, design tokens, breakpoint overrides, and embedded media assets. Re-open later in Weysko Studio or share across Windows, macOS & Linux.")
                            .color(TEXT_MUTED)
                            .size(12.0),
                    );
                    ui.add_space(10.0);
                    let elem_count: usize = state
                        .project
                        .pages
                        .iter()
                        .flat_map(|p| p.sections.iter())
                        .map(|s| s.elements.len())
                        .sum();
                    ui.label(
                        RichText::new(format!(
                            "Bundle contents: {} page(s), {} element(s), {} embedded asset(s), theme '{}'",
                            state.project.pages.len(),
                            elem_count,
                            state.project.assets.len(),
                            state.project.theme.preset.label()
                        ))
                        .color(TEXT_DIM)
                        .size(11.5),
                    );
                    ui.add_space(14.0);
                    if primary_white_button(ui, "Export .weysko File").clicked() {
                        let default_file = format!("{}.weysko", sanitize_slug(&state.project.name));
                        if let Some(path) = rfd::FileDialog::new()
                            .set_file_name(&default_file)
                            .add_filter("Weysko Project", &["weysko"])
                            .save_file()
                        {
                            match export_to_weysko_file(&state.project, &path) {
                                Ok(summary) => {
                                    state.current_file_path = Some(path.clone());
                                    state.dirty = false;
                                    state.recent_projects.record(&state.project, &path);
                                    state.recent_projects.save();
                                    state.notify(
                                        format!(
                                            "Exported .weysko file ({} KB) to {}",
                                            (summary.total_bytes / 1024).max(1),
                                            summary.destination.display()
                                        ),
                                        false,
                                    );
                                }
                                Err(e) => state.notify(format!("Export failed: {}", e), true),
                            }
                        }
                    }
                }
            }
        });
    state.show_export_modal = open;
}

fn draw_pages_modal(ctx: &egui::Context, state: &mut StudioState) {
    let mut open = state.show_pages_modal;
    egui::Window::new("Manage Site Pages")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .fixed_size(egui::vec2(460.0, 380.0))
        .frame(
            Frame::new()
                .fill(BG_PANEL)
                .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
                .corner_radius(CornerRadius::same(12))
                .inner_margin(Margin::same(16)),
        )
        .show(ctx, |ui| {
            section_heading(ui, "Pages");
            let total_pages = state.project.pages.len();
            let pages_meta: Vec<(String, String, String)> = state
                .project
                .pages
                .iter()
                .map(|p| (p.id.clone(), p.title.clone(), p.html_filename()))
                .collect();
            let mut action: Option<PageModalAction> = None;

            ScrollArea::vertical().max_height(210.0).show(ui, |ui| {
                for (idx, (pid, title, filename)) in pages_meta.iter().enumerate() {
                    let active = pid == &state.selected_page_id;
                    Frame::new()
                        .fill(if active { BG_SURFACE } else { BG_INPUT })
                        .stroke(Stroke::new(1.0_f32, if active { WHITE } else { BORDER_STRONG }))
                        .corner_radius(CornerRadius::same(6))
                        .inner_margin(Margin::same(8))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                if ui
                                    .selectable_label(
                                        active,
                                        RichText::new(format!("{}  (/{})", title, filename))
                                            .color(WHITE)
                                            .size(12.5),
                                    )
                                    .clicked()
                                {
                                    action = Some(PageModalAction::Select(pid.clone()));
                                }
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if total_pages > 1
                                        && ui
                                            .small_button(RichText::new("Del").color(DANGER_INK))
                                            .clicked()
                                    {
                                        action = Some(PageModalAction::Delete(pid.clone()));
                                    }
                                    if ui.small_button("Dup").clicked() {
                                        action = Some(PageModalAction::Duplicate(pid.clone()));
                                    }
                                    if idx > 0 && ui.small_button("↑").clicked() {
                                        action = Some(PageModalAction::Swap(idx, idx - 1));
                                    }
                                });
                            });
                        });
                    ui.add_space(4.0);
                }
            });

            if let Some(act) = action {
                match act {
                    PageModalAction::Select(pid) => {
                        state.selected_page_id = pid;
                        state.selected_element_id = None;
                        state.reconcile_selection();
                    }
                    PageModalAction::Delete(pid) => {
                        state.push_undo_snapshot();
                        state.project.pages.retain(|p| p.id != pid);
                        state.reconcile_selection();
                    }
                    PageModalAction::Duplicate(pid) => {
                        if let Some(orig) = state.project.find_page(&pid).cloned() {
                            state.push_undo_snapshot();
                            let copy = orig.duplicate(
                                format!("{} Copy", orig.title),
                                format!("{}-copy", orig.slug),
                            );
                            let new_id = copy.id.clone();
                            state.project.pages.push(copy);
                            state.selected_page_id = new_id;
                        }
                    }
                    PageModalAction::Swap(a, b) => {
                        state.push_undo_snapshot();
                        state.project.pages.swap(a, b);
                    }
                }
            }

            ui.add_space(8.0);
            ui.separator();
            section_heading(ui, "Add New Page");
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut state.new_page_draft.title)
                        .hint_text("Title")
                        .desired_width(140.0),
                );
                ui.add(
                    egui::TextEdit::singleline(&mut state.new_page_draft.slug)
                        .hint_text("slug")
                        .desired_width(120.0),
                );
                if primary_white_button(ui, "+ Add Page").clicked() {
                    let title = if state.new_page_draft.title.trim().is_empty() {
                        "New Page".to_string()
                    } else {
                        state.new_page_draft.title.trim().to_string()
                    };
                    let slug = sanitize_slug(&state.new_page_draft.slug);
                    state.push_undo_snapshot();
                    let mut new_page = Page::new(title, slug);
                    let tokens = state.project.theme.clone();
                    new_page.sections = vec![
                        SectionPreset::NavbarStrip.build(&tokens),
                        SectionPreset::HeroCentered.build(&tokens),
                        SectionPreset::FooterStrip.build(&tokens),
                    ];
                    let pid = new_page.id.clone();
                    state.project.pages.push(new_page);
                    state.selected_page_id = pid;
                    state.selected_element_id = None;
                    state.reconcile_selection();
                    state.notify("Created new page", false);
                }
            });
        });
    state.show_pages_modal = open;
}

enum PageModalAction {
    Select(String),
    Delete(String),
    Duplicate(String),
    Swap(usize, usize),
}

fn draw_shortcuts_modal(ctx: &egui::Context, state: &mut StudioState) {
    let mut open = state.show_shortcuts_modal;
    egui::Window::new("Keyboard Shortcuts & Quick Guide")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .fixed_size(egui::vec2(460.0, 390.0))
        .frame(
            Frame::new()
                .fill(BG_PANEL)
                .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
                .corner_radius(CornerRadius::same(12))
                .inner_margin(Margin::same(18)),
        )
        .show(ctx, |ui| {
            section_heading(ui, "Canvas & Editing");
            for (keys, desc) in [
                ("Double-Click Text", "Edit Heading, Paragraph, Button, or Badge inline"),
                ("Drag / 8-Point Handles", "Move or resize with 8px grid snap & smart guides"),
                ("Right-Click Element/Strip", "Context menu for z-order, lock, duplicate, insert"),
                ("Arrow / Shift+Arrow", "Nudge selected element 1px / 10px"),
                ("Ctrl+Z / Ctrl+Y", "Undo / Redo"),
                ("Ctrl+C / Ctrl+V / Ctrl+D", "Copy / Paste / Duplicate element"),
                ("Delete / Backspace", "Delete selected element"),
                ("Ctrl+S / Ctrl+Shift+S", "Save / Save As (.weysko)"),
                ("Ctrl+= / Ctrl+- / Ctrl+0", "Zoom in / out / auto-fit canvas"),
                ("F1", "Toggle this shortcuts reference"),
            ] {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("{:<24}", keys))
                            .font(egui::FontId::monospace(11.5))
                            .color(WHITE),
                    );
                    ui.label(RichText::new(desc).color(TEXT_MUTED).size(11.5));
                });
            }
        });
    state.show_shortcuts_modal = open;
}

fn draw_toast(ctx: &egui::Context, state: &mut StudioState) {
    let Some(toast) = state.toast.clone() else {
        return;
    };
    if toast.created_at.elapsed().as_secs_f32() > 3.5 {
        state.toast = None;
        return;
    }

    egui::Area::new(egui::Id::new("weysko_toast_area"))
        .anchor(Align2::RIGHT_BOTTOM, egui::vec2(-20.0, -34.0))
        .show(ctx, |ui| {
            let stroke_col = if toast.is_error { DANGER_INK } else { SUCCESS_INK };
            Frame::new()
                .fill(BG_SURFACE)
                .stroke(Stroke::new(1.0_f32, stroke_col))
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::symmetric(14, 10))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new(&toast.text)
                            .color(if toast.is_error { DANGER_INK } else { Color32::WHITE })
                            .size(12.5),
                    );
                });
        });
}
