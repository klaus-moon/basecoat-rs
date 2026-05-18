use std::path::PathBuf;
fn main() {
    println!("cargo:rerun-if-changed=../../style/basecoat.css");
    let workspace_src = PathBuf::from("../../style/basecoat.css");
    let local_asset = PathBuf::from("assets/basecoat.css");
    if workspace_src.exists() {
        std::fs::create_dir_all("assets").ok();
        std::fs::copy(&workspace_src, &local_asset).ok();
    }
    // Re-emit to OUT_DIR so downstream build scripts can locate it.
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(&out_dir).join("basecoat.css");
    if local_asset.exists() {
        std::fs::copy(&local_asset, &out_path).ok();
    }
    println!("cargo:path={}", out_path.display());
}
