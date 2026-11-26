use egui::{Frame, RichText};

use super::state::StudioState;
use super::style::{BG_VOID, TEXT_MUTED};

/// Placeholder canvas for Step 3; replaced with full interactive WYSIWYG canvas in Step 4.
pub fn draw_canvas(ctx: &egui::Context, _state: &mut StudioState) {
    egui::CentralPanel::default()
        .frame(Frame::new().fill(BG_VOID))
        .show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("Websir Studio Canvas").color(TEXT_MUTED));
            });
        });
}
