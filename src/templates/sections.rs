use crate::model::{
    AccordionItem, BoxShadowPreset, DesignTokens, Element, ElementBgType, ElementKind,
    EntranceAnimation, FooterColumn, HoverEffect, ListStyle, NavLink, Rect2D, Section,
    SectionBackground, TextAlign,
};

/// Pre-built Wix-style section strip presets ready to insert onto any page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionPreset {
    NavbarStrip,
    HeroCentered,
    HeroSplit,
    FeaturesGrid,
    AboutSplit,
    PortfolioGallery,
    PricingTiers,
    Testimonials,
    FaqAccordion,
    CtaBanner,
    ContactStrip,
    FooterStrip,
    BlankSection,
}

impl SectionPreset {
    pub fn all() -> &'static [SectionPreset] {
        &[
            SectionPreset::NavbarStrip,
            SectionPreset::HeroCentered,
            SectionPreset::HeroSplit,
            SectionPreset::FeaturesGrid,
            SectionPreset::AboutSplit,
            SectionPreset::PortfolioGallery,
            SectionPreset::PricingTiers,
            SectionPreset::Testimonials,
            SectionPreset::FaqAccordion,
            SectionPreset::CtaBanner,
            SectionPreset::ContactStrip,
            SectionPreset::FooterStrip,
            SectionPreset::BlankSection,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            SectionPreset::NavbarStrip => "Header Navbar",
            SectionPreset::HeroCentered => "Hero — Centered",
            SectionPreset::HeroSplit => "Hero — Split Media",
            SectionPreset::FeaturesGrid => "Features 3-Column",
            SectionPreset::AboutSplit => "About & Story",
            SectionPreset::PortfolioGallery => "Portfolio Grid",
            SectionPreset::PricingTiers => "Pricing Tiers",
            SectionPreset::Testimonials => "Testimonials",
            SectionPreset::FaqAccordion => "FAQ Accordion",
            SectionPreset::CtaBanner => "CTA Banner",
            SectionPreset::ContactStrip => "Contact Form",
            SectionPreset::FooterStrip => "Site Footer",
            SectionPreset::BlankSection => "Blank Strip",
        }
    }

    pub fn category(&self) -> &'static str {
        match self {
            SectionPreset::NavbarStrip | SectionPreset::FooterStrip => "Navigation & Chrome",
            SectionPreset::HeroCentered | SectionPreset::HeroSplit => "Heroes",
            SectionPreset::FeaturesGrid
            | SectionPreset::AboutSplit
            | SectionPreset::PortfolioGallery
            | SectionPreset::PricingTiers
            | SectionPreset::Testimonials => "Content & Showcase",
            SectionPreset::FaqAccordion
            | SectionPreset::CtaBanner
            | SectionPreset::ContactStrip
            | SectionPreset::BlankSection => "Conversion & Utility",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            SectionPreset::NavbarStrip => "Sticky navigation bar with brand, page links & CTA",
            SectionPreset::HeroCentered => "Focused centered headline, subtitle & dual CTAs",
            SectionPreset::HeroSplit => "Asymmetric hero with headline and media frame",
            SectionPreset::FeaturesGrid => "Three minimalist slate feature cards in a row",
            SectionPreset::AboutSplit => "Editorial story block paired with key highlights",
            SectionPreset::PortfolioGallery => "Curated 3-card project showcase grid",
            SectionPreset::PricingTiers => "Transparent 3-tier pricing comparison cards",
            SectionPreset::Testimonials => "Two-column client quotes & attribution",
            SectionPreset::FaqAccordion => "Interactive expandable questions & answers",
            SectionPreset::CtaBanner => "High-contrast call-to-action strip",
            SectionPreset::ContactStrip => "Structured contact form with instant feedback",
            SectionPreset::FooterStrip => "Multi-column footer with copyright & social links",
            SectionPreset::BlankSection => "Empty 400px canvas strip ready for custom layout",
        }
    }

    /// Instantiates a fully configured `Section` styled with the provided `DesignTokens`.
    pub fn build(&self, tokens: &DesignTokens) -> Section {
        match self {
            SectionPreset::NavbarStrip => build_navbar(tokens),
            SectionPreset::HeroCentered => build_hero_centered(tokens),
            SectionPreset::HeroSplit => build_hero_split(tokens),
            SectionPreset::FeaturesGrid => build_features_grid(tokens),
            SectionPreset::AboutSplit => build_about_split(tokens),
            SectionPreset::PortfolioGallery => build_portfolio_gallery(tokens),
            SectionPreset::PricingTiers => build_pricing_tiers(tokens),
            SectionPreset::Testimonials => build_testimonials(tokens),
            SectionPreset::FaqAccordion => build_faq_accordion(tokens),
            SectionPreset::CtaBanner => build_cta_banner(tokens),
            SectionPreset::ContactStrip => build_contact_strip(tokens),
            SectionPreset::FooterStrip => build_footer_strip(tokens),
            SectionPreset::BlankSection => {
                Section::new("Blank Strip", 400.0, &tokens.palette.background)
            }
        }
    }
}

fn make_heading(
    tokens: &DesignTokens,
    text: &str,
    level: u8,
    size: f32,
    align: TextAlign,
    rect: Rect2D,
    z: i32,
) -> Element {
    let mut el = Element::new_default(ElementKind::Heading, tokens, z);
    el.name = format!("H{} {}", level, text.chars().take(18).collect::<String>());
    el.content.text = text.into();
    el.content.heading_level = level;
    el.style.font_size = size;
    el.style.font_weight = tokens.typography.heading_weight;
    el.style.text_align = align;
    el.style.text_color = tokens.palette.text.clone();
    el.rect = rect;
    el
}

fn make_paragraph(
    tokens: &DesignTokens,
    text: &str,
    size: f32,
    align: TextAlign,
    rect: Rect2D,
    z: i32,
) -> Element {
    let mut el = Element::new_default(ElementKind::Paragraph, tokens, z);
    el.content.text = text.into();
    el.style.font_size = size;
    el.style.text_align = align;
    el.style.text_color = tokens.palette.text_muted.clone();
    el.rect = rect;
    el
}

fn make_button(
    tokens: &DesignTokens,
    label: &str,
    href: &str,
    primary: bool,
    rect: Rect2D,
    z: i32,
) -> Element {
    let mut el = Element::new_default(ElementKind::Button, tokens, z);
    el.name = format!("Btn: {}", label);
    el.content.text = label.into();
    el.content.href = href.into();
    if primary {
        el.style.bg_type = ElementBgType::Solid;
        el.style.bg_color = tokens.palette.primary.clone();
        el.style.text_color = tokens.palette.primary_text.clone();
    } else {
        el.style.bg_type = ElementBgType::Solid;
        el.style.bg_color = tokens.palette.surface.clone();
        el.style.text_color = tokens.palette.text.clone();
        el.style.border_width = 1.0;
        el.style.border_color = tokens.palette.border.clone();
    }
    el.rect = rect;
    el
}

fn make_badge(tokens: &DesignTokens, text: &str, rect: Rect2D, z: i32) -> Element {
    let mut el = Element::new_default(ElementKind::Badge, tokens, z);
    el.content.text = text.into();
    el.rect = rect;
    el
}

fn build_navbar(tokens: &DesignTokens) -> Section {
    let mut sec = Section::new("Navigation", 72.0, &tokens.palette.surface);
    sec.sticky = true;
    sec.full_bleed = true;
    sec.anchor_id = "top".into();

    let mut nav = Element::new_default(ElementKind::Navbar, tokens, 1);
    nav.name = "Main Navbar".into();
    nav.style.border_width = 0.0;
    nav.style.bg_type = ElementBgType::None;
    nav.rect = Rect2D::new(0.0, 0.0, 1200.0, 72.0);
    sec.elements.push(nav);
    sec
}

fn build_hero_centered(tokens: &DesignTokens) -> Section {
    let mut sec = Section::new("Hero Centered", 540.0, &tokens.palette.background);
    sec.anchor_id = "hero".into();
    sec.background = SectionBackground::LinearGradient {
        start: tokens.palette.background.clone(),
        end: tokens.palette.surface.clone(),
        angle: 180.0,
    };

    let mut badge = make_badge(
        tokens,
        "MINIMALIST DIGITAL STUDIO",
        Rect2D::new(470.0, 72.0, 260.0, 32.0),
        1,
    );
    badge.animation = EntranceAnimation::FadeIn;

    let mut h1 = make_heading(
        tokens,
        "Build crisp websites with quiet architectural precision.",
        1,
        56.0,
        TextAlign::Center,
        Rect2D::new(150.0, 126.0, 900.0, 148.0),
        2,
    );
    h1.style.line_height = 1.08;
    h1.animation = EntranceAnimation::SlideUp;

    let sub = make_paragraph(
        tokens,
        "A pure visual studio crafted for designers and makers. Customize every section, token, and breakpoint, then export clean static code anywhere.",
        19.0,
        TextAlign::Center,
        Rect2D::new(240.0, 294.0, 720.0, 72.0),
        3,
    );

    let btn_primary = make_button(
        tokens,
        "View Selected Work",
        "#work",
        true,
        Rect2D::new(408.0, 396.0, 196.0, 50.0),
        4,
    );
    let btn_secondary = make_button(
        tokens,
        "Get in Touch",
        "#contact",
        false,
        Rect2D::new(620.0, 396.0, 172.0, 50.0),
        5,
    );

    sec.elements
        .extend([badge, h1, sub, btn_primary, btn_secondary]);
    sec
}

fn build_hero_split(tokens: &DesignTokens) -> Section {
    let mut sec = Section::new("Hero Split", 560.0, &tokens.palette.background);
    sec.anchor_id = "hero".into();

    let badge = make_badge(
        tokens,
        "INDEPENDENT PRACTICE",
        Rect2D::new(60.0, 88.0, 220.0, 32.0),
        1,
    );
    let mut h1 = make_heading(
        tokens,
        "Interfaces shaped by clarity, contrast, and speed.",
        1,
        50.0,
        TextAlign::Left,
        Rect2D::new(60.0, 138.0, 560.0, 180.0),
        2,
    );
    h1.style.line_height = 1.1;

    let sub = make_paragraph(
        tokens,
        "We design and ship fast, enduring static web experiences with zero runtime bloat.",
        18.0,
        TextAlign::Left,
        Rect2D::new(60.0, 330.0, 520.0, 68.0),
        3,
    );

    let btn1 = make_button(
        tokens,
        "Start a Project",
        "#contact",
        true,
        Rect2D::new(60.0, 424.0, 184.0, 48.0),
        4,
    );
    let btn2 = make_button(
        tokens,
        "Explore Capabilities",
        "#features",
        false,
        Rect2D::new(260.0, 424.0, 196.0, 48.0),
        5,
    );

    let mut media = Element::new_default(ElementKind::Image, tokens, 6);
    media.name = "Hero Showcase Frame".into();
    media.content.alt_text = "Minimalist studio showcase".into();
    media.rect = Rect2D::new(660.0, 76.0, 480.0, 408.0);
    media.style.shadow = BoxShadowPreset::Elevated;

    sec.elements
        .extend([badge, h1, sub, btn1, btn2, media]);
    sec
}

fn build_features_grid(tokens: &DesignTokens) -> Section {
    let mut sec = Section::new("Features Grid", 480.0, &tokens.palette.surface);
    sec.anchor_id = "features".into();

    let heading = make_heading(
        tokens,
        "Built for complete visual control",
        2,
        36.0,
        TextAlign::Left,
        Rect2D::new(60.0, 54.0, 620.0, 52.0),
        1,
    );
    let intro = make_paragraph(
        tokens,
        "Every element, breakpoint, and design token stays transparent and editable.",
        17.0,
        TextAlign::Left,
        Rect2D::new(60.0, 112.0, 640.0, 36.0),
        2,
    );

    let cards = [
        (
            "01 / LAYOUT",
            "Freeform Canvas",
            "Drag, resize, snap to 8px grid, and align elements across desktop, tablet, and mobile viewports.",
            "Absolute + Responsive",
            60.0,
        ),
        (
            "02 / DESIGN SYSTEM",
            "Tokenized Slate Themes",
            "Switch palettes globally or fine-tune individual element borders, gradients, shadows, and typography.",
            "6 Curated Palettes",
            428.0,
        ),
        (
            "03 / DEPLOYMENT",
            "Universal Static Export",
            "Compile to clean HTML5/CSS3/JS folders, ZIP archives, or single-file Google Sites embed bundles.",
            "Zero Vendor Lock-in",
            796.0,
        ),
    ];

    sec.elements.extend([heading, intro]);
    for (idx, (eyebrow, title, body, meta, x)) in cards.iter().enumerate() {
        let mut card = Element::new_default(ElementKind::Card, tokens, (idx + 3) as i32);
        card.name = format!("Feature: {}", title);
        card.content.card_eyebrow = (*eyebrow).into();
        card.content.card_title = (*title).into();
        card.content.card_body = (*body).into();
        card.content.card_meta = (*meta).into();
        card.content.card_cta_label = String::new();
        card.style.bg_color = tokens.palette.background.clone();
        card.hover = HoverEffect::Lift;
        card.rect = Rect2D::new(*x, 178.0, 344.0, 240.0);
        sec.elements.push(card);
    }

    sec
}

fn build_about_split(tokens: &DesignTokens) -> Section {
    let mut sec = Section::new("About & Principles", 460.0, &tokens.palette.background);
    sec.anchor_id = "about".into();

    let badge = make_badge(
        tokens,
        "PHILOSOPHY",
        Rect2D::new(60.0, 64.0, 140.0, 30.0),
        1,
    );
    let h2 = make_heading(
        tokens,
        "Crafted manually, compiled deterministically.",
        2,
        36.0,
        TextAlign::Left,
        Rect2D::new(60.0, 108.0, 520.0, 100.0),
        2,
    );
    let p = make_paragraph(
        tokens,
        "No opaque black boxes or bloated frameworks. Your layout becomes clean semantic HTML and modern CSS variables you can inspect, host, and preserve for decades.",
        17.0,
        TextAlign::Left,
        Rect2D::new(60.0, 224.0, 520.0, 100.0),
        3,
    );

    let mut list_box = Element::new_default(ElementKind::Container, tokens, 4);
    list_box.name = "Highlights Frame".into();
    list_box.rect = Rect2D::new(640.0, 74.0, 500.0, 310.0);

    let mut list = Element::new_default(ElementKind::List, tokens, 5);
    list.name = "Principles List".into();
    list.content.list_style = ListStyle::Check;
    list.content.list_items = vec![
        "Zero AI generation — 100% deterministic Rust compiler".into(),
        "Pixel-accurate positioning with automatic mobile stacking".into(),
        "Embedded images export cleanly to folders or inline URIs".into(),
        "Works offline as a native desktop design studio".into(),
    ];
    list.rect = Rect2D::new(676.0, 118.0, 428.0, 224.0);

    sec.elements.extend([badge, h2, p, list_box, list]);
    sec
}

fn build_portfolio_gallery(tokens: &DesignTokens) -> Section {
    let mut sec = Section::new("Selected Work", 520.0, &tokens.palette.surface);
    sec.anchor_id = "work".into();

    let h2 = make_heading(
        tokens,
        "Selected Work",
        2,
        36.0,
        TextAlign::Left,
        Rect2D::new(60.0, 54.0, 480.0, 52.0),
        1,
    );
    let sub = make_paragraph(
        tokens,
        "Recent architectural websites and product systems.",
        17.0,
        TextAlign::Left,
        Rect2D::new(60.0, 112.0, 520.0, 36.0),
        2,
    );

    let items = [
        (
            "01 / FINTECH",
            "Kestrel Ledger",
            "Institutional treasury dashboard and marketing system built in crisp monochrome slate.",
            "2026 • Brand & Web",
            60.0,
        ),
        (
            "02 / HARDWARE",
            "Monolith Audio",
            "Minimalist product launch microsite featuring acoustic specs and editorial photography.",
            "2026 • Product Launch",
            428.0,
        ),
        (
            "03 / ARCHITECTURE",
            "Vespera Studio",
            "Quiet portfolio showcase designed for an architectural practice in Zurich.",
            "2025 • Portfolio",
            796.0,
        ),
    ];

    sec.elements.extend([h2, sub]);
    for (idx, (eyebrow, title, body, meta, x)) in items.iter().enumerate() {
        let mut card = Element::new_default(ElementKind::Card, tokens, (idx + 3) as i32);
        card.name = format!("Project: {}", title);
        card.content.card_eyebrow = (*eyebrow).into();
        card.content.card_title = (*title).into();
        card.content.card_body = (*body).into();
        card.content.card_meta = (*meta).into();
        card.content.card_cta_label = "View Case Study →".into();
        card.content.card_cta_href = "#contact".into();
        card.style.bg_color = tokens.palette.background.clone();
        card.hover = HoverEffect::Lift;
        card.rect = Rect2D::new(*x, 178.0, 344.0, 278.0);
        sec.elements.push(card);
    }

    sec
}

fn build_pricing_tiers(tokens: &DesignTokens) -> Section {
    let mut sec = Section::new("Pricing Tiers", 540.0, &tokens.palette.background);
    sec.anchor_id = "pricing".into();

    let h2 = make_heading(
        tokens,
        "Simple, transparent pricing",
        2,
        36.0,
        TextAlign::Center,
        Rect2D::new(260.0, 48.0, 680.0, 50.0),
        1,
    );
    let sub = make_paragraph(
        tokens,
        "Start locally for free. Deploy your static build on any server or CDN.",
        17.0,
        TextAlign::Center,
        Rect2D::new(280.0, 104.0, 640.0, 36.0),
        2,
    );

    let tiers = [
        (
            "STARTER",
            "$0 / forever",
            "Personal sites, portfolios, and static exports with full theme control.",
            "Single-user desktop studio",
            "Download Free",
            false,
            60.0,
        ),
        (
            "STUDIO",
            "$29 / month",
            "Multi-page systems, custom embed pipelines, and team template libraries.",
            "Most Popular • Unlimited Exports",
            "Choose Studio",
            true,
            428.0,
        ),
        (
            "ENTERPRISE",
            "Custom",
            "Dedicated white-label deployment bundles, audit trails, and SLA support.",
            "Custom Architecture",
            "Contact Sales",
            false,
            796.0,
        ),
    ];

    sec.elements.extend([h2, sub]);
    for (idx, (eyebrow, title, body, meta, cta, highlighted, x)) in tiers.iter().enumerate() {
        let mut card = Element::new_default(ElementKind::Card, tokens, (idx + 3) as i32);
        card.name = format!("Tier: {}", eyebrow);
        card.content.card_eyebrow = (*eyebrow).into();
        card.content.card_title = (*title).into();
        card.content.card_body = (*body).into();
        card.content.card_meta = (*meta).into();
        card.content.card_cta_label = (*cta).into();
        card.content.card_cta_href = "#contact".into();
        if *highlighted {
            card.style.border_width = 2.0;
            card.style.border_color = tokens.palette.primary.clone();
            card.style.shadow = BoxShadowPreset::Glow;
        }
        card.hover = HoverEffect::Lift;
        card.rect = Rect2D::new(*x, 174.0, 344.0, 300.0);
        sec.elements.push(card);
    }

    sec
}

fn build_testimonials(tokens: &DesignTokens) -> Section {
    let mut sec = Section::new("Testimonials", 390.0, &tokens.palette.surface);
    sec.anchor_id = "testimonials".into();

    let h2 = make_heading(
        tokens,
        "Trusted by focused teams",
        2,
        32.0,
        TextAlign::Left,
        Rect2D::new(60.0, 48.0, 480.0, 46.0),
        1,
    );

    let quotes = [
        (
            "QUOTE 01",
            "\"Cleanest static output we have shipped.\"",
            "We moved our entire marketing presence onto static edge hosting in one afternoon with zero layout regressions.",
            "Elena Vance — Principal at Northwind",
            60.0,
        ),
        (
            "QUOTE 02",
            "\"Wix-level visual speed without lock-in.\"",
            "Being able to position blocks visually and immediately inspect the generated CSS variables changed our workflow.",
            "Marcus Chen — Founder at Lattice Labs",
            616.0,
        ),
    ];

    sec.elements.push(h2);
    for (idx, (eyebrow, title, body, meta, x)) in quotes.iter().enumerate() {
        let mut card = Element::new_default(ElementKind::Card, tokens, (idx + 2) as i32);
        card.name = format!("Testimonial {}", idx + 1);
        card.content.card_eyebrow = (*eyebrow).into();
        card.content.card_title = (*title).into();
        card.content.card_body = (*body).into();
        card.content.card_meta = (*meta).into();
        card.content.card_cta_label = String::new();
        card.style.bg_color = tokens.palette.background.clone();
        card.rect = Rect2D::new(*x, 122.0, 524.0, 212.0);
        sec.elements.push(card);
    }

    sec
}

fn build_faq_accordion(tokens: &DesignTokens) -> Section {
    let mut sec = Section::new("FAQ", 410.0, &tokens.palette.background);
    sec.anchor_id = "faq".into();

    let h2 = make_heading(
        tokens,
        "Frequently Asked Questions",
        2,
        34.0,
        TextAlign::Center,
        Rect2D::new(260.0, 46.0, 680.0, 48.0),
        1,
    );

    let mut acc = Element::new_default(ElementKind::Accordion, tokens, 2);
    acc.name = "FAQ List".into();
    acc.content.accordion_items = vec![
        AccordionItem {
            question: "How do I launch my exported website?".into(),
            answer: "Export as a Folder or ZIP and upload to GitHub Pages, Netlify Drop, Cloudflare Pages, Nginx, or any Apache/S3 static host.".into(),
        },
        AccordionItem {
            question: "How does Google Sites integration work?".into(),
            answer: "Choose Export → Google Sites, click Copy Embed HTML, then inside Google Sites open Insert → Embed → Embed code and paste.".into(),
        },
        AccordionItem {
            question: "Does Websir use AI or external cloud servers?".into(),
            answer: "Never. Websir is 100% deterministic Rust running locally on your machine with full manual design control.".into(),
        },
    ];
    acc.rect = Rect2D::new(160.0, 120.0, 880.0, 240.0);

    sec.elements.extend([h2, acc]);
    sec
}

fn build_cta_banner(tokens: &DesignTokens) -> Section {
    let mut sec = Section::new("CTA Banner", 310.0, &tokens.palette.surface);
    sec.anchor_id = "cta".into();

    let mut box_bg = Element::new_default(ElementKind::Container, tokens, 1);
    box_bg.name = "CTA Container".into();
    box_bg.style.bg_color = tokens.palette.background.clone();
    box_bg.style.border_color = tokens.palette.border.clone();
    box_bg.rect = Rect2D::new(60.0, 44.0, 1080.0, 222.0);

    let h2 = make_heading(
        tokens,
        "Ready to launch your next site?",
        2,
        36.0,
        TextAlign::Left,
        Rect2D::new(112.0, 92.0, 620.0, 52.0),
        2,
    );
    let sub = make_paragraph(
        tokens,
        "Customize every section or export directly to your static host in seconds.",
        17.0,
        TextAlign::Left,
        Rect2D::new(112.0, 152.0, 600.0, 44.0),
        3,
    );
    let btn = make_button(
        tokens,
        "Start Building →",
        "#contact",
        true,
        Rect2D::new(884.0, 128.0, 204.0, 52.0),
        4,
    );

    sec.elements.extend([box_bg, h2, sub, btn]);
    sec
}

fn build_contact_strip(tokens: &DesignTokens) -> Section {
    let mut sec = Section::new("Contact", 530.0, &tokens.palette.background);
    sec.anchor_id = "contact".into();

    let badge = make_badge(
        tokens,
        "CONTACT",
        Rect2D::new(60.0, 68.0, 120.0, 30.0),
        1,
    );
    let h2 = make_heading(
        tokens,
        "Let’s build something enduring.",
        2,
        38.0,
        TextAlign::Left,
        Rect2D::new(60.0, 114.0, 460.0, 104.0),
        2,
    );
    let sub = make_paragraph(
        tokens,
        "Configure this form to open mailto:, submit to an HTTP endpoint, integrate with Netlify Forms, or display an instant client confirmation.",
        17.0,
        TextAlign::Left,
        Rect2D::new(60.0, 230.0, 440.0, 92.0),
        3,
    );

    let mut social = Element::new_default(ElementKind::SocialLinks, tokens, 4);
    social.rect = Rect2D::new(60.0, 344.0, 260.0, 44.0);

    let mut form = Element::new_default(ElementKind::ContactForm, tokens, 5);
    form.rect = Rect2D::new(560.0, 52.0, 580.0, 420.0);

    sec.elements.extend([badge, h2, sub, social, form]);
    sec
}

fn build_footer_strip(tokens: &DesignTokens) -> Section {
    let mut sec = Section::new("Footer", 240.0, &tokens.palette.surface);
    sec.full_bleed = true;
    sec.border_bottom = false;
    sec.border_top = true;

    let mut footer = Element::new_default(ElementKind::Footer, tokens, 1);
    footer.name = "Site Footer".into();
    footer.style.bg_type = ElementBgType::None;
    footer.style.border_width = 0.0;
    footer.content.footer_columns = vec![
        FooterColumn {
            heading: "Sections".into(),
            links: vec![
                NavLink {
                    label: "Hero".into(),
                    href: "#hero".into(),
                    page_slug: None,
                },
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
            ],
        },
        FooterColumn {
            heading: "Studio".into(),
            links: vec![
                NavLink {
                    label: "Contact".into(),
                    href: "#contact".into(),
                    page_slug: None,
                },
                NavLink {
                    label: "GitHub".into(),
                    href: "https://github.com".into(),
                    page_slug: None,
                },
            ],
        },
    ];
    footer.rect = Rect2D::new(0.0, 0.0, 1200.0, 240.0);
    sec.elements.push(footer);
    sec
}
