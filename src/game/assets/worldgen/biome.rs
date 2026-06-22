use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct Biome {
    pub id: String,
    pub surface: String,
    pub objects: Option<Vec<BiomeObject>>,
}

#[derive(Debug, Deserialize)]
pub struct BiomeObject {
    pub block: String,
    pub chance: f32,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum BiomeLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Default, TypePath)]
pub struct BiomeLoader;

impl AssetLoader for BiomeLoader {
    type Asset = Biome;
    type Settings = ();
    type Error = BiomeLoaderError;

    fn extensions(&self) -> &[&str] {
        &["biome"]
    }

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let biome: Biome = serde_json::from_slice(&bytes)
            .map_err(|e| BiomeLoaderError::Io(std::io::Error::other(e)))?;

        info!(target: "asset_loader", "Loaded biome: {}", biome.id);

        Ok(biome)
    }
}
