use bevy::prelude::*;
use std::collections::HashMap;

pub struct BlockTextures(HashMap<&'static str, Handle<Image>>);

impl BlockTextures {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, name: &'static str, asset_server: &Res<AssetServer>) {
        self.0
            .insert(name, asset_server.load(format!("{}.png", name)));
    }

    pub fn get(&self, name: &str) -> Option<&Handle<Image>> {
        self.0.get(name)
    }
}

pub fn load_block_textures(asset_server: &Res<AssetServer>) -> BlockTextures {
    let mut textures = BlockTextures::new();

    textures.add("blocks/grass", asset_server);
    textures.add("blocks/tree", asset_server);

    textures
}
