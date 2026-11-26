use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::theme::DesignTokens;

/// 2D rectangle relative to a section's content frame (`1200px` canonical desktop width).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Rect2D {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect2D {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
}

/// Supported visual and structural element kinds on the Websir canvas.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ElementKind {
    Heading,
    Paragraph,
    Button,
    Image,
    Container,
    Divider,
    Badge,
    Navbar,
    Footer,
    ContactForm,
    List,
    Card,
    Accordion,
    VideoEmbed,
    CustomHtml,
    SocialLinks,
}

impl ElementKind {
    /// Returns all 16 element kinds in palette order.
    pub fn all() -> &'static [ElementKind] {
        &[
            ElementKind::Heading,
            ElementKind::Paragraph,
            ElementKind::Button,
            ElementKind::Image,
            ElementKind::Container,
            ElementKind::Divider,
            ElementKind::Badge,
            ElementKind::Navbar,
            ElementKind::Footer,
            ElementKind::ContactForm,
            ElementKind::List,
            ElementKind::Card,
            ElementKind::Accordion,
            ElementKind::VideoEmbed,
            ElementKind::CustomHtml,
            ElementKind::SocialLinks,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            ElementKind::Heading => "Heading",
            ElementKind::Paragraph => "Paragraph",
            ElementKind::Button => "Button",
            ElementKind::Image => "Image",
            ElementKind::Container => "Box / Container",
            ElementKind::Divider => "Divider",
            ElementKind::Badge => "Badge / Pill",
            ElementKind::Navbar => "Navbar",
            ElementKind::Footer => "Footer",
            ElementKind::ContactForm => "Contact Form",
            ElementKind::List => "List",
            ElementKind::Card => "Feature Card",
            ElementKind::Accordion => "FAQ Accordion",
            ElementKind::VideoEmbed => "Video Embed",
            ElementKind::CustomHtml => "Custom HTML",
            ElementKind::SocialLinks => "Social Links",
        }
    }

    pub fn category(&self) -> &'static str {
        match self {
            ElementKind::Heading
            | ElementKind::Paragraph
            | ElementKind::Button
            | ElementKind::Badge
            | ElementKind::List => "Typography & Actions",
            ElementKind::Image | ElementKind::VideoEmbed | ElementKind::SocialLinks => {
                "Media & Social"
            }
            ElementKind::Container | ElementKind::Divider | ElementKind::Card => {
                "Layout & Cards"
            }
            ElementKind::Navbar
            | ElementKind::Footer
            | ElementKind::ContactForm
            | ElementKind::Accordion
            | ElementKind::CustomHtml => "Components & Embeds",
        }
    }

    pub fn short_icon(&self) -> &'static str {
        match self {
            ElementKind::Heading => "H1",
            ElementKind::Paragraph => "Aa",
            ElementKind::Button => "[ ]",
            ElementKind::Image => "IMG",
            ElementKind::Container => "BOX",
            ElementKind::Divider => "---",
            ElementKind::Badge => "TAG",
            ElementKind::Navbar => "NAV",
            ElementKind::Footer => "FTR",
            ElementKind::ContactForm => "FRM",
            ElementKind::List => "LST",
            ElementKind::Card => "CRD",
            ElementKind::Accordion => "FAQ",
            ElementKind::VideoEmbed => "VID",
            ElementKind::CustomHtml => "</>",
            ElementKind::SocialLinks => "@",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ElementBgType {
    None,
    Solid,
    LinearGradient,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BorderStyle {
    Solid,
    Dashed,
    Dotted,
}

impl BorderStyle {
    pub fn as_css(&self) -> &'static str {
        match self {
            BorderStyle::Solid => "solid",
            BorderStyle::Dashed => "dashed",
            BorderStyle::Dotted => "dotted",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

impl TextAlign {
    pub fn as_css(&self) -> &'static str {
        match self {
            TextAlign::Left => "left",
            TextAlign::Center => "center",
            TextAlign::Right => "right",
            TextAlign::Justify => "justify",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObjectFit {
    Cover,
    Contain,
    Fill,
}

impl ObjectFit {
    pub fn as_css(&self) -> &'static str {
        match self {
            ObjectFit::Cover => "cover",
            ObjectFit::Contain => "contain",
            ObjectFit::Fill => "fill",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BoxShadowPreset {
    None,
    Subtle,
    Medium,
    Elevated,
    Glow,
    Custom,
}

impl BoxShadowPreset {
    pub fn label(&self) -> &'static str {
        match self {
            BoxShadowPreset::None => "None",
            BoxShadowPreset::Subtle => "Subtle",
            BoxShadowPreset::Medium => "Medium",
            BoxShadowPreset::Elevated => "Elevated",
            BoxShadowPreset::Glow => "Crisp White Glow",
            BoxShadowPreset::Custom => "Soft Slate",
        }
    }

    pub fn as_css(&self) -> &'static str {
        match self {
            BoxShadowPreset::None => "none",
            BoxShadowPreset::Subtle => "0 2px 8px rgba(0, 0, 0, 0.18)",
            BoxShadowPreset::Medium => "0 10px 25px rgba(0, 0, 0, 0.28)",
            BoxShadowPreset::Elevated => "0 22px 48px rgba(0, 0, 0, 0.42)",
            BoxShadowPreset::Glow => "0 0 28px rgba(255, 255, 255, 0.14)",
            BoxShadowPreset::Custom => "0 12px 32px rgba(15, 23, 42, 0.35)",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HoverEffect {
    None,
    Lift,
    Scale,
    Brighten,
    Invert,
    BorderHighlight,
    Custom { bg_color: String, text_color: String },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntranceAnimation {
    None,
    FadeIn,
    SlideUp,
    SlideLeft,
    ZoomIn,
}

impl EntranceAnimation {
    pub fn label(&self) -> &'static str {
        match self {
            EntranceAnimation::None => "None",
            EntranceAnimation::FadeIn => "Fade In",
            EntranceAnimation::SlideUp => "Slide Up",
            EntranceAnimation::SlideLeft => "Slide Left",
            EntranceAnimation::ZoomIn => "Zoom In",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NavLink {
    pub label: String,
    pub href: String,
    pub page_slug: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FooterColumn {
    pub heading: String,
    pub links: Vec<NavLink>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FormActionMode {
    ToastDemo,
    Mailto,
    PostUrl,
    NetlifyForms,
}

impl FormActionMode {
    pub fn label(&self) -> &'static str {
        match self {
            FormActionMode::ToastDemo => "Interactive Demo Toast",
            FormActionMode::Mailto => "Email (mailto:)",
            FormActionMode::PostUrl => "HTTP POST Endpoint",
            FormActionMode::NetlifyForms => "Netlify Forms",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FormFieldType {
    Text,
    Email,
    Textarea,
}

impl FormFieldType {
    pub fn label(&self) -> &'static str {
        match self {
            FormFieldType::Text => "Text",
            FormFieldType::Email => "Email",
            FormFieldType::Textarea => "Textarea",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FormField {
    pub label: String,
    pub placeholder: String,
    pub field_type: FormFieldType,
    pub required: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ListStyle {
    Bullet,
    Numbered,
    Check,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccordionItem {
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SocialPlatform {
    GitHub,
    X,
    LinkedIn,
    Instagram,
    YouTube,
    Email,
    Website,
}

impl SocialPlatform {
    pub fn all() -> &'static [SocialPlatform] {
        &[
            SocialPlatform::GitHub,
            SocialPlatform::X,
            SocialPlatform::LinkedIn,
            SocialPlatform::Instagram,
            SocialPlatform::YouTube,
            SocialPlatform::Email,
            SocialPlatform::Website,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            SocialPlatform::GitHub => "GitHub",
            SocialPlatform::X => "X / Twitter",
            SocialPlatform::LinkedIn => "LinkedIn",
            SocialPlatform::Instagram => "Instagram",
            SocialPlatform::YouTube => "YouTube",
            SocialPlatform::Email => "Email",
            SocialPlatform::Website => "Website",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialLink {
    pub platform: SocialPlatform,
    pub url: String,
}

/// Complete visual styling for any element.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ElementStyle {
    pub bg_type: ElementBgType,
    pub bg_color: String,
    pub bg_gradient_end: String,
    pub bg_gradient_angle: f32,
    pub text_color: String,
    pub font_family: String,
    pub font_size: f32,
    pub font_weight: u16,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub text_align: TextAlign,
    /// Padding order: `[top, right, bottom, left]` in px.
    pub padding: [f32; 4],
    pub border_width: f32,
    pub border_style: BorderStyle,
    pub border_color: String,
    /// Border radius order: `[top_left, top_right, bottom_right, bottom_left]` in px.
    pub border_radius: [f32; 4],
    pub opacity: f32,
    pub shadow: BoxShadowPreset,
}

impl ElementStyle {
    pub fn default_with_theme(tokens: &DesignTokens) -> Self {
        Self {
            bg_type: ElementBgType::None,
            bg_color: tokens.palette.surface.clone(),
            bg_gradient_end: tokens.palette.surface_alt.clone(),
            bg_gradient_angle: 135.0,
            text_color: tokens.palette.text.clone(),
            font_family: String::new(),
            font_size: 16.0,
            font_weight: 400,
            line_height: 1.5,
            letter_spacing: 0.0,
            text_align: TextAlign::Left,
            padding: [0.0, 0.0, 0.0, 0.0],
            border_width: 0.0,
            border_style: BorderStyle::Solid,
            border_color: tokens.palette.border.clone(),
            border_radius: [tokens.border_radius; 4],
            opacity: 1.0,
            shadow: BoxShadowPreset::None,
        }
    }
}

/// Unified structured content payload supporting all 16 element types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ElementContent {
    pub text: String,
    pub heading_level: u8,
    pub href: String,
    pub link_target_page_slug: Option<String>,
    pub open_in_new_tab: bool,
    pub asset_id: Option<String>,
    pub image_url: String,
    pub alt_text: String,
    pub object_fit: ObjectFit,
    pub nav_brand: String,
    pub nav_auto_pages: bool,
    pub nav_links: Vec<NavLink>,
    pub nav_cta_label: String,
    pub nav_cta_href: String,
    pub footer_copyright: String,
    pub footer_columns: Vec<FooterColumn>,
    pub form_title: String,
    pub form_submit_label: String,
    pub form_action_mode: FormActionMode,
    pub form_action_value: String,
    pub form_fields: Vec<FormField>,
    pub list_style: ListStyle,
    pub list_items: Vec<String>,
    pub card_eyebrow: String,
    pub card_title: String,
    pub card_body: String,
    pub card_meta: String,
    pub card_cta_label: String,
    pub card_cta_href: String,
    pub card_asset_id: Option<String>,
    pub accordion_items: Vec<AccordionItem>,
    pub video_url: String,
    pub video_autoplay: bool,
    pub video_loop: bool,
    pub video_muted: bool,
    pub video_controls: bool,
    pub raw_html: String,
    pub social_links: Vec<SocialLink>,
}

impl Default for ElementContent {
    fn default() -> Self {
        Self {
            text: String::new(),
            heading_level: 2,
            href: "#".into(),
            link_target_page_slug: None,
            open_in_new_tab: false,
            asset_id: None,
            image_url: String::new(),
            alt_text: "Image".into(),
            object_fit: ObjectFit::Cover,
            nav_brand: "WEBSIR".into(),
            nav_auto_pages: true,
            nav_links: vec![
                NavLink {
                    label: "Work".into(),
                    href: "#work".into(),
                    page_slug: None,
                },
                NavLink {
                    label: "About".into(),
                    href: "#about".into(),
                    page_slug: None,
                },
                NavLink {
                    label: "Contact".into(),
                    href: "#contact".into(),
                    page_slug: None,
                },
            ],
            nav_cta_label: "Get Started".into(),
            nav_cta_href: "#contact".into(),
            footer_copyright: "© 2026 Studio. Built with Websir.".into(),
            footer_columns: vec![
                FooterColumn {
                    heading: "Navigation".into(),
                    links: vec![
                        NavLink {
                            label: "Home".into(),
                            href: "index.html".into(),
                            page_slug: Some("index".into()),
                        },
                        NavLink {
                            label: "Work".into(),
                            href: "#work".into(),
                            page_slug: None,
                        },
                    ],
                },
                FooterColumn {
                    heading: "Connect".into(),
                    links: vec![
                        NavLink {
                            label: "Email".into(),
                            href: "mailto:hello@example.com".into(),
                            page_slug: None,
                        },
                        NavLink {
                            label: "GitHub".into(),
                            href: "https://github.com".into(),
                            page_slug: None,
                        },
                    ],
                },
            ],
            form_title: "Start a conversation".into(),
            form_submit_label: "Send Message".into(),
            form_action_mode: FormActionMode::ToastDemo,
            form_action_value: "hello@example.com".into(),
            form_fields: vec![
                FormField {
                    label: "Name".into(),
                    placeholder: "Jane Doe".into(),
                    field_type: FormFieldType::Text,
                    required: true,
                },
                FormField {
                    label: "Email".into(),
                    placeholder: "jane@company.com".into(),
                    field_type: FormFieldType::Email,
                    required: true,
                },
                FormField {
                    label: "Message".into(),
                    placeholder: "Tell us about your project...".into(),
                    field_type: FormFieldType::Textarea,
                    required: true,
                },
            ],
            list_style: ListStyle::Check,
            list_items: vec![
                "Pure static HTML5, CSS3, and vanilla JS".into(),
                "Responsive breakpoints for desktop, tablet, and mobile".into(),
                "Self-contained export for any host or Google Sites".into(),
            ],
            card_eyebrow: "CASE STUDY".into(),
            card_title: "Minimalist Architecture".into(),
            card_body: "Crisp slate surfaces paired with restrained editorial typography and intentional spacing.".into(),
            card_meta: "2026 • Web Design".into(),
            card_cta_label: "Explore →".into(),
            card_cta_href: "#".into(),
            card_asset_id: None,
            accordion_items: vec![
                AccordionItem {
                    question: "Can I host the exported website anywhere?".into(),
                    answer: "Yes. Exported sites use standard HTML, CSS, and JS with zero proprietary runtime dependencies.".into(),
                },
                AccordionItem {
                    question: "Does this support Google Sites?".into(),
                    answer: "Yes. Use the Google Sites Embed export to copy a self-contained single-file HTML bundle directly into Google Sites.".into(),
                },
            ],
            video_url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".into(),
            video_autoplay: false,
            video_loop: false,
            video_muted: false,
            video_controls: true,
            raw_html: "<div style=\"padding:20px;border:1px solid #334155;border-radius:8px;\">Custom HTML Block</div>".into(),
            social_links: vec![
                SocialLink {
                    platform: SocialPlatform::GitHub,
                    url: "https://github.com".into(),
                },
                SocialLink {
                    platform: SocialPlatform::X,
                    url: "https://x.com".into(),
                },
                SocialLink {
                    platform: SocialPlatform::LinkedIn,
                    url: "https://linkedin.com".into(),
                },
                SocialLink {
                    platform: SocialPlatform::Email,
                    url: "mailto:hello@example.com".into(),
                },
            ],
        }
    }
}

/// A single positioned, customizable element inside a section.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Element {
    pub id: String,
    pub name: String,
    pub kind: ElementKind,
    pub content: ElementContent,
    pub rect: Rect2D,
    pub tablet_rect: Option<Rect2D>,
    pub mobile_rect: Option<Rect2D>,
    pub z_index: i32,
    pub locked: bool,
    pub hidden: bool,
    pub full_width: bool,
    pub visible_desktop: bool,
    pub visible_tablet: bool,
    pub visible_mobile: bool,
    pub style: ElementStyle,
    pub hover: HoverEffect,
    pub animation: EntranceAnimation,
    pub html_id: String,
    pub custom_class: String,
    pub custom_css: String,
}

impl Element {
    /// Creates a well-proportioned default element for the given kind and theme.
    pub fn new_default(kind: ElementKind, tokens: &DesignTokens, z_index: i32) -> Self {
        let mut style = ElementStyle::default_with_theme(tokens);
        let mut content = ElementContent::default();
        let mut full_width = false;

        let rect = match kind {
            ElementKind::Heading => {
                content.text = "Designed with Quiet Precision".into();
                content.heading_level = 1;
                style.font_size = 48.0;
                style.font_weight = tokens.typography.heading_weight;
                style.line_height = 1.15;
                style.letter_spacing = -0.025;
                Rect2D::new(80.0, 60.0, 680.0, 84.0)
            }
            ElementKind::Paragraph => {
                content.text = "Craft structured layouts, customize every token, and publish clean static websites without a single line of boilerplate.".into();
                style.font_size = 17.0;
                style.text_color = tokens.palette.text_muted.clone();
                style.line_height = 1.65;
                Rect2D::new(80.0, 156.0, 580.0, 88.0)
            }
            ElementKind::Button => {
                content.text = "Explore Work".into();
                content.href = "#work".into();
                style.bg_type = ElementBgType::Solid;
                style.bg_color = tokens.palette.primary.clone();
                style.text_color = tokens.palette.primary_text.clone();
                style.font_size = 15.0;
                style.font_weight = 600;
                style.text_align = TextAlign::Center;
                style.padding = [12.0, 24.0, 12.0, 24.0];
                style.border_radius = [8.0; 4];
                Rect2D::new(80.0, 260.0, 176.0, 48.0)
            }
            ElementKind::Image => {
                style.bg_type = ElementBgType::Solid;
                style.bg_color = tokens.palette.surface_alt.clone();
                style.border_width = 1.0;
                style.border_color = tokens.palette.border.clone();
                style.border_radius = [12.0; 4];
                Rect2D::new(680.0, 48.0, 440.0, 300.0)
            }
            ElementKind::Container => {
                style.bg_type = ElementBgType::Solid;
                style.bg_color = tokens.palette.surface.clone();
                style.border_width = 1.0;
                style.border_color = tokens.palette.border.clone();
                style.border_radius = [12.0; 4];
                style.padding = [24.0, 24.0, 24.0, 24.0];
                Rect2D::new(80.0, 48.0, 380.0, 220.0)
            }
            ElementKind::Divider => {
                style.bg_type = ElementBgType::Solid;
                style.bg_color = tokens.palette.border.clone();
                style.border_radius = [0.0; 4];
                Rect2D::new(60.0, 120.0, 1080.0, 2.0)
            }
            ElementKind::Badge => {
                content.text = "PURE RUST • ZERO AI".into();
                style.bg_type = ElementBgType::Solid;
                style.bg_color = tokens.palette.surface_alt.clone();
                style.text_color = tokens.palette.accent.clone();
                style.border_width = 1.0;
                style.border_color = tokens.palette.border.clone();
                style.font_size = 12.0;
                style.font_weight = 600;
                style.letter_spacing = 0.08;
                style.text_align = TextAlign::Center;
                style.border_radius = [999.0; 4];
                style.padding = [6.0, 14.0, 6.0, 14.0];
                Rect2D::new(80.0, 24.0, 190.0, 32.0)
            }
            ElementKind::Navbar => {
                full_width = true;
                style.bg_type = ElementBgType::Solid;
                style.bg_color = tokens.palette.surface.clone();
                style.border_width = 1.0;
                style.border_color = tokens.palette.border.clone();
                style.border_radius = [0.0; 4];
                style.padding = [0.0, 24.0, 0.0, 24.0];
                Rect2D::new(0.0, 0.0, 1200.0, 72.0)
            }
            ElementKind::Footer => {
                full_width = true;
                style.bg_type = ElementBgType::Solid;
                style.bg_color = tokens.palette.surface.clone();
                style.border_width = 1.0;
                style.border_color = tokens.palette.border.clone();
                style.border_radius = [0.0; 4];
                style.padding = [40.0, 32.0, 40.0, 32.0];
                Rect2D::new(0.0, 0.0, 1200.0, 240.0)
            }
            ElementKind::ContactForm => {
                style.bg_type = ElementBgType::Solid;
                style.bg_color = tokens.palette.surface.clone();
                style.border_width = 1.0;
                style.border_color = tokens.palette.border.clone();
                style.border_radius = [12.0; 4];
                style.padding = [28.0, 28.0, 28.0, 28.0];
                Rect2D::new(320.0, 40.0, 560.0, 410.0)
            }
            ElementKind::List => {
                style.font_size = 16.0;
                style.line_height = 1.7;
                Rect2D::new(80.0, 60.0, 520.0, 140.0)
            }
            ElementKind::Card => {
                style.bg_type = ElementBgType::Solid;
                style.bg_color = tokens.palette.surface.clone();
                style.border_width = 1.0;
                style.border_color = tokens.palette.border.clone();
                style.border_radius = [12.0; 4];
                style.padding = [24.0, 24.0, 24.0, 24.0];
                Rect2D::new(80.0, 48.0, 340.0, 260.0)
            }
            ElementKind::Accordion => {
                style.bg_type = ElementBgType::Solid;
                style.bg_color = tokens.palette.surface.clone();
                style.border_width = 1.0;
                style.border_color = tokens.palette.border.clone();
                style.border_radius = [10.0; 4];
                style.padding = [16.0, 20.0, 16.0, 20.0];
                Rect2D::new(160.0, 40.0, 880.0, 220.0)
            }
            ElementKind::VideoEmbed => {
                style.bg_type = ElementBgType::Solid;
                style.bg_color = tokens.palette.surface_alt.clone();
                style.border_width = 1.0;
                style.border_color = tokens.palette.border.clone();
                style.border_radius = [12.0; 4];
                Rect2D::new(220.0, 40.0, 760.0, 428.0)
            }
            ElementKind::CustomHtml => {
                style.padding = [12.0, 12.0, 12.0, 12.0];
                Rect2D::new(120.0, 60.0, 460.0, 140.0)
            }
            ElementKind::SocialLinks => {
                style.text_color = tokens.palette.text.clone();
                Rect2D::new(80.0, 80.0, 260.0, 44.0)
            }
        };

        Self {
            id: Uuid::new_v4().to_string(),
            name: kind.label().to_string(),
            kind,
            content,
            rect,
            tablet_rect: None,
            mobile_rect: None,
            z_index,
            locked: false,
            hidden: false,
            full_width,
            visible_desktop: true,
            visible_tablet: true,
            visible_mobile: true,
            style,
            hover: if kind == ElementKind::Button {
                HoverEffect::Lift
            } else {
                HoverEffect::None
            },
            animation: EntranceAnimation::None,
            html_id: String::new(),
            custom_class: String::new(),
            custom_css: String::new(),
        }
    }

    /// Duplicates this element with a fresh ID, slightly offset position, and bumped z-index.
    pub fn duplicate(&self, next_z: i32) -> Self {
        let mut copy = self.clone();
        copy.id = Uuid::new_v4().to_string();
        copy.name = format!("{} Copy", self.name);
        copy.rect.x += 20.0;
        copy.rect.y += 20.0;
        if let Some(tr) = &mut copy.tablet_rect {
            tr.x += 16.0;
            tr.y += 16.0;
        }
        if let Some(mr) = &mut copy.mobile_rect {
            mr.x += 12.0;
            mr.y += 12.0;
        }
        copy.z_index = next_z;
        copy
    }
}
