use crate::game::GameState;
use bevy::prelude::*;

pub mod camera;
pub mod chunk_manager;
pub mod components;
pub mod config;
pub mod generator;
pub mod render;
pub mod time;

pub use components::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedChunks::default())
            .insert_resource(WorldSeed(0))
            .insert_resource(Settings { load_radius: 2 })
            .add_systems(Startup, config::configure_physics)
            .add_systems(
                Update,
                chunk_manager::manage_chunks.run_if(in_state(GameState::Gaming)),
            )
            .add_plugins((
                camera::CameraPlugin,
                generator::GeneratorPlugin,
                render::WorldRenderPlugin,
                time::TimePlugin,
            ));
    }
}
