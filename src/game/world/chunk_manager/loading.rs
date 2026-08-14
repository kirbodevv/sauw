use super::RequiredChunks;
use crate::game::world::{
    components::{Chunk, ChunkCoord, LoadedChunks},
    generator::ChunkGenerateRequest,
    render::chunk_spawner::DespawnChunk,
};
use bevy::prelude::*;

pub(super) fn spawn_required_chunks(
    mut g_writer: MessageWriter<ChunkGenerateRequest>,
    mut loaded: ResMut<LoadedChunks>,
    required: Res<RequiredChunks>,
) {
    for coord in required.set.iter() {
        if !loaded.set.contains(coord) {
            g_writer.write(ChunkGenerateRequest(*coord));
            loaded.set.insert(*coord);
        }
    }
    loaded.set = required.set.clone();
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
