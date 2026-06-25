use crate::game::assets::{
    resource::WorldgenAssets,
    worldgen::{BiomeMapper as RawBiomeMapper, LayerMapper as RawLayerMapper},
};
use bevy::prelude::*;

#[derive(Resource)]
pub struct LayerMapper {
    pub height_scale: f64,
    pub layers: Vec<Layer>,
}

pub struct Layer {
    name: String,
    height: (f64, f64),
}

#[derive(Resource)]
pub struct BiomeMapper {
    pub rules: Vec<BiomeMapperRule>,
    pub temp_scale: f64,
    pub humid_scale: f64,
}

pub struct BiomeMapperRule {
    pub biome: String,
    pub layer: String,
    pub temp: Option<(f64, f64)>,
    pub humid: Option<(f64, f64)>,
    pub priority: u32,
}

impl LayerMapper {
    pub fn get_layer(&self, height: f64) -> &str {
        self.layers
            .iter()
            .find(|layer| height >= layer.height.0 && height <= layer.height.1)
            .map(|layer| layer.name.as_str())
            .unwrap()
    }
}

impl BiomeMapper {
    pub fn get_biome(&self, layer: &str, temp: f64, humid: f64) -> Option<&str> {
        self.rules
            .iter()
            .filter(|rule| rule.layer == layer)
            .filter(|rule| {
                let temp_in_range = rule.temp.is_none_or(|t| temp >= t.0 && temp <= t.1);
                let humid_in_range = rule.humid.is_none_or(|h| humid >= h.0 && humid <= h.1);

                temp_in_range && humid_in_range
            })
            .max_by_key(|r| r.priority)
            .map(|rule| rule.biome.as_str())
    }
}

pub fn init_layer_mapper(
    mut commands: Commands,
    mapper: Res<Assets<RawLayerMapper>>,
    assets: Res<WorldgenAssets>,
) {
    let handle = &assets.layer_mapper;

    let Some(map) = mapper.get(handle) else {
        return;
    };

    let layers = map
        .layers
        .iter()
        .map(|layer| Layer {
            name: layer.name.clone(),
            height: (layer.height[0], layer.height[1]),
        })
        .collect::<Vec<_>>();

    let layer_mapper = LayerMapper {
        height_scale: map.height_noise_scale,
        layers,
    };

    commands.insert_resource(layer_mapper);
}

pub fn init_biome_mapper(
    mut commands: Commands,
    mapper: Res<Assets<RawBiomeMapper>>,
    assets: Res<WorldgenAssets>,
) {
    let handle = &assets.biome_mapper;

    let Some(map) = mapper.get(handle) else {
        return;
    };

    let rules = map
        .rules
        .iter()
        .map(|rule| BiomeMapperRule {
            biome: rule.biome.clone(),
            layer: rule.layer.clone(),
            temp: rule.temperature.map(|t| (t[0], t[1])),
            humid: rule.humidity.map(|h| (h[0], h[1])),
            priority: rule.priority,
        })
        .collect::<Vec<_>>();

    let mapper = BiomeMapper {
        rules,
        temp_scale: map.temperature_noise_scale,
        humid_scale: map.humidity_noise_scale,
    };

    commands.insert_resource(mapper);
}
