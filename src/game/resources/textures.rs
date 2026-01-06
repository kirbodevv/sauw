use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct Textures {
    pub blocks: HashMap<&'static str, Handle<Image>>,
    pub entities: HashMap<&'static str, Handle<Image>>,
}

impl Textures {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            entities: HashMap::new(),
        }
    }
}

pub fn load_texture(asset_server: &AssetServer, texture: &'static str) -> Handle<Image> {
    info!("Asset {} loaded", texture);
    asset_server.load(format!("{}.png", texture))
}
