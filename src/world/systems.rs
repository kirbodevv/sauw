use bevy::{
    ecs::system::Commands, math::Vec2, sprite::Sprite, transform::components::Transform,
    utils::default,
};

use crate::{
    constants::TILE_SIZE,
    game_registry::GameRegistry,
    world::{
        block::{BlockId, Layer},
        components::{BlockEntity, BlockPos},
        textures::BlockTextures,
        world::{ChunkPos, World},
    },
};

pub fn spawn_chunk(
    commands: &mut Commands,
    world: &World,
    registry: &GameRegistry,
    textures: &BlockTextures,
    chunk_pos: ChunkPos,
) {
    let Some(chunk) = world.get_chunk(chunk_pos) else {
        return;
    };

    for y in 0..16 {
        for x in 0..16 {
            let ground = chunk.get(x, y, Layer::Ground);
            spawn_block(commands, registry, textures, ground, chunk_pos, x, y, 0);

            let object = chunk.get(x, y, Layer::Object);
            if object.0 != 0 {
                spawn_block(commands, registry, textures, object, chunk_pos, x, y, 1);
            }
        }
    }
}

pub fn spawn_block(
    commands: &mut Commands,
    registry: &GameRegistry,
    textures: &BlockTextures,
    block_id: BlockId,
    chunk_pos: ChunkPos,
    x: usize,
    y: usize,
    layer: u8,
) {
    let block = registry.blocks.get(block_id);

    if block.texture == None {
        return;
    }

    let texture_handle = textures
        .get(block.texture.unwrap())
        .unwrap_or_else(|| panic!("Texture for block {} not found!", block.name));

    let world_x = (chunk_pos.x * 16) as f32 * TILE_SIZE + x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
    let world_y = (chunk_pos.y * 16) as f32 * TILE_SIZE + y as f32 * TILE_SIZE + TILE_SIZE / 2.0;

    commands.spawn((
        Sprite {
            image: texture_handle.clone(),
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..default()
        },
        Transform::from_xyz(world_x, world_y, layer as f32),
        BlockEntity,
        BlockPos {
            x: x as u8,
            y: y as u8,
            layer,
        },
    ));
}
