use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

use egui::TextureHandle;

use crate::generator::{compile_site, ExportOptions};
use crate::history::HistoryStack;
use crate::model::{Element, ElementKind, Rect2D, SiteProject, ViewportMode};
use crate::preview::PreviewServer;
use crate::templates::{SectionPreset, StarterTemplate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerTab {
    Add,
    Sections,
    Templates,
    Layers,
    Theme,
    Assets,
}

impl DrawerTab {
    pub fn all() -> &'static [DrawerTab] {
        &[
            DrawerTab::Add,
            DrawerTab::Sections,
            DrawerTab::Templates,
            DrawerTab::Layers,
            DrawerTab::Theme,
            DrawerTab::Assets,
        ]
    }

    pub fn short_label(&self) -> &'static str {
        match self {
            DrawerTab::Add => "Add",
            DrawerTab::Sections => "Strips",
            DrawerTab::Templates => "Sites",
            DrawerTab::Layers => "Layers",
            DrawerTab::Theme => "Theme",
            DrawerTab::Assets => "Media",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            DrawerTab::Add => "+",
            DrawerTab::Sections => "[]",
            DrawerTab::Templates => "T",
            DrawerTab::Layers => "=",
            DrawerTab::Theme => "O",
            DrawerTab::Assets => "IMG",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectorTab {
    Content,
    Layout,
    Style,
    Effects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportModalTab {
    Folder,
    Zip,
    SingleFile,
    GoogleSites,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeHandle {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

#[derive(Debug, Clone)]
pub enum DragMode {
    MoveElement {
        section_id: String,
        element_id: String,
        start_rect: Rect2D,
        start_pointer: egui::Pos2,
        pre_drag_snapshot: SiteProject,
    },
    ResizeElement {
        section_id: String,
        element_id: String,
        handle: ResizeHandle,
        start_rect: Rect2D,
        start_pointer: egui::Pos2,
        pre_drag_snapshot: SiteProject,
    },
    ResizeSection {
        section_id: String,
        start_height: f32,
        start_pointer_y: f32,
        pre_drag_snapshot: SiteProject,
    },
}

#[derive(Debug, Clone)]
pub struct InlineEditState {
    pub element_id: String,
    pub buffer: String,
    pub pre_edit_snapshot: SiteProject,
}

#[derive(Debug, Clone)]
pub struct ToastMessage {
    pub text: String,
    pub is_error: bool,
    pub created_at: Instant,
}

#[derive(Debug, Clone)]
pub struct NewPageDraft {
    pub title: String,
    pub slug: String,
}

/// Complete Studio state driving top bar, panels, inspector, canvas, and modals.
pub struct StudioState {
    pub project: SiteProject,
    pub history: HistoryStack<SiteProject>,
    pub selected_page_id: String,
    pub selected_section_id: Option<String>,
    pub selected_element_id: Option<String>,
    pub viewport: ViewportMode,
    pub zoom: f32,
    pub auto_fit_zoom: bool,
    pub show_grid: bool,
    pub snap_to_grid: bool,
    pub show_inspector: bool,
    pub active_drawer: Option<DrawerTab>,
    pub add_search: String,
    pub inspector_tab: InspectorTab,
    pub drag_mode: Option<DragMode>,
    pub inline_edit: Option<InlineEditState>,
    pub clipboard_element: Option<Element>,
    pub current_file_path: Option<PathBuf>,
    pub dirty: bool,
    pub show_export_modal: bool,
    pub export_tab: ExportModalTab,
    pub export_options: ExportOptions,
    pub last_export_folder: Option<PathBuf>,
    pub google_sites_embed_cache: String,
    pub show_pages_modal: bool,
    pub show_shortcuts_modal: bool,
    pub last_inspector_undo_at: Option<Instant>,
    pub new_page_draft: NewPageDraft,
    pub confirm_template: Option<StarterTemplate>,
    pub toast: Option<ToastMessage>,
    pub textures: HashMap<String, TextureHandle>,
    pub preview_server: Option<PreviewServer>,
    pub preview_dirty: bool,
}

impl StudioState {
    pub fn new(project: SiteProject) -> Self {
        let first_page_id = project
            .pages
            .first()
            .map(|p| p.id.clone())
            .unwrap_or_default();
        let first_section_id = project
            .pages
            .first()
            .and_then(|p| p.sections.first())
            .map(|s| s.id.clone());

        Self {
            project,
            history: HistoryStack::new(100),
            selected_page_id: first_page_id,
            selected_section_id: first_section_id,
            selected_element_id: None,
            viewport: ViewportMode::Desktop,
            zoom: 0.72,
            auto_fit_zoom: true,
            show_grid: true,
            snap_to_grid: true,
            show_inspector: true,
            active_drawer: Some(DrawerTab::Add),
            add_search: String::new(),
            inspector_tab: InspectorTab::Content,
            drag_mode: None,
            inline_edit: None,
            clipboard_element: None,
            current_file_path: None,
            dirty: false,
            show_export_modal: false,
            export_tab: ExportModalTab::Folder,
            export_options: ExportOptions::default(),
            last_export_folder: None,
            google_sites_embed_cache: String::new(),
            show_pages_modal: false,
            show_shortcuts_modal: false,
            last_inspector_undo_at: None,
            new_page_draft: NewPageDraft {
                title: "Services".into(),
                slug: "services".into(),
            },
            confirm_template: None,
            toast: None,
            textures: HashMap::new(),
            preview_server: None,
            preview_dirty: false,
        }
    }

    pub fn push_undo_snapshot(&mut self) {
        self.history.push(self.project.clone());
        self.dirty = true;
        self.preview_dirty = true;
    }

    pub fn mark_mutated(&mut self) {
        self.dirty = true;
        self.preview_dirty = true;
    }

    /// Coalesces rapid inspector/theme mutations into clean undo snapshots.
    pub fn record_undo_from(&mut self, before: SiteProject) {
        let should_push = match self.last_inspector_undo_at {
            None => true,
            Some(ts) => ts.elapsed().as_secs_f32() > 1.0,
        };
        if should_push && before != self.project {
            self.history.push(before);
            self.last_inspector_undo_at = Some(Instant::now());
        }
        self.dirty = true;
        self.preview_dirty = true;
    }

    pub fn bring_element_to_front(&mut self, el_id: &str) {
        self.push_undo_snapshot();
        if let Some(page) = self.project.find_page_mut(&self.selected_page_id) {
            for sec in &mut page.sections {
                if sec.elements.iter().any(|e| e.id == el_id) {
                    let top_z = sec.next_z_index();
                    if let Some(el) = sec.elements.iter_mut().find(|e| e.id == el_id) {
                        el.z_index = top_z;
                    }
                    return;
                }
            }
        }
    }

    pub fn send_element_to_back(&mut self, el_id: &str) {
        self.push_undo_snapshot();
        if let Some(page) = self.project.find_page_mut(&self.selected_page_id) {
            for sec in &mut page.sections {
                if sec.elements.iter().any(|e| e.id == el_id) {
                    let min_z = sec.elements.iter().map(|e| e.z_index).min().unwrap_or(1);
                    if let Some(el) = sec.elements.iter_mut().find(|e| e.id == el_id) {
                        el.z_index = (min_z - 1).max(0);
                    }
                    return;
                }
            }
        }
    }

    pub fn toggle_element_lock(&mut self, el_id: &str) {
        self.push_undo_snapshot();
        if let Some(page) = self.project.find_page_mut(&self.selected_page_id) {
            if let Some(el) = page.find_element_mut(el_id) {
                el.locked = !el.locked;
            }
        }
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.history.undo(&self.project) {
            self.project = prev;
            self.reconcile_selection();
            self.dirty = true;
            self.preview_dirty = true;
            self.notify("Undid last change", false);
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.history.redo(&self.project) {
            self.project = next;
            self.reconcile_selection();
            self.dirty = true;
            self.preview_dirty = true;
            self.notify("Redid change", false);
        }
    }

    pub fn reconcile_selection(&mut self) {
        if self.project.find_page(&self.selected_page_id).is_none() {
            self.selected_page_id = self
                .project
                .pages
                .first()
                .map(|p| p.id.clone())
                .unwrap_or_default();
        }
        if let Some(page) = self.project.find_page(&self.selected_page_id) {
            if let Some(el_id) = &self.selected_element_id {
                if page.find_element(el_id).is_none() {
                    self.selected_element_id = None;
                }
            }
            if let Some(sec_id) = &self.selected_section_id {
                if page.find_section(sec_id).is_none() {
                    self.selected_section_id = page.sections.first().map(|s| s.id.clone());
                }
            } else {
                self.selected_section_id = page.sections.first().map(|s| s.id.clone());
            }
        }
    }

    pub fn notify(&mut self, text: impl Into<String>, is_error: bool) {
        self.toast = Some(ToastMessage {
            text: text.into(),
            is_error,
            created_at: Instant::now(),
        });
    }

    pub fn sync_preview_if_running(&mut self) {
        if self.preview_dirty {
            if let Some(server) = &self.preview_server {
                let compiled = compile_site(&self.project);
                server.update_site(compiled);
            }
            self.preview_dirty = false;
        }
    }

    /// Inserts a new element into the currently selected section (or creates one if page has none).
    pub fn insert_element(&mut self, kind: ElementKind) {
        self.push_undo_snapshot();
        let tokens = self.project.theme.clone();
        if let Some(page) = self.project.find_page_mut(&self.selected_page_id) {
            if page.sections.is_empty() {
                page.sections.push(SectionPreset::BlankSection.build(&tokens));
            }
            let target_sec_id = self
                .selected_section_id
                .clone()
                .filter(|sid| page.find_section(sid).is_some())
                .unwrap_or_else(|| page.sections[0].id.clone());

            if let Some(sec) = page.find_section_mut(&target_sec_id) {
                let next_z = sec.next_z_index();
                let mut el = Element::new_default(kind, &tokens, next_z);
                if !el.full_width {
                    let count_offset = ((sec.elements.len() % 5) as f32) * 18.0;
                    el.rect.x = ((1200.0 - el.rect.w) * 0.5 + count_offset).clamp(24.0, 1160.0 - el.rect.w.min(1100.0));
                    el.rect.y = (40.0 + count_offset).min((sec.height - el.rect.h).max(20.0));
                }
                let el_id = el.id.clone();
                sec.elements.push(el);
                self.selected_section_id = Some(target_sec_id);
                self.selected_element_id = Some(el_id);
                self.notify(format!("Added {}", kind.label()), false);
            }
        }
    }

    /// Inserts a pre-built section strip after the currently selected section.
    pub fn insert_section_preset(&mut self, preset: SectionPreset) {
        self.push_undo_snapshot();
        let tokens = self.project.theme.clone();
        let mut new_sec = preset.build(&tokens);
        // If inserting a footer, place at the end; navbar at the top; otherwise after selected section.
        if let Some(page) = self.project.find_page_mut(&self.selected_page_id) {
            let new_sec_id = new_sec.id.clone();
            match preset {
                SectionPreset::NavbarStrip => {
                    page.sections.insert(0, new_sec);
                }
                SectionPreset::FooterStrip => {
                    page.sections.push(new_sec);
                }
                _ => {
                    let insert_idx = self
                        .selected_section_id
                        .as_deref()
                        .and_then(|sid| page.sections.iter().position(|s| s.id == sid))
                        .map(|i| i + 1)
                        .unwrap_or(page.sections.len());
                    page.sections.insert(insert_idx, std::mem::replace(&mut new_sec, SectionPreset::BlankSection.build(&tokens)));
                }
            }
            self.selected_section_id = Some(new_sec_id);
            self.selected_element_id = None;
            self.notify(format!("Inserted {}", preset.label()), false);
        }
    }

    /// Applies a starter template, replacing the current project (saving undo snapshot first).
    pub fn apply_template(&mut self, tmpl: StarterTemplate) {
        self.push_undo_snapshot();
        self.project = tmpl.build();
        self.selected_page_id = self
            .project
            .pages
            .first()
            .map(|p| p.id.clone())
            .unwrap_or_default();
        self.selected_section_id = self
            .project
            .pages
            .first()
            .and_then(|p| p.sections.first())
            .map(|s| s.id.clone());
        self.selected_element_id = None;
        self.textures.clear();
        self.confirm_template = None;
        self.notify(format!("Loaded template: {}", tmpl.label()), false);
    }

    /// Deletes the currently selected element, if any.
    pub fn delete_selected_element(&mut self) {
        let Some(el_id) = self.selected_element_id.clone() else {
            return;
        };
        self.push_undo_snapshot();
        if let Some(page) = self.project.find_page_mut(&self.selected_page_id) {
            for sec in &mut page.sections {
                if let Some(pos) = sec.elements.iter().position(|e| e.id == el_id) {
                    sec.elements.remove(pos);
                    self.selected_element_id = None;
                    self.notify("Deleted element", false);
                    return;
                }
            }
        }
    }

    /// Duplicates the currently selected element.
    pub fn duplicate_selected_element(&mut self) {
        let Some(el_id) = self.selected_element_id.clone() else {
            return;
        };
        self.push_undo_snapshot();
        if let Some(page) = self.project.find_page_mut(&self.selected_page_id) {
            for sec in &mut page.sections {
                if let Some(el) = sec.elements.iter().find(|e| e.id == el_id).cloned() {
                    let next_z = sec.next_z_index();
                    let copy = el.duplicate(next_z);
                    let copy_id = copy.id.clone();
                    sec.elements.push(copy);
                    self.selected_element_id = Some(copy_id);
                    self.notify("Duplicated element", false);
                    return;
                }
            }
        }
    }

    /// Ensures all project image assets are uploaded to `egui` GPU textures.
    pub fn ensure_asset_textures(&mut self, ctx: &egui::Context) {
        for asset in &self.project.assets {
            if !self.textures.contains_key(&asset.id) {
                if let Ok(bytes) = asset.decoded_bytes() {
                    if let Ok(img) = image::load_from_memory(&bytes) {
                        let rgba = img.to_rgba8();
                        let size = [rgba.width() as usize, rgba.height() as usize];
                        let color_img =
                            egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
                        let handle = ctx.load_texture(
                            format!("asset_{}", asset.id),
                            color_img,
                            egui::TextureOptions::LINEAR,
                        );
                        self.textures.insert(asset.id.clone(), handle);
                    }
                }
            }
        }
    }
}
