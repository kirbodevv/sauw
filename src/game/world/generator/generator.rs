use bevy::prelude::*;
use bevy_asset_loader::mapped;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::{
    constants::{CHUNK_SIZE, CHUNK_VOLUME},
    game::{
        registry::{
            biome_registry::{BiomeMapper, BiomeRegistry},
            block_registry::BlockRegistry,
        },
        world::{
            WorldSeed,
            generator::{ChunkGenerateRequest, GeneratedChunk, idx},
        },
    },
};

pub fn generate_chunk(
    biomes: Res<BiomeRegistry>,
    blocks: Res<BlockRegistry>,
    seed: Res<WorldSeed>,
    biome_mapper: Res<BiomeMapper>,
    mut reader: MessageReader<ChunkGenerateRequest>,
    mut writer: MessageWriter<GeneratedChunk>,
) {
    let width = CHUNK_SIZE as usize;
    let height = CHUNK_SIZE as usize;

    let temp_perlin = Perlin::new(seed.0);
    let humid_perlin = Perlin::new(seed.0 + 1337);

    let air = blocks.id_by_name("air");

    for chunk in reader.read() {
        let mut blocks = [air; CHUNK_VOLUME];

        let chunk_coord = chunk.0;

        for x in 0..width {
            for y in 0..height {
                let chunk_x = chunk_coord.x as f64 * 16.0;
                let chunk_y = chunk_coord.y as f64 * 16.0;

                let temp = temp_perlin.get([
                    (x as f64 + chunk_x) * biome_mapper.temp_scale,
                    (y as f64 + chunk_y) * biome_mapper.temp_scale,
                ]);
                let temp = (temp + 1.0) / 2.0;

                let humidity = humid_perlin.get([
                    (x as f64 + chunk_x) * biome_mapper.humid_scale,
                    (y as f64 + chunk_y) * biome_mapper.humid_scale,
                ]);
                let humidity = (humidity + 1.0) / 2.0;

                let biome_name = biome_mapper.get_biome(temp, humidity).unwrap_or("desert");
                let biome = biomes.by_name(biome_name).unwrap();

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
