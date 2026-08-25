use super::{ChunkBlocksStore, ChunkLoadState, ChunkManager, PendingChunkSpawns, RequiredChunks};
use crate::game::world::{
    Chunk, ChunkCoord,
    config::WorldConfig,
    generator::ChunkGenerateRequest,
    render::chunk_spawner::{DespawnChunk, SpawnChunk},
};
use bevy::prelude::*;

pub(super) fn spawn_required_chunks(
    mut g_writer: MessageWriter<ChunkGenerateRequest>,
    mut s_writer: MessageWriter<SpawnChunk>,
    mut pending: ResMut<PendingChunkSpawns>,
    manager: Res<ChunkManager>,
    store: Res<ChunkBlocksStore>,
    required: Res<RequiredChunks>,
    config: Res<WorldConfig>,
) {
    pending.state.retain(|coord, state| {
        if manager.is_spawned(coord) {
            return false;
        }

        if *state == ChunkLoadState::Generating
            && let Some(blocks) = store.blocks.get(coord)
        {
            s_writer.write(SpawnChunk {
                chunk_coord: *coord,
                blocks: *blocks,
            });
            *state = ChunkLoadState::AwaitingSpawn;
        }

        true
    });

    let mut budget = config.max_chunk_ops_per_frame;

    for coord in required.ordered.iter() {
        if budget == 0 {
            break;
        }

        if manager.is_spawned(coord) || pending.state.contains_key(coord) {
            continue;
        }

        if let Some(blocks) = store.blocks.get(coord) {
            s_writer.write(SpawnChunk {
                chunk_coord: *coord,
                blocks: *blocks,
            });
            pending.state.insert(*coord, ChunkLoadState::AwaitingSpawn);
        } else {
            g_writer.write(ChunkGenerateRequest(*coord));
            pending.state.insert(*coord, ChunkLoadState::Generating);
        }

        budget -= 1;
    }
}

pub(super) fn despawn_unrequired_chunks(
    mut d_writer: MessageWriter<DespawnChunk>,
    required: Res<RequiredChunks>,
    chunks: Query<(Entity, &ChunkCoord), With<Chunk>>,
) {
    for (_, chunk) in &chunks {
        if !required.set.contains(chunk) {
            d_writer.write(DespawnChunk {
                chunk_coord: *chunk,
            });
        }
    }
}
