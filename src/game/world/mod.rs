use bevy::prelude::*;

pub mod camera;
pub mod chunk_manager;
pub mod config;
pub mod generator;
pub mod physics;
pub mod render;
pub mod time;
pub mod types;

pub use types::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            physics::PhysicsSetupPlugin,
            config::WorldConfigPlugin,
            chunk_manager::ChunkManagerPlugin,
            camera::CameraPlugin,
            generator::GeneratorPlugin,
            render::WorldRenderPlugin,
            time::TimePlugin,
        ));
    }
}
