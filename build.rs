use std::env;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(rust_analyzer)");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("assets.rs");

    let mut code = String::new();

    code.push_str(
        r#"
        use bevy_asset_loader::asset_collection::AssetCollection;

        #[derive(AssetCollection, Resource)]
        pub struct ImageAssets {"#,
    );

    for entry in fs::read_dir("assets/block").unwrap() {
        code.push_str(&generate(entry.unwrap()));
    }

    for entry in fs::read_dir("assets/entity").unwrap() {
        code.push_str(&generate(entry.unwrap()));
    }

    code.push_str("\n}\n");

    fs::write(&dest_path, code).unwrap();

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets");
}

fn generate(entry: DirEntry) -> String {
    let path = entry.path();

    if path.is_dir() {
        return "".to_string();
    }

    if let Some(ext) = path.extension() {
        if ext != "png" {
            return "".to_string();
        }
    } else {
        return "".to_string();
    }

    let key = path
        .strip_prefix("assets")
        .unwrap()
        .to_string_lossy()
        .replace("\\", "/");

    return format!(
        r#"
        #[asset(path = "{0}")]
        {1}: Handle<Image>,"#,
        key,
        key.replace("/", "_").replace(".png", "")
    );
}
