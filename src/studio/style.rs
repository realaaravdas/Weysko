use egui::{
    Color32, CornerRadius, FontId, Frame, Margin, RichText, Stroke, Ui, Visuals,
};

// Dark & White Minimalist Slate Palette
pub const BG_VOID: Color32 = Color32::from_rgb(9, 13, 22);        // #090D16
pub const BG_PANEL: Color32 = Color32::from_rgb(15, 23, 42);      // #0F172A
pub const BG_SURFACE: Color32 = Color32::from_rgb(30, 41, 59);    // #1E293B
pub const BG_HOVER: Color32 = Color32::from_rgb(44, 58, 82);      // #2C3A52
pub const BG_INPUT: Color32 = Color32::from_rgb(12, 19, 34);      // #0C1322
pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(30, 41, 59); // #1E293B
pub const BORDER_STRONG: Color32 = Color32::from_rgb(51, 65, 85); // #334155
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(248, 250, 252); // #F8FAFC
pub const TEXT_MUTED: Color32 = Color32::from_rgb(148, 163, 184);   // #94A3B8
pub const TEXT_DIM: Color32 = Color32::from_rgb(100, 116, 139);     // #64748B
pub const WHITE: Color32 = Color32::WHITE;                          // #FFFFFF
pub const SLATE_INK: Color32 = Color32::from_rgb(15, 23, 42);       // #0F172A
pub const ICE_ACCENT: Color32 = Color32::from_rgb(203, 213, 225);   // #CBD5E1
pub const DANGER_INK: Color32 = Color32::from_rgb(248, 113, 113);   // #F87171
pub const SUCCESS_INK: Color32 = Color32::from_rgb(134, 239, 172);  // #86EFAC

/// Configures the global dark minimalist slate theme across `egui`.
pub fn apply_slate_theme(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();
    visuals.dark_mode = true;
    visuals.panel_fill = BG_PANEL;
    visuals.window_fill = BG_PANEL;
    visuals.faint_bg_color = BG_SURFACE;
    visuals.extreme_bg_color = BG_INPUT;
    visuals.code_bg_color = BG_INPUT;

    visuals.window_corner_radius = CornerRadius::same(10);
    visuals.menu_corner_radius = CornerRadius::same(8);
    visuals.window_stroke = Stroke::new(1.0_f32, BORDER_STRONG);

    visuals.selection.bg_fill = Color32::from_rgba_unmultiplied(255, 255, 255, 34);
    visuals.selection.stroke = Stroke::new(1.0_f32, WHITE);

    visuals.widgets.noninteractive.bg_fill = BG_PANEL;
    visuals.widgets.noninteractive.weak_bg_fill = BG_SURFACE;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, BORDER_SUBTLE);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, TEXT_PRIMARY);
    visuals.widgets.noninteractive.corner_radius = CornerRadius::same(6);

    visuals.widgets.inactive.bg_fill = BG_SURFACE;
    visuals.widgets.inactive.weak_bg_fill = BG_SURFACE;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, BORDER_STRONG);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, TEXT_PRIMARY);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(6);

    visuals.widgets.hovered.bg_fill = BG_HOVER;
    visuals.widgets.hovered.weak_bg_fill = BG_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, ICE_ACCENT);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, WHITE);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(6);

    visuals.widgets.active.bg_fill = BG_HOVER;
    visuals.widgets.active.weak_bg_fill = BG_HOVER;
    visuals.widgets.active.bg_stroke = Stroke::new(1.2_f32, WHITE);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, WHITE);
    visuals.widgets.active.corner_radius = CornerRadius::same(6);

    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 7.0);
    style.spacing.button_padding = egui::vec2(10.0, 6.0);
    style.spacing.indent = 14.0;
    style.spacing.scroll.bar_width = 6.0;
    ctx.set_style(style);
}

/// Parses `#RRGGBB` or `#RRGGBBAA` into an `egui::Color32`.
pub fn hex_to_color32(hex: &str) -> Color32 {
    let s = hex.trim().trim_start_matches('#');
    if s.len() == 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&s[0..2], 16),
            u8::from_str_radix(&s[2..4], 16),
            u8::from_str_radix(&s[4..6], 16),
        ) {
            return Color32::from_rgb(r, g, b);
        }
    } else if s.len() == 8 {
        if let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
            u8::from_str_radix(&s[0..2], 16),
            u8::from_str_radix(&s[2..4], 16),
            u8::from_str_radix(&s[4..6], 16),
            u8::from_str_radix(&s[6..8], 16),
        ) {
            return Color32::from_rgba_unmultiplied(r, g, b, a);
        }
    }
    BG_SURFACE
}

/// Formats an `egui::Color32` back into `#RRGGBB` (or `#RRGGBBAA` if alpha < 255).
pub fn color32_to_hex(c: Color32) -> String {
    if c.a() == 255 {
        format!("#{:02X}{:02X}{:02X}", c.r(), c.g(), c.b())
    } else {
        format!("#{:02X}{:02X}{:02X}{:02X}", c.r(), c.g(), c.b(), c.a())
    }
}

/// Crisp pure-white primary action button with dark slate ink.
pub fn primary_white_button(ui: &mut Ui, label: &str) -> egui::Response {
    let btn = egui::Button::new(
        RichText::new(label)
            .color(SLATE_INK)
            .strong()
            .size(12.5),
    )
    .fill(WHITE)
    .stroke(Stroke::new(1.0_f32, WHITE))
    .corner_radius(CornerRadius::same(6))
    .min_size(egui::vec2(0.0, 30.0));
    ui.add(btn)
}

/// Secondary slate outline button.
pub fn secondary_slate_button(ui: &mut Ui, label: &str) -> egui::Response {
    let btn = egui::Button::new(RichText::new(label).color(TEXT_PRIMARY).size(12.5))
        .fill(BG_SURFACE)
        .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
        .corner_radius(CornerRadius::same(6))
        .min_size(egui::vec2(0.0, 30.0));
    ui.add(btn)
}

/// Minimalist ghost button used for icon / subtle toolbar actions.
pub fn ghost_button(ui: &mut Ui, label: &str, active: bool) -> egui::Response {
    let (fill, stroke, text_col) = if active {
        (WHITE, Stroke::new(1.0_f32, WHITE), SLATE_INK)
    } else {
        (Color32::TRANSPARENT, Stroke::new(1.0_f32, BORDER_STRONG), TEXT_MUTED)
    };
    let btn = egui::Button::new(RichText::new(label).color(text_col).size(12.0))
        .fill(fill)
        .stroke(stroke)
        .corner_radius(CornerRadius::same(6))
        .min_size(egui::vec2(0.0, 28.0));
    ui.add(btn)
}

/// Minimalist uppercase section header for drawers and inspector groups.
pub fn section_heading(ui: &mut Ui, title: &str) {
    ui.add_space(4.0);
    ui.label(
        RichText::new(title.to_ascii_uppercase())
            .font(FontId::proportional(10.5))
            .color(TEXT_DIM)
            .strong(),
    );
    ui.add_space(2.0);
}

/// Slate elevated card container frame.
pub fn slate_card_frame() -> Frame {
    Frame::new()
        .fill(BG_SURFACE)
        .stroke(Stroke::new(1.0_f32, BORDER_STRONG))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(Margin::same(10))
}

/// Interactive color swatch + hex input field returning `true` if modified.
pub fn color_picker_hex(ui: &mut Ui, label: &str, hex_val: &mut String) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(TEXT_MUTED).size(12.0));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let mut c = hex_to_color32(hex_val);
            if ui.color_edit_button_srgba(&mut c).changed() {
                *hex_val = color32_to_hex(c);
                changed = true;
            }
            let resp = ui.add(
                egui::TextEdit::singleline(hex_val)
                    .desired_width(78.0)
                    .font(FontId::monospace(11.5)),
            );
            if resp.changed() {
                changed = true;
            }
        });
    });
    changed
}

/// Segmented pill tab selector.
pub fn segmented_tabs<T: Copy + PartialEq>(
    ui: &mut Ui,
    current: &mut T,
    items: &[(T, &str)],
) -> bool {
    let mut changed = false;
    Frame::new()
        .fill(BG_INPUT)
        .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
        .corner_radius(CornerRadius::same(7))
        .inner_margin(Margin::same(3))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 3.0;
                for (val, label) in items {
                    let active = *current == *val;
                    let btn = egui::Button::new(
                        RichText::new(*label)
                            .size(11.5)
                            .color(if active { SLATE_INK } else { TEXT_MUTED })
                            .strong(),
                    )
                    .fill(if active { WHITE } else { Color32::TRANSPARENT })
                    .stroke(Stroke::NONE)
                    .corner_radius(CornerRadius::same(5))
                    .min_size(egui::vec2(0.0, 24.0));
                    if ui.add(btn).clicked() && !active {
                        *current = *val;
                        changed = true;
                    }
                }
            });
        });
    changed
}
