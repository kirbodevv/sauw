use noise::NoiseFn;

use crate::game::{
    registry::biome_registry::{BiomeDefinition, BiomeRegistry},
    world::generator::{
        mappers::BiomeMapper,
        noise::{WorldNoise, normalize},
    },
};

pub fn get_biome<'a>(
    noise: &WorldNoise,
    biomes: &'a BiomeRegistry,
    mapper: &BiomeMapper,
    layer: &str,
    x: f64,
    y: f64,
) -> &'a BiomeDefinition {
    let temp = normalize(
        noise
            .temp
            .get([x * mapper.temp_scale, y * mapper.temp_scale]),
    );

    let humid = normalize(
        noise
            .humid
            .get([x * mapper.humid_scale, y * mapper.humid_scale]),
    );
    let biome_name = mapper.get_biome(layer, temp, humid).unwrap_or("desert");

    biomes.by_name(biome_name).unwrap()
}
