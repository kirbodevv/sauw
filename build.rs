use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use image::GenericImage;
use image::RgbaImage;
use serde::Serialize;
extern crate embed_resource;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(rust_analyzer)");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets");

    let target = std::env::var("TARGET").unwrap();
    if target.contains("windows") {
        embed_resource::compile("platform/windows/embed/icon.rc");
    }

    generate_atlas(
        "assets/block",
        "assets/atlas/block.png",
        "assets/atlas/block.json",
        0,
        512,
    );

    generate_atlas(
        "assets/item",
        "assets/atlas/item.png",
        "assets/atlas/item.json",
        0,
        512,
    );
}

#[derive(Serialize)]
struct AtlasEntry([u32; 4]);

#[derive(Serialize)]
struct Atlas {
    width: u32,
    height: u32,
    entries: BTreeMap<String, AtlasEntry>,
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
    let mut map = BTreeMap::new();

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

        const PADDING: u32 = 2;

        x += w + PADDING;
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
