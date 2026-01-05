use std::collections::HashSet;

use bevy::prelude::*;

use crate::{
    constants::{CHUNK_WORLD, LOAD_RADIUS, TILE_SIZE},
    engine::{
        player::{components::Player, resources::CurrentPlayerChunk},
        resources::{BlockTextures, GameRegistry},
        world::{
            components::{BelongsToChunk, BlockEntity, BlockPos, ChunkCoord},
            resources::LoadedChunks,
        },
    },
    game::world::block::BlockId,
};

pub fn manage_chunks(
    mut commands: Commands,
    mut loaded: ResMut<LoadedChunks>,
    mut last_player_chunk: ResMut<CurrentPlayerChunk>,
    registry: Res<GameRegistry>,
    textures: Res<BlockTextures>,
    player: Single<&Transform, With<Player>>,
    tiles_q: Query<(Entity, &BelongsToChunk)>,
) {
    let player_pos = player.translation;
    let current_player_chunk = ChunkCoord {
        x: (player_pos.x / CHUNK_WORLD).floor() as i32,
        y: (player_pos.y / CHUNK_WORLD).floor() as i32,
    };

    if let Some(chunk) = last_player_chunk.0 {
        if current_player_chunk == chunk {
            return;
        }
    }

    last_player_chunk.0 = Some(current_player_chunk);

    let mut required = HashSet::new();

    for cx in (current_player_chunk.x - LOAD_RADIUS)..=(current_player_chunk.x + LOAD_RADIUS) {
        for cy in (current_player_chunk.y - LOAD_RADIUS)..=(current_player_chunk.y + LOAD_RADIUS) {
            required.insert(ChunkCoord { x: cx, y: cy });
        }
    }

    for coord in required.iter() {
        if !loaded.set.contains(coord) {
            spawn_chunk(&mut commands, &registry, &textures, *coord);
            loaded.set.insert(*coord);
        }
    }

    let mut unloaded = HashSet::new();

    for (entity, belongs) in &tiles_q {
        if !required.contains(&belongs.0) {
            unloaded.insert(belongs.0);
            commands.entity(entity).despawn();
        }
    }

    for c in unloaded {
        info!("Despawn chunk ({}, {})", c.x, c.y);
    }

    loaded.set = required;
}

pub fn spawn_chunk(
    commands: &mut Commands,
    registry: &GameRegistry,
    textures: &BlockTextures,
    chunk_coord: ChunkCoord,
) {
    info!("Spawn chunk: ({}, {})", chunk_coord.x, chunk_coord.y);

    for y in 0..16 {
        for x in 0..16 {
            for layer in 0..2 {
                spawn_block(
                    commands,
                    registry,
                    textures,
                    BlockId(1),
                    chunk_coord,
                    BlockPos { x, y, layer },
                );
            }
        }
    }
}

pub fn spawn_block(
    commands: &mut Commands,
    registry: &GameRegistry,
    textures: &BlockTextures,
    block_id: BlockId,
    chunk_coord: ChunkCoord,
    pos: BlockPos,
) {
    let block = registry.blocks.get(block_id);

    if block.texture == None {
        return;
    }

    let texture_handle = textures
        .get(block.texture.unwrap())
        .unwrap_or_else(|| panic!("Texture for block {} not found!", block.name));

    let world_x =
        (chunk_coord.x * 16) as f32 * TILE_SIZE + pos.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
    let world_y =
        (chunk_coord.y * 16) as f32 * TILE_SIZE + pos.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;

    commands.spawn((
        Sprite {
            image: texture_handle.clone(),
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..default()
        },
        Transform::from_xyz(world_x, world_y, pos.layer as f32),
        BlockEntity,
        BelongsToChunk(chunk_coord),
        pos,
    ));
}
