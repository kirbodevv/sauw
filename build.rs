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

    generate_normal_atlas(
        "assets/block",
        "assets/block_normal",
        "assets/atlas/block_normal.png",
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

fn generate_normal_atlas(
    src_input_dir: &str,
    normal_input_dir: &str,
    out_png: &str,
    fixed_tile_size: u32,
    atlas_width: u32,
) {
    let mut images = Vec::new();

    for entry in fs::read_dir(src_input_dir).unwrap() {
        let entity_path = entry.unwrap().path();
        if entity_path
            .extension()
            .and_then(|extension| extension.to_str())
            != Some("png")
        {
            continue;
        }

        let block_image = image::open(&entity_path).unwrap().to_rgba8();
        let name = entity_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let normal_path = Path::new(normal_input_dir).join(format!("{name}.png"));

        let normal_image = if normal_path.exists() {
            let normal_image = image::open(&normal_path).unwrap().to_rgba8();
            assert_eq!(
                normal_image.dimensions(),
                block_image.dimensions(),
                "Normal map {} must have the same dimensions as {}",
                normal_path.display(),
                entity_path.display()
            );
            normal_image
        } else {
            RgbaImage::from_fn(block_image.width(), block_image.height(), |x, y| {
                image::Rgba([128, 128, 255, block_image.get_pixel(x, y).0[3]])
            })
        };

        images.push((name, normal_image));
    }

    images.sort_by(|a, b| a.0.cmp(&b.0));

    let mut atlas_height = 0;
    let mut x = 0;
    let mut y = 0;
    let mut row_height = 0;
    let mut atlas = RgbaImage::new(atlas_width, 2048);

    for (_, image) in images {
        let (width, height) = if fixed_tile_size > 0 {
            (fixed_tile_size, fixed_tile_size)
        } else {
            image.dimensions()
        };

        if x + width > atlas_width {
            x = 0;
            y += row_height + 1;
            row_height = 0;
        }

        atlas.copy_from(&image, x, y).unwrap();

        const PADDING: u32 = 2;
        x += width + PADDING;
        row_height = row_height.max(height + 1);
        atlas_height = (y + row_height).max(atlas_height);
    }

    let atlas_image = image::imageops::crop_imm(&atlas, 0, 0, atlas_width, atlas_height).to_image();

    fs::create_dir_all(Path::new(out_png).parent().unwrap()).unwrap();
    atlas_image.save(out_png).unwrap();
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
