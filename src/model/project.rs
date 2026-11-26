use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::asset::AssetItem;
use super::element::{Element, Rect2D};
use super::theme::DesignTokens;

/// Active viewport breakpoint in the studio designer.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ViewportMode {
    Desktop,
    Tablet,
    Mobile,
}

impl ViewportMode {
    pub fn width_px(&self) -> f32 {
        match self {
            ViewportMode::Desktop => 1280.0,
            ViewportMode::Tablet => 768.0,
            ViewportMode::Mobile => 390.0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ViewportMode::Desktop => "Desktop",
            ViewportMode::Tablet => "Tablet",
            ViewportMode::Mobile => "Mobile",
        }
    }
}

/// How a section arranges its elements on mobile viewports when no manual mobile rect is set.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MobileLayoutMode {
    AutoStack,
    Scale,
}

impl MobileLayoutMode {
    pub fn label(&self) -> &'static str {
        match self {
            MobileLayoutMode::AutoStack => "Auto Stack (Recommended)",
            MobileLayoutMode::Scale => "Proportional Scale",
        }
    }
}

/// Background styling for a section strip.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SectionBackground {
    Solid { color: String },
    LinearGradient { start: String, end: String, angle: f32 },
    Image { asset_id: Option<String>, overlay_color: String, overlay_opacity: f32 },
}

/// Horizontal strip section inside a page (Wix-style strip).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Section {
    pub id: String,
    pub name: String,
    pub anchor_id: String,
    pub height: f32,
    pub full_bleed: bool,
    pub sticky: bool,
    pub background: SectionBackground,
    pub border_top: bool,
    pub border_bottom: bool,
    pub mobile_layout: MobileLayoutMode,
    pub elements: Vec<Element>,
}

impl Section {
    /// Creates an empty section with the theme's default background.
    pub fn new(name: impl Into<String>, height: f32, bg_color: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            anchor_id: String::new(),
            height,
            full_bleed: false,
            sticky: false,
            background: SectionBackground::Solid {
                color: bg_color.into(),
            },
            border_top: false,
            border_bottom: true,
            mobile_layout: MobileLayoutMode::AutoStack,
            elements: Vec::new(),
        }
    }

    /// Returns the next z-index value higher than any existing element in this section.
    pub fn next_z_index(&self) -> i32 {
        self.elements.iter().map(|e| e.z_index).max().unwrap_or(0) + 1
    }

    /// Duplicates the section and assigns fresh UUIDs to itself and all contained elements.
    pub fn duplicate(&self) -> Self {
        let mut copy = self.clone();
        copy.id = Uuid::new_v4().to_string();
        copy.name = format!("{} Copy", self.name);
        for el in &mut copy.elements {
            el.id = Uuid::new_v4().to_string();
        }
        copy
    }
}

/// Single page inside a website project.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub meta_description: String,
    pub custom_head_html: String,
    pub background_override: Option<String>,
    pub sections: Vec<Section>,
}

impl Page {
    pub fn new(title: impl Into<String>, slug: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title: title.into(),
            slug: sanitize_slug(&slug.into()),
            meta_description: String::new(),
            custom_head_html: String::new(),
            background_override: None,
            sections: Vec::new(),
        }
    }

    /// Returns the generated HTML file name for this page (`index.html` or `<slug>.html`).
    pub fn html_filename(&self) -> String {
        if self.slug == "index" || self.slug.is_empty() {
            "index.html".to_string()
        } else {
            format!("{}.html", sanitize_slug(&self.slug))
        }
    }

    pub fn find_section(&self, section_id: &str) -> Option<&Section> {
        self.sections.iter().find(|s| s.id == section_id)
    }

    pub fn find_section_mut(&mut self, section_id: &str) -> Option<&mut Section> {
        self.sections.iter_mut().find(|s| s.id == section_id)
    }

    pub fn find_element(&self, element_id: &str) -> Option<(&Section, &Element)> {
        for sec in &self.sections {
            if let Some(el) = sec.elements.iter().find(|e| e.id == element_id) {
                return Some((sec, el));
            }
        }
        None
    }

    pub fn find_element_mut(&mut self, element_id: &str) -> Option<&mut Element> {
        for sec in &mut self.sections {
            if let Some(el) = sec.elements.iter_mut().find(|e| e.id == element_id) {
                return Some(el);
            }
        }
        None
    }

    pub fn duplicate(&self, new_title: String, new_slug: String) -> Self {
        let mut copy = self.clone();
        copy.id = Uuid::new_v4().to_string();
        copy.title = new_title;
        copy.slug = sanitize_slug(&new_slug);
        copy.sections = copy.sections.iter().map(|s| s.duplicate()).collect();
        copy
    }
}

/// Complete serializable Websir website project.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SiteProject {
    pub schema_version: u32,
    pub name: String,
    pub base_url: String,
    pub favicon_asset_id: Option<String>,
    pub theme: DesignTokens,
    pub assets: Vec<AssetItem>,
    pub pages: Vec<Page>,
}

impl SiteProject {
    pub const CURRENT_SCHEMA_VERSION: u32 = 1;

    pub fn new(name: impl Into<String>, theme: DesignTokens) -> Self {
        Self {
            schema_version: Self::CURRENT_SCHEMA_VERSION,
            name: name.into(),
            base_url: "https://example.com".into(),
            favicon_asset_id: None,
            theme,
            assets: Vec::new(),
            pages: vec![Page::new("Home", "index")],
        }
    }

    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }

    pub fn find_page(&self, page_id: &str) -> Option<&Page> {
        self.pages.iter().find(|p| p.id == page_id)
    }

    pub fn find_page_mut(&mut self, page_id: &str) -> Option<&mut Page> {
        self.pages.iter_mut().find(|p| p.id == page_id)
    }

    pub fn find_asset(&self, asset_id: &str) -> Option<&AssetItem> {
        self.assets.iter().find(|a| a.id == asset_id)
    }

    /// Resolves the effective rect of an element for a given viewport, keeping desktop as fallback.
    pub fn effective_rect(el: &Element, viewport: ViewportMode, canvas_width: f32) -> Rect2D {
        match viewport {
            ViewportMode::Desktop => el.rect,
            ViewportMode::Tablet => el.tablet_rect.unwrap_or_else(|| {
                let scale = canvas_width / 1200.0;
                Rect2D::new(
                    el.rect.x * scale,
                    el.rect.y,
                    (el.rect.w * scale).max(40.0).min(canvas_width - 16.0),
                    el.rect.h,
                )
            }),
            ViewportMode::Mobile => el.mobile_rect.unwrap_or_else(|| {
                let scale = canvas_width / 1200.0;
                Rect2D::new(
                    (el.rect.x * scale).min((canvas_width - 48.0).max(0.0)),
                    el.rect.y,
                    (el.rect.w * scale).max(60.0).min(canvas_width - 16.0),
                    el.rect.h,
                )
            }),
        }
    }
}

/// Sanitizes a URL slug to lowercase alphanumeric + hyphens (`"index"` if empty).
pub fn sanitize_slug(input: &str) -> String {
    let cleaned: String = input
        .trim()
        .to_ascii_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c
            } else if c == ' ' || c == '_' || c == '-' {
                '-'
            } else {
                '-'
            }
        })
        .collect();
    let collapsed = cleaned
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    if collapsed.is_empty() {
        "index".to_string()
    } else {
        collapsed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_sanitization_and_project_json_roundtrip() {
        assert_eq!(sanitize_slug("About Us! "), "about-us");
        assert_eq!(sanitize_slug("   "), "index");
        assert_eq!(sanitize_slug("pricing_2026"), "pricing-2026");

        let mut project = SiteProject::new("Slate Studio", DesignTokens::default());
        let el = Element::new_default(crate::model::ElementKind::Heading, &project.theme, 1);
        let el_id = el.id.clone();
        let mut sec = Section::new("Hero", 420.0, &project.theme.palette.background);
        sec.elements.push(el);
        project.pages[0].sections.push(sec);

        assert!(project.pages[0].find_element_mut(&el_id).is_some());

        let json = project.to_json_pretty().unwrap();
        let parsed = SiteProject::from_json(&json).unwrap();
        assert_eq!(parsed.name, "Slate Studio");
        assert_eq!(parsed.pages[0].sections[0].elements[0].id, el_id);
    }
}
