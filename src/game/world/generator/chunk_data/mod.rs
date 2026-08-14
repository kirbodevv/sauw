use crate::{
    constants::{CHUNK_LAYER_VOLUME, CHUNK_SIZE},
    game::{
        registry::biome_registry::BiomeId,
        world::{
            ChunkCoord,
            generator::{
                chunk_data::climate::CellClimate,
                idx_2d,
                mappers::{BiomeMapper, LayerMapper},
                noise::WorldNoise,
            },
        },
    },
};

pub mod biome;
pub mod climate;
pub mod height;
pub mod layer;

pub struct ChunkData {
    pub climate_map: [CellClimate; CHUNK_LAYER_VOLUME],
    pub height_map: [f64; CHUNK_LAYER_VOLUME],
    pub biome_map: [BiomeId; CHUNK_LAYER_VOLUME],
}

pub fn generate(
    coord: ChunkCoord,
    noise: &WorldNoise,
    biome_mapper: &BiomeMapper,
    layer_mapper: &LayerMapper,
) -> ChunkData {
    const WIDTH: usize = CHUNK_SIZE;
    const HEIGHT: usize = CHUNK_SIZE;

    let mut climate_map = [CellClimate::default(); CHUNK_LAYER_VOLUME];
    let mut height_map = [0.0; CHUNK_LAYER_VOLUME];
    let mut biome_map = [BiomeId(0); CHUNK_LAYER_VOLUME];

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let rx = coord.x as f64 * CHUNK_SIZE as f64 + x as f64;
            let ry = coord.y as f64 * CHUNK_SIZE as f64 + y as f64;

            let climate = climate::generate(noise, rx, ry);
            let height = height::generate(noise, rx, ry);
            let layer = layer::generate(layer_mapper, height);
            let biome = biome::generate(biome_mapper, layer, climate);

            let index = idx_2d(x, y);

            climate_map[index] = climate;
            height_map[index] = height;
            biome_map[index] = biome;
        }
    }

    ChunkData {
        climate_map,
        height_map,
        biome_map,
    }
}
