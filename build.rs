fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        // Only set icon if it exists to avoid build failure
        if std::path::Path::new("assets/logo.ico").exists() {
            res.set_icon("assets/logo.ico");
        }
        res.set("ProductName", "MetaLens");
        res.set("FileDescription", "MetaLens — Deep Metadata Analyzer");
        res.set("LegalCopyright", "Copyright © 2024 Hamza-op");
        let _ = res.compile();
    }
}
