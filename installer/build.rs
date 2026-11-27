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

    let ico_path = repo_root.join("assets").join("websir.ico");
    let readme_path = repo_root.join("README.md");
    let license_path = repo_root.join("LICENSE");
    let release_exe = repo_root.join("target").join("release").join("websir.exe");
    let debug_exe = repo_root.join("target").join("debug").join("websir.exe");

    println!("cargo:rerun-if-changed={}", ico_path.display());
    println!("cargo:rerun-if-changed={}", readme_path.display());
    println!("cargo:rerun-if-changed={}", license_path.display());
    println!("cargo:rerun-if-changed={}", release_exe.display());

    #[cfg(windows)]
    if ico_path.exists() {
        let mut res = winresource::WindowsResource::new();
        res.set_icon(ico_path.to_str().unwrap());
        res.set("ProductName", "Websir Visual Website Studio Setup");
        res.set("FileDescription", "Websir v1.1.0 Installer");
        res.set("LegalCopyright", "Copyright (c) 2026 Aarav Das");
        res.set("OriginalFilename", "websir-setup.exe");
        res.compile().expect("failed to compile installer resources");
    }

    let payload_zip = out_dir.join("websir-payload.zip");
    let file = File::create(&payload_zip).expect("create websir-payload.zip");
    let mut writer = ZipWriter::new(file);
    let opts = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    let exe_bytes = if release_exe.exists() {
        fs::read(&release_exe).expect("read target/release/websir.exe")
    } else if profile == "release" {
        panic!(
            "target/release/websir.exe not found! Build the main app first: cargo build --release -p websir"
        );
    } else if debug_exe.exists() {
        fs::read(&debug_exe).unwrap_or_default()
    } else {
        Vec::new()
    };

    writer.start_file("websir.exe", opts).unwrap();
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
        writer.start_file("websir.ico", opts).unwrap();
        writer.write_all(&fs::read(&ico_path).unwrap()).unwrap();
    }

    writer.finish().unwrap();
}
