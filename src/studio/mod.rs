pub mod branding;
pub mod canvas;
pub mod inspector;
pub mod left_panel;
pub mod modals;
pub mod state;
pub mod style;
pub mod top_bar;

use eframe::egui;

use crate::templates::StarterTemplate;

use self::state::StudioState;
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

    pub fn new_default() -> Self {
        Self::new(StarterTemplate::MinimalistPortfolio.build())
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
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
}

impl eframe::App for WeyskoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.theme_initialized {
            apply_slate_theme(ctx);
            self.theme_initialized = true;
        }

        self.handle_shortcuts(ctx);
        self.state.ensure_asset_textures(ctx);
        top_bar::draw_top_bar(ctx, &mut self.state);
        left_panel::draw_left_panel(ctx, &mut self.state);
        inspector::draw_inspector(ctx, &mut self.state);
        canvas::draw_canvas(ctx, &mut self.state);
        modals::draw_modals_and_toasts(ctx, &mut self.state);

        self.state.sync_preview_if_running();
    }
}
