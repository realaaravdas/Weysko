pub mod css;
pub mod exporter;
pub mod html;
pub mod js;

use std::collections::BTreeMap;

use crate::model::SiteProject;

/// Target export format supported by Websir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    StaticFolder,
    ZipArchive,
    SingleFileHtml,
    GoogleSitesEmbed,
}

impl ExportFormat {
    pub fn label(&self) -> &'static str {
        match self {
            ExportFormat::StaticFolder => "Static Folder",
            ExportFormat::ZipArchive => "ZIP Archive",
            ExportFormat::SingleFileHtml => "Single-File HTML",
            ExportFormat::GoogleSitesEmbed => "Google Sites Embed",
        }
    }
}

/// Configurable options when compiling and exporting a site.
#[derive(Debug, Clone)]
pub struct ExportOptions {
    pub format: ExportFormat,
    pub include_sitemap: bool,
    pub include_robots: bool,
    pub include_nojekyll: bool,
    pub minify_whitespace: bool,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            format: ExportFormat::StaticFolder,
            include_sitemap: true,
            include_robots: true,
            include_nojekyll: true,
            minify_whitespace: false,
        }
    }
}

/// Complete compiled output ready to serve or write to disk.
#[derive(Debug, Clone)]
pub struct GeneratedSite {
    pub files: BTreeMap<String, Vec<u8>>,
    pub single_file_html: String,
    pub entry_file: String,
}

/// Compiles a `SiteProject` into static HTML5, CSS3, vanilla JS, and asset files.
///
/// Always populates both `files` (for multi-page static hosting & preview server)
/// and `single_file_html` (self-contained HTML with inline CSS/JS/data-URIs).
pub fn compile_site(project: &SiteProject) -> GeneratedSite {
    let css_text = css::generate_css(project, false);
    let css_single = css::generate_css(project, true);
    let js_multi = js::generate_js(false);
    let js_single = js::generate_js(true);

    let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    files.insert("styles.css".to_string(), css_text.into_bytes());
    files.insert("site.js".to_string(), js_multi.into_bytes());

    // Emit decoded binary assets under assets/<sanitized_filename>.
    for asset in &project.assets {
        if let Ok(bytes) = asset.decoded_bytes() {
            files.insert(format!("assets/{}", asset.sanitized_filename()), bytes);
        }
    }

    // Emit each page as a standalone semantic HTML5 document.
    for page in &project.pages {
        let html_doc = html::render_multi_page_document(project, page);
        files.insert(page.html_filename(), html_doc.into_bytes());
    }

    // Emit clean 404 fallback page.
    let not_found_html = html::render_404_page(project);
    files.insert("404.html".to_string(), not_found_html.into_bytes());

    let single_file_html = html::render_single_file_document(project, &css_single, &js_single);
    let entry_file = project
        .pages
        .first()
        .map(|p| p.html_filename())
        .unwrap_or_else(|| "index.html".to_string());

    GeneratedSite {
        files,
        single_file_html,
        entry_file,
    }
}
