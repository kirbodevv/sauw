use bevy::prelude::*;

use crate::{
    constants::{CHUNK_SIZE, CHUNK_VOLUME},
    game::{
        GameState,
        registry::block_registry::BlockId,
        world::{
            ChunkCoord,
            generator::{generator::generate_chunk, spawner::spawn_chunk},
        },
    },
};

pub mod generator;
pub mod spawner;

#[inline]
pub fn idx(x: usize, y: usize, layer: usize) -> usize {
    x + y * CHUNK_SIZE + layer * CHUNK_SIZE * CHUNK_SIZE
}

#[derive(Message)]
pub struct ChunkGenerateRequest(pub ChunkCoord);

#[derive(Message)]
pub struct GeneratedChunk {
    pub chunk_coord: ChunkCoord,
    pub blocks: [BlockId; CHUNK_VOLUME],
}

pub struct GeneratorPlugin;

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ChunkGenerateRequest>()
            .add_message::<GeneratedChunk>()
            .add_systems(
                Update,
                (generate_chunk, spawn_chunk)
                    .chain()
                    .run_if(in_state(GameState::Gaming)),
            );
    }
}
