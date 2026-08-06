use crate::game::assets::{
    resource::WorldgenMapperAssets,
    worldgen::{BiomeMapperAsset as RawBiomeMapper, LayerMapperAsset as RawLayerMapper},
};
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LayerId(pub u8);

#[derive(Resource)]
pub struct LayerMapper {
    pub layers: Vec<Layer>,
}

pub struct Layer {
    pub name: String,
    pub height: (f64, f64),
}

#[derive(Resource)]
pub struct BiomeMapper {
    pub rules: Vec<BiomeMapperRule>,
}

pub struct BiomeMapperRule {
    pub biome: String,
    pub layer: LayerId,
    pub temp: Option<(f64, f64)>,
    pub humid: Option<(f64, f64)>,
    pub priority: u32,
}

impl LayerMapper {
    pub fn id_by_name(&self, name: &str) -> LayerId {
        self.layers
            .iter()
            .position(|layer| layer.name.as_str() == name)
            .map(|i| LayerId(i as u8))
            .unwrap()
    }
}

pub fn init_layer_mapper(
    mut commands: Commands,
    mapper: Res<Assets<RawLayerMapper>>,
    assets: Res<WorldgenMapperAssets>,
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

    let layer_mapper = LayerMapper { layers };

    commands.insert_resource(layer_mapper);
}

pub fn init_biome_mapper(
    mut commands: Commands,
    layer_mapper: Res<LayerMapper>,
    mapper: Res<Assets<RawBiomeMapper>>,
    assets: Res<WorldgenMapperAssets>,
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
            layer: layer_mapper.id_by_name(&rule.layer),
            temp: rule.temperature.map(|t| (t[0], t[1])),
            humid: rule.humidity.map(|h| (h[0], h[1])),
            priority: rule.priority,
        })
        .collect::<Vec<_>>();

    let mapper = BiomeMapper { rules };

    commands.insert_resource(mapper);
}
