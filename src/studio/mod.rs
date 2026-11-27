pub mod branding;
pub mod canvas;
pub mod home_screen;
pub mod inspector;
pub mod left_panel;
pub mod modals;
pub mod recent;
pub mod state;
pub mod style;
pub mod top_bar;

use eframe::egui;

use crate::templates::StarterTemplate;

use self::state::{AppScreen, StudioState};
use self::style::apply_slate_theme;

/// Native `eframe` application hosting the Weysko Studio workspace.
pub struct WeyskoApp {
    pub state: StudioState,
    theme_initialized: bool,
}

impl WeyskoApp {
    pub fn new(initial_project: crate::model::SiteProject) -> Self {
        Self {
            state: StudioState::new(initial_project),
            theme_initialized: false,
        }
    }

    pub fn new_home() -> Self {
        Self {
            state: StudioState::new_at_home(),
            theme_initialized: false,
        }
    }

    pub fn new_default() -> Self {
        Self::new(StarterTemplate::MinimalistPortfolio.build())
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        if self.state.screen == AppScreen::Home {
            return;
        }
        // Do not intercept shortcuts when typing in text fields or inline canvas edit.
        if ctx.wants_keyboard_input() || self.state.inline_edit.is_some() {
            return;
        }

        let ctrl = ctx.input(|i| i.modifiers.ctrl || i.modifiers.mac_cmd);
        let shift = ctx.input(|i| i.modifiers.shift);

        if ctrl && ctx.input(|i| i.key_pressed(egui::Key::Z)) {
            if shift {
                self.state.redo();
            } else {
                self.state.undo();
            }
            return;
        }
        if ctrl && ctx.input(|i| i.key_pressed(egui::Key::Y)) {
            self.state.redo();
            return;
        }
        if ctrl && ctx.input(|i| i.key_pressed(egui::Key::S)) {
            top_bar::save_project_dialog(&mut self.state, shift);
            return;
        }
        if ctrl && ctx.input(|i| i.key_pressed(egui::Key::D)) {
            self.state.duplicate_selected_element();
            return;
        }
        if ctrl && ctx.input(|i| i.key_pressed(egui::Key::C)) {
            if let Some(el_id) = self.state.selected_element_id.clone() {
                if let Some(page) = self.state.project.find_page(&self.state.selected_page_id) {
                    if let Some((_, el)) = page.find_element(&el_id) {
                        self.state.clipboard_element = Some(el.clone());
                        self.state.notify("Copied element", false);
                    }
                }
            }
            return;
        }
        if ctrl && ctx.input(|i| i.key_pressed(egui::Key::V)) {
            if let Some(clip) = self.state.clipboard_element.clone() {
                self.state.push_undo_snapshot();
                let pid = self.state.selected_page_id.clone();
                if let Some(page) = self.state.project.find_page_mut(&pid) {
                    let target_sid = self
                        .state
                        .selected_section_id
                        .clone()
                        .filter(|s| page.find_section(s).is_some())
                        .or_else(|| page.sections.first().map(|s| s.id.clone()));
                    if let Some(sid) = target_sid {
                        if let Some(sec) = page.find_section_mut(&sid) {
                            let copy = clip.duplicate(sec.next_z_index());
                            let copy_id = copy.id.clone();
                            sec.elements.push(copy);
                            self.state.selected_section_id = Some(sid);
                            self.state.selected_element_id = Some(copy_id);
                            self.state.notify("Pasted element", false);
                        }
                    }
                }
            }
            return;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)) {
            self.state.delete_selected_element();
            return;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::F1)) {
            self.state.show_shortcuts_modal = !self.state.show_shortcuts_modal;
            return;
        }
        if ctrl && ctx.input(|i| i.key_pressed(egui::Key::Num0)) {
            self.state.auto_fit_zoom = true;
            return;
        }
        if ctrl && ctx.input(|i| i.key_pressed(egui::Key::Equals) || i.key_pressed(egui::Key::Plus)) {
            self.state.auto_fit_zoom = false;
            self.state.zoom = (self.state.zoom + 0.1).min(1.5);
            return;
        }
        if ctrl && ctx.input(|i| i.key_pressed(egui::Key::Minus)) {
            self.state.auto_fit_zoom = false;
            self.state.zoom = (self.state.zoom - 0.1).max(0.35);
            return;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            if self.state.show_shortcuts_modal {
                self.state.show_shortcuts_modal = false;
            } else if self.state.show_export_modal {
                self.state.show_export_modal = false;
            } else if self.state.show_pages_modal {
                self.state.show_pages_modal = false;
            } else {
                self.state.selected_element_id = None;
            }
            return;
        }

        // Arrow key nudging for selected element
        let step = if shift { 10.0 } else { 1.0 };
        let mut dx = 0.0_f32;
        let mut dy = 0.0_f32;
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            dx -= step;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            dx += step;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            dy -= step;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            dy += step;
        }
        if (dx != 0.0 || dy != 0.0) && self.state.selected_element_id.is_some() {
            let el_id = self.state.selected_element_id.clone().unwrap();
            let pid = self.state.selected_page_id.clone();
            self.state.push_undo_snapshot();
            if let Some(page) = self.state.project.find_page_mut(&pid) {
                if let Some(el) = page.find_element_mut(&el_id) {
                    if !el.locked {
                        el.rect.x = (el.rect.x + dx).max(0.0);
                        el.rect.y = (el.rect.y + dy).max(0.0);
                    }
                }
            }
        }
    }
    /// Executes one full studio UI frame (`Home` or `Editor`) cleanly inside `ctx`.
    pub fn draw_frame(&mut self, ctx: &egui::Context) {
        if !self.theme_initialized {
            apply_slate_theme(ctx);
            self.theme_initialized = true;
        }

        match self.state.screen {
            AppScreen::Home => {
                home_screen::draw_home_screen(ctx, &mut self.state);
                modals::draw_modals_and_toasts(ctx, &mut self.state);
            }
            AppScreen::Editor => {
                self.state
                    .adapt_panels_to_width(ctx.screen_rect().width());
                self.handle_shortcuts(ctx);
                self.state.ensure_asset_textures(ctx);
                top_bar::draw_top_bar(ctx, &mut self.state);
                top_bar::draw_status_bar(ctx, &self.state);
                left_panel::draw_left_panel(ctx, &mut self.state);
                inspector::draw_inspector(ctx, &mut self.state);
                canvas::draw_canvas(ctx, &mut self.state);
                modals::draw_modals_and_toasts(ctx, &mut self.state);
                self.state.sync_preview_if_running();
            }
        }
    }
}

impl eframe::App for WeyskoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.draw_frame(ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FrameRunReport {
        used_rect: egui::Rect,
        wide_rects: Vec<(egui::Id, egui::Rect)>,
        text_boxes: Vec<(String, egui::Rect)>,
    }

    fn run_frames_at(app: &mut WeyskoApp, w: f32, h: f32) -> FrameRunReport {
        let ctx = egui::Context::default();
        let raw = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(w, h),
            )),
            ..Default::default()
        };
        let mut center_used = egui::Rect::NOTHING;
        let mut wide_rects = Vec::new();
        let mut text_boxes = Vec::new();
        for _ in 0..3 {
            wide_rects.clear();
            text_boxes.clear();
            let out = ctx.run(raw.clone(), |ctx| {
                app.draw_frame(ctx);
                center_used = ctx.used_rect();
            });
            for clipped in &out.shapes {
                let r = clipped.shape.visual_bounding_rect();
                if r.max.x > w + 2.0 {
                    wide_rects.push((egui::Id::new("shape"), r));
                }
                collect_text_boxes(
                    &clipped.shape,
                    clipped.clip_rect.intersect(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(w, h),
                    )),
                    &mut text_boxes,
                );
            }
        }
        FrameRunReport {
            used_rect: center_used,
            wide_rects,
            text_boxes,
        }
    }

    fn collect_text_boxes(
        shape: &egui::epaint::Shape,
        clip_rect: egui::Rect,
        out: &mut Vec<(String, egui::Rect)>,
    ) {
        match shape {
            egui::epaint::Shape::Text(text_shape) => {
                let raw_text = text_shape.galley.text().trim();
                if raw_text.is_empty() {
                    return;
                }
                let vis = text_shape.visual_bounding_rect().intersect(clip_rect);
                if vis.is_positive() && vis.width() > 1.0 && vis.height() > 1.0 {
                    out.push((raw_text.to_string(), vis));
                }
            }
            egui::epaint::Shape::Vec(shapes) => {
                for child in shapes {
                    collect_text_boxes(child, clip_rect, out);
                }
            }
            _ => {}
        }
    }

    fn find_text_overlaps(text_boxes: &[(String, egui::Rect)]) -> Vec<String> {
        let mut overlaps = Vec::new();
        for i in 0..text_boxes.len() {
            for j in (i + 1)..text_boxes.len() {
                let (ref ta, ra) = text_boxes[i];
                let (ref tb, rb) = text_boxes[j];
                let inter = ra.intersect(rb);
                if inter.is_positive() && inter.width() > 4.0 && inter.height() > 4.0 {
                    overlaps.push(format!(
                        "{:?} {:?} overlaps {:?} {:?} (inter={}x{})",
                        ta,
                        ra,
                        tb,
                        rb,
                        inter.width(),
                        inter.height()
                    ));
                }
            }
        }
        overlaps
    }

    #[test]
    fn start_and_editor_render_cleanly_at_non_fullscreen_sizes() {
        use super::recent::RecentProjectEntry;
        use super::state::{DrawerTab, HomeSectionTab};
        use std::path::PathBuf;

        let sizes = [
            (680.0, 480.0),
            (820.0, 560.0),
            (980.0, 660.0),
            (1280.0, 780.0),
            (1440.0, 900.0),
        ];

        for (w, h) in sizes {
            for tab in [
                HomeSectionTab::Home,
                HomeSectionTab::Templates,
                HomeSectionTab::NewFromScratch,
            ] {
                for with_recents in [false, true] {
                    if tab != HomeSectionTab::Home && with_recents {
                        continue;
                    }
                    let mut home_app = WeyskoApp::new_home();
                    home_app.state.home_tab = tab;
                    if with_recents {
                        home_app.state.recent_projects.entries.push(RecentProjectEntry {
                            name: "Meridian Cloud Flagship Production Portal".into(),
                            path: PathBuf::from(
                                r"C:\Users\developer\Documents\clients\meridian\meridian-cloud.weysko",
                            ),
                            theme_label: "Obsidian Slate".into(),
                            page_count: 3,
                            element_count: 24,
                            opened_at_unix: 1_795_000_000,
                        });
                    } else {
                        home_app.state.recent_projects.entries.clear();
                    }

                    let FrameRunReport {
                        used_rect: used_home,
                        wide_rects: wide_home,
                        text_boxes: texts_home,
                    } = run_frames_at(&mut home_app, w, h);
                    assert!(
                        used_home.max.x <= w + 2.0,
                        "Home ({:?}, recents={}) exceeded width {} at {}x{}: {:?} wide_shapes={:?}",
                        tab,
                        with_recents,
                        w,
                        w,
                        h,
                        used_home,
                        wide_home
                    );
                    let overlaps = find_text_overlaps(&texts_home);
                    assert!(
                        overlaps.is_empty(),
                        "Home ({:?}, recents={}) text overlap at {}x{}:\n{}",
                        tab,
                        with_recents,
                        w,
                        h,
                        overlaps.join("\n")
                    );
                }
            }

            let mut editor_app = WeyskoApp::new(StarterTemplate::MeridianDemo.build());
            let first_el = editor_app
                .state
                .project
                .pages
                .first()
                .and_then(|p| p.sections.first())
                .and_then(|s| s.elements.first())
                .map(|e| e.id.clone());
            editor_app.state.selected_element_id = first_el;

            for drawer in DrawerTab::all() {
                editor_app.state.active_drawer = Some(*drawer);
                let FrameRunReport {
                    used_rect: used_ed,
                    wide_rects: wide_ed,
                    text_boxes: texts_ed,
                } = run_frames_at(&mut editor_app, w, h);
                assert!(
                    used_ed.max.x <= w + 2.0,
                    "Editor ({:?}) exceeded width {} at {}x{}: {:?} wide_shapes={:?}",
                    drawer,
                    w,
                    w,
                    h,
                    used_ed,
                    wide_ed
                );
                assert!(
                    editor_app.state.zoom >= 0.24 && editor_app.state.zoom <= 1.0,
                    "Zoom out of expected fit bounds at {}x{}: {}",
                    w,
                    h,
                    editor_app.state.zoom
                );
                if w < 1080.0 {
                    assert!(!editor_app.state.show_inspector);
                }
                let chrome_texts: Vec<_> = texts_ed
                    .into_iter()
                    .filter(|(_, r)| {
                        let c = r.center();
                        !editor_app
                            .state
                            .last_canvas_strip_rects
                            .iter()
                            .any(|strip| strip.contains(c))
                    })
                    .collect();
                let overlaps = find_text_overlaps(&chrome_texts);
                assert!(
                    overlaps.is_empty(),
                    "Editor chrome ({:?}) text overlap at {}x{}:\n{}",
                    drawer,
                    w,
                    h,
                    overlaps.join("\n")
                );
            }

            // Also verify Inspector panels cleanly at every size across Content / Style / Layout tabs.
            editor_app.state.active_drawer = None;
            editor_app.state.show_inspector = true;
            for itab in [
                super::state::InspectorTab::Content,
                super::state::InspectorTab::Style,
                super::state::InspectorTab::Layout,
            ] {
                editor_app.state.inspector_tab = itab;
                let FrameRunReport {
                    used_rect: used_ed,
                    wide_rects: wide_ed,
                    text_boxes: texts_ed,
                } = run_frames_at(&mut editor_app, w, h);
                assert!(
                    used_ed.max.x <= w + 2.0,
                    "Editor Inspector ({:?}) exceeded width {} at {}x{}: {:?} wide_shapes={:?}",
                    itab,
                    w,
                    w,
                    h,
                    used_ed,
                    wide_ed
                );
                let chrome_texts: Vec<_> = texts_ed
                    .into_iter()
                    .filter(|(_, r)| {
                        let c = r.center();
                        !editor_app
                            .state
                            .last_canvas_strip_rects
                            .iter()
                            .any(|strip| strip.contains(c))
                    })
                    .collect();
                let overlaps = find_text_overlaps(&chrome_texts);
                assert!(
                    overlaps.is_empty(),
                    "Editor Inspector ({:?}) text overlap at {}x{}:\n{}",
                    itab,
                    w,
                    h,
                    overlaps.join("\n")
                );
            }
        }
    }
}
