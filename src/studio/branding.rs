use egui::{Color32, CornerRadius, IconData, Painter, Pos2, Rect, Vec2};

/// Generates the crisp 64x64 RGBA application window icon matching `assets/logo-mark.svg`.
pub fn app_icon_data() -> IconData {
    let size = 64u32;
    let mut rgba = vec![0u8; (size * size * 4) as usize];

    let bg = [15u8, 23, 42, 255];      // #0F172A
    let border = [51u8, 65, 85, 255];  // #334155
    let white = [255u8, 255, 255, 255];
    let cyan = [0u8, 188, 235, 255];   // #00BCEB
    let muted = [148u8, 163, 184, 255];

    // Rounded background square
    for y in 0..size {
        for x in 0..size {
            let fx = x as f32 + 0.5;
            let fy = y as f32 + 0.5;
            let dx = (fx - 32.0).abs() - 18.0;
            let dy = (fy - 32.0).abs() - 18.0;
            let dist = dx.max(0.0).hypot(dy.max(0.0)) + dx.max(dy).min(0.0) - 13.0;
            let idx = ((y * size + x) * 4) as usize;
            if dist <= 0.0 {
                let col = if dist > -1.8 { border } else { bg };
                rgba[idx..idx + 4].copy_from_slice(&col);
            }
        }
    }

    // 7 precision architectural bars forming the Websir W emblem
    let bars: [(f32, f32, f32, f32, [u8; 4]); 7] = [
        (11.5, 17.0, 4.5, 24.0, white),
        (18.0, 24.0, 4.5, 23.0, cyan),
        (24.5, 20.0, 4.5, 24.0, white),
        (31.0, 27.0, 4.5, 15.0, muted),
        (37.5, 20.0, 4.5, 24.0, white),
        (44.0, 24.0, 4.5, 23.0, cyan),
        (50.5, 17.0, 4.5, 24.0, white),
    ];

    for (bx, by, bw, bh, col) in bars {
        let x0 = bx.floor() as u32;
        let x1 = (bx + bw).ceil().min(size as f32) as u32;
        let y0 = by.floor() as u32;
        let y1 = (by + bh).ceil().min(size as f32) as u32;
        for y in y0..y1 {
            for x in x0..x1 {
                let idx = ((y * size + x) * 4) as usize;
                rgba[idx..idx + 4].copy_from_slice(&col);
            }
        }
    }

    IconData {
        rgba,
        width: size,
        height: size,
    }
}

/// Paints the 7-bar geometric Websir mark crisply into any `egui` rect.
pub fn paint_logo_mark(painter: &Painter, rect: Rect, primary: Color32, accent: Color32) {
    let w = rect.width();
    let h = rect.height();
    let bars: [(f32, f32, f32, Color32); 7] = [
        (0.06, 0.18, 0.56, primary),
        (0.20, 0.34, 0.54, accent),
        (0.34, 0.22, 0.56, primary),
        (0.48, 0.38, 0.36, Color32::from_rgb(148, 163, 184)),
        (0.62, 0.22, 0.56, primary),
        (0.76, 0.34, 0.54, accent),
        (0.90, 0.18, 0.56, primary),
    ];
    let bar_w = (w * 0.095).max(1.8);
    let cr = CornerRadius::same((bar_w * 0.5).round() as u8);
    for (cx_frac, y_frac, h_frac, col) in bars {
        let pos = Pos2::new(rect.min.x + w * cx_frac - bar_w * 0.5, rect.min.y + h * y_frac);
        let size = Vec2::new(bar_w, h * h_frac);
        painter.rect_filled(Rect::from_min_size(pos, size), cr, col);
    }
}
