use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

use crate::{
    constants::{CHUNK_SIZE, CHUNK_VOLUME},
    game::{
        registry::{
            biome_registry::{BiomeMapper, BiomeRegistry, LayerMapper},
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
    layer_mapper: Res<LayerMapper>,
    biome_mapper: Res<BiomeMapper>,
    mut reader: MessageReader<ChunkGenerateRequest>,
    mut writer: MessageWriter<GeneratedChunk>,
) {
    let width = CHUNK_SIZE as usize;
    let height = CHUNK_SIZE as usize;

    let temp_perlin = Perlin::new(seed.0);
    let humid_perlin = Perlin::new(seed.0 + 1337);
    let height_perlin = Perlin::new(seed.0 + 2674);

    let air = blocks.id_by_name("air");

    for chunk in reader.read() {
        let mut blocks = [air; CHUNK_VOLUME];

        let chunk_coord = chunk.0;

        for x in 0..width {
            for y in 0..height {
                let cx = chunk_coord.x as f64 * 16.0;
                let cy = chunk_coord.y as f64 * 16.0;
                let x = x as f64;
                let y = y as f64;

                let temp = generate_value(&temp_perlin, x, y, cx, cy, biome_mapper.temp_scale);
                let humid = generate_value(&humid_perlin, x, y, cx, cy, biome_mapper.humid_scale);
                let height =
                    generate_value(&height_perlin, x, y, cx, cy, layer_mapper.height_scale);
                let layer = layer_mapper.get_layer(height);

                let biome_name = biome_mapper
                    .get_biome(layer, temp, humid)
                    .unwrap_or("desert");
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

                blocks[idx(x as usize, y as usize, 0)] = surface;
                blocks[idx(x as usize, y as usize, 1)] = top;
            }
        }

        writer.write(GeneratedChunk {
            chunk_coord,
            blocks,
        });
    }
}

#[inline]
fn generate_value(perlin: &Perlin, x: f64, y: f64, chunk_x: f64, chunk_y: f64, scale: f64) -> f64 {
    normalize(perlin.get([(x + chunk_x) * scale, (y + chunk_y) * scale]))
}

#[inline]
fn normalize(val: f64) -> f64 {
    (val + 1.0) / 2.0
}
