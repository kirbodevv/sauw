use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};

use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct TextureId(String);

impl TextureId {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }

    pub fn get_name(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Deserialize)]
pub struct AtlasEntryAsset([u32; 4]);

impl AtlasEntryAsset {
    pub fn x(&self) -> u32 {
        self.0[0]
    }
    pub fn y(&self) -> u32 {
        self.0[1]
    }
    pub fn width(&self) -> u32 {
        self.0[2]
    }
    pub fn height(&self) -> u32 {
        self.0[3]
    }
}

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct AtlasAsset {
    pub width: u32,
    pub height: u32,
    pub entries: HashMap<TextureId, AtlasEntryAsset>,
}

#[derive(Default, TypePath)]
pub struct AtlasAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AtlasAssetLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for AtlasAssetLoader {
    type Asset = AtlasAsset;
    type Settings = ();
    type Error = AtlasAssetLoaderError;

    fn extensions(&self) -> &[&str] {
        &["json"]
    }

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let atlas: AtlasAsset = serde_json::from_slice(&bytes)
            .map_err(|e| Self::Error::Io(std::io::Error::other(e)))?;

        info!(
            target: "asset_loader",
            "Loaded atlas with {} entries",
            atlas.entries.len()
        );

        Ok(atlas)
    }
}
