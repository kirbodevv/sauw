use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct BlockTextures(HashMap<&'static str, Handle<Image>>);

impl BlockTextures {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, name: &'static str, asset_server: &Res<AssetServer>) {
        self.0
            .insert(name, asset_server.load(format!("{}.png", name)));
        info!("Asset {} loaded", name);
    }

    pub fn get(&self, name: &str) -> Option<&Handle<Image>> {
        self.0.get(name)
    }
}

pub fn load_block_textures(
    asset_server: Res<AssetServer>,
    mut block_textures: ResMut<BlockTextures>,
) {
    block_textures.add("block/grass", &asset_server);
    block_textures.add("block/tree", &asset_server);
}
