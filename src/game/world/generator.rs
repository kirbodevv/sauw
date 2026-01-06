use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::{
    constants::{CHUNK_SIZE, TILE_SIZE},
    game::{
        registry::block_registry::BlockDefinition,
        resources::{GameRegistry, Textures},
        world::{
            block::BlockId,
            components::{BelongsToChunk, BlockEntity, BlockPos, ChunkCoord},
            resources::WorldSeed,
        },
    },
};

pub fn spawn_chunk(
    commands: &mut Commands,
    registry: &GameRegistry,
    textures: &Textures,
    chunk_coord: ChunkCoord,
    seed: u32,
) {
    let width = CHUNK_SIZE as usize;
    let height = CHUNK_SIZE as usize;

    let perlin = Perlin::new(seed);

    let air = registry.blocks.by_name("air");
    let grass = registry.blocks.by_name("grass");
    let sand = registry.blocks.by_name("sand");
    let water = registry.blocks.by_name("water");
    let flowers = registry.blocks.by_name("flowers");
    let tree = registry.blocks.by_name("tree");

    let freq = 0.1;

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

            spawn_block(
                commands,
                textures,
                surface,
                chunk_coord,
                BlockPos {
                    x: x as u8,
                    y: y as u8,
                    layer: 0,
                },
            );

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

            spawn_block(
                commands,
                textures,
                top,
                chunk_coord,
                BlockPos {
                    x: x as u8,
                    y: y as u8,
                    layer: 1,
                },
            );
        }
    }
}

pub fn spawn_block(
    commands: &mut Commands,
    textures: &Textures,
    block: &BlockDefinition,
    chunk_coord: ChunkCoord,
    pos: BlockPos,
) {
    if block.texture == None {
        return;
    }

    let texture_handle = textures
        .blocks
        .get(block.texture.unwrap())
        .unwrap_or_else(|| panic!("Texture for block {} not found!", block.name));

    let size = block.custom_size.unwrap_or(Vec2::splat(TILE_SIZE));

    let world_x =
        (chunk_coord.x * 16) as f32 * TILE_SIZE + pos.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
    let world_y =
        (chunk_coord.y * 16) as f32 * TILE_SIZE + pos.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;

    commands.spawn((
        Sprite {
            image: texture_handle.clone(),
            custom_size: Some(size),
            ..default()
        },
        Transform::from_xyz(world_x, world_y, pos.layer as f32),
        BlockEntity,
        BelongsToChunk(chunk_coord),
        pos,
    ));
}
