use bevy::prelude::*;

use crate::{
    constants::{CHUNK_SIZE, CHUNK_VOLUME},
    game::{
        GameState,
        registry::block_registry::BlockId,
        world::{
            ChunkCoord,
            generator::{
                chunk::generate_chunk,
                mappers::{init_biome_mapper, init_layer_mapper},
                noise::init_noise,
            },
        },
    },
};

pub mod chunk;
pub mod chunk_data;
pub mod mappers;
pub mod noise;

#[inline]
pub fn idx(x: usize, y: usize, layer: usize) -> usize {
    x + y * CHUNK_SIZE + layer * CHUNK_SIZE * CHUNK_SIZE
}

#[inline]
pub fn idx_2d(x: usize, y: usize) -> usize {
    x + y * CHUNK_SIZE
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
                OnEnter(GameState::Gaming),
                (init_noise, init_layer_mapper, init_biome_mapper).chain(),
            )
            .add_systems(Update, generate_chunk.run_if(in_state(GameState::Gaming)));
    }
}
