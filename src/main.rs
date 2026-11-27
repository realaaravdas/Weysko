#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::thread;
use std::time::Duration;

use weysko::generator::exporter::{
    export_to_folder, export_to_single_file, export_to_weysko_file, export_to_zip,
};
use weysko::generator::{compile_site, ExportOptions};
use weysko::model::SiteProject;
use weysko::preview::PreviewServer;
use weysko::studio::WeyskoApp;
use weysko::templates::StarterTemplate;

#[cfg(all(windows, not(debug_assertions)))]
fn attach_parent_console_if_cli() {
    extern "system" {
        fn AttachConsole(dwProcessId: u32) -> i32;
    }
    unsafe {
        let _ = AttachConsole(0xFFFF_FFFF);
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        run_gui_home();
        return;
    }

    #[cfg(all(windows, not(debug_assertions)))]
    attach_parent_console_if_cli();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }

    // CLI: --init-template <key> --out <file.weysko>
    if let Some(idx) = args.iter().position(|a| a == "--init-template") {
        let key = args.get(idx + 1).map(String::as_str).unwrap_or("portfolio");
        let out_path = find_flag(&args, "--out").unwrap_or_else(|| "site.weysko".into());
        let tmpl = StarterTemplate::from_cli_key(key).unwrap_or_else(|| {
            eprintln!(
                "Unknown template '{}'. Valid: meridian, portfolio, saas, agency, bio, blank",
                key
            );
            process::exit(1);
        });
        let project = tmpl.build();
        let out_pb = PathBuf::from(&out_path);
        if out_pb
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("json"))
            .unwrap_or(false)
        {
            let json = project.to_json_pretty().expect("serialize template");
            fs::write(&out_pb, json).expect("write project file");
        } else {
            export_to_weysko_file(&project, &out_pb).expect("write .weysko project");
        }
        println!("Created {} from template '{}'", out_path, tmpl.label());
        return;
    }

    // CLI: --export <source> --out <path> [--format <folder|zip|single-file|google-sites|weysko>]
    if let Some(src) = find_flag(&args, "--export") {
        let project = load_project_source(&src);
        let out_path = PathBuf::from(
            find_flag(&args, "--out").unwrap_or_else(|| "./weysko-dist".into()),
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
            "weysko" | "project" => {
                let summary =
                    export_to_weysko_file(&project, &out_path).expect("export .weysko file");
                println!(
                    "Exported .weysko project ({} bytes) to {}",
                    summary.total_bytes,
                    summary.destination.display()
                );
            }
            other => {
                eprintln!(
                    "Unknown format '{}'. Valid: folder, zip, single-file, google-sites, weysko",
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
        println!(
            "Weysko live preview serving '{}' at {}",
            project.name,
            server.url()
        );
        println!("Press Ctrl+C to stop.");
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    }

    // Positional path to .weysko or .weysko.json opens GUI directly in the Editor
    let path = PathBuf::from(&args[0]);
    if path.exists() {
        let project = load_project_source(&args[0]);
        run_gui_project(project, path);
    } else {
        eprintln!("File not found: {}", args[0]);
        print_help();
        process::exit(1);
    }
}

fn native_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Weysko — Pure-Rust Visual Website Studio")
            .with_inner_size([1280.0, 780.0])
            .with_min_inner_size([680.0, 480.0])
            .with_icon(weysko::studio::branding::app_icon_data()),
        ..Default::default()
    }
}

fn run_gui_home() {
    let _ = eframe::run_native(
        "Weysko",
        native_options(),
        Box::new(|_cc| Ok(Box::new(WeyskoApp::new_home()))),
    );
}

fn run_gui_project(project: SiteProject, file_path: PathBuf) {
    let _ = eframe::run_native(
        "Weysko",
        native_options(),
        Box::new(move |_cc| {
            let mut app = WeyskoApp::new(project.clone());
            app.state.current_file_path = Some(file_path.clone());
            app.state.recent_projects.record(&project, &file_path);
            app.state.recent_projects.save();
            Ok(Box::new(app))
        }),
    );
}

fn load_project_source(src: &str) -> SiteProject {
    if let Some(key) = src.strip_prefix("template:") {
        return StarterTemplate::from_cli_key(key)
            .unwrap_or_else(|| {
                eprintln!(
                    "Unknown template '{}'. Valid: meridian, portfolio, saas, agency, bio, blank",
                    key
                );
                process::exit(1);
            })
            .build();
    }
    let raw = fs::read_to_string(src).unwrap_or_else(|e| {
        eprintln!("Failed to read '{}': {}", src, e);
        process::exit(1);
    });
    SiteProject::from_weysko_or_json(&raw).unwrap_or_else(|e| {
        eprintln!("Invalid Weysko project '{}': {}", src, e);
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
        r#"Weysko — Pure-Rust, Zero-AI Visual Website Studio

USAGE:
  weysko                                      Launch Start screen & visual studio GUI
  weysko <project.weysko>                     Open .weysko or .weysko.json project in studio GUI
  weysko --init-template <key> --out <file>   Create a starter .weysko project file
  weysko --export <src> --out <path> [--format <folder|zip|single-file|google-sites|weysko>]
  weysko --serve <src> [--port <8080>]        Serve compiled site over localhost

SOURCES (<src>):
  <file.weysko | file.weysko.json> or template:<meridian|portfolio|saas|agency|bio|blank>
"#
    );
}
