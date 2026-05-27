use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

use image::GenericImage;
use image::RgbaImage;
use serde::Serialize;
extern crate embed_resource;

fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target.contains("windows") {
        embed_resource::compile("platform/windows/embed/icon.rc");
    }

    println!("cargo::rustc-check-cfg=cfg(rust_analyzer)");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("assets.rs");

    let mut code = String::new();

    code.push_str(
        r#"
        use bevy_asset_loader::asset_collection::AssetCollection;
        use crate::game::atlas::Atlas;

        #[derive(AssetCollection, Resource)]
        pub struct ImageAssets {"#,
    );

    for entry in fs::read_dir("assets/block").unwrap() {
        code.push_str(&generate(entry.unwrap()));
    }

    for entry in fs::read_dir("assets/entity").unwrap() {
        code.push_str(&generate(entry.unwrap()));
    }

    for entry in fs::read_dir("assets/ui").unwrap() {
        code.push_str(&generate(entry.unwrap()));
    }

    for entry in fs::read_dir("assets/atlas").unwrap() {
        code.push_str(&generate(entry.unwrap()));
    }

    code.push_str("\n}\n");

    fs::write(&dest_path, code).unwrap();

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets");

    generate_atlas(
        "assets/block",
        "assets/atlas/block_texture.png",
        "assets/atlas/block.json",
        0,
        512,
    );
}

fn generate(entry: DirEntry) -> String {
    let path = entry.path();

    if path.is_dir() {
        return "".to_string();
    }

    let ext = if let Some(ext) = path.extension() {
        ext.to_str().unwrap_or("")
    } else {
        return "".to_string();
    };

    let asset_type = match ext {
        "json" => "Atlas".to_string(),
        "png" => "Image".to_string(),
        _ => return "".to_string(),
    };

    let key = path
        .strip_prefix("assets")
        .unwrap()
        .to_string_lossy()
        .replace("\\", "/");

    return format!(
        r#"
        #[asset(path = "{0}")]
        {1}: Handle<{2}>,"#,
        key,
        key.replace("/", "_").replace(&format!(".{}", ext), ""),
        asset_type,
    );
}

#[derive(Serialize)]
struct AtlasEntry([u32; 4]);

#[derive(Serialize)]
struct Atlas {
    width: u32,
    height: u32,
    entries: HashMap<String, AtlasEntry>,
}

fn generate_atlas(
    input_dir: &str,
    out_png: &str,
    out_json: &str,
    fixed_tile_size: u32,
    atlas_width: u32,
) {
    let mut images = Vec::new();

    for entry in fs::read_dir(input_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("png") {
            continue;
        }

        let img = image::open(&path).unwrap().to_rgba8();
        let name = path.file_stem().unwrap().to_str().unwrap().to_string();
        images.push((name, img));
    }

    images.sort_by(|a, b| a.0.cmp(&b.0));

    let mut atlas_height = 0;
    let mut x = 0;
    let mut y = 0;
    let mut row_height = 0;

    let mut atlas = RgbaImage::new(atlas_width, 2048);
    let mut map = HashMap::new();

    for (name, img) in images {
        let (w, h) = if fixed_tile_size > 0 {
            (fixed_tile_size, fixed_tile_size)
        } else {
            img.dimensions()
        };

        if x + w > atlas_width {
            x = 0;
            y += row_height + 1;
            row_height = 0;
        }

        atlas.copy_from(&img, x, y).unwrap();

        map.insert(name, AtlasEntry([x, y, w, h]));

        x += w + 1;
        row_height = row_height.max(h + 1);
        atlas_height = (y + row_height).max(atlas_height);
    }

    let atlas = image::imageops::crop_imm(&atlas, 0, 0, atlas_width, atlas_height).to_image();

    fs::create_dir_all(Path::new(out_png).parent().unwrap()).unwrap();
    atlas.save(out_png).unwrap();

    let atlas = Atlas {
        width: atlas_width,
        height: atlas_height,
        entries: map,
    };
    fs::write(out_json, serde_json::to_string_pretty(&atlas).unwrap()).unwrap();

    println!(
        "Generated atlas {} ({} entries)",
        out_png,
        atlas.entries.len()
    );
}
