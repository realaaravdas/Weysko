use serde::{Deserialize, Serialize};

/// Site-wide color palette stored as CSS hex strings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorPalette {
    pub background: String,
    pub surface: String,
    pub surface_alt: String,
    pub primary: String,
    pub primary_text: String,
    pub accent: String,
    pub text: String,
    pub text_muted: String,
    pub border: String,
}

/// Global typography tokens for generated pages.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypographyTokens {
    pub heading_font: String,
    pub body_font: String,
    pub mono_font: String,
    pub use_google_fonts: bool,
    pub base_font_size: f32,
    pub line_height: f32,
    pub heading_weight: u16,
    pub letter_spacing: f32,
}

/// Curated zero-AI preset themes for instant site restyling.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PresetTheme {
    ObsidianSlate,
    PureEditorial,
    MidnightMono,
    NordicSteel,
    WarmAlabaster,
    CarbonInk,
}

impl PresetTheme {
    /// Returns all available curated preset themes.
    pub fn all() -> &'static [PresetTheme] {
        &[
            PresetTheme::ObsidianSlate,
            PresetTheme::PureEditorial,
            PresetTheme::MidnightMono,
            PresetTheme::NordicSteel,
            PresetTheme::WarmAlabaster,
            PresetTheme::CarbonInk,
        ]
    }

    /// Human-readable display name.
    pub fn label(&self) -> &'static str {
        match self {
            PresetTheme::ObsidianSlate => "Obsidian Slate",
            PresetTheme::PureEditorial => "Pure Editorial",
            PresetTheme::MidnightMono => "Midnight Mono",
            PresetTheme::NordicSteel => "Nordic Steel",
            PresetTheme::WarmAlabaster => "Warm Alabaster",
            PresetTheme::CarbonInk => "Carbon Ink",
        }
    }

    /// Short description of the preset aesthetic.
    pub fn description(&self) -> &'static str {
        match self {
            PresetTheme::ObsidianSlate => "Deep slate-950 backdrop with crisp white typography",
            PresetTheme::PureEditorial => "High-contrast paper white with deep charcoal ink",
            PresetTheme::MidnightMono => "Minimalist obsidian dark with monospace clarity",
            PresetTheme::NordicSteel => "Cool glacial slate surfaces and sharp contrast",
            PresetTheme::WarmAlabaster => "Soft architectural stone white and slate ink",
            PresetTheme::CarbonInk => "Ultra-dark carbon surfaces with stark white accents",
        }
    }

    /// Builds a complete `DesignTokens` configuration for this preset.
    pub fn to_tokens(&self) -> DesignTokens {
        match self {
            PresetTheme::ObsidianSlate => DesignTokens {
                preset: *self,
                palette: ColorPalette {
                    background: "#090D16".into(),
                    surface: "#0F172A".into(),
                    surface_alt: "#1E293B".into(),
                    primary: "#FFFFFF".into(),
                    primary_text: "#0F172A".into(),
                    accent: "#CBD5E1".into(),
                    text: "#F8FAFC".into(),
                    text_muted: "#94A3B8".into(),
                    border: "#334155".into(),
                },
                typography: TypographyTokens {
                    heading_font: "Inter".into(),
                    body_font: "Inter".into(),
                    mono_font: "JetBrains Mono".into(),
                    use_google_fonts: true,
                    base_font_size: 16.0,
                    line_height: 1.6,
                    heading_weight: 700,
                    letter_spacing: -0.02,
                },
                content_max_width: 1200.0,
                border_radius: 10.0,
                smooth_scroll: true,
                custom_global_css: String::new(),
            },
            PresetTheme::PureEditorial => DesignTokens {
                preset: *self,
                palette: ColorPalette {
                    background: "#FFFFFF".into(),
                    surface: "#F8FAFC".into(),
                    surface_alt: "#F1F5F9".into(),
                    primary: "#0F172A".into(),
                    primary_text: "#FFFFFF".into(),
                    accent: "#334155".into(),
                    text: "#0F172A".into(),
                    text_muted: "#475569".into(),
                    border: "#E2E8F0".into(),
                },
                typography: TypographyTokens {
                    heading_font: "Plus Jakarta Sans".into(),
                    body_font: "Inter".into(),
                    mono_font: "JetBrains Mono".into(),
                    use_google_fonts: true,
                    base_font_size: 16.0,
                    line_height: 1.65,
                    heading_weight: 700,
                    letter_spacing: -0.02,
                },
                content_max_width: 1180.0,
                border_radius: 8.0,
                smooth_scroll: true,
                custom_global_css: String::new(),
            },
            PresetTheme::MidnightMono => DesignTokens {
                preset: *self,
                palette: ColorPalette {
                    background: "#0B0F17".into(),
                    surface: "#111827".into(),
                    surface_alt: "#1F2937".into(),
                    primary: "#F9FAFB".into(),
                    primary_text: "#111827".into(),
                    accent: "#9CA3AF".into(),
                    text: "#F3F4F6".into(),
                    text_muted: "#9CA3AF".into(),
                    border: "#374151".into(),
                },
                typography: TypographyTokens {
                    heading_font: "Space Grotesk".into(),
                    body_font: "Inter".into(),
                    mono_font: "IBM Plex Mono".into(),
                    use_google_fonts: true,
                    base_font_size: 16.0,
                    line_height: 1.6,
                    heading_weight: 600,
                    letter_spacing: -0.02,
                },
                content_max_width: 1200.0,
                border_radius: 6.0,
                smooth_scroll: true,
                custom_global_css: String::new(),
            },
            PresetTheme::NordicSteel => DesignTokens {
                preset: *self,
                palette: ColorPalette {
                    background: "#F1F5F9".into(),
                    surface: "#FFFFFF".into(),
                    surface_alt: "#E2E8F0".into(),
                    primary: "#1E293B".into(),
                    primary_text: "#FFFFFF".into(),
                    accent: "#475569".into(),
                    text: "#0F172A".into(),
                    text_muted: "#64748B".into(),
                    border: "#CBD5E1".into(),
                },
                typography: TypographyTokens {
                    heading_font: "DM Sans".into(),
                    body_font: "DM Sans".into(),
                    mono_font: "JetBrains Mono".into(),
                    use_google_fonts: true,
                    base_font_size: 16.0,
                    line_height: 1.6,
                    heading_weight: 700,
                    letter_spacing: -0.01,
                },
                content_max_width: 1200.0,
                border_radius: 12.0,
                smooth_scroll: true,
                custom_global_css: String::new(),
            },
            PresetTheme::WarmAlabaster => DesignTokens {
                preset: *self,
                palette: ColorPalette {
                    background: "#FAF9F6".into(),
                    surface: "#FFFFFF".into(),
                    surface_alt: "#F3F1EC".into(),
                    primary: "#18181B".into(),
                    primary_text: "#FAFAFA".into(),
                    accent: "#52525B".into(),
                    text: "#18181B".into(),
                    text_muted: "#71717A".into(),
                    border: "#E4E4E7".into(),
                },
                typography: TypographyTokens {
                    heading_font: "Instrument Serif".into(),
                    body_font: "Inter".into(),
                    mono_font: "JetBrains Mono".into(),
                    use_google_fonts: true,
                    base_font_size: 16.0,
                    line_height: 1.65,
                    heading_weight: 600,
                    letter_spacing: -0.01,
                },
                content_max_width: 1140.0,
                border_radius: 8.0,
                smooth_scroll: true,
                custom_global_css: String::new(),
            },
            PresetTheme::CarbonInk => DesignTokens {
                preset: *self,
                palette: ColorPalette {
                    background: "#05070B".into(),
                    surface: "#0D1117".into(),
                    surface_alt: "#161B22".into(),
                    primary: "#FFFFFF".into(),
                    primary_text: "#05070B".into(),
                    accent: "#E2E8F0".into(),
                    text: "#F8FAFC".into(),
                    text_muted: "#8B949E".into(),
                    border: "#30363D".into(),
                },
                typography: TypographyTokens {
                    heading_font: "Syne".into(),
                    body_font: "Inter".into(),
                    mono_font: "JetBrains Mono".into(),
                    use_google_fonts: true,
                    base_font_size: 16.0,
                    line_height: 1.6,
                    heading_weight: 700,
                    letter_spacing: -0.02,
                },
                content_max_width: 1240.0,
                border_radius: 4.0,
                smooth_scroll: true,
                custom_global_css: String::new(),
            },
        }
    }
}

/// Global site design tokens controlling colors, typography, layout, and custom CSS.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DesignTokens {
    pub preset: PresetTheme,
    pub palette: ColorPalette,
    pub typography: TypographyTokens,
    pub content_max_width: f32,
    pub border_radius: f32,
    pub smooth_scroll: bool,
    pub custom_global_css: String,
}

impl Default for DesignTokens {
    fn default() -> Self {
        PresetTheme::ObsidianSlate.to_tokens()
    }
}

/// Curated Google / system font options presented in theme & element selectors.
pub fn available_font_families() -> &'static [&'static str] {
    &[
        "Inter",
        "Plus Jakarta Sans",
        "Space Grotesk",
        "DM Sans",
        "Syne",
        "Instrument Serif",
        "Manrope",
        "Outfit",
        "IBM Plex Mono",
        "JetBrains Mono",
        "System Sans",
        "System Serif",
    ]
}
