use crate::{
    constants::CHUNK_WORLD,
    game::{
        player::{CurrentPlayerChunk, Player},
        world::{
            components::{Chunk, ChunkCoord, LoadedChunks, Settings},
            generator::ChunkGenerateRequest,
            render::chunk_spawner::DespawnChunk,
        },
    },
};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Resource, Default)]
pub struct ChunkManager {
    pub entities: HashMap<ChunkCoord, Entity>,
}

pub fn manage_chunks(
    mut g_writer: MessageWriter<ChunkGenerateRequest>,
    mut d_writer: MessageWriter<DespawnChunk>,
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
            g_writer.write(ChunkGenerateRequest(*coord));
            loaded.set.insert(*coord);
        }
    }

    for (_, chunk) in &chunks {
        if !required.contains(chunk) {
            d_writer.write(DespawnChunk {
                chunk_coord: *chunk,
            });
        }
    }

    loaded.set = required;
}

pub struct ChunkManagerPlugin;

impl Plugin for ChunkManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkManager::default());
    }
}
