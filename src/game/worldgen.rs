use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};
use serde::Deserialize;
use std::io::{Error as IoError, ErrorKind};
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
    pub temperature: [f64; 2],
    pub humidity: [f64; 2],
}

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

#[derive(Default, TypePath)]
pub struct BiomeMapperLoader;

impl AssetLoader for BiomeMapperLoader {
    type Asset = BiomeMapper;
    type Settings = ();
    type Error = BiomeLoaderError;

    fn extensions(&self) -> &[&str] {
        &["mapper"]
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
            .map_err(|e| BiomeLoaderError::Io(IoError::new(ErrorKind::Other, e)))?;

        info!(
            "[ Asset Loader ] Loaded biome mapper with {} rules",
            biome_mapper.rules.len()
        );

        Ok(biome_mapper)
    }
}

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
            .map_err(|e| BiomeLoaderError::Io(IoError::new(ErrorKind::Other, e)))?;

        info!("[ Asset Loader ] Loaded biome: {}", biome.id);

        Ok(biome)
    }
}
