use bevy::prelude::*;

use crate::game::{
    GameState,
    world::{
        ChunkCoord,
        generator::{chunk::generate_chunk, noise::init_noise},
    },
};

pub mod chunk;
pub mod chunk_data;
pub mod mappers;
pub mod noise;

#[derive(Message)]
pub struct ChunkGenerateRequest(pub ChunkCoord);

pub struct GeneratorPlugin;

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ChunkGenerateRequest>()
            .add_systems(OnEnter(GameState::Gaming), init_noise)
            .add_systems(Update, generate_chunk.run_if(in_state(GameState::Gaming)))
            .add_plugins(mappers::MappersPlugin);
    }
}
