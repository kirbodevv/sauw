use crate::game::{
    assets::{resource::WorldgenMapperAssets, worldgen::BiomeMapperAsset},
    registry::biome_registry::{BiomeId, BiomeRegistry},
    world::generator::mappers::{LayerMapper, layer::LayerId},
};
use bevy::prelude::*;

#[derive(Resource)]
pub struct BiomeMapper {
    pub rules: Vec<BiomeMapperRule>,
}

pub struct BiomeMapperRule {
    pub biome: BiomeId,
    pub layer: LayerId,
    pub temp: Option<(f64, f64)>,
    pub humid: Option<(f64, f64)>,
    pub priority: u32,
}

pub fn init_biome_mapper(
    mut commands: Commands,
    layer_mapper: Res<LayerMapper>,
    biomes: Res<BiomeRegistry>,
    mapper: Res<Assets<BiomeMapperAsset>>,
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
            biome: biomes.id_by_name(&rule.biome),
            layer: layer_mapper.id_by_name(&rule.layer),
            temp: rule.temperature.map(|t| (t[0], t[1])),
            humid: rule.humidity.map(|h| (h[0], h[1])),
            priority: rule.priority,
        })
        .collect::<Vec<_>>();

    let mapper = BiomeMapper { rules };

    commands.insert_resource(mapper);
}
