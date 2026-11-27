pub mod sections;

pub use sections::SectionPreset;

use crate::model::{DesignTokens, ElementKind, Page, PresetTheme, SiteProject};

/// Complete multi-page or single-page zero-AI starter website templates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarterTemplate {
    MinimalistPortfolio,
    SaasLanding,
    StudioAgency,
    PersonalBio,
    BlankCanvas,
}

impl StarterTemplate {
    pub fn all() -> &'static [StarterTemplate] {
        &[
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
            StarterTemplate::StudioAgency => "agency",
            StarterTemplate::PersonalBio => "bio",
            StarterTemplate::BlankCanvas => "blank",
        }
    }

    pub fn from_cli_key(key: &str) -> Option<Self> {
        match key.trim().to_ascii_lowercase().as_str() {
            "portfolio" | "minimalist-portfolio" => Some(StarterTemplate::MinimalistPortfolio),
            "saas" | "saas-landing" => Some(StarterTemplate::SaasLanding),
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

    /// Creates a brand new `SiteProject` from this template.
    pub fn build(&self) -> SiteProject {
        match self {
            StarterTemplate::MinimalistPortfolio => build_portfolio(),
            StarterTemplate::SaasLanding => build_saas(),
            StarterTemplate::StudioAgency => build_agency(),
            StarterTemplate::PersonalBio => build_bio(),
            StarterTemplate::BlankCanvas => build_blank(),
        }
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

fn build_portfolio() -> SiteProject {
    let tokens = PresetTheme::ObsidianSlate.to_tokens();
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

fn build_saas() -> SiteProject {
    let tokens = PresetTheme::ObsidianSlate.to_tokens();
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

fn build_agency() -> SiteProject {
    let tokens = DesignTokens::default();
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

fn build_bio() -> SiteProject {
    let tokens = PresetTheme::CarbonInk.to_tokens();
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

fn build_blank() -> SiteProject {
    let tokens = PresetTheme::ObsidianSlate.to_tokens();
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
        }

        let tokens = DesignTokens::default();
        for preset in SectionPreset::all() {
            let sec = preset.build(&tokens);
            assert!(sec.height > 0.0);
        }
    }
}
