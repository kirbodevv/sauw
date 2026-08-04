use bevy::prelude::*;

use crate::game::{GameState, world::render::chunk_spawner::spawn_chunk};

pub mod chunk_mesh;
pub mod chunk_spawner;

pub struct WorldRenderPlugin;

impl Plugin for WorldRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_chunk.run_if(in_state(GameState::Gaming)));
    }
}
