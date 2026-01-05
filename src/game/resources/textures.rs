use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct Textures {
    pub blocks: HashMap<&'static str, Handle<Image>>,
}

impl Textures {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }
}

fn load_texture(asset_server: &AssetServer, texture: &'static str) -> Handle<Image> {
    asset_server.load(format!("{}.png", texture))
}

pub fn load_block_textures(asset_server: Res<AssetServer>, mut textures: ResMut<Textures>) {
    textures
        .blocks
        .insert("block/grass", load_texture(&asset_server, "block/grass"));
    textures
        .blocks
        .insert("block/tree", load_texture(&asset_server, "block/tree"));
}
