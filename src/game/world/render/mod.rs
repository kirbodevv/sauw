use bevy::prelude::*;

use crate::game::GameState;

pub mod chunk_mesh;
pub mod chunk_spawner;
pub mod y_sort;

pub use y_sort::*;

pub struct WorldRenderPlugin;

impl Plugin for WorldRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (chunk_spawner::spawn_chunk, y_sort::apply_y_sort).run_if(in_state(GameState::Gaming)),
        );
    }
}
