use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir.parent().unwrap().to_path_buf();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

    let ico_path = repo_root.join("assets").join("weysko.ico");
    let readme_path = repo_root.join("README.md");
    let license_path = repo_root.join("LICENSE");
    let release_exe = repo_root.join("target").join("release").join("weysko.exe");
    let debug_exe = repo_root.join("target").join("debug").join("weysko.exe");

    println!("cargo:rerun-if-changed={}", ico_path.display());
    println!("cargo:rerun-if-changed={}", readme_path.display());
    println!("cargo:rerun-if-changed={}", license_path.display());
    println!("cargo:rerun-if-changed={}", release_exe.display());

    #[cfg(windows)]
    if ico_path.exists() {
        let mut res = winresource::WindowsResource::new();
        res.set_icon(ico_path.to_str().unwrap());
        res.set("ProductName", "Weysko Visual Website Studio Setup");
        res.set("FileDescription", "Weysko v1.3.1 Installer");
        res.set("LegalCopyright", "Copyright (c) 2026 Aarav Das");
        res.set("OriginalFilename", "weysko-setup.exe");
        res.compile().expect("failed to compile installer resources");
    }

    let payload_zip = out_dir.join("weysko-payload.zip");
    let file = File::create(&payload_zip).expect("create weysko-payload.zip");
    let mut writer = ZipWriter::new(file);
    let opts = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    let exe_bytes = if release_exe.exists() {
        fs::read(&release_exe).expect("read target/release/weysko.exe")
    } else if profile == "release" {
        panic!(
            "target/release/weysko.exe not found! Build the main app first: cargo build --release -p weysko"
        );
    } else if debug_exe.exists() {
        fs::read(&debug_exe).unwrap_or_default()
    } else {
        Vec::new()
    };

    writer.start_file("weysko.exe", opts).unwrap();
    writer.write_all(&exe_bytes).unwrap();

    if readme_path.exists() {
        writer.start_file("README.md", opts).unwrap();
        writer.write_all(&fs::read(&readme_path).unwrap()).unwrap();
    }
    if license_path.exists() {
        writer.start_file("LICENSE", opts).unwrap();
        writer.write_all(&fs::read(&license_path).unwrap()).unwrap();
    }
    if ico_path.exists() {
        writer.start_file("weysko.ico", opts).unwrap();
        writer.write_all(&fs::read(&ico_path).unwrap()).unwrap();
    }

    writer.finish().unwrap();
}
