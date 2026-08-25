use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct NoiseSettingsAsset {
    pub height_scale: f64,
    pub temperature_scale: f64,
    pub humidity_scale: f64,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum NoiseSettingsLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Default, TypePath)]
pub struct NoiseSettingsAssetLoader;

impl AssetLoader for NoiseSettingsAssetLoader {
    type Asset = NoiseSettingsAsset;
    type Settings = ();
    type Error = NoiseSettingsLoaderError;

    fn extensions(&self) -> &[&str] {
        &["noise"]
    }

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let noise_settings: NoiseSettingsAsset = serde_json::from_slice(&bytes)
            .map_err(|e| NoiseSettingsLoaderError::Io(std::io::Error::other(e)))?;

        info!(target: "asset_loader", "Loaded noise settings: height_scale={} temperature_scale={} humidity_scale={}", noise_settings.height_scale, noise_settings.temperature_scale, noise_settings.humidity_scale);

        Ok(noise_settings)
    }
}
