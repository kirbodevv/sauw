use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};

use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureId(pub usize);

#[derive(Debug, Deserialize, Clone, Copy)]
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

    pub fn rect_with_padding(&self, padding: f32) -> Rect {
        Rect::new(
            self.x() as f32 + padding,
            self.y() as f32 + padding,
            (self.x() + self.width()) as f32 - padding,
            (self.y() + self.height()) as f32 - padding,
        )
    }
}

#[derive(Debug, Deserialize)]
struct AtlasAssetRaw {
    width: u32,
    height: u32,
    entries: HashMap<String, AtlasEntryAsset>,
}

#[derive(Asset, TypePath, Debug)]
pub struct AtlasAsset {
    pub width: u32,
    pub height: u32,
    entries: Vec<AtlasEntryAsset>,
    name_to_id: HashMap<String, usize>,
}

impl AtlasAsset {
    pub fn get(&self, id: TextureId) -> &AtlasEntryAsset {
        &self.entries[id.0]
    }

    pub fn try_id_by_name(&self, name: &str) -> Option<TextureId> {
        self.name_to_id.get(name).map(|&id| TextureId(id))
    }

    pub fn id_by_name(&self, name: &str) -> TextureId {
        self.try_id_by_name(name)
            .unwrap_or_else(|| panic!("Unknown texture {:?} in atlas", name))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &AtlasEntryAsset)> {
        self.name_to_id
            .iter()
            .map(move |(name, &id)| (name.as_str(), &self.entries[id]))
    }
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

        let raw: AtlasAssetRaw = serde_json::from_slice(&bytes)
            .map_err(|e| Self::Error::Io(std::io::Error::other(e)))?;

        let mut entries = Vec::with_capacity(raw.entries.len());
        let mut name_to_id = HashMap::with_capacity(raw.entries.len());

        for (name, entry) in raw.entries {
            name_to_id.insert(name, entries.len());
            entries.push(entry);
        }

        info!(
            target: "asset_loader",
            "Loaded atlas with {} entries",
            entries.len()
        );

        Ok(AtlasAsset {
            width: raw.width,
            height: raw.height,
            entries,
            name_to_id,
        })
    }
}
