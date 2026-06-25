use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct BiomeMapper {
    pub temperature_noise_scale: f64,
    pub humidity_noise_scale: f64,
    pub rules: Vec<BiomeMapperRules>,
}

#[derive(Debug, Deserialize)]
pub struct BiomeMapperRules {
    pub biome: String,
    pub layer: String,
    pub temperature: Option<[f64; 2]>,
    pub humidity: Option<[f64; 2]>,
    pub priority: u32,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum BiomeMapperLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Default, TypePath)]
pub struct BiomeMapperLoader;

impl AssetLoader for BiomeMapperLoader {
    type Asset = BiomeMapper;
    type Settings = ();
    type Error = BiomeMapperLoaderError;

    fn extensions(&self) -> &[&str] {
        &["bmap"]
    }

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let biome_mapper: BiomeMapper = serde_json::from_slice(&bytes)
            .map_err(|e| BiomeMapperLoaderError::Io(std::io::Error::other(e)))?;

        info!(
            target: "asset_loader",
            "Loaded biome mapper with {} rules",
            biome_mapper.rules.len()
        );

        Ok(biome_mapper)
    }
}
