fn main() {
    println!(
        "cargo:rustc-env=API_URL={}",
        std::env::var("API_URL").unwrap_or_default()
    );
    tauri_build::build()
}
