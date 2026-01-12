use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

use crate::game::{
    GameState,
    registry::GameRegistry,
    world::{
        camera::CameraPlugin,
        resources::{LoadedChunks, WorldSeed},
        systems::manage_chunks,
    },
};

pub mod block;
pub mod camera;
pub mod components;
pub mod generator;
pub mod resources;
pub mod systems;

fn configure_physics(mut rapier_config: Query<&mut RapierConfiguration>) {
    let Ok(mut rapier_config) = rapier_config.single_mut() else {
        return;
    };
    rapier_config.gravity = Vec2::ZERO;
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedChunks::new())
            .insert_resource(WorldSeed(0))
            .add_systems(Startup, configure_physics)
            .add_systems(
                Update,
                manage_chunks
                    .run_if(in_state(GameState::Gaming))
                    .run_if(resource_exists::<GameRegistry>),
            )
            .add_plugins(CameraPlugin);
    }
}
