use bevy::prelude::*;

mod loading;
mod required;
mod resources;

pub use resources::*;

use crate::game::GameState;

pub struct ChunkManagerPlugin;

impl Plugin for ChunkManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadedChunks>()
            .init_resource::<ChunkManager>()
            .init_resource::<ChunkBlocksStore>()
            .init_resource::<RequiredChunks>()
            .add_systems(
                Update,
                (
                    required::compute_required_chunks,
                    loading::spawn_required_chunks,
                    loading::despawn_unrequired_chunks,
                )
                    .chain()
                    .run_if(in_state(GameState::Gaming)),
            );
    }
}
