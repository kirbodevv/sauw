use crate::game::{ImageAssets, registry::Registry, worldgen::Biome};
use bevy::prelude::*;

pub struct BiomeRegistry {
    inner: Registry<Handle<Biome>>,
}

impl BiomeRegistry {
    pub fn new(assets: &ImageAssets) -> Self {
        let mut inner = Registry::new();

        inner.insert(assets.worldgen_biome_forest.clone(), "forest");
        inner.insert(assets.worldgen_biome_ocean.clone(), "ocean");
        inner.insert(assets.worldgen_biome_plains.clone(), "plains");
        inner.insert(assets.worldgen_biome_desert.clone(), "desert");

        Self { inner }
    }

    pub fn biomes(&self) -> Vec<Handle<Biome>> {
        self.inner
            .entries
            .iter()
            .map(|handle| handle.clone())
            .collect()
    }
}
