pub mod sections;

pub use sections::SectionPreset;

use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder};

use crate::model::{
    AssetItem, DesignTokens, ElementKind, NavLink, Page, PresetTheme, SiteProject,
};

/// Complete multi-page or single-page zero-AI starter website templates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarterTemplate {
    MinimalistPortfolio,
    SaasLanding,
    MeridianDemo,
    StudioAgency,
    PersonalBio,
    BlankCanvas,
}

impl StarterTemplate {
    pub fn all() -> &'static [StarterTemplate] {
        &[
            StarterTemplate::MeridianDemo,
            StarterTemplate::MinimalistPortfolio,
            StarterTemplate::SaasLanding,
            StarterTemplate::StudioAgency,
            StarterTemplate::PersonalBio,
            StarterTemplate::BlankCanvas,
        ]
    }

    pub fn cli_key(&self) -> &'static str {
        match self {
            StarterTemplate::MinimalistPortfolio => "portfolio",
            StarterTemplate::SaasLanding => "saas",
            StarterTemplate::MeridianDemo => "meridian",
            StarterTemplate::StudioAgency => "agency",
            StarterTemplate::PersonalBio => "bio",
            StarterTemplate::BlankCanvas => "blank",
        }
    }

    pub fn from_cli_key(key: &str) -> Option<Self> {
        match key.trim().to_ascii_lowercase().as_str() {
            "portfolio" | "minimalist-portfolio" => Some(StarterTemplate::MinimalistPortfolio),
            "saas" | "saas-landing" => Some(StarterTemplate::SaasLanding),
            "meridian" | "meridian-cloud" | "example" => Some(StarterTemplate::MeridianDemo),
            "agency" | "studio-agency" => Some(StarterTemplate::StudioAgency),
            "bio" | "personal-bio" => Some(StarterTemplate::PersonalBio),
            "blank" | "blank-canvas" => Some(StarterTemplate::BlankCanvas),
            _ => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            StarterTemplate::MinimalistPortfolio => "Minimalist Slate Portfolio",
            StarterTemplate::SaasLanding => "SaaS Product Landing",
            StarterTemplate::MeridianDemo => "Meridian Cloud (Example Site)",
            StarterTemplate::StudioAgency => "Design Studio & Agency",
            StarterTemplate::PersonalBio => "Personal Bio & Resume",
            StarterTemplate::BlankCanvas => "Blank Canvas",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            StarterTemplate::MinimalistPortfolio => {
                "Obsidian slate portfolio with split hero, work gallery, principles, contact & footer"
            }
            StarterTemplate::SaasLanding => {
                "Multi-page SaaS site (Home, Pricing, Contact) with features, tiers, FAQ & CTA"
            }
            StarterTemplate::MeridianDemo => {
                "Complete 3-page cloud infrastructure showcase (Home, Pricing, Contact) built as the Weysko v1.3.0 demo"
            }
            StarterTemplate::StudioAgency => {
                "Multi-page agency site (Home, Services, About, Contact) with testimonials"
            }
            StarterTemplate::PersonalBio => {
                "Focused single-page digital card, bio, key highlights & social links"
            }
            StarterTemplate::BlankCanvas => {
                "Minimalist starting structure with sticky navbar, blank hero strip & footer"
            }
        }
    }

    /// Default preset theme recommended for this template.
    pub fn default_theme(&self) -> PresetTheme {
        match self {
            StarterTemplate::PersonalBio => PresetTheme::CarbonInk,
            _ => PresetTheme::ObsidianSlate,
        }
    }

    /// Creates a brand new `SiteProject` using this template's default theme.
    pub fn build(&self) -> SiteProject {
        self.build_with_theme(self.default_theme(), None)
    }

    /// Creates a brand new `SiteProject` styled cleanly with any chosen `PresetTheme`.
    pub fn build_with_theme(&self, preset: PresetTheme, site_name: Option<&str>) -> SiteProject {
        let tokens = preset.to_tokens();
        let mut proj = match self {
            StarterTemplate::MinimalistPortfolio => build_portfolio(tokens),
            StarterTemplate::SaasLanding => build_saas(tokens),
            StarterTemplate::MeridianDemo => build_meridian_example(tokens),
            StarterTemplate::StudioAgency => build_agency(tokens),
            StarterTemplate::PersonalBio => build_bio(tokens),
            StarterTemplate::BlankCanvas => build_blank(tokens),
        };
        if let Some(name) = site_name {
            let trimmed = name.trim();
            if !trimmed.is_empty() {
                proj.name = trimmed.to_string();
            }
        }
        proj
    }
}

fn customize_navbar(page: &mut Page, brand: &str, cta_label: &str, cta_href: &str) {
    for sec in &mut page.sections {
        for el in &mut sec.elements {
            if el.kind == ElementKind::Navbar {
                el.content.nav_brand = brand.into();
                el.content.nav_cta_label = cta_label.into();
                el.content.nav_cta_href = cta_href.into();
            }
        }
    }
}

fn build_portfolio(tokens: DesignTokens) -> SiteProject {
    let mut project = SiteProject::new("Slate Portfolio", tokens.clone());
    project.pages.clear();

    let mut home = Page::new("Home — Slate Portfolio", "index");
    home.meta_description =
        "Minimalist slate architectural portfolio built with Weysko.".into();
    home.sections = vec![
        SectionPreset::NavbarStrip.build(&tokens),
        SectionPreset::HeroSplit.build(&tokens),
        SectionPreset::PortfolioGallery.build(&tokens),
        SectionPreset::AboutSplit.build(&tokens),
        SectionPreset::ContactStrip.build(&tokens),
        SectionPreset::FooterStrip.build(&tokens),
    ];
    customize_navbar(&mut home, "VANCE.STUDIO", "Hire Me", "#contact");
    project.pages.push(home);
    project
}

fn build_saas(tokens: DesignTokens) -> SiteProject {
    let mut project = SiteProject::new("Kestrel SaaS", tokens.clone());
    project.pages.clear();

    let mut home = Page::new("Home", "index");
    home.meta_description = "Modern static web engineering platform.".into();
    home.sections = vec![
        SectionPreset::NavbarStrip.build(&tokens),
        SectionPreset::HeroCentered.build(&tokens),
        SectionPreset::FeaturesGrid.build(&tokens),
        SectionPreset::Testimonials.build(&tokens),
        SectionPreset::CtaBanner.build(&tokens),
        SectionPreset::FooterStrip.build(&tokens),
    ];
    customize_navbar(&mut home, "KESTREL", "Get Started", "contact.html");

    let mut pricing = Page::new("Pricing", "pricing");
    pricing.meta_description = "Transparent pricing tiers for individuals and teams.".into();
    pricing.sections = vec![
        SectionPreset::NavbarStrip.build(&tokens),
        SectionPreset::PricingTiers.build(&tokens),
        SectionPreset::FaqAccordion.build(&tokens),
        SectionPreset::FooterStrip.build(&tokens),
    ];
    customize_navbar(&mut pricing, "KESTREL", "Get Started", "contact.html");

    let mut contact = Page::new("Contact", "contact");
    contact.meta_description = "Talk with the Kestrel team.".into();
    contact.sections = vec![
        SectionPreset::NavbarStrip.build(&tokens),
        SectionPreset::ContactStrip.build(&tokens),
        SectionPreset::FooterStrip.build(&tokens),
    ];
    customize_navbar(&mut contact, "KESTREL", "Home", "index.html");

    project.pages.extend([home, pricing, contact]);
    project
}

/// Builds the official Meridian Cloud 3-page example website exercising all major studio features.
fn build_meridian_example(tokens: DesignTokens) -> SiteProject {
    let mut project = SiteProject::new("Meridian Cloud", tokens.clone());
    project.base_url = "https://meridian.example.com".into();
    project.pages.clear();

    // Embed a pure-Rust rasterized architectural topology graphic asset
    let diagram_png = render_meridian_topology_png(480, 408);
    let asset = AssetItem::from_bytes("meridian-topology.png", "image/png", &diagram_png);
    let asset_id = asset.id.clone();
    project.assets.push(asset);

    let nav_links = vec![
        NavLink {
            label: "Home".into(),
            href: "index.html".into(),
            page_slug: Some("index".into()),
        },
        NavLink {
            label: "Architecture".into(),
            href: "index.html#features".into(),
            page_slug: None,
        },
        NavLink {
            label: "Pricing".into(),
            href: "pricing.html".into(),
            page_slug: Some("pricing".into()),
        },
        NavLink {
            label: "Contact".into(),
            href: "contact.html".into(),
            page_slug: Some("contact".into()),
        },
    ];

    let apply_meridian_nav = |page: &mut Page, cta_lbl: &str, cta_href: &str| {
        for sec in &mut page.sections {
            for el in &mut sec.elements {
                if el.kind == ElementKind::Navbar {
                    el.content.nav_brand = "MERIDIAN".into();
                    el.content.nav_links = nav_links.clone();
                    el.content.nav_cta_label = cta_lbl.into();
                    el.content.nav_cta_href = cta_href.into();
                }
            }
        }
    };

    // Page 1: Home
    let mut home = Page::new("Home", "index");
    home.meta_description =
        "Meridian Cloud — Deterministic global compute and static edge fabric compiled with Weysko."
            .into();

    let mut hero = SectionPreset::HeroSplit.build(&tokens);
    hero.name = "Meridian Hero".into();
    for el in &mut hero.elements {
        match el.kind {
            ElementKind::Badge => {
                el.content.text = "MERIDIAN FABRIC v4".into();
            }
            ElementKind::Heading => {
                el.content.text = "Deterministic edge compute for distributed teams.".into();
            }
            ElementKind::Paragraph => {
                el.content.text = "Deploy reproducible workloads across 42 bare-metal regions with hardware-isolated networking and sub-15ms global routing.".into();
            }
            ElementKind::Button if el.content.href == "#contact" => {
                el.content.text = "View Pricing".into();
                el.content.href = "pricing.html".into();
            }
            ElementKind::Button => {
                el.content.text = "Explore Stack".into();
                el.content.href = "#features".into();
            }
            ElementKind::Image => {
                el.name = "Topology Graphic".into();
                el.content.asset_id = Some(asset_id.clone());
                el.content.alt_text = "Meridian Cloud global edge topology".into();
            }
            _ => {}
        }
    }

    let mut features = SectionPreset::FeaturesGrid.build(&tokens);
    features.name = "Core Architecture".into();
    let mut feat_titles = [
        ("Zero-Jitter Edge", "Dedicated NVMe instances with deterministic packet scheduling and zero noisy-neighbor throttling."),
        ("Immutable Deploys", "Cryptographic artifact manifests ensure every production rollout matches staging byte-for-byte."),
        ("Private Mesh", "Encrypted WireGuard overlays link regional clusters without public internet exposure."),
    ]
    .into_iter();
    for el in &mut features.elements {
        if el.kind == ElementKind::Heading && el.content.heading_level == 2 {
            el.content.text = "Engineered for predictable uptime".into();
        } else if el.kind == ElementKind::Paragraph && el.rect.y < 140.0 {
            el.content.text =
                "Bare-metal speed paired with declarative routing across global points of presence."
                    .into();
        } else if el.kind == ElementKind::Card {
            if let Some((t, b)) = feat_titles.next() {
                el.content.card_title = t.into();
                el.content.card_body = b.into();
            }
        }
    }

    let testimonials = SectionPreset::Testimonials.build(&tokens);
    let mut cta = SectionPreset::CtaBanner.build(&tokens);
    for el in &mut cta.elements {
        if el.kind == ElementKind::Heading {
            el.content.text = "Ready to run on Meridian?".into();
        } else if el.kind == ElementKind::Button {
            el.content.text = "Launch Cluster".into();
            el.content.href = "contact.html".into();
        }
    }

    home.sections = vec![
        SectionPreset::NavbarStrip.build(&tokens),
        hero,
        features,
        testimonials,
        cta,
        SectionPreset::FooterStrip.build(&tokens),
    ];
    apply_meridian_nav(&mut home, "Deploy Now", "contact.html");

    // Page 2: Pricing
    let mut pricing = Page::new("Pricing", "pricing");
    pricing.meta_description = "Predictable Meridian Cloud compute and bandwidth tiers.".into();
    pricing.sections = vec![
        SectionPreset::NavbarStrip.build(&tokens),
        SectionPreset::PricingTiers.build(&tokens),
        SectionPreset::FaqAccordion.build(&tokens),
        SectionPreset::FooterStrip.build(&tokens),
    ];
    apply_meridian_nav(&mut pricing, "Deploy Now", "contact.html");

    // Page 3: Contact
    let mut contact = Page::new("Contact", "contact");
    contact.meta_description = "Connect with Meridian Cloud solutions engineering.".into();
    contact.sections = vec![
        SectionPreset::NavbarStrip.build(&tokens),
        SectionPreset::ContactStrip.build(&tokens),
        SectionPreset::FooterStrip.build(&tokens),
    ];
    apply_meridian_nav(&mut contact, "Back Home", "index.html");

    project.pages.extend([home, pricing, contact]);
    project
}

/// Generates a crisp pure-Rust geometric network topology graphic as encoded PNG bytes.
fn render_meridian_topology_png(width: u32, height: u32) -> Vec<u8> {
    let mut rgba = vec![0u8; (width * height * 4) as usize];
    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            let nx = x as f32 / width as f32;
            let ny = y as f32 / height as f32;

            // Deep slate backdrop gradient + subtle blueprint grid every 24px
            let mut r = (9.0 + nx * 8.0 + ny * 12.0) as u8;
            let mut g = (15.0 + nx * 12.0 + ny * 20.0) as u8;
            let mut b = (28.0 + nx * 24.0 + ny * 34.0) as u8;

            if x % 24 == 0 || y % 24 == 0 {
                r = r.saturating_add(14);
                g = g.saturating_add(18);
                b = b.saturating_add(24);
            }

            // Concentric orbital rings centered at (0.55, 0.50)
            let dx = (x as f32) - (width as f32 * 0.55);
            let dy = (y as f32) - (height as f32 * 0.50);
            let dist = dx.hypot(dy);
            for ring in [56.0_f32, 108.0_f32, 160.0_f32] {
                if (dist - ring).abs() < 1.3 {
                    r = 0;
                    g = 188;
                    b = 235;
                }
            }

            // Precision nodes
            for (cx, cy, nr, col) in [
                (0.55, 0.50, 9.0_f32, [255u8, 255, 255]),
                (0.32, 0.34, 6.0_f32, [0, 188, 235]),
                (0.76, 0.32, 6.0_f32, [0, 96, 255]),
                (0.72, 0.72, 6.0_f32, [0, 188, 235]),
                (0.36, 0.70, 5.5_f32, [203, 213, 225]),
            ] {
                let nd = (x as f32 - width as f32 * cx).hypot(y as f32 - height as f32 * cy);
                if nd <= nr {
                    r = col[0];
                    g = col[1];
                    b = col[2];
                }
            }

            rgba[idx] = r;
            rgba[idx + 1] = g;
            rgba[idx + 2] = b;
            rgba[idx + 3] = 255;
        }
    }

    let mut out = Vec::new();
    PngEncoder::new(&mut out)
        .write_image(&rgba, width, height, ExtendedColorType::Rgba8)
        .expect("encode meridian topology png");
    out
}

fn build_agency(tokens: DesignTokens) -> SiteProject {
    let mut project = SiteProject::new("Monolith Agency", tokens.clone());
    project.pages.clear();

    let mut home = Page::new("Home", "index");
    home.sections = vec![
        SectionPreset::NavbarStrip.build(&tokens),
        SectionPreset::HeroCentered.build(&tokens),
        SectionPreset::PortfolioGallery.build(&tokens),
        SectionPreset::Testimonials.build(&tokens),
        SectionPreset::CtaBanner.build(&tokens),
        SectionPreset::FooterStrip.build(&tokens),
    ];
    customize_navbar(&mut home, "MONOLITH", "Start Project", "contact.html");

    let mut services = Page::new("Services", "services");
    services.sections = vec![
        SectionPreset::NavbarStrip.build(&tokens),
        SectionPreset::FeaturesGrid.build(&tokens),
        SectionPreset::PricingTiers.build(&tokens),
        SectionPreset::FooterStrip.build(&tokens),
    ];
    customize_navbar(&mut services, "MONOLITH", "Start Project", "contact.html");

    let mut about = Page::new("About", "about");
    about.sections = vec![
        SectionPreset::NavbarStrip.build(&tokens),
        SectionPreset::AboutSplit.build(&tokens),
        SectionPreset::FaqAccordion.build(&tokens),
        SectionPreset::FooterStrip.build(&tokens),
    ];
    customize_navbar(&mut about, "MONOLITH", "Start Project", "contact.html");

    let mut contact = Page::new("Contact", "contact");
    contact.sections = vec![
        SectionPreset::NavbarStrip.build(&tokens),
        SectionPreset::ContactStrip.build(&tokens),
        SectionPreset::FooterStrip.build(&tokens),
    ];
    customize_navbar(&mut contact, "MONOLITH", "Back Home", "index.html");

    project.pages.extend([home, services, about, contact]);
    project
}

fn build_bio(tokens: DesignTokens) -> SiteProject {
    let mut project = SiteProject::new("Personal Bio", tokens.clone());
    project.pages.clear();

    let mut home = Page::new("Home", "index");
    home.meta_description = "Independent engineer & product designer.".into();
    home.sections = vec![
        SectionPreset::HeroSplit.build(&tokens),
        SectionPreset::AboutSplit.build(&tokens),
        SectionPreset::ContactStrip.build(&tokens),
        SectionPreset::FooterStrip.build(&tokens),
    ];

    project.pages.push(home);
    project
}

fn build_blank(tokens: DesignTokens) -> SiteProject {
    let mut project = SiteProject::new("Untitled Site", tokens.clone());
    project.pages.clear();

    let mut home = Page::new("Home", "index");
    home.sections = vec![
        SectionPreset::NavbarStrip.build(&tokens),
        SectionPreset::BlankSection.build(&tokens),
        SectionPreset::FooterStrip.build(&tokens),
    ];
    project.pages.push(home);
    project
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_templates_and_section_presets_construct_validly() {
        for tmpl in StarterTemplate::all() {
            let proj = tmpl.build();
            assert!(!proj.pages.is_empty());
            assert_eq!(proj.pages[0].slug, "index");
            assert!(!proj.pages[0].sections.is_empty());

            // Verify build_with_theme works cleanly for every preset
            let themed = tmpl.build_with_theme(PresetTheme::PureEditorial, Some("Custom Brand"));
            assert_eq!(themed.theme.preset, PresetTheme::PureEditorial);
            assert_eq!(themed.name, "Custom Brand");
        }

        let tokens = DesignTokens::default();
        for preset in SectionPreset::all() {
            let sec = preset.build(&tokens);
            assert!(sec.height > 0.0);
        }
    }
}
