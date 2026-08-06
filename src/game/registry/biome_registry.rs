use crate::game::{
    assets::{resource::BiomeAssets, worldgen::BiomeAsset},
    registry::{
        Registry,
        block_registry::{BlockId, BlockRegistry},
    },
};
use bevy::prelude::*;

#[derive(Copy, Clone)]
pub struct BiomeId(pub u16);

pub struct BiomeDefinition {
    pub name: String,
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

    pub fn by_name(&self, name: &str) -> Option<&BiomeDefinition> {
        self.inner.by_name(name)
    }

    pub fn by_id(&self, id: BiomeId) -> &BiomeDefinition {
        self.inner.entries.get(id.0 as usize).unwrap()
    }

    pub fn id_by_name(&self, name: &str) -> BiomeId {
        self.inner
            .ids
            .get(name)
            .map(|&id| BiomeId(id as u16))
            .unwrap()
    }
}

pub fn init_biomes(
    mut commands: Commands,
    biomes: Res<Assets<BiomeAsset>>,
    block_registry: Res<BlockRegistry>,
) {
    let mut inner = Registry::new("biome");

    for (_id, biome) in biomes.iter() {
        let name = biome.id.clone();
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
                name,
                surface,
                objects,
            },
            biome.id.as_str(),
        );
    }

    commands.insert_resource(BiomeRegistry { inner });
    commands.remove_resource::<BiomeAssets>();
}
