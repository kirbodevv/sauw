use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::{
    constants::{CHUNK_SIZE, CHUNK_VOLUME},
    game::{
        registry::GameRegistry,
        world::{
            WorldSeed,
            generator::{ChunkGenerateRequest, GeneratedChunk, idx},
        },
    },
};

pub fn generate_chunk(
    registry: Res<GameRegistry>,
    seed: Res<WorldSeed>,
    mut reader: MessageReader<ChunkGenerateRequest>,
    mut writer: MessageWriter<GeneratedChunk>,
) {
    let width = CHUNK_SIZE as usize;
    let height = CHUNK_SIZE as usize;

    let perlin = Perlin::new(seed.0);

    let air = registry.blocks.id_by_name("air");
    let grass = registry.blocks.id_by_name("grass");
    let sand = registry.blocks.id_by_name("sand");
    let water = registry.blocks.id_by_name("water");
    let flowers = registry.blocks.id_by_name("lily");
    let tree = registry.blocks.id_by_name("tree");

    let freq = 0.1;

    for chunk in reader.read() {
        let mut blocks = [air; CHUNK_VOLUME];

        let chunk_coord = chunk.0;

        for x in 0..width {
            for y in 0..height {
                let chunk_x = chunk_coord.x as f64 * 16.0;
                let chunk_y = chunk_coord.y as f64 * 16.0;

                let noise_value =
                    perlin.get([(x as f64 + chunk_x) * freq, (y as f64 + chunk_y) * freq]);
                let normalized = (noise_value + 1.0) / 2.0;

                let surface = if normalized < 0.2 {
                    water
                } else if normalized < 0.3 {
                    sand
                } else {
                    grass
                };

                let mut top = air;

                if surface == grass {
                    let mut rng = rand::rng();
                    let r: f64 = rng.random();

                    if r < 0.05 {
                        top = tree;
                    } else if r < 0.15 {
                        top = flowers;
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
