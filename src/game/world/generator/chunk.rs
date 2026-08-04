use bevy::prelude::*;

use crate::{
    constants::{CHUNK_SIZE, CHUNK_VOLUME},
    game::{
        registry::{biome_registry::BiomeRegistry, block_registry::BlockRegistry},
        world::generator::{
            ChunkGenerateRequest, GeneratedChunk,
            biome::get_biome,
            idx,
            mappers::{BiomeMapper, LayerMapper},
            noise::WorldNoise,
            terrain::terrain_height,
        },
    },
};

pub fn generate_chunk(
    biomes: Res<BiomeRegistry>,
    blocks: Res<BlockRegistry>,
    layer_mapper: Res<LayerMapper>,
    biome_mapper: Res<BiomeMapper>,
    noise: Res<WorldNoise>,
    mut reader: MessageReader<ChunkGenerateRequest>,
    mut writer: MessageWriter<GeneratedChunk>,
) {
    if reader.is_empty() {
        return;
    }

    const WIDTH: usize = CHUNK_SIZE;
    const HEIGHT: usize = CHUNK_SIZE;

    let air = blocks.id_by_name("air");

    for chunk in reader.read() {
        let mut blocks = [air; CHUNK_VOLUME];

        let chunk_coord = chunk.0;

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let rx = chunk_coord.x as f64 * CHUNK_SIZE as f64 + x as f64;
                let ry = chunk_coord.y as f64 * CHUNK_SIZE as f64 + y as f64;

                let layer = layer_mapper.get_layer(terrain_height(&noise, &layer_mapper, rx, ry));
                let biome = get_biome(&noise, &biomes, &biome_mapper, layer, rx, ry);

                let surface = biome.surface;

                let mut top = air;

                if let Some(objects) = &biome.objects {
                    let r: f32 = rand::random();

                    let mut cumulative = 0.0;

                    for object in objects {
                        cumulative += object.chance;

                        if r < cumulative {
                            top = object.block;
                            break;
                        }
                    }
                }

                blocks[idx(x, y, 0)] = surface;
                blocks[idx(x, y, 1)] = top;
            }
        }

        writer.write(GeneratedChunk {
            chunk_coord,
            blocks,
        });
    }
}
