use crate::game::{
    registry::{
        Registry,
        block_registry::{BlockId, BlockRegistry},
    },
    worldgen::Biome,
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
