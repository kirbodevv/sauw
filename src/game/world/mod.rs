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
        app.add_plugins((
            config::WorldConfigPlugin,
            chunk_manager::ChunkManagerPlugin,
            camera::CameraPlugin,
            generator::GeneratorPlugin,
            render::WorldRenderPlugin,
            time::TimePlugin,
        ));
    }
}
