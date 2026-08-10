use crate::game::{
    GameState,
    player::CurrentPlayerChunk,
    world::{
        components::{Chunk, ChunkCoord, LoadedChunks},
        config::WorldConfig,
        generator::ChunkGenerateRequest,
        render::chunk_spawner::DespawnChunk,
    },
};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Resource, Default)]
pub struct ChunkManager {
    pub entities: HashMap<ChunkCoord, Entity>,
}

#[derive(Resource, Default)]
pub struct RequiredChunks {
    pub set: HashSet<ChunkCoord>,
}

pub fn compute_required_chunks(
    mut required_chunks: ResMut<RequiredChunks>,
    player_chunk: Res<CurrentPlayerChunk>,
    config: Res<WorldConfig>,
) {
    if !player_chunk.is_changed() {
        return;
    }

    let Some(player_chunk) = player_chunk.0 else {
        return;
    };

    let load_radius = config.load_radius;

    let mut required = HashSet::new();
    for cx in (player_chunk.x - load_radius)..=(player_chunk.x + load_radius) {
        for cy in (player_chunk.y - load_radius)..=(player_chunk.y + load_radius) {
            required.insert(ChunkCoord { x: cx, y: cy });
        }
    }
    required_chunks.set = required;
}

pub fn emit_chunk_spawn_despawn(
    mut g_writer: MessageWriter<ChunkGenerateRequest>,
    mut d_writer: MessageWriter<DespawnChunk>,
    mut loaded: ResMut<LoadedChunks>,
    required: Res<RequiredChunks>,
    chunks: Query<(Entity, &ChunkCoord), With<Chunk>>,
) {
    for coord in required.set.iter() {
        if !loaded.set.contains(coord) {
            g_writer.write(ChunkGenerateRequest(*coord));
            loaded.set.insert(*coord);
        }
    }

    for (_, chunk) in &chunks {
        if !required.set.contains(chunk) {
            d_writer.write(DespawnChunk {
                chunk_coord: *chunk,
            });
        }
    }

    loaded.set = required.set.clone();
}

pub struct ChunkManagerPlugin;

impl Plugin for ChunkManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadedChunks>()
            .init_resource::<ChunkManager>()
            .init_resource::<RequiredChunks>()
            .add_systems(
                Update,
                (compute_required_chunks, emit_chunk_spawn_despawn)
                    .chain()
                    .run_if(in_state(GameState::Gaming)),
            );
    }
}
