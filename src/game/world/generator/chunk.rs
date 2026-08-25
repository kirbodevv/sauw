use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::{
    constants::{CHUNK_SIZE, CHUNK_VOLUME, GROUND_LAYER, OBJECT_LAYER},
    game::{
        registry::{biome_registry::BiomeRegistry, block_registry::BlockId},
        world::{
            generator::{
                ChunkGenerateRequest, chunk_data, idx, idx_2d,
                mappers::{BiomeMapper, LayerMapper},
                noise::WorldNoise,
            },
            render::chunk_spawner::SpawnChunk,
        },
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

    const WIDTH: usize = CHUNK_SIZE;
    const HEIGHT: usize = CHUNK_SIZE;

    for chunk in reader.read() {
        let mut blocks = [BlockId::AIR; CHUNK_VOLUME];

        let chunk_coord = chunk.0;

        let chunk_data = chunk_data::generate(chunk_coord, &noise, &biome_mapper, &layer_mapper);

        let seed = noise.settings.seed.0 as u64;
        let x = chunk_coord.x as i64 as u64;
        let y = chunk_coord.y as i64 as u64;

        let mut objects_rng = SmallRng::seed_from_u64(seed.wrapping_add(x).wrapping_add(y));

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
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

                blocks[idx(x, y, GROUND_LAYER)] = surface;
                blocks[idx(x, y, OBJECT_LAYER)] = top;
            }
        }

        writer.write(SpawnChunk {
            chunk_coord,
            blocks,
        });
    }
}
