use std::collections::HashSet;

use bevy::prelude::*;

use crate::{
    constants::CHUNK_WORLD,
    game::{
        player::{CurrentPlayerChunk, Player},
        resources::{GameRegistry, Textures},
        world::{
            components::{BelongsToChunk, ChunkCoord},
            generator::spawn_chunk,
            resources::{LoadedChunks, Settings, WorldSeed},
        },
    },
};

pub fn manage_chunks(
    mut commands: Commands,
    mut loaded: ResMut<LoadedChunks>,
    mut last_player_chunk: ResMut<CurrentPlayerChunk>,
    registry: Res<GameRegistry>,
    textures: Res<Textures>,
    seed: Res<WorldSeed>,
    settings: Res<Settings>,
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

    let load_radius = settings.load_radius;

    for cx in (current_player_chunk.x - load_radius)..=(current_player_chunk.x + load_radius) {
        for cy in (current_player_chunk.y - load_radius)..=(current_player_chunk.y + load_radius) {
            required.insert(ChunkCoord { x: cx, y: cy });
        }
    }

    for coord in required.iter() {
        if !loaded.set.contains(coord) {
            spawn_chunk(&mut commands, &registry, &textures, *coord, seed.0);
            loaded.set.insert(*coord);
        }
    }

    for (entity, belongs) in &tiles_q {
        if !required.contains(&belongs.0) {
            commands.entity(entity).despawn();
        }
    }

    loaded.set = required;
}
