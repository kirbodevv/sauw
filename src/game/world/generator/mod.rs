use bevy::prelude::*;

use crate::game::{
    GameState,
    world::{
        ChunkCoord,
        generator::{
            chunk::generate_chunk,
            context::{GenerationContextHandle, init_generation_context},
            noise::init_noise,
        },
    },
};

pub mod chunk;
pub mod chunk_data;
pub mod context;
pub mod mappers;
pub mod noise;

#[derive(Message)]
pub struct ChunkGenerateRequest(pub ChunkCoord);

pub struct GeneratorPlugin;

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ChunkGenerateRequest>()
            .add_systems(OnEnter(GameState::Gaming), init_noise)
            .add_systems(
                Update,
                (
                    init_generation_context.run_if(not(resource_exists::<GenerationContextHandle>)),
                    generate_chunk.run_if(resource_exists::<GenerationContextHandle>),
                )
                    .chain()
                    .run_if(in_state(GameState::Gaming)),
            )
            .add_plugins(mappers::MappersPlugin);
    }
}
