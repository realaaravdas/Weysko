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

/// Native `eframe` application hosting the Websir Studio workspace.
pub struct WebsirApp {
    pub state: StudioState,
    theme_initialized: bool,
}

impl WebsirApp {
    pub fn new(initial_project: crate::model::SiteProject) -> Self {
        Self {
            state: StudioState::new(initial_project),
            theme_initialized: false,
        }
    }

    pub fn new_default() -> Self {
        Self::new(StarterTemplate::MinimalistPortfolio.build())
    }
}

impl eframe::App for WebsirApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.theme_initialized {
            apply_slate_theme(ctx);
            self.theme_initialized = true;
        }

        self.state.ensure_asset_textures(ctx);
        top_bar::draw_top_bar(ctx, &mut self.state);
        left_panel::draw_left_panel(ctx, &mut self.state);
        inspector::draw_inspector(ctx, &mut self.state);
        canvas::draw_canvas(ctx, &mut self.state);
        modals::draw_modals_and_toasts(ctx, &mut self.state);

        self.state.sync_preview_if_running();
    }
}
