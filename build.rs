use std::env;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(rust_analyzer)");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("load_textures.rs");

    let mut code = String::new();

    code.push_str(
        r#"
        use bevy::prelude::*;
        use crate::game::resources::load_texture;

        pub fn load_textures(asset_server: Res<AssetServer>, mut textures: ResMut<Textures>) {"#,
    );

    for entry in fs::read_dir("assets/block").unwrap() {
        code.push_str(&generate(entry.unwrap(), "blocks"));
    }

    for entry in fs::read_dir("assets/entity").unwrap() {
        code.push_str(&generate(entry.unwrap(), "entities"));
    }

    code.push_str("}\n");

    fs::write(dest_path, code).unwrap();

    println!("cargo::rerun-if-changed=build.rs");
}

fn generate(entry: DirEntry, namespace: &str) -> String {
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
        .with_extension("")
        .to_string_lossy()
        .replace("\\", "/");

    return format!(
        r#"textures
                .{}
                .insert("{}", load_texture(&asset_server, "{}"));
                "#,
        namespace, key, key
    );
}
