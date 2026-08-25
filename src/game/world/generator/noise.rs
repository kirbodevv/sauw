use bevy::prelude::*;
use noise::{Fbm, Perlin};

use crate::game::{
    assets::{resource::NoiseSettingsAssets, worldgen::NoiseSettingsAsset},
    world::config::{WorldConfig, WorldSeed},
};

#[derive(Resource)]
pub struct WorldNoise {
    pub terrain: Fbm<Perlin>,
    pub continent: Fbm<Perlin>,
    pub temp: Perlin,
    pub humid: Perlin,
    pub settings: NoiseSettings,
}

pub struct NoiseSettings {
    pub height_scale: f64,
    pub temperature_scale: f64,
    pub humidity_scale: f64,
    pub seed: WorldSeed,
}

impl WorldNoise {
    pub fn new(settings: NoiseSettings) -> Self {
        Self {
            terrain: Fbm::<Perlin>::new(settings.seed.0),
            continent: Fbm::<Perlin>::new(settings.seed.0 + 9999),
            temp: Perlin::new(settings.seed.0),
            humid: Perlin::new(settings.seed.0 + 1337),
            settings,
        }
    }
}

pub fn init_noise(
    mut commands: Commands,
    config: Res<WorldConfig>,
    noise_settings: Res<Assets<NoiseSettingsAsset>>,
    assets: Res<NoiseSettingsAssets>,
) {
    let handle = &assets.noise_settings;

    let Some(map) = noise_settings.get(handle) else {
        return;
    };

    let settings = NoiseSettings {
        height_scale: map.height_scale,
        temperature_scale: map.temperature_scale,
        humidity_scale: map.humidity_scale,
        seed: config.seed.clone(),
    };

    commands.insert_resource(WorldNoise::new(settings));
}

#[inline]
pub fn normalize(v: f64) -> f64 {
    (v + 1.0) / 2.0
}
