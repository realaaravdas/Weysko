fn main() {
    #[cfg(windows)]
    {
        println!("cargo:rerun-if-changed=assets/websir.ico");
        if std::path::Path::new("assets/websir.ico").exists() {
            let mut res = winresource::WindowsResource::new();
            res.set_icon("assets/websir.ico");
            res.set("ProductName", "Websir Visual Website Studio");
            res.set("FileDescription", "Websir — Pure-Rust Visual Website Studio");
            res.set("LegalCopyright", "Copyright (c) 2026 Aarav Das");
            res.set("OriginalFilename", "websir.exe");
            res.compile().expect("failed to compile Windows resources");
        }
    }
}
