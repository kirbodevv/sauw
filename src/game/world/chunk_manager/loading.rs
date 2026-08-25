use super::{ChunkBlocksStore, ChunkManager, PendingChunkSpawns, RequiredChunks};
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
    pending
        .set
        .retain(|coord| !manager.entities.contains_key(coord));

    let mut budget = config.max_chunk_ops_per_frame;

    for coord in required.ordered.iter() {
        if budget == 0 {
            break;
        }

        if manager.entities.contains_key(coord) || pending.set.contains(coord) {
            continue;
        }

        if let Some(blocks) = store.blocks.get(coord) {
            s_writer.write(SpawnChunk {
                chunk_coord: *coord,
                blocks: *blocks,
            });
        } else {
            g_writer.write(ChunkGenerateRequest(*coord));
        }

        pending.set.insert(*coord);
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
