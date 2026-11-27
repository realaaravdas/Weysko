use std::fmt::Write;

use crate::model::{
    Element, ElementBgType, ElementKind, EntranceAnimation, HoverEffect,
    MobileLayoutMode, Section, SectionBackground, SiteProject,
};

/// Generates the full CSS3 stylesheet for the project.
pub fn generate_css(project: &SiteProject, single_file_mode: bool) -> String {
    let mut out = String::with_capacity(8192);
    let theme = &project.theme;
    let p = &theme.palette;
    let t = &theme.typography;

    // Optional Google Fonts import.
    if t.use_google_fonts {
        let mut families = Vec::new();
        for font in [&t.heading_font, &t.body_font, &t.mono_font] {
            if !font.starts_with("System") && !font.trim().is_empty() && !families.contains(font) {
                families.push(font.clone());
            }
        }
        if !families.is_empty() {
            let query = families
                .iter()
                .map(|f| format!("family={}:wght@400;500;600;700", f.replace(' ', "+")))
                .collect::<Vec<_>>()
                .join("&");
            let _ = writeln!(
                out,
                "@import url('https://fonts.googleapis.com/css2?{}&display=swap');\n",
                query
            );
        }
    }

    let heading_stack = font_stack(&t.heading_font, "sans-serif");
    let body_stack = font_stack(&t.body_font, "sans-serif");
    let mono_stack = font_stack(&t.mono_font, "monospace");

    let _ = writeln!(
        out,
        ":root {{
  --ws-bg: {};
  --ws-surface: {};
  --ws-surface-alt: {};
  --ws-primary: {};
  --ws-primary-text: {};
  --ws-accent: {};
  --ws-text: {};
  --ws-text-muted: {};
  --ws-border: {};
  --ws-font-heading: {};
  --ws-font-body: {};
  --ws-font-mono: {};
  --ws-base-size: {}px;
  --ws-line-height: {};
  --ws-max-width: {}px;
  --ws-radius: {}px;
}}",
        p.background,
        p.surface,
        p.surface_alt,
        p.primary,
        p.primary_text,
        p.accent,
        p.text,
        p.text_muted,
        p.border,
        heading_stack,
        body_stack,
        mono_stack,
        t.base_font_size,
        t.line_height,
        theme.content_max_width,
        theme.border_radius
    );

    if theme.smooth_scroll {
        out.push_str("html { scroll-behavior: smooth; }\n");
    }

    out.push_str(
        r#"*, *::before, *::after { box-sizing: border-box; }
body {
  margin: 0;
  padding: 0;
  background-color: var(--ws-bg);
  color: var(--ws-text);
  font-family: var(--ws-font-body);
  font-size: var(--ws-base-size);
  line-height: var(--ws-line-height);
  -webkit-font-smoothing: antialiased;
  overflow-x: hidden;
}
img, video, iframe { max-width: 100%; }
a { color: inherit; text-decoration: none; }
button, input, textarea { font-family: inherit; }

.ws-section {
  position: relative;
  width: 100%;
  overflow: visible;
}
.ws-section.ws-sticky {
  position: sticky;
  top: 0;
  z-index: 900;
  backdrop-filter: blur(12px);
}
.ws-section-overlay {
  position: absolute;
  inset: 0;
  pointer-events: none;
}
.ws-section-inner {
  position: relative;
  width: 100%;
  max-width: var(--ws-max-width);
  margin: 0 auto;
}
.ws-section.ws-full-bleed .ws-section-inner {
  max-width: 100%;
}

.ws-el {
  position: absolute;
  transition: transform 0.22s ease, opacity 0.25s ease, filter 0.22s ease, border-color 0.22s ease, background-color 0.22s ease, color 0.22s ease;
}

.ws-heading {
  margin: 0;
  font-family: var(--ws-font-heading);
  font-size: inherit;
  font-weight: inherit;
  line-height: inherit;
  letter-spacing: inherit;
  overflow-wrap: break-word;
}
.ws-paragraph {
  margin: 0;
  white-space: pre-wrap;
  overflow-wrap: break-word;
}
.ws-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
  cursor: pointer;
  border: none;
  text-decoration: none;
  white-space: nowrap;
}
.ws-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
  white-space: nowrap;
}
.ws-image-wrap {
  width: 100%;
  height: 100%;
  overflow: hidden;
}
.ws-image-wrap img {
  width: 100%;
  height: 100%;
  display: block;
}
.ws-image-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--ws-text-muted);
  font-size: 13px;
  letter-spacing: 0.04em;
  background: linear-gradient(135deg, rgba(255,255,255,0.03), rgba(255,255,255,0.01));
}
.ws-divider {
  width: 100%;
  height: 100%;
}
.ws-container {
  width: 100%;
  height: 100%;
}

/* Navbar */
.ws-nav {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 24px;
  position: relative;
}
.ws-nav-brand {
  font-family: var(--ws-font-heading);
  font-weight: 700;
  font-size: 18px;
  letter-spacing: 0.06em;
  color: var(--ws-text);
  white-space: nowrap;
}
.ws-nav-right {
  display: flex;
  align-items: center;
  gap: 28px;
}
.ws-nav-links {
  display: flex;
  align-items: center;
  gap: 24px;
}
.ws-nav-link {
  font-size: 14px;
  font-weight: 500;
  color: var(--ws-text-muted);
  transition: color 0.18s ease;
}
.ws-nav-link:hover {
  color: var(--ws-text);
}
.ws-nav-cta {
  padding: 9px 18px;
  border-radius: 7px;
  background: var(--ws-primary);
  color: var(--ws-primary-text);
  font-size: 13px;
  font-weight: 600;
  white-space: nowrap;
}
.ws-nav-toggle {
  display: none;
  background: transparent;
  border: 1px solid var(--ws-border);
  color: var(--ws-text);
  border-radius: 6px;
  padding: 6px 10px;
  cursor: pointer;
  font-size: 16px;
}

/* Footer */
.ws-footer {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 28px;
}
.ws-footer-top {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 40px;
  flex-wrap: wrap;
}
.ws-footer-cols {
  display: flex;
  gap: 56px;
  flex-wrap: wrap;
}
.ws-footer-col h4 {
  margin: 0 0 12px;
  font-size: 12px;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--ws-text-muted);
}
.ws-footer-col a {
  display: block;
  margin-bottom: 8px;
  font-size: 14px;
  color: var(--ws-text);
  opacity: 0.85;
}
.ws-footer-col a:hover {
  opacity: 1;
}
.ws-footer-bottom {
  padding-top: 20px;
  border-top: 1px solid var(--ws-border);
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
  flex-wrap: wrap;
  font-size: 13px;
  color: var(--ws-text-muted);
}

/* Cards */
.ws-card {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  gap: 12px;
}
.ws-card-eyebrow {
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  color: var(--ws-text-muted);
}
.ws-card-title {
  margin: 0;
  font-family: var(--ws-font-heading);
  font-size: 21px;
  font-weight: 700;
  color: var(--ws-text);
}
.ws-card-body {
  margin: 0;
  font-size: 15px;
  line-height: 1.6;
  color: var(--ws-text-muted);
}
.ws-card-foot {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  padding-top: 10px;
  border-top: 1px solid var(--ws-border);
  font-size: 13px;
  color: var(--ws-text-muted);
}
.ws-card-cta {
  color: var(--ws-text);
  font-weight: 600;
}

/* Lists */
.ws-list {
  margin: 0;
  padding-left: 20px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.ws-list.ws-list-check {
  list-style: none;
  padding-left: 0;
}
.ws-list.ws-list-check li::before {
  content: "✓  ";
  color: var(--ws-primary);
  font-weight: 700;
}

/* Contact Form */
.ws-form {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 14px;
}
.ws-form-title {
  margin: 0 0 4px;
  font-family: var(--ws-font-heading);
  font-size: 22px;
  font-weight: 700;
}
.ws-form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.ws-form-group label {
  font-size: 13px;
  font-weight: 500;
  color: var(--ws-text-muted);
}
.ws-form-group input,
.ws-form-group textarea {
  width: 100%;
  padding: 10px 12px;
  border-radius: 7px;
  border: 1px solid var(--ws-border);
  background: var(--ws-bg);
  color: var(--ws-text);
  font-size: 14px;
  outline: none;
}
.ws-form-group input:focus,
.ws-form-group textarea:focus {
  border-color: var(--ws-primary);
}
.ws-form-submit {
  margin-top: 4px;
  padding: 12px 20px;
  border-radius: 8px;
  border: none;
  background: var(--ws-primary);
  color: var(--ws-primary-text);
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
}

/* Accordion */
.ws-acc {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.ws-acc details {
  border: 1px solid var(--ws-border);
  border-radius: 8px;
  padding: 14px 16px;
  background: var(--ws-bg);
}
.ws-acc summary {
  cursor: pointer;
  font-weight: 600;
  font-size: 15px;
  list-style: none;
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.ws-acc summary::after {
  content: "+";
  font-size: 18px;
  color: var(--ws-text-muted);
}
.ws-acc details[open] summary::after {
  content: "−";
}
.ws-acc details p {
  margin: 10px 0 0;
  font-size: 14px;
  color: var(--ws-text-muted);
  line-height: 1.6;
}

/* Video Embed */
.ws-video {
  width: 100%;
  height: 100%;
  overflow: hidden;
  border-radius: inherit;
}
.ws-video iframe, .ws-video video {
  width: 100%;
  height: 100%;
  border: 0;
  display: block;
}

/* Social Links */
.ws-social {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
}
.ws-social-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 38px;
  height: 38px;
  border-radius: 8px;
  border: 1px solid var(--ws-border);
  background: var(--ws-surface);
  color: var(--ws-text);
  transition: transform 0.16s ease, border-color 0.16s ease;
}
.ws-social-btn:hover {
  transform: translateY(-2px);
  border-color: var(--ws-primary);
}

/* Hover Effects */
.ws-hover-lift:hover { transform: translateY(-4px); }
.ws-hover-scale:hover { transform: scale(1.025); }
.ws-hover-brighten:hover { filter: brightness(1.12); }
.ws-hover-invert:hover {
  background: var(--ws-text) !important;
  color: var(--ws-bg) !important;
}
.ws-hover-border:hover { border-color: var(--ws-primary) !important; }

/* Animations */
@keyframes wsFadeIn { from { opacity: 0; } to { opacity: 1; } }
@keyframes wsSlideUp { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }
@keyframes wsSlideLeft { from { opacity: 0; transform: translateX(24px); } to { opacity: 1; transform: translateX(0); } }
@keyframes wsZoomIn { from { opacity: 0; transform: scale(0.95); } to { opacity: 1; transform: scale(1); } }

.ws-anim { opacity: 0; }
.ws-anim.ws-anim-visible.ws-anim-fade { animation: wsFadeIn 0.55s ease forwards; }
.ws-anim.ws-anim-visible.ws-anim-slide-up { animation: wsSlideUp 0.6s cubic-bezier(0.16, 1, 0.3, 1) forwards; }
.ws-anim.ws-anim-visible.ws-anim-slide-left { animation: wsSlideLeft 0.6s cubic-bezier(0.16, 1, 0.3, 1) forwards; }
.ws-anim.ws-anim-visible.ws-anim-zoom { animation: wsZoomIn 0.55s ease forwards; }

/* Toast */
.ws-toast {
  position: fixed;
  right: 24px;
  bottom: 24px;
  z-index: 9999;
  padding: 12px 18px;
  border-radius: 8px;
  background: var(--ws-primary);
  color: var(--ws-primary-text);
  font-size: 14px;
  font-weight: 600;
  box-shadow: 0 12px 32px rgba(0,0,0,0.4);
  opacity: 0;
  pointer-events: none;
  transform: translateY(10px);
  transition: opacity 0.2s ease, transform 0.2s ease;
}
.ws-toast.is-shown {
  opacity: 1;
  transform: translateY(0);
}
"#,
    );

    if single_file_mode {
        out.push_str(
            ".ws-page-view { display: none; }\n.ws-page-view.is-active { display: block; }\n",
        );
    }

    // Emit page-specific overrides, section rules, and per-element layout/styles.
    for page in &project.pages {
        if let Some(bg) = &page.background_override {
            let _ = writeln!(out, "body.ws-page-{} {{ background-color: {}; }}", page.id, bg);
        }
        for section in &page.sections {
            emit_section_css(&mut out, project, section);
        }
    }

    // Mobile responsive navbar drawer rule.
    out.push_str(
        r#"@media (max-width: 768px) {
  .ws-nav-toggle { display: inline-flex; }
  .ws-nav-right {
    display: none;
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    flex-direction: column;
    align-items: flex-start;
    gap: 14px;
    padding: 18px 20px;
    background: var(--ws-surface);
    border-top: 1px solid var(--ws-border);
    border-bottom: 1px solid var(--ws-border);
    z-index: 950;
  }
  .ws-nav.is-open .ws-nav-right {
    display: flex;
  }
  .ws-nav-links {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  h1.ws-heading {
    font-size: clamp(24px, 8vw, 36px);
  }
  h2.ws-heading {
    font-size: clamp(20px, 6.5vw, 28px);
  }
}
"#,
    );

    if !theme.custom_global_css.trim().is_empty() {
        let _ = writeln!(out, "\n/* Custom Global CSS */\n{}", theme.custom_global_css);
    }

    out
}

fn emit_section_css(out: &mut String, project: &SiteProject, section: &Section) {
    let sec_sel = format!(".ws-sec-{}", css_id(&section.id));
    let mut bg_css = String::new();
    match &section.background {
        SectionBackground::Solid { color } => {
            let _ = write!(bg_css, "background-color: {};", color);
        }
        SectionBackground::LinearGradient { start, end, angle } => {
            let _ = write!(
                bg_css,
                "background: linear-gradient({}deg, {}, {});",
                angle, start, end
            );
        }
        SectionBackground::Image { asset_id, .. } => {
            if let Some(aid) = asset_id {
                if let Some(asset) = project.find_asset(aid) {
                    let _ = write!(
                        bg_css,
                        "background-image: url('assets/{}'); background-size: cover; background-position: center;",
                        asset.sanitized_filename()
                    );
                }
            }
        }
    }

    let border_top = if section.border_top {
        "border-top: 1px solid var(--ws-border);"
    } else {
        ""
    };
    let border_bottom = if section.border_bottom {
        "border-bottom: 1px solid var(--ws-border);"
    } else {
        ""
    };

    let _ = writeln!(
        out,
        "{} {{ {} {} {} }}\n{} .ws-section-inner {{ min-height: {}px; }}",
        sec_sel, bg_css, border_top, border_bottom, sec_sel, section.height
    );

    if let SectionBackground::Image {
        overlay_color,
        overlay_opacity,
        ..
    } = &section.background
    {
        let _ = writeln!(
            out,
            "{} .ws-section-overlay {{ background: {}; opacity: {}; }}",
            sec_sel, overlay_color, overlay_opacity
        );
    }

    // Sort elements by (y, x) to determine deterministic AutoStack mobile order.
    let mut ordered: Vec<&Element> = section.elements.iter().collect();
    ordered.sort_by(|a, b| {
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

    for el in &section.elements {
        emit_element_css(out, el);
    }

    // Mobile AutoStack mode
    if section.mobile_layout == MobileLayoutMode::AutoStack {
        let _ = writeln!(
            out,
            "@media (max-width: 768px) {{
  {} .ws-section-inner {{
    display: flex;
    flex-direction: column;
    gap: 16px;
    min-height: 0 !important;
    padding: {}px 18px;
  }}",
            sec_sel,
            if section.elements.iter().any(|e| matches!(e.kind, ElementKind::Navbar)) {
                0
            } else {
                32
            }
        );

        for (order_idx, el) in ordered.iter().enumerate() {
            let el_sel = format!(".ws-el-{}", css_id(&el.id));
            if let Some(mr) = el.mobile_rect {
                let left_pct = (mr.x / 390.0) * 100.0;
                let width_pct = ((mr.w / 390.0) * 100.0).min(100.0);
                let _ = writeln!(
                    out,
                    "  {} {{ order: {}; position: relative; left: {:.2}%; width: {:.2}%; min-height: {}px; top: auto; }}",
                    el_sel, order_idx, left_pct, width_pct, mr.h
                );
            } else {
                let extra = match el.kind {
                    ElementKind::Image | ElementKind::VideoEmbed => "min-height: 210px; height: auto;",
                    ElementKind::Divider => "height: 2px; min-height: 2px;",
                    ElementKind::Navbar => "min-height: 64px;",
                    ElementKind::Button => "min-height: 46px;",
                    _ => "height: auto; min-height: 0;",
                };
                let _ = writeln!(
                    out,
                    "  {} {{ order: {}; position: relative !important; left: auto !important; top: auto !important; width: 100% !important; {} }}",
                    el_sel, order_idx, extra
                );
            }
        }
        out.push_str("}\n");
    }
}

fn emit_element_css(out: &mut String, el: &Element) {
    let sel = format!(".ws-el-{}", css_id(&el.id));
    let s = &el.style;

    let (left_css, width_css) = if el.full_width {
        ("0%".to_string(), "100%".to_string())
    } else {
        (
            format!("{:.3}%", (el.rect.x / 1200.0) * 100.0),
            format!("{:.3}%", (el.rect.w / 1200.0) * 100.0),
        )
    };

    let height_rule = match el.kind {
        ElementKind::Divider
        | ElementKind::Image
        | ElementKind::VideoEmbed
        | ElementKind::Button
        | ElementKind::Badge
        | ElementKind::Navbar => format!("height: {}px;", el.rect.h),
        _ => format!("min-height: {}px;", el.rect.h),
    };

    let bg_rule = match s.bg_type {
        ElementBgType::None => "background: transparent;".to_string(),
        ElementBgType::Solid => format!("background-color: {};", s.bg_color),
        ElementBgType::LinearGradient => format!(
            "background: linear-gradient({}deg, {}, {});",
            s.bg_gradient_angle, s.bg_color, s.bg_gradient_end
        ),
    };

    let border_rule = if s.border_width > 0.0 {
        format!(
            "border: {}px {} {};",
            s.border_width,
            s.border_style.as_css(),
            s.border_color
        )
    } else {
        String::new()
    };

    let radius_rule = format!(
        "border-radius: {}px {}px {}px {}px;",
        s.border_radius[0], s.border_radius[1], s.border_radius[2], s.border_radius[3]
    );
    let pad_rule = format!(
        "padding: {}px {}px {}px {}px;",
        s.padding[0], s.padding[1], s.padding[2], s.padding[3]
    );

    let font_family_rule = if s.font_family.trim().is_empty() {
        String::new()
    } else {
        format!("font-family: {};", font_stack(&s.font_family, "sans-serif"))
    };

    let shadow_rule = if s.shadow != crate::model::BoxShadowPreset::None {
        format!("box-shadow: {};", s.shadow.as_css())
    } else {
        String::new()
    };

    let _ = writeln!(
        out,
        "{} {{
  left: {};
  top: {}px;
  width: {};
  {}
  z-index: {};
  {}
  color: {};
  {}
  font-size: {}px;
  font-weight: {};
  line-height: {};
  letter-spacing: {}em;
  text-align: {};
  {}
  {}
  {}
  opacity: {};
  {}
  {}
}}",
        sel,
        left_css,
        el.rect.y,
        width_css,
        height_rule,
        el.z_index,
        bg_rule,
        s.text_color,
        font_family_rule,
        s.font_size,
        s.font_weight,
        s.line_height,
        s.letter_spacing,
        s.text_align.as_css(),
        pad_rule,
        border_rule,
        radius_rule,
        s.opacity,
        shadow_rule,
        el.custom_css
    );

    if let HoverEffect::Custom {
        bg_color,
        text_color,
    } = &el.hover
    {
        let _ = writeln!(
            out,
            "{}:hover {{ background-color: {} !important; color: {} !important; }}",
            sel, bg_color, text_color
        );
    }

    if let Some(tr) = el.tablet_rect {
        let _ = writeln!(
            out,
            "@media (max-width: 1024px) and (min-width: 769px) {{ {} {{ left: {:.3}%; top: {}px; width: {:.3}%; min-height: {}px; }} }}",
            sel,
            (tr.x / 768.0) * 100.0,
            tr.y,
            (tr.w / 768.0) * 100.0,
            tr.h
        );
    }

    if !el.visible_desktop {
        let _ = writeln!(
            out,
            "@media (min-width: 1025px) {{ {} {{ display: none !important; }} }}",
            sel
        );
    }
    if !el.visible_tablet {
        let _ = writeln!(
            out,
            "@media (max-width: 1024px) and (min-width: 769px) {{ {} {{ display: none !important; }} }}",
            sel
        );
    }
    if !el.visible_mobile || el.hidden {
        if el.hidden {
            let _ = writeln!(out, "{} {{ display: none !important; }}", sel);
        } else {
            let _ = writeln!(
                out,
                "@media (max-width: 768px) {{ {} {{ display: none !important; }} }}",
                sel
            );
        }
    }
}

pub fn hover_class(effect: &HoverEffect) -> &'static str {
    match effect {
        HoverEffect::None | HoverEffect::Custom { .. } => "",
        HoverEffect::Lift => "ws-hover-lift",
        HoverEffect::Scale => "ws-hover-scale",
        HoverEffect::Brighten => "ws-hover-brighten",
        HoverEffect::Invert => "ws-hover-invert",
        HoverEffect::BorderHighlight => "ws-hover-border",
    }
}

pub fn animation_class(anim: EntranceAnimation) -> &'static str {
    match anim {
        EntranceAnimation::None => "",
        EntranceAnimation::FadeIn => "ws-anim ws-anim-fade",
        EntranceAnimation::SlideUp => "ws-anim ws-anim-slide-up",
        EntranceAnimation::SlideLeft => "ws-anim ws-anim-slide-left",
        EntranceAnimation::ZoomIn => "ws-anim ws-anim-zoom",
    }
}

pub fn css_id(raw: &str) -> String {
    raw.chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '-' })
        .collect()
}

fn font_stack(name: &str, fallback: &str) -> String {
    match name {
        "System Sans" => "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif".into(),
        "System Serif" => "Georgia, 'Times New Roman', serif".into(),
        other if !other.trim().is_empty() => {
            format!("'{}', -apple-system, BlinkMacSystemFont, {}", other, fallback)
        }
        _ => fallback.into(),
    }
}
