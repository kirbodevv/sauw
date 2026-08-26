use std::sync::Arc;

use bevy::prelude::*;

use crate::game::{
    registry::biome_registry::BiomeRegistry,
    world::generator::{
        mappers::{BiomeMapper, LayerMapper},
        noise::WorldNoise,
    },
};

pub struct GenerationContext {
    pub noise: WorldNoise,
    pub biome_mapper: BiomeMapper,
    pub layer_mapper: LayerMapper,
    pub biomes: BiomeRegistry,
}

#[derive(Resource, Clone)]
pub struct GenerationContextHandle(pub Arc<GenerationContext>);

pub fn init_generation_context(world: &mut World) {
    if !world.contains_resource::<WorldNoise>()
        || !world.contains_resource::<BiomeMapper>()
        || !world.contains_resource::<LayerMapper>()
        || !world.contains_resource::<BiomeRegistry>()
    {
        return;
    }

    let noise = world.remove_resource::<WorldNoise>().unwrap();
    let biome_mapper = world.remove_resource::<BiomeMapper>().unwrap();
    let layer_mapper = world.remove_resource::<LayerMapper>().unwrap();
    let biomes = world.remove_resource::<BiomeRegistry>().unwrap();

    world.insert_resource(GenerationContextHandle(Arc::new(GenerationContext {
        noise,
        biome_mapper,
        layer_mapper,
        biomes,
    })));
}
