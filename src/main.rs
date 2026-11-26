use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::thread;
use std::time::Duration;

use websir::generator::exporter::{
    export_to_folder, export_to_single_file, export_to_zip,
};
use websir::generator::{compile_site, ExportOptions};
use websir::model::SiteProject;
use websir::preview::PreviewServer;
use websir::studio::WebsirApp;
use websir::templates::StarterTemplate;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        run_gui(StarterTemplate::MinimalistPortfolio.build(), None);
        return;
    }

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }

    // CLI: --init-template <key> --out <file.websir.json>
    if let Some(idx) = args.iter().position(|a| a == "--init-template") {
        let key = args.get(idx + 1).map(String::as_str).unwrap_or("portfolio");
        let out_path = find_flag(&args, "--out").unwrap_or_else(|| "site.websir.json".into());
        let tmpl = StarterTemplate::from_cli_key(key).unwrap_or_else(|| {
            eprintln!("Unknown template '{}'. Valid: portfolio, saas, agency, bio, blank", key);
            process::exit(1);
        });
        let project = tmpl.build();
        let json = project.to_json_pretty().expect("serialize template");
        fs::write(&out_path, json).expect("write project file");
        println!("Created {} from template '{}'", out_path, tmpl.label());
        return;
    }

    // CLI: --export <source> --out <path> [--format <folder|zip|single-file|google-sites>]
    if let Some(src) = find_flag(&args, "--export") {
        let project = load_project_source(&src);
        let out_path = PathBuf::from(
            find_flag(&args, "--out").unwrap_or_else(|| "./websir-dist".into()),
        );
        let fmt = find_flag(&args, "--format").unwrap_or_else(|| "folder".into());
        let opts = ExportOptions::default();

        match fmt.to_ascii_lowercase().as_str() {
            "folder" | "static" => {
                let summary = export_to_folder(&project, &out_path, &opts)
                    .expect("export to folder");
                println!(
                    "Exported {} files ({} bytes) to {}",
                    summary.file_count,
                    summary.total_bytes,
                    summary.destination.display()
                );
            }
            "zip" => {
                let summary =
                    export_to_zip(&project, &out_path, &opts).expect("export to zip");
                println!(
                    "Exported ZIP with {} files ({} bytes) to {}",
                    summary.file_count,
                    summary.total_bytes,
                    summary.destination.display()
                );
            }
            "single-file" | "single" | "google-sites" => {
                let summary =
                    export_to_single_file(&project, &out_path).expect("export single-file");
                println!(
                    "Exported single-file HTML ({} bytes) to {}",
                    summary.total_bytes,
                    summary.destination.display()
                );
            }
            other => {
                eprintln!(
                    "Unknown format '{}'. Valid: folder, zip, single-file, google-sites",
                    other
                );
                process::exit(1);
            }
        }
        return;
    }

    // CLI: --serve <source> [--port <8080>]
    if let Some(src) = find_flag(&args, "--serve") {
        let project = load_project_source(&src);
        let port = find_flag(&args, "--port").and_then(|p| p.parse::<u16>().ok());
        let compiled = compile_site(&project);
        let server = PreviewServer::start(compiled, port).expect("start preview server");
        println!("Websir live preview serving '{}' at {}", project.name, server.url());
        println!("Press Ctrl+C to stop.");
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    }

    // Positional path to .websir.json opens GUI with that project
    let path = PathBuf::from(&args[0]);
    if path.exists() {
        let project = load_project_source(&args[0]);
        run_gui(project, Some(path));
    } else {
        eprintln!("File not found: {}", args[0]);
        print_help();
        process::exit(1);
    }
}

fn run_gui(project: SiteProject, file_path: Option<PathBuf>) {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Websir — Pure-Rust Visual Website Studio")
            .with_inner_size([1440.0, 900.0])
            .with_min_inner_size([1060.0, 680.0])
            .with_icon(websir::studio::branding::app_icon_data()),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Websir",
        options,
        Box::new(move |_cc| {
            let mut app = WebsirApp::new(project);
            app.state.current_file_path = file_path;
            Ok(Box::new(app))
        }),
    );
}

fn load_project_source(src: &str) -> SiteProject {
    if let Some(key) = src.strip_prefix("template:") {
        return StarterTemplate::from_cli_key(key)
            .unwrap_or_else(|| {
                eprintln!("Unknown template '{}'. Valid: portfolio, saas, agency, bio, blank", key);
                process::exit(1);
            })
            .build();
    }
    let raw = fs::read_to_string(src).unwrap_or_else(|e| {
        eprintln!("Failed to read '{}': {}", src, e);
        process::exit(1);
    });
    SiteProject::from_json(&raw).unwrap_or_else(|e| {
        eprintln!("Invalid Websir project JSON '{}': {}", src, e);
        process::exit(1);
    })
}

fn find_flag(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn print_help() {
    println!(
        r#"Websir — Pure-Rust, Zero-AI Visual Website Studio

USAGE:
  websir                                      Launch visual studio GUI
  websir <project.websir.json>                Open project in studio GUI
  websir --init-template <key> --out <file>   Create a starter project JSON
  websir --export <src> --out <path> [--format <folder|zip|single-file|google-sites>]
  websir --serve <src> [--port <8080>]        Serve compiled site over localhost

SOURCES (<src>):
  <file.websir.json> or template:<portfolio|saas|agency|bio|blank>
"#
    );
}
