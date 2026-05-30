use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::{
    constants::{CHUNK_SIZE, CHUNK_VOLUME},
    game::{
        registry::block_registry::BlockRegistry,
        world::{
            WorldSeed,
            generator::{ChunkGenerateRequest, GeneratedChunk, idx},
        },
    },
};

enum Biome {
    Ocean,
    Desert,
    Plains,
    Forest,
}

fn get_biome(v: f64) -> Biome {
    match v {
        v if v < 0.25 => Biome::Ocean,
        v if v < 0.45 => Biome::Desert,
        v if v < 0.75 => Biome::Plains,
        _ => Biome::Forest,
    }
}

pub fn generate_chunk(
    registry: Res<BlockRegistry>,
    seed: Res<WorldSeed>,
    mut reader: MessageReader<ChunkGenerateRequest>,
    mut writer: MessageWriter<GeneratedChunk>,
) {
    let width = CHUNK_SIZE as usize;
    let height = CHUNK_SIZE as usize;

    let perlin = Perlin::new(seed.0);
    let biome_perlin = Perlin::new(seed.0 + 1337);
    let biome_freq = 0.02;

    let air = registry.id_by_name("air");
    let grass = registry.id_by_name("grass");
    let sand = registry.id_by_name("sand");
    let water = registry.id_by_name("water");
    let flowers = registry.id_by_name("lily");
    let tree = registry.id_by_name("tree");
    let cactus = registry.id_by_name("cactus");

    let freq = 0.1;

    for chunk in reader.read() {
        let mut blocks = [air; CHUNK_VOLUME];

        let chunk_coord = chunk.0;

        for x in 0..width {
            for y in 0..height {
                let chunk_x = chunk_coord.x as f64 * 16.0;
                let chunk_y = chunk_coord.y as f64 * 16.0;

                let biome_value = biome_perlin.get([
                    (x as f64 + chunk_x) * biome_freq,
                    (y as f64 + chunk_y) * biome_freq,
                ]);

                let biome = get_biome((biome_value + 1.0) / 2.0);

                let height_value =
                    perlin.get([(x as f64 + chunk_x) * freq, (y as f64 + chunk_y) * freq]);
                let normalized = (height_value + 1.0) / 2.0;

                let surface = match biome {
                    Biome::Ocean => water,
                    Biome::Desert => sand,
                    Biome::Plains => grass,
                    Biome::Forest => grass,
                };

                let top = match biome {
                    Biome::Forest => {
                        let mut rng = rand::rng();
                        let r: f64 = rng.random();

                        if r < 0.05 {
                            tree
                        } else if r < 0.15 {
                            flowers
                        } else {
                            air
                        }
                    }

                    Biome::Plains => {
                        let mut rng = rand::rng();
                        if rng.random::<f64>() < 0.08 {
                            flowers
                        } else {
                            air
                        }
                    }

                    Biome::Desert => {
                        let mut rng = rand::rng();
                        if rng.random::<f64>() < 0.08 {
                            cactus
                        } else {
                            air
                        }
                    }

                    _ => air,
                };

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
