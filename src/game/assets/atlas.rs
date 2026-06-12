use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};

use serde::Deserialize;
use std::{
    collections::HashMap,
    io::{Error as IoError, ErrorKind},
};
use thiserror::Error;

#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct TextureId(String);

impl TextureId {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

#[derive(Debug, Deserialize)]
pub struct AtlasEntry([u32; 4]);

impl AtlasEntry {
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
pub struct Atlas {
    pub width: u32,
    pub height: u32,
    pub entries: HashMap<TextureId, AtlasEntry>,
}

#[derive(Default, TypePath)]
pub struct AtlasLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AtlasLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for AtlasLoader {
    type Asset = Atlas;
    type Settings = ();
    type Error = AtlasLoaderError;

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

        let atlas: Atlas = serde_json::from_slice(&bytes)
            .map_err(|e| AtlasLoaderError::Io(IoError::new(ErrorKind::Other, e)))?;

        info!(
            target: "asset_loader",
            "Loaded atlas with {} entries",
            atlas.entries.len()
        );

        Ok(atlas)
    }
}
