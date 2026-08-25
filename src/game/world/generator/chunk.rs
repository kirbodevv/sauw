use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::game::{
    registry::{biome_registry::BiomeRegistry, block_registry::BlockId},
    world::{
        chunk_positions,
        generator::{
            ChunkGenerateRequest, chunk_data,
            mappers::{BiomeMapper, LayerMapper},
            noise::WorldNoise,
        },
        render::chunk_spawner::SpawnChunk,
        types::{ChunkBlocks, idx_2d},
    },
};

pub fn generate_chunk(
    biomes: Res<BiomeRegistry>,
    layer_mapper: Res<LayerMapper>,
    biome_mapper: Res<BiomeMapper>,
    noise: Res<WorldNoise>,
    mut reader: MessageReader<ChunkGenerateRequest>,
    mut writer: MessageWriter<SpawnChunk>,
) {
    if reader.is_empty() {
        return;
    }

    for chunk in reader.read() {
        let mut blocks = ChunkBlocks::default();

        let chunk_coord = chunk.0;

        let chunk_data = chunk_data::generate(chunk_coord, &noise, &biome_mapper, &layer_mapper);

        let seed = noise.settings.seed.0 as u64;
        let x = chunk_coord.x as i64 as u64;
        let y = chunk_coord.y as i64 as u64;

        let mut objects_rng = SmallRng::seed_from_u64(seed.wrapping_add(x).wrapping_add(y));

        for (x, y) in chunk_positions() {
            let index = idx_2d(x, y);
            let biome = biomes.by_id(chunk_data.biome_map[index]);

            let surface = biome.surface;

            let mut top = BlockId::AIR;

            if let Some(objects) = &biome.objects {
                let r: f32 = objects_rng.random();

                let mut cumulative = 0.0;

                for object in objects {
                    cumulative += object.chance;

                    if r < cumulative {
                        top = object.block;
                        break;
                    }
                }
            }

            blocks.ground[idx_2d(x, y)] = surface;
            blocks.objects[idx_2d(x, y)] = top;
        }

        writer.write(SpawnChunk {
            chunk_coord,
            blocks,
        });
    }
}
