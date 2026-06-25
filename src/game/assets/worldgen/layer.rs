use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct LayerMapperAsset {
    pub height_noise_scale: f64,
    pub layers: Vec<LayerAsset>,
}

#[derive(Debug, Deserialize)]
pub struct LayerAsset {
    pub name: String,
    pub height: [f64; 2],
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LayerMapperAssetLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Default, TypePath)]
pub struct LayerMapperAssetLoader;

impl AssetLoader for LayerMapperAssetLoader {
    type Asset = LayerMapperAsset;
    type Settings = ();
    type Error = LayerMapperAssetLoaderError;

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

        let layer_mapper: LayerMapperAsset = serde_json::from_slice(&bytes)
            .map_err(|e| Self::Error::Io(std::io::Error::other(e)))?;

        info!(
            target: "asset_loader",
            "Loaded layer mapper with {} layers",
            layer_mapper.layers.len()
        );

        Ok(layer_mapper)
    }
}
