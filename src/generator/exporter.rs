use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

use crate::model::SiteProject;

use super::{compile_site, ExportOptions};

/// Summary returned after exporting a site.
#[derive(Debug, Clone)]
pub struct ExportSummary {
    pub destination: PathBuf,
    pub file_count: usize,
    pub total_bytes: usize,
}

/// Exports the compiled multi-page website into a directory on disk.
pub fn export_to_folder(
    project: &SiteProject,
    out_dir: &Path,
    options: &ExportOptions,
) -> io::Result<ExportSummary> {
    let mut site = compile_site(project);
    add_optional_meta_files(project, &mut site.files, options);

    fs::create_dir_all(out_dir)?;
    let mut total_bytes = 0usize;
    let mut file_count = 0usize;

    for (rel_path, bytes) in &site.files {
        let full_path = out_dir.join(rel_path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&full_path, bytes)?;
        total_bytes += bytes.len();
        file_count += 1;
    }

    Ok(ExportSummary {
        destination: out_dir.to_path_buf(),
        file_count,
        total_bytes,
    })
}

/// Exports the compiled multi-page website into a standard DEFLATE `.zip` archive.
pub fn export_to_zip(
    project: &SiteProject,
    zip_path: &Path,
    options: &ExportOptions,
) -> io::Result<ExportSummary> {
    let mut site = compile_site(project);
    add_optional_meta_files(project, &mut site.files, options);

    if let Some(parent) = zip_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    let file = fs::File::create(zip_path)?;
    let mut writer = ZipWriter::new(file);
    let zip_opts = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    let mut total_bytes = 0usize;
    let mut file_count = 0usize;

    for (rel_path, bytes) in &site.files {
        writer
            .start_file(rel_path, zip_opts)
            .map_err(io::Error::other)?;
        writer.write_all(bytes)?;
        total_bytes += bytes.len();
        file_count += 1;
    }

    writer
        .finish()
        .map_err(io::Error::other)?;

    Ok(ExportSummary {
        destination: zip_path.to_path_buf(),
        file_count,
        total_bytes,
    })
}

/// Exports a self-contained single-file HTML bundle (inline CSS, JS, images, and SPA routing).
pub fn export_to_single_file(project: &SiteProject, html_path: &Path) -> io::Result<ExportSummary> {
    let site = compile_site(project);
    if let Some(parent) = html_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    let bytes = site.single_file_html.as_bytes();
    fs::write(html_path, bytes)?;
    Ok(ExportSummary {
        destination: html_path.to_path_buf(),
        file_count: 1,
        total_bytes: bytes.len(),
    })
}

/// Produces self-contained HTML ready to paste into Google Sites (`Insert -> Embed -> Embed code`).
pub fn generate_google_sites_embed(project: &SiteProject) -> String {
    compile_site(project).single_file_html
}

fn add_optional_meta_files(
    project: &SiteProject,
    files: &mut std::collections::BTreeMap<String, Vec<u8>>,
    options: &ExportOptions,
) {
    let base = project.base_url.trim_end_matches('/');
    if options.include_sitemap {
        let mut sitemap = String::from(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
        );
        for page in &project.pages {
            let loc = if page.slug == "index" {
                format!("{}/", base)
            } else {
                format!("{}/{}", base, page.html_filename())
            };
            sitemap.push_str(&format!("  <url><loc>{}</loc></url>\n", loc));
        }
        sitemap.push_str("</urlset>\n");
        files.insert("sitemap.xml".to_string(), sitemap.into_bytes());
    }

    if options.include_robots {
        let robots = format!("User-agent: *\nAllow: /\nSitemap: {}/sitemap.xml\n", base);
        files.insert("robots.txt".to_string(), robots.into_bytes());
    }

    if options.include_nojekyll {
        files.insert(".nojekyll".to_string(), Vec::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AssetItem, ElementKind};
    use crate::templates::StarterTemplate;
    use std::io::Read;

    #[test]
    fn multi_page_folder_zip_single_file_and_xss_safety() {
        let mut project = StarterTemplate::SaasLanding.build();
        // Inject an asset and an XSS payload in heading text to verify escaping.
        let asset = AssetItem::from_bytes("logo.png", "image/png", &[137, 80, 78, 71, 13, 10, 26, 10]);
        let asset_id = asset.id.clone();
        project.assets.push(asset);

        if let Some(el) = project.pages[0]
            .sections
            .iter_mut()
            .flat_map(|s| s.elements.iter_mut())
            .find(|e| e.kind == ElementKind::Heading)
        {
            el.content.text = "<script>alert('xss')</script> & Slate".into();
        }
        if let Some(sec) = project.pages[0].sections.get_mut(1) {
            let mut img = crate::model::Element::new_default(ElementKind::Image, &project.theme, 99);
            img.content.asset_id = Some(asset_id);
            sec.elements.push(img);
        }

        let tmp = std::env::temp_dir().join(format!("websir_test_{}", uuid::Uuid::new_v4()));
        let folder_out = tmp.join("site_folder");
        let zip_out = tmp.join("site.zip");
        let single_out = tmp.join("single.html");

        let opts = ExportOptions::default();
        let folder_summary = export_to_folder(&project, &folder_out, &opts).unwrap();
        assert!(folder_summary.file_count >= 7);
        assert!(folder_out.join("index.html").exists());
        assert!(folder_out.join("pricing.html").exists());
        assert!(folder_out.join("contact.html").exists());
        assert!(folder_out.join("styles.css").exists());
        assert!(folder_out.join("site.js").exists());
        assert!(folder_out.join("sitemap.xml").exists());
        assert!(folder_out.join("robots.txt").exists());
        assert!(folder_out.join(".nojekyll").exists());

        let index_html = fs::read_to_string(folder_out.join("index.html")).unwrap();
        assert!(!index_html.contains("<script>alert('xss')</script>"));
        assert!(index_html.contains("&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt; &amp; Slate"));

        // Verify ZIP archive opens cleanly
        export_to_zip(&project, &zip_out, &opts).unwrap();
        let zip_file = fs::File::open(&zip_out).unwrap();
        let mut archive = zip::ZipArchive::new(zip_file).unwrap();
        assert!(archive.by_name("index.html").is_ok());
        assert!(archive.by_name("styles.css").is_ok());
        let mut pricing_entry = archive.by_name("pricing.html").unwrap();
        let mut pricing_str = String::new();
        pricing_entry.read_to_string(&mut pricing_str).unwrap();
        assert!(pricing_str.contains("Pricing"));

        // Verify Single-file / Google Sites embed
        export_to_single_file(&project, &single_out).unwrap();
        let embed = generate_google_sites_embed(&project);
        assert!(embed.contains("<style>"));
        assert!(embed.contains("<script>"));
        assert!(embed.contains("data:image/png;base64,"));
        assert!(embed.contains("data-ws-page=\"pricing\""));
        assert!(!embed.contains("href=\"pricing.html\""));

        let _ = fs::remove_dir_all(&tmp);
    }
}
