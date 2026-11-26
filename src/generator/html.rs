use std::fmt::Write;

use crate::model::{
    Element, ElementKind, FormActionMode, FormFieldType, ListStyle, Page, Section, SiteProject,
    SocialPlatform,
};

use super::css::{animation_class, css_id, generate_css, hover_class};

/// Renders a standalone multi-page HTML5 document referencing `styles.css` and `site.js`.
pub fn render_multi_page_document(project: &SiteProject, page: &Page) -> String {
    let mut out = String::with_capacity(8192);
    let title = if page.title.is_empty() {
        project.name.clone()
    } else {
        format!("{} — {}", page.title, project.name)
    };

    let favicon_tag = project
        .favicon_asset_id
        .as_deref()
        .and_then(|id| project.find_asset(id))
        .map(|a| {
            format!(
                "<link rel=\"icon\" href=\"assets/{}\" type=\"{}\">\n",
                escape_attr(&a.sanitized_filename()),
                escape_attr(&a.mime_type)
            )
        })
        .unwrap_or_default();

    let _ = writeln!(
        out,
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"utf-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n  <title>{}</title>\n  <meta name=\"description\" content=\"{}\">\n  {}\n  <link rel=\"stylesheet\" href=\"styles.css\">\n  {}\n</head>\n<body class=\"ws-page-{}\">",
        escape_html(&title),
        escape_attr(&page.meta_description),
        favicon_tag,
        page.custom_head_html,
        css_id(&page.id)
    );

    for section in &page.sections {
        render_section(&mut out, project, page, section, false);
    }

    out.push_str(
        "  <div id=\"ws-toast\" class=\"ws-toast\" role=\"status\" aria-live=\"polite\"></div>\n  <script src=\"site.js\"></script>\n</body>\n</html>\n",
    );
    out
}

/// Renders a single self-contained HTML5 document (for Single-File Export & Google Sites Embed)
/// with inline `<style>`, inline `<script>`, base64 asset data URIs, and client-side page switching.
pub fn render_single_file_document(project: &SiteProject, css_text: &str, js_text: &str) -> String {
    let mut out = String::with_capacity(16384);
    let first_page = project.pages.first();
    let title = project.name.clone();
    let desc = first_page
        .map(|p| p.meta_description.as_str())
        .unwrap_or("");

    let favicon_tag = project
        .favicon_asset_id
        .as_deref()
        .and_then(|id| project.find_asset(id))
        .map(|a| {
            format!(
                "<link rel=\"icon\" href=\"{}\">\n",
                escape_attr(&a.data_uri())
            )
        })
        .unwrap_or_default();

    let _ = writeln!(
        out,
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"utf-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n  <title>{}</title>\n  <meta name=\"description\" content=\"{}\">\n  {}\n  <style>\n{}\n  </style>\n</head>\n<body>",
        escape_html(&title),
        escape_attr(desc),
        favicon_tag,
        css_text
    );

    for (idx, page) in project.pages.iter().enumerate() {
        let active = if idx == 0 { " is-active" } else { "" };
        let _ = writeln!(
            out,
            "  <div class=\"ws-page-view{}\" data-ws-page-view=\"{}\">",
            active,
            escape_attr(&page.slug)
        );
        for section in &page.sections {
            render_section(&mut out, project, page, section, true);
        }
        out.push_str("  </div>\n");
    }

    let _ = writeln!(
        out,
        "  <div id=\"ws-toast\" class=\"ws-toast\" role=\"status\" aria-live=\"polite\"></div>\n  <script>\n{}\n  </script>\n</body>\n</html>",
        js_text
    );

    // Ensure section background images in single-file mode also use inline CSS data URIs if any.
    let _ = generate_css; // keep module linkage clean
    out
}

/// Renders a clean 404 page matching the project's slate design tokens.
pub fn render_404_page(project: &SiteProject) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>404 — {}</title>
  <link rel="stylesheet" href="styles.css">
</head>
<body style="display:flex;min-height:100vh;align-items:center;justify-content:center;padding:24px;text-align:center;">
  <div style="max-width:480px;">
    <p style="font-size:12px;letter-spacing:0.12em;color:var(--ws-text-muted);text-transform:uppercase;">404 Error</p>
    <h1 style="font-family:var(--ws-font-heading);font-size:40px;margin:10px 0 14px;">Page not found</h1>
    <p style="color:var(--ws-text-muted);margin:0 0 24px;">The requested page does not exist on this site.</p>
    <a href="index.html" style="display:inline-block;padding:12px 24px;border-radius:8px;background:var(--ws-primary);color:var(--ws-primary-text);font-weight:600;">Return Home</a>
  </div>
</body>
</html>
"#,
        escape_html(&project.name)
    )
}

fn render_section(
    out: &mut String,
    project: &SiteProject,
    _page: &Page,
    section: &Section,
    single_file_mode: bool,
) {
    let mut classes = vec![format!("ws-section ws-sec-{}", css_id(&section.id))];
    if section.full_bleed {
        classes.push("ws-full-bleed".to_string());
    }
    if section.sticky {
        classes.push("ws-sticky".to_string());
    }

    let id_attr = if section.anchor_id.trim().is_empty() {
        String::new()
    } else {
        format!(" id=\"{}\"", escape_attr(section.anchor_id.trim()))
    };

    // In single-file mode, if the section background uses an embedded asset image, inline its data URI.
    let inline_bg_style = if single_file_mode {
        if let crate::model::SectionBackground::Image {
            asset_id: Some(aid),
            ..
        } = &section.background
        {
            project
                .find_asset(aid)
                .map(|a| {
                    format!(
                        " style=\"background-image:url('{}');background-size:cover;background-position:center;\"",
                        escape_attr(&a.data_uri())
                    )
                })
                .unwrap_or_default()
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let _ = writeln!(
        out,
        "  <section{} class=\"{}\"{}>",
        id_attr,
        classes.join(" "),
        inline_bg_style
    );
    if matches!(
        section.background,
        crate::model::SectionBackground::Image { .. }
    ) {
        out.push_str("    <div class=\"ws-section-overlay\"></div>\n");
    }
    out.push_str("    <div class=\"ws-section-inner\">\n");

    let mut elements: Vec<&Element> = section.elements.iter().collect();
    elements.sort_by_key(|e| e.z_index);
    for el in elements {
        if !el.hidden {
            render_element(out, project, el, single_file_mode);
        }
    }

    out.push_str("    </div>\n  </section>\n");
}

fn render_element(out: &mut String, project: &SiteProject, el: &Element, single_file_mode: bool) {
    let mut classes = vec![format!("ws-el ws-el-{}", css_id(&el.id))];
    let hc = hover_class(&el.hover);
    if !hc.is_empty() {
        classes.push(hc.to_string());
    }
    let ac = animation_class(el.animation);
    if !ac.is_empty() {
        classes.push(ac.to_string());
    }
    if !el.custom_class.trim().is_empty() {
        classes.push(escape_attr(el.custom_class.trim()));
    }

    let id_attr = if el.html_id.trim().is_empty() {
        String::new()
    } else {
        format!(" id=\"{}\"", escape_attr(el.html_id.trim()))
    };

    let _ = writeln!(
        out,
        "      <div{} class=\"{}\">",
        id_attr,
        classes.join(" ")
    );

    match el.kind {
        ElementKind::Heading => {
            let lvl = el.content.heading_level.clamp(1, 6);
            let _ = writeln!(
                out,
                "        <h{} class=\"ws-heading\">{}</h{}>",
                lvl,
                escape_html(&el.content.text),
                lvl
            );
        }
        ElementKind::Paragraph => {
            let _ = writeln!(
                out,
                "        <p class=\"ws-paragraph\">{}</p>",
                escape_html(&el.content.text)
            );
        }
        ElementKind::Button => {
            let (href, page_attr) = resolve_link(
                &el.content.href,
                el.content.link_target_page_slug.as_deref(),
                single_file_mode,
            );
            let target = if el.content.open_in_new_tab {
                " target=\"_blank\" rel=\"noopener noreferrer\""
            } else {
                ""
            };
            let _ = writeln!(
                out,
                "        <a class=\"ws-btn\" href=\"{}\"{}{}>{}</a>",
                escape_attr(&href),
                page_attr,
                target,
                escape_html(&el.content.text)
            );
        }
        ElementKind::Badge => {
            let _ = writeln!(
                out,
                "        <span class=\"ws-badge\">{}</span>",
                escape_html(&el.content.text)
            );
        }
        ElementKind::Container => {
            out.push_str("        <div class=\"ws-container\"></div>\n");
        }
        ElementKind::Divider => {
            out.push_str("        <div class=\"ws-divider\" role=\"separator\"></div>\n");
        }
        ElementKind::Image => {
            let src = resolve_image_src(
                project,
                el.content.asset_id.as_deref(),
                &el.content.image_url,
                single_file_mode,
            );
            if let Some(src_val) = src {
                let _ = writeln!(
                    out,
                    "        <div class=\"ws-image-wrap\"><img src=\"{}\" alt=\"{}\" style=\"object-fit:{};\"></div>",
                    escape_attr(&src_val),
                    escape_attr(&el.content.alt_text),
                    el.content.object_fit.as_css()
                );
            } else {
                let _ = writeln!(
                    out,
                    "        <div class=\"ws-image-placeholder\">{}</div>",
                    escape_html(&el.content.alt_text)
                );
            }
        }
        ElementKind::Navbar => {
            render_navbar(out, project, el, single_file_mode);
        }
        ElementKind::Footer => {
            render_footer(out, el, single_file_mode);
        }
        ElementKind::ContactForm => {
            render_contact_form(out, el);
        }
        ElementKind::List => {
            let (tag, extra_class) = match el.content.list_style {
                ListStyle::Bullet => ("ul", ""),
                ListStyle::Numbered => ("ol", ""),
                ListStyle::Check => ("ul", " ws-list-check"),
            };
            let _ = writeln!(out, "        <{} class=\"ws-list{}\">", tag, extra_class);
            for item in &el.content.list_items {
                let _ = writeln!(out, "          <li>{}</li>", escape_html(item));
            }
            let _ = writeln!(out, "        </{}>", tag);
        }
        ElementKind::Card => {
            out.push_str("        <article class=\"ws-card\">\n          <div>\n");
            if !el.content.card_eyebrow.trim().is_empty() {
                let _ = writeln!(
                    out,
                    "            <div class=\"ws-card-eyebrow\">{}</div>",
                    escape_html(&el.content.card_eyebrow)
                );
            }
            let _ = writeln!(
                out,
                "            <h3 class=\"ws-card-title\" style=\"margin-top:8px;\">{}</h3>\n            <p class=\"ws-card-body\" style=\"margin-top:10px;\">{}</p>\n          </div>",
                escape_html(&el.content.card_title),
                escape_html(&el.content.card_body)
            );
            if !el.content.card_meta.trim().is_empty()
                || !el.content.card_cta_label.trim().is_empty()
            {
                out.push_str("          <div class=\"ws-card-foot\">\n");
                let _ = writeln!(
                    out,
                    "            <span>{}</span>",
                    escape_html(&el.content.card_meta)
                );
                if !el.content.card_cta_label.trim().is_empty() {
                    let (href, page_attr) =
                        resolve_link(&el.content.card_cta_href, None, single_file_mode);
                    let _ = writeln!(
                        out,
                        "            <a class=\"ws-card-cta\" href=\"{}\"{}>{}</a>",
                        escape_attr(&href),
                        page_attr,
                        escape_html(&el.content.card_cta_label)
                    );
                }
                out.push_str("          </div>\n");
            }
            out.push_str("        </article>\n");
        }
        ElementKind::Accordion => {
            out.push_str("        <div class=\"ws-acc\">\n");
            for item in &el.content.accordion_items {
                let _ = writeln!(
                    out,
                    "          <details>\n            <summary>{}</summary>\n            <p>{}</p>\n          </details>",
                    escape_html(&item.question),
                    escape_html(&item.answer)
                );
            }
            out.push_str("        </div>\n");
        }
        ElementKind::VideoEmbed => {
            render_video_embed(out, el);
        }
        ElementKind::CustomHtml => {
            let _ = writeln!(out, "        {}", el.content.raw_html);
        }
        ElementKind::SocialLinks => {
            out.push_str("        <div class=\"ws-social\">\n");
            for link in &el.content.social_links {
                let _ = writeln!(
                    out,
                    "          <a class=\"ws-social-btn\" href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\" aria-label=\"{}\">{}</a>",
                    escape_attr(&link.url),
                    escape_attr(link.platform.label()),
                    social_svg(link.platform)
                );
            }
            out.push_str("        </div>\n");
        }
    }

    out.push_str("      </div>\n");
}

fn render_navbar(
    out: &mut String,
    project: &SiteProject,
    el: &Element,
    single_file_mode: bool,
) {
    let (home_href, home_attr) = resolve_link("index.html", Some("index"), single_file_mode);
    out.push_str("        <nav class=\"ws-nav\">\n");
    let _ = writeln!(
        out,
        "          <a class=\"ws-nav-brand\" href=\"{}\"{}>{}</a>",
        escape_attr(&home_href),
        home_attr,
        escape_html(&el.content.nav_brand)
    );
    out.push_str(
        "          <button class=\"ws-nav-toggle\" type=\"button\" aria-label=\"Toggle menu\" data-ws-nav-toggle>☰</button>\n          <div class=\"ws-nav-right\">\n            <div class=\"ws-nav-links\">\n",
    );

    if el.content.nav_auto_pages && project.pages.len() > 1 {
        for page in &project.pages {
            let (href, attr) = resolve_link(&page.html_filename(), Some(&page.slug), single_file_mode);
            let _ = writeln!(
                out,
                "              <a class=\"ws-nav-link\" href=\"{}\"{}>{}</a>",
                escape_attr(&href),
                attr,
                escape_html(&page.title)
            );
        }
    } else {
        for link in &el.content.nav_links {
            let (href, attr) =
                resolve_link(&link.href, link.page_slug.as_deref(), single_file_mode);
            let _ = writeln!(
                out,
                "              <a class=\"ws-nav-link\" href=\"{}\"{}>{}</a>",
                escape_attr(&href),
                attr,
                escape_html(&link.label)
            );
        }
    }

    out.push_str("            </div>\n");
    if !el.content.nav_cta_label.trim().is_empty() {
        let (cta_href, cta_attr) =
            resolve_link(&el.content.nav_cta_href, None, single_file_mode);
        let _ = writeln!(
            out,
            "            <a class=\"ws-nav-cta\" href=\"{}\"{}>{}</a>",
            escape_attr(&cta_href),
            cta_attr,
            escape_html(&el.content.nav_cta_label)
        );
    }
    out.push_str("          </div>\n        </nav>\n");
}

fn render_footer(out: &mut String, el: &Element, single_file_mode: bool) {
    out.push_str("        <footer class=\"ws-footer\">\n          <div class=\"ws-footer-top\">\n");
    let _ = writeln!(
        out,
        "            <div>\n              <div class=\"ws-nav-brand\">{}</div>\n              <p style=\"color:var(--ws-text-muted);font-size:14px;margin:10px 0 0;max-width:280px;\">Minimalist web experiences compiled cleanly.</p>\n            </div>",
        escape_html(&el.content.nav_brand)
    );
    out.push_str("            <div class=\"ws-footer-cols\">\n");
    for col in &el.content.footer_columns {
        out.push_str("              <div class=\"ws-footer-col\">\n");
        let _ = writeln!(out, "                <h4>{}</h4>", escape_html(&col.heading));
        for link in &col.links {
            let (href, attr) =
                resolve_link(&link.href, link.page_slug.as_deref(), single_file_mode);
            let _ = writeln!(
                out,
                "                <a href=\"{}\"{}>{}</a>",
                escape_attr(&href),
                attr,
                escape_html(&link.label)
            );
        }
        out.push_str("              </div>\n");
    }
    out.push_str("            </div>\n          </div>\n          <div class=\"ws-footer-bottom\">\n");
    let _ = writeln!(
        out,
        "            <span>{}</span>",
        escape_html(&el.content.footer_copyright)
    );
    out.push_str("            <div class=\"ws-social\">\n");
    for link in &el.content.social_links {
        let _ = writeln!(
            out,
            "              <a class=\"ws-social-btn\" href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\" aria-label=\"{}\">{}</a>",
            escape_attr(&link.url),
            escape_attr(link.platform.label()),
            social_svg(link.platform)
        );
    }
    out.push_str("            </div>\n          </div>\n        </footer>\n");
}

fn render_contact_form(out: &mut String, el: &Element) {
    let mode_attr = match el.content.form_action_mode {
        FormActionMode::ToastDemo => " data-ws-form-mode=\"demo\"".to_string(),
        FormActionMode::Mailto => format!(
            " data-ws-form-mode=\"mailto\" data-ws-mailto=\"{}\"",
            escape_attr(&el.content.form_action_value)
        ),
        FormActionMode::PostUrl => format!(
            " action=\"{}\" method=\"POST\"",
            escape_attr(&el.content.form_action_value)
        ),
        FormActionMode::NetlifyForms => format!(
            " name=\"{}\" method=\"POST\" data-netlify=\"true\"",
            escape_attr(&css_id(&el.name))
        ),
    };

    let _ = writeln!(out, "        <form class=\"ws-form\"{}>", mode_attr);
    if el.content.form_action_mode == FormActionMode::NetlifyForms {
        let _ = writeln!(
            out,
            "          <input type=\"hidden\" name=\"form-name\" value=\"{}\">",
            escape_attr(&css_id(&el.name))
        );
    }
    if !el.content.form_title.trim().is_empty() {
        let _ = writeln!(
            out,
            "          <h3 class=\"ws-form-title\">{}</h3>",
            escape_html(&el.content.form_title)
        );
    }

    for (idx, field) in el.content.form_fields.iter().enumerate() {
        let input_id = format!("ws-f-{}-{}", css_id(&el.id), idx);
        let req = if field.required { " required" } else { "" };
        out.push_str("          <div class=\"ws-form-group\">\n");
        let _ = writeln!(
            out,
            "            <label for=\"{}\">{}</label>",
            input_id,
            escape_html(&field.label)
        );
        match field.field_type {
            FormFieldType::Text => {
                let _ = writeln!(
                    out,
                    "            <input id=\"{}\" name=\"{}\" type=\"text\" placeholder=\"{}\"{}>",
                    input_id,
                    escape_attr(&field.label),
                    escape_attr(&field.placeholder),
                    req
                );
            }
            FormFieldType::Email => {
                let _ = writeln!(
                    out,
                    "            <input id=\"{}\" name=\"{}\" type=\"email\" placeholder=\"{}\"{}>",
                    input_id,
                    escape_attr(&field.label),
                    escape_attr(&field.placeholder),
                    req
                );
            }
            FormFieldType::Textarea => {
                let _ = writeln!(
                    out,
                    "            <textarea id=\"{}\" name=\"{}\" rows=\"4\" placeholder=\"{}\"{}></textarea>",
                    input_id,
                    escape_attr(&field.label),
                    escape_attr(&field.placeholder),
                    req
                );
            }
        }
        out.push_str("          </div>\n");
    }

    let _ = writeln!(
        out,
        "          <button class=\"ws-form-submit\" type=\"submit\">{}</button>\n        </form>",
        escape_html(&el.content.form_submit_label)
    );
}

fn render_video_embed(out: &mut String, el: &Element) {
    let url = el.content.video_url.trim();
    out.push_str("        <div class=\"ws-video\">\n");
    if let Some(yt_id) = extract_youtube_id(url) {
        let mut params = vec!["rel=0".to_string()];
        if el.content.video_autoplay {
            params.push("autoplay=1".into());
        }
        if el.content.video_loop {
            params.push(format!("loop=1&playlist={}", yt_id));
        }
        if el.content.video_muted {
            params.push("mute=1".into());
        }
        if !el.content.video_controls {
            params.push("controls=0".into());
        }
        let _ = writeln!(
            out,
            "          <iframe src=\"https://www.youtube.com/embed/{}?{}\" title=\"Video player\" allow=\"accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture\" allowfullscreen></iframe>",
            escape_attr(&yt_id),
            params.join("&")
        );
    } else if let Some(vimeo_id) = extract_vimeo_id(url) {
        let _ = writeln!(
            out,
            "          <iframe src=\"https://player.vimeo.com/video/{}\" title=\"Video player\" allow=\"autoplay; fullscreen; picture-in-picture\" allowfullscreen></iframe>",
            escape_attr(&vimeo_id)
        );
    } else if !url.is_empty() {
        let autoplay = if el.content.video_autoplay { " autoplay" } else { "" };
        let loop_attr = if el.content.video_loop { " loop" } else { "" };
        let muted = if el.content.video_muted { " muted" } else { "" };
        let controls = if el.content.video_controls { " controls" } else { "" };
        let _ = writeln!(
            out,
            "          <video src=\"{}\"{}{}{}{}></video>",
            escape_attr(url),
            autoplay,
            loop_attr,
            muted,
            controls
        );
    } else {
        out.push_str("          <div class=\"ws-image-placeholder\">VIDEO EMBED</div>\n");
    }
    out.push_str("        </div>\n");
}

fn resolve_image_src(
    project: &SiteProject,
    asset_id: Option<&str>,
    fallback_url: &str,
    single_file_mode: bool,
) -> Option<String> {
    if let Some(aid) = asset_id {
        if let Some(asset) = project.find_asset(aid) {
            if single_file_mode {
                return Some(asset.data_uri());
            } else {
                return Some(format!("assets/{}", asset.sanitized_filename()));
            }
        }
    }
    if !fallback_url.trim().is_empty() {
        Some(fallback_url.trim().to_string())
    } else {
        None
    }
}

fn resolve_link(
    raw_href: &str,
    explicit_page_slug: Option<&str>,
    single_file_mode: bool,
) -> (String, String) {
    let detected_slug = explicit_page_slug.map(|s| s.to_string()).or_else(|| {
        let trimmed = raw_href.trim();
        if trimmed.ends_with(".html") && !trimmed.contains("://") && !trimmed.starts_with('/') {
            Some(trimmed.trim_end_matches(".html").to_string())
        } else {
            None
        }
    });

    if single_file_mode {
        if let Some(slug) = detected_slug {
            return (
                format!("#page-{}", escape_attr(&slug)),
                format!(" data-ws-page=\"{}\"", escape_attr(&slug)),
            );
        }
    } else if let Some(slug) = explicit_page_slug {
        let filename = if slug == "index" || slug.is_empty() {
            "index.html".to_string()
        } else {
            format!("{}.html", slug)
        };
        return (filename, String::new());
    }

    let href = if raw_href.trim().is_empty() {
        "#"
    } else {
        raw_href.trim()
    };
    (href.to_string(), String::new())
}

fn extract_youtube_id(url: &str) -> Option<String> {
    if let Some(idx) = url.find("youtu.be/") {
        let rest = &url[idx + 9..];
        let id: String = rest.chars().take_while(|c| *c != '?' && *c != '&' && *c != '/').collect();
        if !id.is_empty() {
            return Some(id);
        }
    }
    if url.contains("youtube.com") {
        if let Some(idx) = url.find("v=") {
            let rest = &url[idx + 2..];
            let id: String = rest.chars().take_while(|c| *c != '&' && *c != '#').collect();
            if !id.is_empty() {
                return Some(id);
            }
        }
        if let Some(idx) = url.find("/embed/") {
            let rest = &url[idx + 7..];
            let id: String = rest.chars().take_while(|c| *c != '?' && *c != '/').collect();
            if !id.is_empty() {
                return Some(id);
            }
        }
    }
    None
}

fn extract_vimeo_id(url: &str) -> Option<String> {
    if let Some(idx) = url.find("vimeo.com/") {
        let rest = &url[idx + 10..];
        let id: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        if !id.is_empty() {
            return Some(id);
        }
    }
    None
}

fn social_svg(platform: SocialPlatform) -> &'static str {
    match platform {
        SocialPlatform::GitHub => "<svg width=\"17\" height=\"17\" viewBox=\"0 0 24 24\" fill=\"currentColor\"><path d=\"M12 .5C5.65.5.5 5.65.5 12c0 5.08 3.29 9.39 7.86 10.91.57.11.78-.25.78-.55v-1.93c-3.2.7-3.87-1.54-3.87-1.54-.52-1.33-1.28-1.68-1.28-1.68-1.04-.71.08-.7.08-.7 1.15.08 1.76 1.19 1.76 1.19 1.02 1.76 2.69 1.25 3.35.96.1-.74.4-1.25.73-1.54-2.55-.29-5.24-1.28-5.24-5.68 0-1.25.45-2.28 1.19-3.08-.12-.29-.52-1.46.11-3.04 0 0 .97-.31 3.17 1.18a11 11 0 0 1 5.78 0c2.2-1.49 3.17-1.18 3.17-1.18.63 1.58.23 2.75.11 3.04.74.8 1.19 1.83 1.19 3.08 0 4.41-2.69 5.39-5.26 5.67.41.36.78 1.06.78 2.14v3.17c0 .3.21.66.79.55A11.51 11.51 0 0 0 23.5 12C23.5 5.65 18.35.5 12 .5Z\"/></svg>",
        SocialPlatform::X => "<svg width=\"16\" height=\"16\" viewBox=\"0 0 24 24\" fill=\"currentColor\"><path d=\"M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z\"/></svg>",
        SocialPlatform::LinkedIn => "<svg width=\"16\" height=\"16\" viewBox=\"0 0 24 24\" fill=\"currentColor\"><path d=\"M4.98 3.5C4.98 4.88 3.87 6 2.5 6S0 4.88 0 3.5 1.12 1 2.5 1s2.48 1.12 2.48 2.5zM.5 8h4V23h-4V8zm7.5 0h3.8v2.05h.05c.53-1 1.83-2.05 3.77-2.05 4.03 0 4.78 2.65 4.78 6.1V23h-4v-7.8c0-1.86-.03-4.25-2.59-4.25-2.59 0-2.99 2.02-2.99 4.11V23h-4V8z\"/></svg>",
        SocialPlatform::Instagram => "<svg width=\"16\" height=\"16\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"><rect x=\"2\" y=\"2\" width=\"20\" height=\"20\" rx=\"5\"/><circle cx=\"12\" cy=\"12\" r=\"4\"/><circle cx=\"17.5\" cy=\"6.5\" r=\"1\" fill=\"currentColor\"/></svg>",
        SocialPlatform::YouTube => "<svg width=\"17\" height=\"17\" viewBox=\"0 0 24 24\" fill=\"currentColor\"><path d=\"M23.5 6.2a3 3 0 0 0-2.1-2.1C19.5 3.6 12 3.6 12 3.6s-7.5 0-9.4.5A3 3 0 0 0 .5 6.2 31 31 0 0 0 0 12a31 31 0 0 0 .5 5.8 3 3 0 0 0 2.1 2.1c1.9.5 9.4.5 9.4.5s7.5 0 9.4-.5a3 3 0 0 0 2.1-2.1A31 31 0 0 0 24 12a31 31 0 0 0-.5-5.8zM9.6 15.6V8.4l6.3 3.6-6.3 3.6z\"/></svg>",
        SocialPlatform::Email => "<svg width=\"16\" height=\"16\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"><rect x=\"2\" y=\"4\" width=\"20\" height=\"16\" rx=\"2\"/><path d=\"m22 7-10 7L2 7\"/></svg>",
        SocialPlatform::Website => "<svg width=\"16\" height=\"16\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"><circle cx=\"12\" cy=\"12\" r=\"10\"/><path d=\"M2 12h20M12 2a15.3 15.3 0 0 1 0 20M12 2a15.3 15.3 0 0 0 0 20\"/></svg>",
    }
}

pub fn escape_html(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

pub fn escape_attr(input: &str) -> String {
    escape_html(input)
}
