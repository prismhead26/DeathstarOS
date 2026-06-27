fn main() {
    tauri_build::build();

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-search=framework=/System/Library/PrivateFrameworks");
        println!("cargo:rustc-link-lib=framework=DisplayServices");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
    }
}
