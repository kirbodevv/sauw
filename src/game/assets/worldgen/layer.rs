use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};
use serde::Deserialize;
use std::io::{Error as IoError, ErrorKind};
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct LayerMapper {
    pub height_noise_scale: f64,
    pub layers: Vec<Layer>,
}

#[derive(Debug, Deserialize)]
pub struct Layer {
    pub name: String,
    pub height: [f64; 2],
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LayerMapperLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Default, TypePath)]
pub struct LayerMapperLoader;

impl AssetLoader for LayerMapperLoader {
    type Asset = LayerMapper;
    type Settings = ();
    type Error = LayerMapperLoaderError;

    fn extensions(&self) -> &[&str] {
        &["lmap"]
    }

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let layer_mapper: LayerMapper = serde_json::from_slice(&bytes)
            .map_err(|e| LayerMapperLoaderError::Io(IoError::new(ErrorKind::Other, e)))?;

        info!(
            "[ Asset Loader ] Loaded layer mapper with {} layers",
            layer_mapper.layers.len()
        );

        Ok(layer_mapper)
    }
}
