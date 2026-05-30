use crate::game::{
    ImageAssets,
    registry::{
        Registry,
        block_registry::{BlockId, BlockRegistry},
    },
    worldgen::{Biome, BiomeMapper as RawBiomeMapper},
};
use bevy::prelude::*;

pub struct BiomeId(pub String);

pub struct BiomeDefinition {
    pub id: BiomeId,
    pub surface: BlockId,
    pub objects: Option<Vec<BiomeObjectDefinition>>,
}

pub struct BiomeObjectDefinition {
    pub block: BlockId,
    pub chance: f32,
}

#[derive(Resource)]
pub struct BiomeRegistry {
    inner: Registry<BiomeDefinition>,
}

impl BiomeRegistry {
    pub fn new(inner: Registry<BiomeDefinition>) -> Self {
        BiomeRegistry { inner }
    }

    pub fn iter(&self) -> impl Iterator<Item = &BiomeDefinition> {
        self.inner.entries.iter()
    }
}

#[derive(Resource)]
pub struct BiomeMapper {
    pub rules: Vec<BiomeMapperRule>,
    pub temp_scale: f32,
    pub humid_scale: f32,
}

pub struct BiomeMapperRule {
    pub biome: String,
    pub temp: (f32, f32),
    pub humid: (f32, f32),
}

pub fn init_biomes(
    mut commands: Commands,
    biomes: Res<Assets<Biome>>,
    block_registry: Res<BlockRegistry>,
) {
    let mut inner = Registry::new();

    for (_id, biome) in biomes.iter() {
        let id = BiomeId(biome.id.clone());
        let surface = block_registry.id_by_name(&biome.surface);
        let objects = biome.objects.as_ref().map(|objects| {
            objects
                .iter()
                .map(|object| BiomeObjectDefinition {
                    block: block_registry.id_by_name(&object.block),
                    chance: object.chance,
                })
                .collect()
        });

        inner.insert(
            BiomeDefinition {
                id,
                surface,
                objects,
            },
            biome.id.as_str(),
        );
    }

    commands.insert_resource(BiomeRegistry { inner });
}

pub fn init_biome_mapper(
    mut commands: Commands,
    mapper: Res<Assets<RawBiomeMapper>>,
    mapper_handle: Res<ImageAssets>,
) {
    let handle = &mapper_handle.worldgen_biome_mapper;

    let Some(map) = mapper.get(handle) else {
        return;
    };

    let rules = map
        .rules
        .iter()
        .map(|rule| BiomeMapperRule {
            biome: rule.biome.clone(),
            temp: (rule.temperature[0], rule.temperature[1]),
            humid: (rule.humidity[0], rule.humidity[1]),
        })
        .collect::<Vec<_>>();

    let mapper = BiomeMapper {
        rules,
        temp_scale: map.temperature_noise_scale,
        humid_scale: map.humidity_noise_scale,
    };

    commands.insert_resource(mapper);
}
