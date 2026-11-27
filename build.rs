fn main() {
    #[cfg(windows)]
    {
        println!("cargo:rerun-if-changed=assets/weysko.ico");
        if std::path::Path::new("assets/weysko.ico").exists() {
            let mut res = winresource::WindowsResource::new();
            res.set_icon("assets/weysko.ico");
            res.set("ProductName", "Weysko Visual Website Studio");
            res.set("FileDescription", "Weysko — Pure-Rust Visual Website Studio");
            res.set("LegalCopyright", "Copyright (c) 2026 Aarav Das");
            res.set("OriginalFilename", "weysko.exe");
            res.compile().expect("failed to compile Windows resources");
        }
    }
}
