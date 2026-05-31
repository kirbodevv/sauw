use bevy::prelude::*;
use bevy_rapier2d::na::clamp;
use noise::{Fbm, NoiseFn, Perlin};

use crate::{
    constants::{CHUNK_SIZE, CHUNK_VOLUME},
    game::{
        registry::{biome_registry::BiomeRegistry, block_registry::BlockRegistry},
        world::{
            WorldSeed,
            generator::{
                ChunkGenerateRequest, GeneratedChunk, idx,
                mappers::{BiomeMapper, LayerMapper},
            },
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

    let terrain_fbm = Fbm::<Perlin>::new(seed.0);
    let continent_fbm = Fbm::<Perlin>::new(seed.0 + 9999);

    let air = blocks.id_by_name("air");

    for chunk in reader.read() {
        let mut blocks = [air; CHUNK_VOLUME];

        let chunk_coord = chunk.0;

        for x in 0..width {
            for y in 0..height {
                let cx = chunk_coord.x as f64 * CHUNK_SIZE as f64;
                let cy = chunk_coord.y as f64 * CHUNK_SIZE as f64;
                let x = x as f64;
                let y = y as f64;

                let terrain = normalize(terrain_fbm.get([
                    (x + cx) * layer_mapper.height_scale,
                    (y + cy) * layer_mapper.height_scale,
                ]));

                let continent =
                    normalize(continent_fbm.get([(x + cx) * 0.0001, (y + cy) * 0.0001]));

                let continent_bias = (continent - 0.5) * 2.0;

                let height = terrain + continent_bias * 0.35;
                let height = clamp(height, 0.0, 1.0);
                let height = height.powf(1.3);

                let layer = layer_mapper.get_layer(height);

                let temp = generate_value(&temp_perlin, x, y, cx, cy, biome_mapper.temp_scale);
                let humid = generate_value(&humid_perlin, x, y, cx, cy, biome_mapper.humid_scale);

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
