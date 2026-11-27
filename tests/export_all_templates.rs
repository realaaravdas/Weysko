use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;

use weysko::generator::exporter::{
    export_to_folder, export_to_single_file, export_to_weysko_file, export_to_zip,
    generate_google_sites_embed,
};
use weysko::generator::{compile_site, ExportOptions};
use weysko::model::{Element, ElementKind, PresetTheme, SiteProject};
use weysko::preview::PreviewServer;
use weysko::templates::StarterTemplate;

#[test]
fn all_starter_templates_export_and_serve_cleanly() {
    let tmp_root =
        std::env::temp_dir().join(format!("weysko_all_templates_{}", uuid::Uuid::new_v4()));

    for tmpl in StarterTemplate::all() {
        let mut project = tmpl.build();
        // Also ensure every preset theme applies cleanly before compiling.
        for preset in PresetTheme::all() {
            project.theme = preset.to_tokens();
            let compiled = compile_site(&project);
            assert!(compiled.files.contains_key("index.html"));
            assert!(compiled.files.contains_key("styles.css"));
            assert!(compiled.files.contains_key("site.js"));
            assert!(compiled.single_file_html.contains("<!DOCTYPE html>"));
        }

        // Restore template default and test every export format on disk.
        let project = tmpl.build();
        let key = tmpl.cli_key();
        let opts = ExportOptions::default();

        let folder = tmp_root.join(format!("{}_folder", key));
        let zip_path = tmp_root.join(format!("{}.zip", key));
        let single_path = tmp_root.join(format!("{}.html", key));
        let weysko_path = tmp_root.join(format!("{}.weysko", key));

        let f_sum = export_to_folder(&project, &folder, &opts).unwrap();
        assert!(f_sum.file_count >= 5);
        assert!(folder.join("index.html").exists());
        assert!(folder.join("styles.css").exists());

        let z_sum = export_to_zip(&project, &zip_path, &opts).unwrap();
        assert!(z_sum.total_bytes > 0);
        assert!(zip_path.exists());

        let s_sum = export_to_single_file(&project, &single_path).unwrap();
        assert!(s_sum.total_bytes > 0);
        assert!(single_path.exists());

        let w_sum = export_to_weysko_file(&project, &weysko_path).unwrap();
        assert_eq!(w_sum.file_count, 1);
        let raw = fs::read_to_string(&weysko_path).unwrap();
        let roundtrip = SiteProject::from_weysko_or_json(&raw).unwrap();
        assert_eq!(roundtrip.name, project.name);
        assert_eq!(roundtrip.pages.len(), project.pages.len());

        let embed = generate_google_sites_embed(&project);
        assert!(embed.contains("<style>"));
        assert!(embed.contains("<script>"));
    }

    // Verify Meridian Cloud example includes embedded PNG asset and 3 pages
    let meridian = StarterTemplate::MeridianDemo.build();
    assert_eq!(meridian.pages.len(), 3);
    assert!(!meridian.assets.is_empty());

    // Verify all 16 element kinds compile cleanly on a custom page and serve over HTTP.
    let mut custom = StarterTemplate::BlankCanvas.build();
    for (idx, kind) in ElementKind::all().iter().enumerate() {
        let el = Element::new_default(*kind, &custom.theme, (idx + 10) as i32);
        custom.pages[0].sections[1].elements.push(el);
    }
    let site = compile_site(&custom);
    let server = PreviewServer::start(site, None).unwrap();
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", server.port())).unwrap();
    stream
        .write_all(b"GET /index.html HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains("ws-form"));
    assert!(response.contains("ws-acc"));
    server.stop();

    let _ = fs::remove_dir_all(&tmp_root);
}
