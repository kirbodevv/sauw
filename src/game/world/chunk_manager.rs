use crate::{
    constants::CHUNK_WORLD,
    game::{
        player::{CurrentPlayerChunk, Player},
        world::{
            components::{Chunk, ChunkCoord, LoadedChunks, Settings},
            generator::ChunkGenerateRequest,
        },
    },
};
use bevy::prelude::*;
use std::collections::HashSet;

pub fn manage_chunks(
    mut commands: Commands,
    mut writer: MessageWriter<ChunkGenerateRequest>,
    mut loaded: ResMut<LoadedChunks>,
    mut last_player_chunk: ResMut<CurrentPlayerChunk>,
    settings: Res<Settings>,
    player: Single<&Transform, With<Player>>,
    chunks: Query<(Entity, &ChunkCoord), With<Chunk>>,
) {
    let player_pos = player.translation;
    let current_player_chunk = ChunkCoord {
        x: (player_pos.x / CHUNK_WORLD).floor() as i32,
        y: (player_pos.y / CHUNK_WORLD).floor() as i32,
    };

    if let Some(chunk) = last_player_chunk.0
        && current_player_chunk == chunk
    {
        return;
    }

    last_player_chunk.0 = Some(current_player_chunk);

    let mut required = HashSet::new();

    let load_radius = settings.load_radius;

    for cx in (current_player_chunk.x - load_radius)..=(current_player_chunk.x + load_radius) {
        for cy in (current_player_chunk.y - load_radius)..=(current_player_chunk.y + load_radius) {
            required.insert(ChunkCoord { x: cx, y: cy });
        }
    }

    for coord in required.iter() {
        if !loaded.set.contains(coord) {
            writer.write(ChunkGenerateRequest(*coord));
            loaded.set.insert(*coord);
        }
    }

    for (entity, chunk) in &chunks {
        if !required.contains(chunk) {
            commands.entity(entity).despawn();
        }
    }

    loaded.set = required;
}
